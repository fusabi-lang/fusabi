# RFC-004: Async Tokio Integration

**Status**: Draft
**Author**: Claude (AI Assistant)
**Created**: 2025-12-14
**Requires**: RFC-002 (Async Computation Expressions)

## Summary

This RFC proposes integrating Fusabi's async computation expressions with the Tokio runtime, enabling real non-blocking I/O operations. This is critical for building responsive TUI applications with proper event loops.

## Motivation

### Current Limitation

RFC-002 defined async syntax, but operations are simulated:

```fsharp
// Current: Uses shell command, blocks thread
let sleep ms = async {
    do! shellExec (sprintf "sleep %f" (float ms / 1000.0))
    return ()
}
```

### Problems Solved

1. **Real Async I/O**: HTTP requests, file operations, timers without blocking
2. **Responsive TUI**: Event loops that don't freeze during I/O
3. **Concurrent Operations**: Run multiple async tasks in parallel
4. **Interop**: Integrate with Rust async ecosystem (reqwest, tokio-fs, etc.)
5. **Cancellation**: Cancel long-running operations gracefully

### Use Cases

#### Non-Blocking TUI Event Loop
```fsharp
let rec eventLoop model = async {
    // Non-blocking: yields to Tokio scheduler
    let! event = Events.nextAsync ()

    match event with
    | Tick -> do! render model
    | Key k -> return! eventLoop (update k model)
    | Quit -> return ()
}
```

#### Parallel API Calls
```fsharp
let fetchDashboard () = async {
    let! (metrics, alerts, logs) = Async.parallel3
        (Api.getMetrics ())
        (Api.getAlerts ())
        (Api.getLogs ())

    return { metrics; alerts; logs }
}
```

#### HTTP with Timeout
```fsharp
let fetchWithTimeout url timeoutMs = async {
    let! result = Async.withTimeout timeoutMs (Http.getAsync url)
    match result with
    | Some response -> return Ok response
    | None -> return Error "Timeout"
}
```

## Design

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Fusabi Script (.fsx)                     │
│  async { let! x = Http.getAsync url; return x }            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Fusabi VM (fusabi-vm)                    │
│  AsyncValue::Pending(task_id) / AsyncValue::Ready(value)   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│               Async Runtime (fusabi-vm/async)               │
│  AsyncRuntime { tokio_handle, tasks, channels }            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Tokio Runtime                            │
│  tokio::runtime::Runtime::new_multi_thread()               │
└─────────────────────────────────────────────────────────────┘
```

### Core Types

```rust
// crates/fusabi-vm/src/async_runtime.rs

use tokio::sync::{mpsc, oneshot};
use std::collections::HashMap;

/// Unique identifier for async tasks
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct TaskId(u64);

/// The state of an async computation
#[derive(Debug, Clone)]
pub enum AsyncState {
    /// Task is still running
    Pending,
    /// Task completed with a value
    Ready(Value),
    /// Task failed with an error
    Failed(String),
    /// Task was cancelled
    Cancelled,
}

/// Async value in the VM
#[derive(Debug, Clone)]
pub enum AsyncValue {
    /// Reference to a running task
    Task(TaskId),
    /// Completed value
    Value(Value),
}

/// Message types for async runtime communication
pub enum AsyncMessage {
    /// Spawn a new async task
    Spawn {
        task: Box<dyn FnOnce() -> BoxFuture<'static, Value> + Send>,
        reply: oneshot::Sender<TaskId>,
    },
    /// Poll task status
    Poll {
        task_id: TaskId,
        reply: oneshot::Sender<AsyncState>,
    },
    /// Cancel a task
    Cancel {
        task_id: TaskId,
    },
    /// Shutdown the runtime
    Shutdown,
}

/// The async runtime that bridges Fusabi VM to Tokio
pub struct AsyncRuntime {
    /// Handle to send messages to the runtime thread
    sender: mpsc::Sender<AsyncMessage>,
    /// Next task ID to assign
    next_id: AtomicU64,
}

impl AsyncRuntime {
    /// Create a new async runtime
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(1024);

