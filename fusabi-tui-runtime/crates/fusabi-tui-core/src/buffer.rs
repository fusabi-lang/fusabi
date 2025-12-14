//! Terminal buffer management for rendering TUI elements.
//!
//! This module provides `Cell` and `Buffer` types for managing terminal content
//! with styling and efficient diff computation.

use crate::layout::Rect;
use crate::style::{Color, Modifier, Style};
use unicode_width::UnicodeWidthStr;

/// A single cell in the terminal buffer.
///
/// Contains the character to display and its styling information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    /// The character(s) to display in this cell
    pub symbol: String,
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
    /// Text modifiers
    pub modifier: Modifier,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: " ".to_string(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::EMPTY,
        }
    }
}

impl Cell {
    /// Creates a new cell with the given symbol.
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            ..Default::default()
        }
    }

    /// Sets the foreground color.
    #[inline]
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Sets the background color.
    #[inline]
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Sets the text modifier.
    #[inline]
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }

    /// Applies a style to this cell.
    pub fn set_style(&mut self, style: Style) {
        if let Some(fg) = style.fg {
            self.fg = fg;
        }
        if let Some(bg) = style.bg {
            self.bg = bg;
        }
        self.modifier = self.modifier.insert(style.modifiers);
    }

    /// Resets this cell to default values.
    pub fn reset(&mut self) {
        self.symbol = " ".to_string();
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.modifier = Modifier::EMPTY;
    }
}

/// A buffer representing the terminal screen or a portion of it.
///
/// The buffer is a 2D grid of cells indexed by (x, y) coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    /// The area covered by this buffer
    pub area: Rect,
    /// The cells in the buffer, stored in row-major order
    content: Vec<Cell>,
}

impl Buffer {
    /// Creates a new buffer with the given area.
    ///
    /// All cells are initialized with default values.
    pub fn new(area: Rect) -> Self {
        let cell_count = (area.width as usize) * (area.height as usize);
        Self {
            area,
            content: vec![Cell::default(); cell_count],
        }
    }

    /// Creates a new buffer filled with empty cells (spaces).
    pub fn empty(area: Rect) -> Self {
        Self::new(area)
    }

    /// Creates a new buffer filled with the given cell.
    pub fn filled(area: Rect, cell: &Cell) -> Self {
        let cell_count = (area.width as usize) * (area.height as usize);
        Self {
            area,
            content: vec![cell.clone(); cell_count],
        }
    }

    /// Returns the index of the cell at the given coordinates.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    #[inline]
    fn index_of(&self, x: u16, y: u16) -> Option<usize> {
        if x >= self.area.width || y >= self.area.height {
            return None;
        }
        Some((y as usize) * (self.area.width as usize) + (x as usize))
    }

