//! Async runtime bridging Fusabi VM to Tokio
//!
//! This module provides a runtime for executing async tasks using Tokio.

use crate::async_types::{AsyncState, TaskId};
use crate::{Value, VmError};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

/// Command sent from VM to async runtime
enum RuntimeCommand {
    /// Spawn a new async task
    Spawn {
        id: TaskId,
        task: Box<dyn FnOnce() -> Result<Value, VmError> + Send + 'static>,
    },
    /// Cancel a running task
    Cancel { id: TaskId },
    /// Shutdown the runtime
    Shutdown,
}

/// Response from runtime to VM
enum RuntimeResponse {
    /// Task completed
    Completed { id: TaskId, result: AsyncState },
}

/// AsyncRuntime manages the Tokio runtime in a dedicated thread
#[derive(Debug)]
pub struct AsyncRuntime {
    /// Channel for sending commands to runtime thread
    command_tx: mpsc::UnboundedSender<RuntimeCommand>,
    /// Shared state for task results
    task_states: Arc<Mutex<HashMap<TaskId, AsyncState>>>,
    /// Handle to runtime thread
    _runtime_handle: thread::JoinHandle<()>,
}

impl AsyncRuntime {
    /// Create a new AsyncRuntime with dedicated Tokio runtime
    pub fn new() -> Result<Self, VmError> {
        let (command_tx, mut command_rx) = mpsc::unbounded_channel::<RuntimeCommand>();
        let (response_tx, mut response_rx) = mpsc::unbounded_channel::<RuntimeResponse>();
        let task_states = Arc::new(Mutex::new(HashMap::new()));
        let task_states_clone = Arc::clone(&task_states);

        // Spawn the runtime thread
        let runtime_handle = thread::spawn(move || {
            // Create Tokio runtime
            let rt = Runtime::new().expect("Failed to create Tokio runtime");

            // Spawn task to handle responses
            let states = Arc::clone(&task_states_clone);
            rt.spawn(async move {
                while let Some(response) = response_rx.recv().await {
                    match response {
                        RuntimeResponse::Completed { id, result } => {
                            states.lock().unwrap().insert(id, result);
                        }
                    }
                }
            });

            // Main event loop
            rt.block_on(async {
                let mut tasks = HashMap::new();

                while let Some(command) = command_rx.recv().await {
                    match command {
                        RuntimeCommand::Spawn { id, task } => {
                            let response_tx = response_tx.clone();
                            let handle = tokio::spawn(async move {
                                let result = match tokio::task::spawn_blocking(task).await
                                {
                                    Ok(Ok(value)) => AsyncState::Ready(value),
                                    Ok(Err(e)) => AsyncState::Failed(format!("{}", e)),
                                    Err(e) => AsyncState::Failed(format!("Task panicked: {}", e)),
                                };

                                let _ = response_tx.send(RuntimeResponse::Completed { id, result });
                            });

                            tasks.insert(id, handle);
                        }
                        RuntimeCommand::Cancel { id } => {
                            if let Some(handle) = tasks.remove(&id) {
                                handle.abort();
                                let _ =
                                    response_tx.send(RuntimeResponse::Completed {
                                        id,
                                        result: AsyncState::Cancelled,
                                    });
                            }
                        }
                        RuntimeCommand::Shutdown => {
                            // Cancel all running tasks
                            for (_, handle) in tasks.drain() {
                                handle.abort();
                            }
                            break;
                        }
                    }
                }
            });
        });

        Ok(AsyncRuntime {
            command_tx,
            task_states,
            _runtime_handle: runtime_handle,
        })
    }

    /// Spawn an async task and return its TaskId
    pub fn spawn<F>(&self, task: F) -> TaskId
    where
        F: FnOnce() -> Result<Value, VmError> + Send + 'static,
    {
        let id = TaskId::new();
        let boxed_task = Box::new(task);

        // Mark as pending
        self.task_states
            .lock()
            .unwrap()
            .insert(id, AsyncState::Pending);

        // Send spawn command
        let _ = self.command_tx.send(RuntimeCommand::Spawn {
            id,
            task: boxed_task,
        });

        id
    }

    /// Non-blocking poll for task status
    pub fn poll(&self, task_id: TaskId) -> AsyncState {
        self.task_states
            .lock()
            .unwrap()
            .get(&task_id)
            .cloned()
            .unwrap_or(AsyncState::Failed(format!("Unknown task: {}", task_id)))
    }