        // Spawn the Tokio runtime in a dedicated thread
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(4)
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime");

            rt.block_on(async move {
                Self::run_event_loop(receiver).await;
            });
        });

        Self {
            sender,
            next_id: AtomicU64::new(0),
        }
    }

    /// Spawn an async task, returns TaskId immediately
    pub fn spawn<F, Fut>(&self, f: F) -> TaskId
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Value> + Send + 'static,
    {
        let id = TaskId(self.next_id.fetch_add(1, Ordering::SeqCst));
        let (reply_tx, reply_rx) = oneshot::channel();

        let task = Box::new(move || Box::pin(f()) as BoxFuture<'static, Value>);

        self.sender.blocking_send(AsyncMessage::Spawn {
            task,
            reply: reply_tx,
        }).expect("Runtime channel closed");

        id
    }

    /// Poll task status (non-blocking)
    pub fn poll(&self, task_id: TaskId) -> AsyncState {
        let (reply_tx, reply_rx) = oneshot::channel();

        self.sender.blocking_send(AsyncMessage::Poll {
            task_id,
            reply: reply_tx,
        }).expect("Runtime channel closed");

        reply_rx.blocking_recv().unwrap_or(AsyncState::Failed("Channel closed".into()))
    }

    /// Block until task completes
    pub fn block_on(&self, task_id: TaskId) -> Value {
        loop {
            match self.poll(task_id) {
                AsyncState::Ready(value) => return value,
                AsyncState::Failed(err) => panic!("Async task failed: {}", err),
                AsyncState::Cancelled => panic!("Async task was cancelled"),
                AsyncState::Pending => {
                    std::thread::sleep(Duration::from_micros(100));
                }
            }
        }
    }

    /// Cancel a running task
    pub fn cancel(&self, task_id: TaskId) {
        let _ = self.sender.blocking_send(AsyncMessage::Cancel { task_id });
    }

    async fn run_event_loop(mut receiver: mpsc::Receiver<AsyncMessage>) {
        let mut tasks: HashMap<TaskId, JoinHandle<Value>> = HashMap::new();
        let mut results: HashMap<TaskId, AsyncState> = HashMap::new();

        while let Some(msg) = receiver.recv().await {
            match msg {
                AsyncMessage::Spawn { task, reply } => {
                    let id = TaskId(/* assigned by caller */);
                    let handle = tokio::spawn(async move { task().await });
                    tasks.insert(id, handle);
                    let _ = reply.send(id);
                }
                AsyncMessage::Poll { task_id, reply } => {
                    let state = if let Some(result) = results.get(&task_id) {
                        result.clone()
                    } else if let Some(handle) = tasks.get(&task_id) {
                        if handle.is_finished() {
                            match tasks.remove(&task_id).unwrap().await {
                                Ok(value) => {
                                    let state = AsyncState::Ready(value);
                                    results.insert(task_id, state.clone());
                                    state
                                }
                                Err(e) => AsyncState::Failed(e.to_string()),
                            }
                        } else {
                            AsyncState::Pending
                        }
                    } else {
                        AsyncState::Failed("Unknown task".into())
                    };
                    let _ = reply.send(state);
                }
                AsyncMessage::Cancel { task_id } => {
                    if let Some(handle) = tasks.remove(&task_id) {
                        handle.abort();
                        results.insert(task_id, AsyncState::Cancelled);
                    }
                }
                AsyncMessage::Shutdown => break,
            }
        }
    }
}
```

### VM Integration

```rust
// crates/fusabi-vm/src/value.rs

#[derive(Debug, Clone)]
pub enum Value {
    // ... existing variants ...

    /// Async computation reference
    Async(AsyncValue),
}

// crates/fusabi-vm/src/vm.rs

impl Vm {
    /// Execute an async operation, returns immediately with TaskId
    pub fn exec_async<F>(&mut self, f: F) -> Value
    where
        F: FnOnce() -> BoxFuture<'static, Value> + Send + 'static,
    {
        let task_id = self.async_runtime.spawn(f);
        Value::Async(AsyncValue::Task(task_id))
    }