    /// Returns a reference to the cell at the given coordinates.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        self.index_of(x, y).map(|i| &self.content[i])
    }

    /// Returns a mutable reference to the cell at the given coordinates.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        self.index_of(x, y).map(|i| &mut self.content[i])
    }

    /// Sets the string at the given coordinates with the given style.
    ///
    /// Returns the number of cells written.
    pub fn set_string(&mut self, x: u16, y: u16, string: &str, style: Style) -> usize {
        let mut written = 0;
        let mut current_x = x;

        for grapheme in string.chars() {
            if current_x >= self.area.width {
                break;
            }

            let width = UnicodeWidthStr::width(grapheme.to_string().as_str()).max(1);

            if let Some(cell) = self.get_mut(current_x, y) {
                cell.symbol = grapheme.to_string();
                cell.set_style(style);
                written += 1;
            }

            current_x += width as u16;

            // Clear cells covered by wide characters
            for i in 1..width {
                if let Some(cell) = self.get_mut(current_x - width as u16 + i as u16, y) {
                    if i > 0 {
                        cell.symbol = String::new();
                    }
                }
            }
        }

        written
    }

    /// Sets the style for all cells in the given area.
    pub fn set_style(&mut self, area: Rect, style: Style) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                if let Some(cell) = self.get_mut(x, y) {
                    cell.set_style(style);
                }
            }
        }
    }

    /// Clears the entire buffer.
    pub fn clear(&mut self) {
        for cell in &mut self.content {
            cell.reset();
        }
    }

    /// Resizes the buffer to a new area.
    ///
    /// Content is preserved where possible; new cells are initialized with default values.
    pub fn resize(&mut self, area: Rect) {
        let new_cell_count = (area.width as usize) * (area.height as usize);
        let mut new_content = vec![Cell::default(); new_cell_count];

        // Copy existing content where it fits
        let min_height = self.area.height.min(area.height);
        let min_width = self.area.width.min(area.width);

        for y in 0..min_height {
            for x in 0..min_width {
                let old_idx = (y as usize) * (self.area.width as usize) + (x as usize);
                let new_idx = (y as usize) * (area.width as usize) + (x as usize);
                new_content[new_idx] = self.content[old_idx].clone();
            }
        }

        self.area = area;
        self.content = new_content;
    }

    /// Computes the difference between this buffer and another buffer.
    ///
    /// Returns a vector of (x, y, cell) tuples representing cells that differ.
    pub fn diff<'a>(&self, other: &'a Buffer) -> Vec<(u16, u16, &'a Cell)> {
        let mut updates = Vec::new();

        if self.area != other.area {
            // If areas differ, return all cells from the other buffer
            for y in 0..other.area.height {
                for x in 0..other.area.width {
                    if let Some(cell) = other.get(x, y) {
                        updates.push((x, y, cell));
                    }
                }
            }
            return updates;
        }

        // Compare cell by cell
        for y in 0..self.area.height {
            for x in 0..self.area.width {
                let self_cell = self.get(x, y);
                let other_cell = other.get(x, y);

                if self_cell != other_cell {
                    if let Some(cell) = other_cell {
                        updates.push((x, y, cell));
                    }
                }
            }
        }

        updates
    }

    /// Merges another buffer into this buffer at the given position.
    pub fn merge(&mut self, other: &Buffer) {
        let offset_x = other.area.x.saturating_sub(self.area.x);
        let offset_y = other.area.y.saturating_sub(self.area.y);

        for y in 0..other.area.height {
            for x in 0..other.area.width {
                if let Some(cell) = other.get(x, y) {
                    let target_x = offset_x + x;
                    let target_y = offset_y + y;
                    if let Some(target_cell) = self.get_mut(target_x, target_y) {
                        *target_cell = cell.clone();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_default() {
        let cell = Cell::default();
        assert_eq!(cell.symbol, " ");
        assert_eq!(cell.fg, Color::Reset);
        assert_eq!(cell.bg, Color::Reset);
        assert!(cell.modifier.is_empty());
    }

    #[test]
    fn test_cell_new() {
        let cell = Cell::new("a");
        assert_eq!(cell.symbol, "a");
    }

    #[test]
    fn test_cell_builder() {
        let cell = Cell::new("X")
            .fg(Color::Red)
            .bg(Color::Black)
            .modifier(Modifier::BOLD);

        assert_eq!(cell.symbol, "X");
        assert_eq!(cell.fg, Color::Red);
        assert_eq!(cell.bg, Color::Black);
        assert!(cell.modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_cell_set_style() {
        let mut cell = Cell::new("a");
        let style = Style::new()
            .fg(Color::Blue)
            .bg(Color::White)
            .add_modifier(Modifier::ITALIC);

        cell.set_style(style);
        assert_eq!(cell.fg, Color::Blue);
        assert_eq!(cell.bg, Color::White);
        assert!(cell.modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_cell_reset() {
        let mut cell = Cell::new("X")
            .fg(Color::Red)
            .bg(Color::Black)
            .modifier(Modifier::BOLD);

        cell.reset();
        assert_eq!(cell.symbol, " ");
        assert_eq!(cell.fg, Color::Reset);
        assert_eq!(cell.bg, Color::Reset);
        assert!(cell.modifier.is_empty());
    }

    #[test]
    fn test_buffer_new() {
        let area = Rect::new(0, 0, 10, 5);
        let buffer = Buffer::new(area);
        assert_eq!(buffer.area, area);
        assert_eq!(buffer.content.len(), 50);
    }

    #[test]
    fn test_buffer_filled() {
        let area = Rect::new(0, 0, 5, 5);
        let cell = Cell::new("X").fg(Color::Red);
        let buffer = Buffer::filled(area, &cell);

        for y in 0..5 {
            for x in 0..5 {
                let c = buffer.get(x, y).unwrap();
                assert_eq!(c.symbol, "X");
                assert_eq!(c.fg, Color::Red);
            }
        }
    }

    #[test]
    fn test_buffer_get_set() {
        let area = Rect::new(0, 0, 5, 5);
        let mut buffer = Buffer::new(area);

        assert!(buffer.get(0, 0).is_some());
        assert!(buffer.get(4, 4).is_some());
        assert!(buffer.get(5, 5).is_none());

        if let Some(cell) = buffer.get_mut(2, 2) {
            cell.symbol = "X".to_string();
        }

        assert_eq!(buffer.get(2, 2).unwrap().symbol, "X");
    }

    #[test]
    fn test_buffer_set_string() {
        let area = Rect::new(0, 0, 10, 1);
        let mut buffer = Buffer::new(area);
        let style = Style::new().fg(Color::Green);

        let written = buffer.set_string(0, 0, "Hello", style);
        assert_eq!(written, 5);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, "H");
        assert_eq!(buffer.get(1, 0).unwrap().symbol, "e");
        assert_eq!(buffer.get(4, 0).unwrap().symbol, "o");
        assert_eq!(buffer.get(0, 0).unwrap().fg, Color::Green);
    }

    #[test]
    fn test_buffer_set_style() {
        let area = Rect::new(0, 0, 5, 5);
        let mut buffer = Buffer::new(area);
        let style = Style::new().bg(Color::Blue);

        buffer.set_style(Rect::new(1, 1, 3, 3), style);

        assert_eq!(buffer.get(0, 0).unwrap().bg, Color::Reset);
        assert_eq!(buffer.get(1, 1).unwrap().bg, Color::Blue);
        assert_eq!(buffer.get(3, 3).unwrap().bg, Color::Blue);
        assert_eq!(buffer.get(4, 4).unwrap().bg, Color::Reset);
    }

    #[test]
    fn test_buffer_clear() {
        let area = Rect::new(0, 0, 5, 5);
        let mut buffer = Buffer::filled(area, &Cell::new("X"));

        buffer.clear();

        for y in 0..5 {
            for x in 0..5 {
                let cell = buffer.get(x, y).unwrap();
                assert_eq!(cell.symbol, " ");
            }
        }
    }

    #[test]
    fn test_buffer_resize() {
        let area1 = Rect::new(0, 0, 3, 3);
        let mut buffer = Buffer::new(area1);

        if let Some(cell) = buffer.get_mut(1, 1) {
            cell.symbol = "X".to_string();
        }

        let area2 = Rect::new(0, 0, 5, 5);
        buffer.resize(area2);

        assert_eq!(buffer.area, area2);
        assert_eq!(buffer.content.len(), 25);
        assert_eq!(buffer.get(1, 1).unwrap().symbol, "X");
    }

    #[test]
    fn test_buffer_diff() {
        let area = Rect::new(0, 0, 3, 3);
        let buffer1 = Buffer::new(area);
        let mut buffer2 = Buffer::new(area);

        if let Some(cell) = buffer2.get_mut(1, 1) {
            cell.symbol = "X".to_string();
        }

        let diff = buffer1.diff(&buffer2);
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 1);
        assert_eq!(diff[0].1, 1);
        assert_eq!(diff[0].2.symbol, "X");
    }

    #[test]
    fn test_buffer_merge() {
        let area1 = Rect::new(0, 0, 5, 5);
        let mut buffer1 = Buffer::new(area1);

        let area2 = Rect::new(1, 1, 3, 3);
        let mut buffer2 = Buffer::new(area2);
        if let Some(cell) = buffer2.get_mut(0, 0) {
            cell.symbol = "X".to_string();
        }

        buffer1.merge(&buffer2);
        assert_eq!(buffer1.get(1, 1).unwrap().symbol, "X");
    }
}