    /// Block until task completes and return result
    pub fn block_on(&self, task_id: TaskId) -> Result<Value, VmError> {
        loop {
            let state = self.poll(task_id);
            match state {
                AsyncState::Pending => {
                    // Sleep briefly to avoid busy-waiting
                    thread::sleep(std::time::Duration::from_millis(1));
                }
                AsyncState::Ready(value) => {
                    // Clean up task state
                    self.task_states.lock().unwrap().remove(&task_id);
                    return Ok(value);
                }
                AsyncState::Failed(err) => {
                    // Clean up task state
                    self.task_states.lock().unwrap().remove(&task_id);
                    return Err(VmError::Runtime(err));
                }
                AsyncState::Cancelled => {
                    // Clean up task state
                    self.task_states.lock().unwrap().remove(&task_id);
                    return Err(VmError::Runtime("Task was cancelled".to_string()));
                }
            }
        }
    }

    /// Cancel a running task
    pub fn cancel(&self, task_id: TaskId) -> Result<(), VmError> {
        let _ = self.command_tx.send(RuntimeCommand::Cancel { id: task_id });
        Ok(())
    }
}

impl Default for AsyncRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create AsyncRuntime")
    }
}

impl Drop for AsyncRuntime {
    fn drop(&mut self) {
        // Send shutdown command
        let _ = self.command_tx.send(RuntimeCommand::Shutdown);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_and_block_on() {
        let runtime = AsyncRuntime::new().unwrap();

        // Spawn a simple task
        let task_id = runtime.spawn(|| Ok(Value::Int(42)));

        // Block until completion
        let result = runtime.block_on(task_id).unwrap();
        assert_eq!(result, Value::Int(42));
    }

    #[test]
    fn test_parallel_execution() {
        let runtime = AsyncRuntime::new().unwrap();

        // Spawn multiple tasks
        let task1 = runtime.spawn(|| {
            thread::sleep(std::time::Duration::from_millis(10));
            Ok(Value::Int(1))
        });

        let task2 = runtime.spawn(|| {
            thread::sleep(std::time::Duration::from_millis(10));
            Ok(Value::Int(2))
        });

        let task3 = runtime.spawn(|| {
            thread::sleep(std::time::Duration::from_millis(10));
            Ok(Value::Int(3))
        });

        // Wait for all tasks
        let result1 = runtime.block_on(task1).unwrap();
        let result2 = runtime.block_on(task2).unwrap();
        let result3 = runtime.block_on(task3).unwrap();

        assert_eq!(result1, Value::Int(1));
        assert_eq!(result2, Value::Int(2));
        assert_eq!(result3, Value::Int(3));
    }

    #[test]
    fn test_poll_pending() {
        let runtime = AsyncRuntime::new().unwrap();

        // Spawn a long-running task
        let task_id = runtime.spawn(|| {
            thread::sleep(std::time::Duration::from_millis(100));
            Ok(Value::Int(42))
        });

        // Poll immediately - should be pending
        let state = runtime.poll(task_id);
        assert!(matches!(state, AsyncState::Pending));

        // Wait for completion
        let result = runtime.block_on(task_id).unwrap();
        assert_eq!(result, Value::Int(42));
    }

    #[test]
    fn test_task_error() {
        let runtime = AsyncRuntime::new().unwrap();

        // Spawn a task that fails
        let task_id = runtime.spawn(|| Err(VmError::Runtime("Task failed".to_string())));

        // Should return error
        let result = runtime.block_on(task_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_cancellation() {
        let runtime = AsyncRuntime::new().unwrap();

        // Spawn a long-running task
        let task_id = runtime.spawn(|| {
            thread::sleep(std::time::Duration::from_secs(10));
            Ok(Value::Int(42))
        });

        // Cancel the task
        runtime.cancel(task_id).unwrap();

        // Wait a bit for cancellation to process
        thread::sleep(std::time::Duration::from_millis(50));

        // Poll should show cancelled
        let state = runtime.poll(task_id);
        assert!(matches!(state, AsyncState::Cancelled));
    }

    #[test]
    fn test_unknown_task() {
        let runtime = AsyncRuntime::new().unwrap();

        // Poll non-existent task
        let fake_id = TaskId::new();
        let state = runtime.poll(fake_id);
        assert!(matches!(state, AsyncState::Failed(_)));
    }
}