    /// Await an async value (may block or yield)
    pub fn await_async(&mut self, value: Value) -> Value {
        match value {
            Value::Async(AsyncValue::Task(task_id)) => {
                // In sync context, block. In async context, yield.
                self.async_runtime.block_on(task_id)
            }
            Value::Async(AsyncValue::Value(v)) => *v,
            other => other, // Not async, return as-is
        }
    }
}
```

### Standard Library Functions

```rust
// crates/fusabi-vm/src/stdlib/async_ops.rs

use reqwest;
use tokio::time::{sleep, Duration};
use tokio::fs;

pub fn register_async_stdlib(vm: &mut Vm) {
    // Async.sleep : int -> Async<unit>
    vm.register_fn("Async.sleep", |vm, args| {
        let ms = args[0].as_int()?;
        vm.exec_async(move || async move {
            sleep(Duration::from_millis(ms as u64)).await;
            Value::Unit
        })
    });

    // Async.delay : int -> (unit -> 'a) -> Async<'a>
    vm.register_fn("Async.delay", |vm, args| {
        let ms = args[0].as_int()?;
        let f = args[1].as_closure()?;
        vm.exec_async(move || async move {
            sleep(Duration::from_millis(ms as u64)).await;
            f.call(vec![Value::Unit])
        })
    });

    // Http.getAsync : string -> Async<string>
    vm.register_fn("Http.getAsync", |vm, args| {
        let url = args[0].as_string()?.to_string();
        vm.exec_async(move || async move {
            match reqwest::get(&url).await {
                Ok(resp) => match resp.text().await {
                    Ok(body) => Value::String(body),
                    Err(e) => Value::Error(e.to_string()),
                }
                Err(e) => Value::Error(e.to_string()),
            }
        })
    });

    // Http.postAsync : string -> string -> Async<string>
    vm.register_fn("Http.postAsync", |vm, args| {
        let url = args[0].as_string()?.to_string();
        let body = args[1].as_string()?.to_string();
        vm.exec_async(move || async move {
            let client = reqwest::Client::new();
            match client.post(&url).body(body).send().await {
                Ok(resp) => match resp.text().await {
                    Ok(body) => Value::String(body),
                    Err(e) => Value::Error(e.to_string()),
                }
                Err(e) => Value::Error(e.to_string()),
            }
        })
    });

    // File.readAsync : string -> Async<string>
    vm.register_fn("File.readAsync", |vm, args| {
        let path = args[0].as_string()?.to_string();
        vm.exec_async(move || async move {
            match fs::read_to_string(&path).await {
                Ok(contents) => Value::String(contents),
                Err(e) => Value::Error(e.to_string()),
            }
        })
    });

    // File.writeAsync : string -> string -> Async<unit>
    vm.register_fn("File.writeAsync", |vm, args| {
        let path = args[0].as_string()?.to_string();
        let contents = args[1].as_string()?.to_string();
        vm.exec_async(move || async move {
            match fs::write(&path, &contents).await {
                Ok(()) => Value::Unit,
                Err(e) => Value::Error(e.to_string()),
            }
        })
    });

    // Async.parallel : Async<'a> list -> Async<'a list>
    vm.register_fn("Async.parallel", |vm, args| {
        let tasks: Vec<TaskId> = args[0].as_list()?
            .iter()
            .filter_map(|v| match v {
                Value::Async(AsyncValue::Task(id)) => Some(*id),
                _ => None,
            })
            .collect();

        vm.exec_async(move || async move {
            let mut results = Vec::with_capacity(tasks.len());
            for task_id in tasks {
                // In a real impl, we'd use join_all
                results.push(vm.async_runtime.block_on(task_id));
            }
            Value::List(results)
        })
    });

    // Async.withTimeout : int -> Async<'a> -> Async<Option<'a>>
    vm.register_fn("Async.withTimeout", |vm, args| {
        let timeout_ms = args[0].as_int()?;
        let task_id = match &args[1] {
            Value::Async(AsyncValue::Task(id)) => *id,
            _ => return Err("Expected async task".into()),
        };

        vm.exec_async(move || async move {
            tokio::select! {
                result = async {
                    loop {
                        match vm.async_runtime.poll(task_id) {
                            AsyncState::Ready(v) => break v,
                            AsyncState::Pending => tokio::task::yield_now().await,
                            _ => break Value::None,
                        }
                    }
                } => Value::Some(Box::new(result)),
                _ = sleep(Duration::from_millis(timeout_ms as u64)) => {
                    vm.async_runtime.cancel(task_id);
                    Value::None
                }
            }
        })
    });

    // Async.catch : Async<'a> -> Async<Result<'a, string>>
    vm.register_fn("Async.catch", |vm, args| {
        let task_id = match &args[0] {
            Value::Async(AsyncValue::Task(id)) => *id,
            _ => return Err("Expected async task".into()),
        };

        vm.exec_async(move || async move {
            match vm.async_runtime.poll(task_id) {
                AsyncState::Ready(v) => Value::Ok(Box::new(v)),
                AsyncState::Failed(e) => Value::Error(e),
                AsyncState::Cancelled => Value::Error("Cancelled".into()),
                AsyncState::Pending => {
                    // Wait for completion
                    let result = vm.async_runtime.block_on(task_id);
                    Value::Ok(Box::new(result))
                }
            }
        })
    });
}
```

### Channel Support

```rust
// crates/fusabi-vm/src/stdlib/channels.rs

