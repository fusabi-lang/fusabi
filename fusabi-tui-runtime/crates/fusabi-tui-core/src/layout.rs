//! Layout primitives for positioning and sizing TUI elements.
//!
//! This module provides types for defining rectangular areas and splitting them
//! into smaller regions based on constraints.

use std::cmp::{max, min};

/// A rectangular area in the terminal.
///
/// Coordinates are 0-indexed, with (0, 0) at the top-left corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rect {
    /// X coordinate of the top-left corner
    pub x: u16,
    /// Y coordinate of the top-left corner
    pub y: u16,
    /// Width of the rectangle
    pub width: u16,
    /// Height of the rectangle
    pub height: u16,
}

impl Rect {
    /// Creates a new rectangle.
    #[inline]
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns the area (width × height) of the rectangle.
    #[inline]
    pub const fn area(self) -> u16 {
        self.width.saturating_mul(self.height)
    }

    /// Returns the X coordinate of the left edge.
    #[inline]
    pub const fn left(self) -> u16 {
        self.x
    }

    /// Returns the X coordinate one past the right edge.
    #[inline]
    pub const fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the Y coordinate of the top edge.
    #[inline]
    pub const fn top(self) -> u16 {
        self.y
    }

    /// Returns the Y coordinate one past the bottom edge.
    #[inline]
    pub const fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Returns a new rectangle with the given margin applied.
    ///
    /// The margin is applied to all sides, reducing the width and height by 2× the margin.
    pub fn inner(self, margin: u16) -> Self {
        let doubled_margin = margin.saturating_mul(2);
        if self.width < doubled_margin || self.height < doubled_margin {
            Self::default()
        } else {
            Self {
                x: self.x.saturating_add(margin),
                y: self.y.saturating_add(margin),
                width: self.width.saturating_sub(doubled_margin),
                height: self.height.saturating_sub(doubled_margin),
            }
        }
    }

    /// Returns the intersection of this rectangle with another.
    pub fn intersection(self, other: Self) -> Self {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.right(), other.right());
        let y2 = min(self.bottom(), other.bottom());

        Self {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    /// Returns the union of this rectangle with another.
    pub fn union(self, other: Self) -> Self {
        let x1 = min(self.x, other.x);
        let y1 = min(self.y, other.y);
        let x2 = max(self.right(), other.right());
        let y2 = max(self.bottom(), other.bottom());

        Self {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    /// Checks if this rectangle contains the given point.
    #[inline]
    pub fn contains(self, x: u16, y: u16) -> bool {
        x >= self.left() && x < self.right() && y >= self.top() && y < self.bottom()
    }

    /// Checks if this rectangle intersects with another.
    #[inline]
    pub fn intersects(self, other: Self) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }
}

/// A constraint for layout calculations.
///
/// Determines how much space a layout element should occupy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Constraint {
    /// Fixed number of cells
    Length(u16),
    /// Percentage of available space (0-100)
    Percentage(u16),
    /// Ratio of available space (numerator, denominator)
    Ratio(u32, u32),
    /// Minimum number of cells
    Min(u16),
    /// Maximum number of cells
    Max(u16),
    /// Fill remaining space equally among all Fill constraints
    Fill(u16),
}

impl Constraint {
    /// Applies this constraint to the given available space.
    fn apply(&self, length: u16) -> u16 {
        match *self {
            Constraint::Length(l) => min(length, l),
            Constraint::Percentage(p) => {
                let p = min(p, 100);
                (length as u32 * p as u32 / 100) as u16
            }
            Constraint::Ratio(numerator, denominator) => {
                if denominator == 0 {
                    0
                } else {
                    (length as u32 * numerator / denominator) as u16
                }
            }
            Constraint::Min(m) => max(length, m),
            Constraint::Max(m) => min(length, m),
            Constraint::Fill(_) => length,
        }
    }
}

/// Direction for splitting a layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Split horizontally (left to right)
    Horizontal,
    /// Split vertically (top to bottom)
    Vertical,
}

