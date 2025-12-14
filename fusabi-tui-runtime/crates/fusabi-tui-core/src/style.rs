//! Styling primitives for TUI elements.
//!
//! This module provides the building blocks for styling text and UI elements in the terminal,
//! including colors, text modifiers, and combined styles.

use std::fmt;

/// Represents a color in the terminal.
///
/// Supports both standard ANSI colors and extended 256-color/RGB modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// Black color (ANSI 0)
    Black,
    /// Red color (ANSI 1)
    Red,
    /// Green color (ANSI 2)
    Green,
    /// Yellow color (ANSI 3)
    Yellow,
    /// Blue color (ANSI 4)
    Blue,
    /// Magenta color (ANSI 5)
    Magenta,
    /// Cyan color (ANSI 6)
    Cyan,
    /// White color (ANSI 7)
    White,
    /// Dark gray color (ANSI 8)
    DarkGray,
    /// Light red color (ANSI 9)
    LightRed,
    /// Light green color (ANSI 10)
    LightGreen,
    /// Light yellow color (ANSI 11)
    LightYellow,
    /// Light blue color (ANSI 12)
    LightBlue,
    /// Light magenta color (ANSI 13)
    LightMagenta,
    /// Light cyan color (ANSI 14)
    LightCyan,
    /// Light white color (ANSI 15)
    LightWhite,
    /// RGB color with red, green, and blue components (0-255)
    Rgb(u8, u8, u8),
    /// Indexed color (0-255) from the 256-color palette
    Indexed(u8),
    /// Reset to the terminal's default color
    Reset,
}

impl Default for Color {
    fn default() -> Self {
        Color::Reset
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Black => write!(f, "Black"),
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Yellow => write!(f, "Yellow"),
            Color::Blue => write!(f, "Blue"),
            Color::Magenta => write!(f, "Magenta"),
            Color::Cyan => write!(f, "Cyan"),
            Color::White => write!(f, "White"),
            Color::DarkGray => write!(f, "DarkGray"),
            Color::LightRed => write!(f, "LightRed"),
            Color::LightGreen => write!(f, "LightGreen"),
            Color::LightYellow => write!(f, "LightYellow"),
            Color::LightBlue => write!(f, "LightBlue"),
            Color::LightMagenta => write!(f, "LightMagenta"),
            Color::LightCyan => write!(f, "LightCyan"),
            Color::LightWhite => write!(f, "LightWhite"),
            Color::Rgb(r, g, b) => write!(f, "Rgb({}, {}, {})", r, g, b),
            Color::Indexed(i) => write!(f, "Indexed({})", i),
            Color::Reset => write!(f, "Reset"),
        }
    }
}

/// Text modifiers that can be applied to styled text.
///
/// These can be combined using bitwise operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modifier(u16);

impl Modifier {
    /// No modifiers
    pub const EMPTY: Self = Self(0b0000_0000_0000);
    /// Bold text
    pub const BOLD: Self = Self(0b0000_0000_0001);
    /// Dim/faint text
    pub const DIM: Self = Self(0b0000_0000_0010);
    /// Italic text
    pub const ITALIC: Self = Self(0b0000_0000_0100);
    /// Underlined text
    pub const UNDERLINED: Self = Self(0b0000_0000_1000);
    /// Slowly blinking text (less than 150 blinks per minute)
    pub const SLOW_BLINK: Self = Self(0b0000_0001_0000);
    /// Rapidly blinking text (150+ blinks per minute)
    pub const RAPID_BLINK: Self = Self(0b0000_0010_0000);
    /// Reversed foreground and background colors
    pub const REVERSED: Self = Self(0b0000_0100_0000);
    /// Hidden/invisible text
    pub const HIDDEN: Self = Self(0b0000_1000_0000);
    /// Crossed out text
    pub const CROSSED_OUT: Self = Self(0b0001_0000_0000);