use tokio::sync::mpsc;

pub fn register_channel_stdlib(vm: &mut Vm) {
    // Channel.create : unit -> (Sender<'a> * Receiver<'a>)
    vm.register_fn("Channel.create", |vm, _args| {
        let (tx, rx) = mpsc::channel::<Value>(100);
        Value::Tuple(vec![
            Value::ChannelSender(tx),
            Value::ChannelReceiver(rx),
        ])
    });

    // Channel.send : Sender<'a> -> 'a -> Async<unit>
    vm.register_fn("Channel.send", |vm, args| {
        let tx = args[0].as_channel_sender()?;
        let value = args[1].clone();
        vm.exec_async(move || async move {
            match tx.send(value).await {
                Ok(()) => Value::Unit,
                Err(e) => Value::Error(e.to_string()),
            }
        })
    });

    // Channel.receive : Receiver<'a> -> Async<Option<'a>>
    vm.register_fn("Channel.receive", |vm, args| {
        let mut rx = args[0].as_channel_receiver()?;
        vm.exec_async(move || async move {
            match rx.recv().await {
                Some(value) => Value::Some(Box::new(value)),
                None => Value::None,
            }
        })
    });
}
```

## Fusabi API

### Core Async Module

```fsharp
module Async =
    /// Sleep for specified milliseconds
    val sleep : int -> Async<unit>

    /// Delay execution of function
    val delay : int -> (unit -> 'a) -> Async<'a>

    /// Run async synchronously (blocks current thread)
    val runSynchronously : Async<'a> -> 'a

    /// Start async without waiting (fire and forget)
    val start : Async<unit> -> unit

    /// Run multiple async operations in parallel
    val parallel : Async<'a> list -> Async<'a list>

    /// Run two async operations in parallel
    val parallel2 : Async<'a> -> Async<'b> -> Async<('a * 'b)>

    /// Run three async operations in parallel
    val parallel3 : Async<'a> -> Async<'b> -> Async<'c> -> Async<('a * 'b * 'c)>

    /// Add timeout to async operation
    val withTimeout : int -> Async<'a> -> Async<Option<'a>>

    /// Catch exceptions in async
    val catch : Async<'a> -> Async<Result<'a, string>>

    /// Cancel async operation
    val cancel : Async<'a> -> unit

    /// Ignore result of async
    val ignore : Async<'a> -> Async<unit>
```

### HTTP Module

```fsharp
module Http =
    /// GET request
    val getAsync : string -> Async<string>

    /// POST request with body
    val postAsync : string -> string -> Async<string>

    /// GET with headers
    val getWithHeadersAsync : string -> (string * string) list -> Async<string>

    /// POST JSON
    val postJsonAsync : string -> 'a -> Async<string>
