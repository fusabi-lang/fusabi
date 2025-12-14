//! Error types for the rendering system.

use fusabi_tui_core::layout::Rect;

/// Errors that can occur during rendering operations.
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    /// I/O error occurred
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Backend-specific error
    #[error("Backend error: {0}")]
    Backend(String),

    /// Buffer size doesn't match terminal size
    #[error("Size mismatch: expected {expected:?}, got {actual:?}")]
    SizeMismatch {
        /// Expected size
        expected: Rect,
        /// Actual size
        actual: Rect,
    },
}

/// Type alias for Results in the rendering system.
pub type Result<T> = std::result::Result<T, RenderError>;