    /// Creates a new modifier with no flags set.
    #[inline]
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    /// Creates a new modifier from raw bits.
    #[inline]
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// Returns the raw bits of this modifier.
    #[inline]
    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Checks if this modifier contains the given flags.
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Inserts the given flags into this modifier.
    #[inline]
    pub const fn insert(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Removes the given flags from this modifier.
    #[inline]
    pub const fn remove(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Toggles the given flags in this modifier.
    #[inline]
    pub const fn toggle(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Checks if no modifiers are set.
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl Default for Modifier {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl std::ops::BitOr for Modifier {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Modifier {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for Modifier {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Modifier {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::Not for Modifier {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

/// A style combining foreground color, background color, and text modifiers.
///
/// # Examples
///
/// ```
/// use fusabi_tui_core::style::{Style, Color, Modifier};
///
/// let style = Style::default()
///     .fg(Color::Red)
///     .bg(Color::Black)
///     .add_modifier(Modifier::BOLD);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Style {
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color
    pub bg: Option<Color>,
    /// Text modifiers
    pub modifiers: Modifier,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: None,
            bg: None,
            modifiers: Modifier::EMPTY,
        }
    }
}

impl Style {
    /// Creates a new style with default values.
    #[inline]
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            modifiers: Modifier::EMPTY,
        }
    }

    /// Sets the foreground color.
    #[inline]
    pub const fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Sets the background color.
    #[inline]
    pub const fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Adds the given modifiers to this style.
    #[inline]
    pub const fn add_modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers = self.modifiers.insert(modifier);
        self
    }

    /// Removes the given modifiers from this style.
    #[inline]
    pub const fn remove_modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers = self.modifiers.remove(modifier);
        self
    }

    /// Resets this style to default values.
    #[inline]
    pub const fn reset(mut self) -> Self {
        self.fg = None;
        self.bg = None;
        self.modifiers = Modifier::EMPTY;
        self
    }

    /// Patches this style with another style.
    ///
    /// Fields in `other` that are set will override the corresponding fields in `self`.
    pub fn patch(mut self, other: Style) -> Self {
        if other.fg.is_some() {
            self.fg = other.fg;
        }
        if other.bg.is_some() {
            self.bg = other.bg;
        }
        self.modifiers = self.modifiers.insert(other.modifiers);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_default() {
        assert_eq!(Color::default(), Color::Reset);
    }

    #[test]
    fn test_color_display() {
        assert_eq!(Color::Red.to_string(), "Red");
        assert_eq!(Color::Rgb(255, 128, 0).to_string(), "Rgb(255, 128, 0)");
        assert_eq!(Color::Indexed(42).to_string(), "Indexed(42)");
    }

    #[test]
    fn test_modifier_empty() {
        let m = Modifier::empty();
        assert!(m.is_empty());
        assert_eq!(m.bits(), 0);
    }

    #[test]
    fn test_modifier_insert() {
        let m = Modifier::BOLD.insert(Modifier::ITALIC);
        assert!(m.contains(Modifier::BOLD));
        assert!(m.contains(Modifier::ITALIC));
        assert!(!m.contains(Modifier::UNDERLINED));
    }

    #[test]
    fn test_modifier_remove() {
        let m = Modifier::BOLD
            .insert(Modifier::ITALIC)
            .remove(Modifier::BOLD);
        assert!(!m.contains(Modifier::BOLD));
        assert!(m.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_modifier_toggle() {
        let m = Modifier::BOLD.toggle(Modifier::BOLD);
        assert!(!m.contains(Modifier::BOLD));

        let m = Modifier::BOLD.toggle(Modifier::ITALIC);
        assert!(m.contains(Modifier::BOLD));
        assert!(m.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_modifier_bitops() {
        let m1 = Modifier::BOLD | Modifier::ITALIC;
        assert!(m1.contains(Modifier::BOLD));
        assert!(m1.contains(Modifier::ITALIC));

        let m2 = m1 & Modifier::BOLD;
        assert!(m2.contains(Modifier::BOLD));
        assert!(!m2.contains(Modifier::ITALIC));

        let m3 = !Modifier::BOLD;
        assert!(!m3.contains(Modifier::BOLD));
    }

    #[test]
    fn test_style_default() {
        let style = Style::default();
        assert_eq!(style.fg, None);
        assert_eq!(style.bg, None);
        assert!(style.modifiers.is_empty());
    }

    #[test]
    fn test_style_builder() {
        let style = Style::new()
            .fg(Color::Red)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC);

        assert_eq!(style.fg, Some(Color::Red));
        assert_eq!(style.bg, Some(Color::Black));
        assert!(style.modifiers.contains(Modifier::BOLD));
        assert!(style.modifiers.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_style_remove_modifier() {
        let style = Style::new()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC)
            .remove_modifier(Modifier::BOLD);

        assert!(!style.modifiers.contains(Modifier::BOLD));
        assert!(style.modifiers.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_style_reset() {
        let style = Style::new()
            .fg(Color::Red)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD)
            .reset();

        assert_eq!(style.fg, None);
        assert_eq!(style.bg, None);
        assert!(style.modifiers.is_empty());
    }

    #[test]
    fn test_style_patch() {
        let style1 = Style::new()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD);

        let style2 = Style::new()
            .bg(Color::Black)
            .add_modifier(Modifier::ITALIC);

        let patched = style1.patch(style2);
        assert_eq!(patched.fg, Some(Color::Red));
        assert_eq!(patched.bg, Some(Color::Black));
        assert!(patched.modifiers.contains(Modifier::BOLD));
        assert!(patched.modifiers.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_style_patch_override() {
        let style1 = Style::new().fg(Color::Red);
        let style2 = Style::new().fg(Color::Blue);
        let patched = style1.patch(style2);
        assert_eq!(patched.fg, Some(Color::Blue));
    }
}