```

### File Module

```fsharp
module File =
    /// Read file contents
    val readAsync : string -> Async<string>

    /// Write file contents
    val writeAsync : string -> string -> Async<unit>

    /// Append to file
    val appendAsync : string -> string -> Async<unit>

    /// Check if file exists
    val existsAsync : string -> Async<bool>

    /// Delete file
    val deleteAsync : string -> Async<unit>
```

### Channel Module

```fsharp
module Channel =
    /// Create unbounded channel
    val create : unit -> (Sender<'a> * Receiver<'a>)

    /// Create bounded channel
    val bounded : int -> (Sender<'a> * Receiver<'a>)

    /// Send value to channel
    val send : Sender<'a> -> 'a -> Async<unit>

    /// Receive from channel
    val receive : Receiver<'a> -> Async<Option<'a>>

    /// Try receive (non-blocking)
    val tryReceive : Receiver<'a> -> Option<'a>
```

## Examples

### Example 1: Async HTTP Client

```fsharp
let fetchJson url = async {
    let! response = Http.getAsync url
    return Json.parse response
}

let fetchAllUsers () = async {
    let urls = [
        "https://api.example.com/users/1"
        "https://api.example.com/users/2"
        "https://api.example.com/users/3"
    ]

    let! results = Async.parallel (List.map fetchJson urls)
    return results
}

// Usage
let users = Async.runSynchronously (fetchAllUsers ())
```

### Example 2: TUI Event Loop with Async

```fsharp
type Model = { count: int; running: bool }

type Msg =
    | Tick
    | Increment
    | Decrement
    | Quit

let update msg model =
    match msg with
    | Tick -> model
    | Increment -> { model with count = model.count + 1 }
    | Decrement -> { model with count = model.count - 1 }
    | Quit -> { model with running = false }

let rec eventLoop model = async {
    // Render current state
    do! render model

    // Wait for next event (non-blocking with timeout)
    let! eventOpt = Async.withTimeout 100 (Events.nextAsync ())

    match eventOpt with
    | Some event ->
        let msg = match event with
            | Key 'q' -> Quit
            | Key '+' -> Increment
            | Key '-' -> Decrement
            | _ -> Tick
        let newModel = update msg model
        if newModel.running then
            return! eventLoop newModel
        else
            return ()
    | None ->
        // Timeout, just tick
        let newModel = update Tick model
        if newModel.running then
            return! eventLoop newModel
        else
            return ()
}

let main () =
    let initialModel = { count = 0; running = true }
    Async.runSynchronously (eventLoop initialModel)
```

### Example 3: Parallel Data Fetching for Dashboard

```fsharp
let fetchDashboardData () = async {
    // Fetch all data sources in parallel
    let! (metrics, alerts, logs) = Async.parallel3
        (Api.getMetricsAsync ())
        (Api.getAlertsAsync ())
        (Api.getLogsAsync ())

    return {
        metrics = metrics
        alerts = alerts
        logs = logs
        timestamp = DateTime.now ()
    }
}

let refreshLoop interval = async {
    while true do
        let! data = fetchDashboardData ()
        do! updateUI data
        do! Async.sleep interval
}

// Start background refresh
Async.start (refreshLoop 5000)
```

### Example 4: Producer-Consumer with Channels

```fsharp
let producer (tx: Sender<int>) = async {
    for i in 1..100 do
        do! Channel.send tx i
        do! Async.sleep 10
}

let consumer (rx: Receiver<int>) = async {
    let mutable sum = 0
    let rec loop () = async {
        let! msgOpt = Channel.receive rx
        match msgOpt with
        | Some n ->
            sum <- sum + n
            return! loop ()
        | None ->
            return sum
    }
    return! loop ()
}

