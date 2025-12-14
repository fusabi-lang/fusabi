//! Async types for Fusabi VM
//!
//! This module provides types for async computation expressions backed by Tokio.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

/// Unique identifier for async tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(1);

impl TaskId {
    /// Generate a new unique task ID
    pub fn new() -> Self {
        TaskId(NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed))
    }

    /// Get the raw task ID value
    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Task({})", self.0)
    }
}

/// State of an async task
#[derive(Debug, Clone, PartialEq)]
pub enum AsyncState {
    /// Task is still running
    Pending,
    /// Task completed successfully with a value
    Ready(crate::Value),
    /// Task failed with an error message
    Failed(String),
    /// Task was cancelled
    Cancelled,
}

impl fmt::Display for AsyncState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsyncState::Pending => write!(f, "Pending"),
            AsyncState::Ready(v) => write!(f, "Ready({})", v),
            AsyncState::Failed(e) => write!(f, "Failed({})", e),
            AsyncState::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// Async value wrapping either a task reference or a completed value
#[derive(Debug, Clone, PartialEq)]
pub enum AsyncValue {
    /// Reference to a running or completed task
    Task(TaskId),
    /// A pre-computed value (for Async.return)
    Value(Box<crate::Value>),
}

impl fmt::Display for AsyncValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsyncValue::Task(id) => write!(f, "Async({})", id),
            AsyncValue::Value(v) => write!(f, "Async({})", v),
        }
    }
}