/// A layout for dividing a rectangular area into smaller regions.
///
/// # Examples
///
/// ```
/// use fusabi_tui_core::layout::{Layout, Direction, Constraint, Rect};
///
/// let area = Rect::new(0, 0, 100, 50);
/// let chunks = Layout::default()
///     .direction(Direction::Vertical)
///     .constraints(&[
///         Constraint::Length(3),
///         Constraint::Min(0),
///         Constraint::Length(3),
///     ])
///     .split(area);
///
/// assert_eq!(chunks.len(), 3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Layout {
    direction: Direction,
    margin: u16,
    constraints: Vec<Constraint>,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            direction: Direction::Vertical,
            margin: 0,
            constraints: Vec::new(),
        }
    }
}

impl Layout {
    /// Creates a new layout with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the direction for splitting.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the margin to apply before splitting.
    pub fn margin(mut self, margin: u16) -> Self {
        self.margin = margin;
        self
    }

    /// Sets the constraints for splitting.
    pub fn constraints(mut self, constraints: &[Constraint]) -> Self {
        self.constraints = constraints.to_vec();
        self
    }

    /// Splits the given area according to this layout's constraints.
    ///
    /// Returns a vector of rectangles, one for each constraint.
    pub fn split(&self, area: Rect) -> Vec<Rect> {
        let area = area.inner(self.margin);

        if self.constraints.is_empty() {
            return vec![area];
        }

        let (main_axis_size, cross_axis_size) = match self.direction {
            Direction::Horizontal => (area.width, area.height),
            Direction::Vertical => (area.height, area.width),
        };

        // First pass: calculate sizes for non-Fill constraints
        let mut sizes = vec![0u16; self.constraints.len()];
        let mut remaining = main_axis_size;
        let mut fill_count = 0;

        for (i, constraint) in self.constraints.iter().enumerate() {
            match constraint {
                Constraint::Fill(_) => {
                    fill_count += 1;
                }
                Constraint::Percentage(_) | Constraint::Ratio(_, _) => {
                    // Percentage and Ratio apply to the original size, not remaining
                    let size = constraint.apply(main_axis_size);
                    sizes[i] = size;
                    remaining = remaining.saturating_sub(size);
                }
                _ => {
                    let size = constraint.apply(remaining);
                    sizes[i] = size;
                    remaining = remaining.saturating_sub(size);
                }
            }
        }

        // Second pass: distribute remaining space among Fill constraints
        if fill_count > 0 {
            let fill_size = remaining / fill_count as u16;
            let fill_remainder = remaining % fill_count as u16;
            let mut remainder_distributed = 0;

            for (i, constraint) in self.constraints.iter().enumerate() {
                if let Constraint::Fill(_) = constraint {
                    sizes[i] = fill_size;
                    if remainder_distributed < fill_remainder {
                        sizes[i] += 1;
                        remainder_distributed += 1;
                    }
                }
            }
        }

        // Third pass: apply Min and Max constraints
        for (i, constraint) in self.constraints.iter().enumerate() {
            match constraint {
                Constraint::Min(min) => {
                    if sizes[i] < *min {
                        sizes[i] = *min;
                    }
                }
                Constraint::Max(max) => {
                    if sizes[i] > *max {
                        sizes[i] = *max;
                    }
                }
                _ => {}
            }
        }

        // Build the result rectangles
        let mut results = Vec::with_capacity(self.constraints.len());
        let mut offset = 0;

        for size in sizes {
            let rect = match self.direction {
                Direction::Horizontal => Rect {
                    x: area.x + offset,
                    y: area.y,
                    width: size,
                    height: cross_axis_size,
                },
                Direction::Vertical => Rect {
                    x: area.x,
                    y: area.y + offset,
                    width: cross_axis_size,
                    height: size,
                },
            };
            results.push(rect);
            offset += size;
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_new() {
        let rect = Rect::new(1, 2, 10, 20);
        assert_eq!(rect.x, 1);
        assert_eq!(rect.y, 2);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    #[test]
    fn test_rect_area() {
        let rect = Rect::new(0, 0, 10, 20);
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_rect_edges() {
        let rect = Rect::new(5, 10, 20, 30);
        assert_eq!(rect.left(), 5);
        assert_eq!(rect.right(), 25);
        assert_eq!(rect.top(), 10);
        assert_eq!(rect.bottom(), 40);
    }

    #[test]
    fn test_rect_inner() {
        let rect = Rect::new(0, 0, 10, 10);
        let inner = rect.inner(1);
        assert_eq!(inner.x, 1);
        assert_eq!(inner.y, 1);
        assert_eq!(inner.width, 8);
        assert_eq!(inner.height, 8);
    }

    #[test]
    fn test_rect_inner_too_small() {
        let rect = Rect::new(0, 0, 3, 3);
        let inner = rect.inner(2);
        assert_eq!(inner, Rect::default());
    }

    #[test]
    fn test_rect_intersection() {
        let rect1 = Rect::new(0, 0, 10, 10);
        let rect2 = Rect::new(5, 5, 10, 10);
        let inter = rect1.intersection(rect2);
        assert_eq!(inter, Rect::new(5, 5, 5, 5));
    }

    #[test]
    fn test_rect_union() {
        let rect1 = Rect::new(0, 0, 5, 5);
        let rect2 = Rect::new(3, 3, 5, 5);
        let union = rect1.union(rect2);
        assert_eq!(union, Rect::new(0, 0, 8, 8));
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(5, 5, 10, 10);
        assert!(rect.contains(5, 5));
        assert!(rect.contains(10, 10));
        assert!(rect.contains(14, 14));
        assert!(!rect.contains(15, 15));
        assert!(!rect.contains(4, 5));
    }

    #[test]
    fn test_rect_intersects() {
        let rect1 = Rect::new(0, 0, 10, 10);
        let rect2 = Rect::new(5, 5, 10, 10);
        let rect3 = Rect::new(20, 20, 10, 10);

        assert!(rect1.intersects(rect2));
        assert!(rect2.intersects(rect1));
        assert!(!rect1.intersects(rect3));
    }

    #[test]
    fn test_constraint_length() {
        let c = Constraint::Length(10);
        assert_eq!(c.apply(20), 10);
        assert_eq!(c.apply(5), 5);
    }

    #[test]
    fn test_constraint_percentage() {
        let c = Constraint::Percentage(50);
        assert_eq!(c.apply(100), 50);

        let c = Constraint::Percentage(150);
        assert_eq!(c.apply(100), 100);
    }

    #[test]
    fn test_constraint_ratio() {
        let c = Constraint::Ratio(1, 3);
        assert_eq!(c.apply(99), 33);

        let c = Constraint::Ratio(1, 0);
        assert_eq!(c.apply(100), 0);
    }

    #[test]
    fn test_layout_vertical() {
        let area = Rect::new(0, 0, 100, 100);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&[
                Constraint::Length(10),
                Constraint::Length(20),
                Constraint::Length(30),
            ])
            .split(area);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], Rect::new(0, 0, 100, 10));
        assert_eq!(chunks[1], Rect::new(0, 10, 100, 20));
        assert_eq!(chunks[2], Rect::new(0, 30, 100, 30));
    }

    #[test]
    fn test_layout_horizontal() {
        let area = Rect::new(0, 0, 100, 100);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(&[
                Constraint::Length(10),
                Constraint::Length(20),
                Constraint::Length(30),
            ])
            .split(area);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], Rect::new(0, 0, 10, 100));
        assert_eq!(chunks[1], Rect::new(10, 0, 20, 100));
        assert_eq!(chunks[2], Rect::new(30, 0, 30, 100));
    }

    #[test]
    fn test_layout_percentage() {
        let area = Rect::new(0, 0, 100, 100);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&[Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);

        assert_eq!(chunks[0].height, 25);
        assert_eq!(chunks[1].height, 75);
    }

    #[test]
    fn test_layout_fill() {
        let area = Rect::new(0, 0, 100, 100);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&[
                Constraint::Length(10),
                Constraint::Fill(1),
                Constraint::Length(10),
            ])
            .split(area);

        assert_eq!(chunks[0].height, 10);
        assert_eq!(chunks[1].height, 80);
        assert_eq!(chunks[2].height, 10);
    }

    #[test]
    fn test_layout_multiple_fill() {
        let area = Rect::new(0, 0, 100, 100);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&[
                Constraint::Length(10),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(area);

        assert_eq!(chunks[0].height, 10);
        assert_eq!(chunks[1].height, 45);
        assert_eq!(chunks[2].height, 45);
    }

    #[test]
    fn test_layout_margin() {
        let area = Rect::new(0, 0, 100, 100);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints(&[Constraint::Percentage(100)])
            .split(area);

        assert_eq!(chunks[0], Rect::new(5, 5, 90, 90));
    }

    #[test]
    fn test_layout_empty_constraints() {
        let area = Rect::new(0, 0, 100, 100);
        let chunks = Layout::default().split(area);

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], area);
    }
}