let main () = async {
    let (tx, rx) = Channel.create ()

    // Start producer and consumer in parallel
    Async.start (producer tx)
    let! total = consumer rx

    printfn "Total: %d" total
}
```

## Feature Flags

```toml
# Cargo.toml for fusabi-vm
[features]
default = ["async"]
async = ["tokio", "reqwest"]
async-full = ["async", "tokio/full"]
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_sleep() {
        let runtime = AsyncRuntime::new();
        let start = std::time::Instant::now();

        let task = runtime.spawn(|| async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            Value::Unit
        });

        runtime.block_on(task);
        let elapsed = start.elapsed();

        assert!(elapsed >= Duration::from_millis(100));
        assert!(elapsed < Duration::from_millis(200));
    }

    #[tokio::test]
    async fn test_async_parallel() {
        let runtime = AsyncRuntime::new();

        let task1 = runtime.spawn(|| async { Value::Int(1) });
        let task2 = runtime.spawn(|| async { Value::Int(2) });

        let r1 = runtime.block_on(task1);
        let r2 = runtime.block_on(task2);

        assert_eq!(r1, Value::Int(1));
        assert_eq!(r2, Value::Int(2));
    }

    #[tokio::test]
    async fn test_async_cancellation() {
        let runtime = AsyncRuntime::new();

        let task = runtime.spawn(|| async {
            tokio::time::sleep(Duration::from_secs(10)).await;
            Value::Unit
        });

        runtime.cancel(task);
        let state = runtime.poll(task);

        assert!(matches!(state, AsyncState::Cancelled));
    }
}
```

### Integration Tests (Fusabi Scripts)

```fsharp
// test/async/sleep_test.fsx
test "async sleep works" {
    let start = Time.now ()
    Async.runSynchronously (Async.sleep 100)
    let elapsed = Time.now () - start
    assert (elapsed >= 100)
}

// test/async/parallel_test.fsx
test "parallel async runs concurrently" {
    let start = Time.now ()

    let tasks = [
        Async.sleep 100
        Async.sleep 100
        Async.sleep 100
    ]

    Async.runSynchronously (Async.parallel tasks)
    let elapsed = Time.now () - start

    // Should complete in ~100ms, not 300ms
    assert (elapsed < 200)
}

// test/async/http_test.fsx
test "http get works" {
    let result = Async.runSynchronously (Http.getAsync "https://httpbin.org/get")
    assert (String.contains "origin" result)
}
```

## Migration Path

### From Shell-Based Async

```fsharp
// Old (shell-based, blocking)
let sleep ms =
    shellExec (sprintf "sleep %f" (float ms / 1000.0))

// New (Tokio-based, non-blocking)
let sleep ms = Async.sleep ms
```

### From Synchronous Code

```fsharp
// Old (synchronous)
let fetchUser id =
    let response = Http.get (sprintf "/users/%d" id)
    Json.parse response

// New (async)
let fetchUser id = async {
    let! response = Http.getAsync (sprintf "/users/%d" id)
    return Json.parse response
}
```

## Open Questions

1. **Structured Concurrency**: Should we support task scopes for cleanup?
2. **Async Streams**: Should we add `asyncSeq { }` for streaming?
3. **Sync Context**: Should there be a way to run async on specific threads?
4. **Error Types**: Should we use `Result` or exceptions for async errors?

## Dependencies

```toml
# Required additions to fusabi-vm/Cargo.toml
[dependencies]
tokio = { version = "1.36", features = ["rt-multi-thread", "sync", "time", "macros"] }
reqwest = { version = "0.11", features = ["json"], optional = true }
```

## Performance Considerations

1. **Thread Pool**: Tokio uses work-stealing scheduler, efficient for I/O
2. **Task Overhead**: ~200 bytes per task, acceptable for TUI use
3. **Channel Buffering**: Configurable buffer sizes for producer-consumer
4. **Polling Frequency**: Configurable poll interval for VM integration

## Security Considerations

1. **Network Access**: HTTP functions should respect capability system
2. **File Access**: File operations should be sandboxed if needed
3. **Resource Limits**: Max concurrent tasks, request timeouts
4. **Cancellation**: Ensure cleanup on task cancellation

## References

- [RFC-002: Async Computation Expressions](./RFC-002-ASYNC-CE.md)
- [Tokio Documentation](https://tokio.rs/tokio/tutorial)
- [F# Async Programming](https://docs.microsoft.com/en-us/dotnet/fsharp/tutorials/async)
- [Reqwest HTTP Client](https://docs.rs/reqwest/)
