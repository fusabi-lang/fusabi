//! Core renderer trait and abstractions.

use fusabi_tui_core::buffer::Buffer;
use fusabi_tui_core::layout::Rect;

use crate::error::{RenderError, Result};

/// A renderer that can draw buffers to a terminal or other output device.
///
/// The renderer trait provides a common interface for different rendering backends
/// (crossterm, shared memory, testing, etc.) to draw terminal buffers.
pub trait Renderer: Send {
    /// Draw a buffer to the terminal.
    ///
    /// This method should update the terminal display to match the contents of the buffer.
    /// Implementations should use differential rendering when possible to minimize updates.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer containing the content to render
    ///
    /// # Errors
    ///
    /// Returns an error if the draw operation fails (e.g., I/O error, size mismatch).
    fn draw(&mut self, buffer: &Buffer) -> Result<()>;

    /// Flush any buffered output to the terminal.
    ///
    /// This ensures all pending drawing operations are written to the terminal.
    ///
    /// # Errors
    ///
    /// Returns an error if the flush operation fails.
    fn flush(&mut self) -> Result<()>;

    /// Get the current size of the terminal.
    ///
    /// Returns a rectangle representing the terminal dimensions, typically with
    /// x=0, y=0, and width/height set to the terminal size.
    ///
    /// # Errors
    ///
    /// Returns an error if the terminal size cannot be determined.
    fn size(&self) -> Result<Rect>;

    /// Clear the entire terminal.
    ///
    /// This removes all content from the terminal screen.
    ///
    /// # Errors
    ///
    /// Returns an error if the clear operation fails.
    fn clear(&mut self) -> Result<()>;

    /// Show or hide the terminal cursor.
    ///
    /// # Arguments
    ///
    /// * `show` - If true, the cursor will be visible; if false, it will be hidden
    ///
    /// # Errors
    ///
    /// Returns an error if the cursor visibility cannot be changed.
    fn show_cursor(&mut self, show: bool) -> Result<()>;

    /// Move the cursor to a specific position.
    ///
    /// # Arguments
    ///
    /// * `x` - The column position (0-indexed)
    /// * `y` - The row position (0-indexed)
    ///
    /// # Errors
    ///
    /// Returns an error if the cursor cannot be moved to the specified position.
    fn set_cursor(&mut self, x: u16, y: u16) -> Result<()>;
}
