//! Renderer abstraction for Fusabi TUI.
//!
//! This crate provides a unified renderer interface for the Fusabi TUI framework,
//! supporting multiple backends:
//!
//! - **Crossterm backend** (feature: `crossterm-backend`) - Standalone terminal rendering
//! - **Test backend** - In-memory rendering for unit tests
//!
//! # Example
//!
//! ```no_run
//! use fusabi_tui_render::prelude::*;
//! use fusabi_tui_core::buffer::Buffer;
//! use fusabi_tui_core::layout::Rect;
//! use std::io::stdout;
//!
//! # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//! // Create a crossterm renderer
//! let mut renderer = CrosstermRenderer::new(stdout())?;
//!
//! // Create a buffer and draw to it
//! let mut buffer = Buffer::new(Rect::new(0, 0, 80, 24));
//!
//! // Draw the buffer
//! renderer.draw(&buffer)?;
//! renderer.flush()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Testing
//!
//! The test renderer is useful for unit testing TUI applications:
//!
//! ```
//! use fusabi_tui_render::test::TestRenderer;
//! use fusabi_tui_render::renderer::Renderer;
//! use fusabi_tui_core::buffer::Buffer;
//! use fusabi_tui_core::layout::Rect;
//! use fusabi_tui_core::style::Style;
//!
//! let mut renderer = TestRenderer::new(10, 5);
//! let mut buffer = Buffer::new(Rect::new(0, 0, 10, 5));
//!
//! buffer.set_string(0, 0, "Test", Style::default());
//! renderer.draw(&buffer).unwrap();
//!
//! assert_eq!(renderer.buffer().get(0, 0).unwrap().symbol, "T");
//! ```

#![warn(clippy::all)]

// Re-export fusabi-tui-core types for convenience
pub use fusabi_tui_core;

// Core module exports
pub mod error;
pub mod renderer;
pub mod test;

// Feature-gated modules
#[cfg(feature = "crossterm-backend")]
pub mod crossterm;

// Prelude for convenient imports
pub mod prelude {
    //! Convenient re-exports for common types and traits.

    pub use crate::error::{RenderError, Result};
    pub use crate::renderer::Renderer;
    pub use crate::test::TestRenderer;

    #[cfg(feature = "crossterm-backend")]
    pub use crate::crossterm::CrosstermRenderer;
}