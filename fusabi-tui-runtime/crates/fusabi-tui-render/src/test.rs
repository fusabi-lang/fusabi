//! Test renderer for unit testing.
//!
//! This module provides a renderer that stores output in memory,
//! making it easy to test TUI applications without a real terminal.

use fusabi_tui_core::buffer::Buffer;
use fusabi_tui_core::layout::Rect;

use crate::error::Result;
use crate::renderer::Renderer;

/// A test renderer that stores output in memory.
///
/// This renderer is useful for unit testing TUI applications without
/// needing a real terminal. It maintains an internal buffer and cursor state
/// that can be inspected after rendering operations.
#[derive(Debug, Clone)]
pub struct TestRenderer {
    /// The internal buffer
    buffer: Buffer,
    /// Current cursor position
    cursor: (u16, u16),
    /// Whether the cursor is visible
    cursor_visible: bool,
}

impl TestRenderer {
    /// Creates a new test renderer with the given dimensions.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: Buffer::new(Rect::new(0, 0, width, height)),
            cursor: (0, 0),
            cursor_visible: true,
        }
    }

    /// Returns a reference to the internal buffer.
    ///
    /// This can be used to inspect the rendered content in tests.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Returns the current cursor position.
    pub fn cursor(&self) -> (u16, u16) {
        self.cursor
    }

    /// Returns whether the cursor is currently visible.
    pub fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    /// Asserts that the internal buffer matches the expected buffer.
    ///
    /// This is a convenience method for tests that panics with a helpful
    /// message if the buffers don't match.
    ///
    /// # Panics
    ///
    /// Panics if the buffers don't match, with a diff showing the differences.
    pub fn assert_buffer(&self, expected: &Buffer) {
        if self.buffer != *expected {
            let diff = self.buffer.diff(expected);
            panic!(
                "Buffer mismatch! {} cells differ:\n{:#?}",
                diff.len(),
                diff
            );
        }
    }

    /// Returns a string representation of the buffer for debugging.
    ///
    /// Each row is separated by a newline, making it easy to see what's rendered.
    pub fn debug_output(&self) -> String {
        let mut output = String::new();
        for y in 0..self.buffer.area.height {
            for x in 0..self.buffer.area.width {
                if let Some(cell) = self.buffer.get(x, y) {
                    output.push_str(&cell.symbol);
                }
            }
            if y < self.buffer.area.height - 1 {
                output.push('\n');
            }
        }
        output
    }
}

impl Renderer for TestRenderer {
    fn draw(&mut self, buffer: &Buffer) -> Result<()> {
        // Simply copy the buffer
        self.buffer = buffer.clone();
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        // No-op for test renderer
        Ok(())
    }

    fn size(&self) -> Result<Rect> {
        Ok(self.buffer.area)
    }

    fn clear(&mut self) -> Result<()> {
        self.buffer.clear();
        Ok(())
    }

    fn show_cursor(&mut self, show: bool) -> Result<()> {
        self.cursor_visible = show;
        Ok(())
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        self.cursor = (x, y);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fusabi_tui_core::style::Style;

    #[test]
    fn test_new() {
        let renderer = TestRenderer::new(80, 24);
        assert_eq!(renderer.buffer.area, Rect::new(0, 0, 80, 24));
        assert_eq!(renderer.cursor, (0, 0));
        assert!(renderer.cursor_visible);
    }

    #[test]
    fn test_draw() {
        let mut renderer = TestRenderer::new(10, 5);
        let mut buffer = Buffer::new(Rect::new(0, 0, 10, 5));

        buffer.set_string(0, 0, "Hello", Style::default());

        renderer.draw(&buffer).unwrap();

        assert_eq!(renderer.buffer().get(0, 0).unwrap().symbol, "H");
        assert_eq!(renderer.buffer().get(4, 0).unwrap().symbol, "o");
    }

    #[test]
    fn test_clear() {
        let mut renderer = TestRenderer::new(10, 5);
        let mut buffer = Buffer::new(Rect::new(0, 0, 10, 5));

        buffer.set_string(0, 0, "Hello", Style::default());
        renderer.draw(&buffer).unwrap();

        renderer.clear().unwrap();

        for y in 0..5 {
            for x in 0..10 {
                let cell = renderer.buffer().get(x, y).unwrap();
                assert_eq!(cell.symbol, " ");
            }
        }
    }

    #[test]
    fn test_cursor_operations() {
        let mut renderer = TestRenderer::new(10, 5);

        assert_eq!(renderer.cursor(), (0, 0));
        assert!(renderer.cursor_visible());

        renderer.set_cursor(5, 3).unwrap();
        assert_eq!(renderer.cursor(), (5, 3));

        renderer.show_cursor(false).unwrap();
        assert!(!renderer.cursor_visible());

        renderer.show_cursor(true).unwrap();
        assert!(renderer.cursor_visible());
    }

    #[test]
    fn test_size() {
        let renderer = TestRenderer::new(100, 50);
        let size = renderer.size().unwrap();
        assert_eq!(size, Rect::new(0, 0, 100, 50));
    }

    #[test]
    fn test_assert_buffer_success() {
        let mut renderer = TestRenderer::new(5, 1);
        let mut buffer = Buffer::new(Rect::new(0, 0, 5, 1));

        buffer.set_string(0, 0, "Test", Style::default());
        renderer.draw(&buffer).unwrap();

        // This should not panic
        renderer.assert_buffer(&buffer);
    }

    #[test]
    #[should_panic(expected = "Buffer mismatch")]
    fn test_assert_buffer_failure() {
        let mut renderer = TestRenderer::new(5, 1);
        let mut buffer1 = Buffer::new(Rect::new(0, 0, 5, 1));
        let mut buffer2 = Buffer::new(Rect::new(0, 0, 5, 1));

        buffer1.set_string(0, 0, "Test", Style::default());
        buffer2.set_string(0, 0, "Fail", Style::default());

        renderer.draw(&buffer1).unwrap();

        // This should panic
        renderer.assert_buffer(&buffer2);
    }

    #[test]
    fn test_debug_output() {
        let mut renderer = TestRenderer::new(5, 3);
        let mut buffer = Buffer::new(Rect::new(0, 0, 5, 3));

        buffer.set_string(0, 0, "Hello", Style::default());
        buffer.set_string(0, 1, "World", Style::default());

        renderer.draw(&buffer).unwrap();

        let output = renderer.debug_output();
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].starts_with("Hello"));
        assert!(lines[1].starts_with("World"));
    }
}
