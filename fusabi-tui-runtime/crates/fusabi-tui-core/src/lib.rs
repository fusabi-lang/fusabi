//! Core TUI primitives for Fusabi.
//!
//! This crate provides the foundational building blocks for creating terminal user interfaces
//! in the Fusabi ecosystem. It includes primitives for styling, layout, buffering, and
//! rendering terminal content.
//!
//! # Overview
//!
//! The crate is organized into several modules:
//!
//! - [`style`] - Color, text modifiers, and combined styles
//! - [`buffer`] - Terminal cell and buffer management
//! - [`layout`] - Rectangular areas and constraint-based layouts
//! - [`symbols`] - Unicode characters for drawing borders and UI elements
//!
//! # Quick Start
//!
//! ```rust
//! use fusabi_tui_core::{
//!     buffer::{Buffer, Cell},
//!     layout::{Constraint, Direction, Layout, Rect},
//!     style::{Color, Modifier, Style},
//! };
//!
//! // Create a buffer for a terminal area
//! let area = Rect::new(0, 0, 80, 24);
//! let mut buffer = Buffer::new(area);
//!
//! // Create a styled cell
//! let style = Style::new()
//!     .fg(Color::Green)
//!     .bg(Color::Black)
//!     .add_modifier(Modifier::BOLD);
//!
//! // Write text to the buffer
//! buffer.set_string(0, 0, "Hello, Fusabi!", style);
//!
//! // Split the area into sections
//! let chunks = Layout::default()
//!     .direction(Direction::Vertical)
//!     .constraints(&[
//!         Constraint::Length(3),
//!         Constraint::Fill(1),
//!         Constraint::Length(3),
//!     ])
//!     .split(area);
//! ```
//!
//! # Design Philosophy
//!
//! This crate is designed to be:
//!
//! - **Lightweight**: Minimal dependencies and overhead
//! - **Type-safe**: Leveraging Rust's type system for correctness
//! - **Composable**: Small, focused primitives that combine well
//! - **Performance-oriented**: Zero-cost abstractions where possible
//!
//! # Thread Safety
//!
//! All types in this crate are `Send` and `Sync` where appropriate, making them safe
//! to use in multi-threaded contexts.

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod buffer;
pub mod layout;
pub mod style;
pub mod symbols;

// Re-export commonly used types at the crate root for convenience
pub use buffer::{Buffer, Cell};
pub use layout::{Constraint, Direction, Layout, Rect};
pub use style::{Color, Modifier, Style};
