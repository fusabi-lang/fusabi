//! Unicode symbols for drawing borders, boxes, and other UI elements.
//!
//! This module provides commonly used Unicode characters organized by category.

/// Line drawing symbols.
pub mod line {
    /// Vertical line: │
    pub const VERTICAL: &str = "│";
    /// Horizontal line: ─
    pub const HORIZONTAL: &str = "─";
    /// Top-left corner: ┌
    pub const TOP_LEFT: &str = "┌";
    /// Top-right corner: ┐
    pub const TOP_RIGHT: &str = "┐";
    /// Bottom-left corner: └
    pub const BOTTOM_LEFT: &str = "└";
    /// Bottom-right corner: ┘
    pub const BOTTOM_RIGHT: &str = "┘";
    /// Vertical-right junction: ├
    pub const VERTICAL_RIGHT: &str = "├";
    /// Vertical-left junction: ┤
    pub const VERTICAL_LEFT: &str = "┤";
    /// Horizontal-down junction: ┬
    pub const HORIZONTAL_DOWN: &str = "┬";
    /// Horizontal-up junction: ┴
    pub const HORIZONTAL_UP: &str = "┴";
    /// Cross junction: ┼
    pub const CROSS: &str = "┼";

    /// Thick vertical line: ┃
    pub const THICK_VERTICAL: &str = "┃";
    /// Thick horizontal line: ━
    pub const THICK_HORIZONTAL: &str = "━";
    /// Thick top-left corner: ┏
    pub const THICK_TOP_LEFT: &str = "┏";
    /// Thick top-right corner: ┓
    pub const THICK_TOP_RIGHT: &str = "┓";
    /// Thick bottom-left corner: ┗
    pub const THICK_BOTTOM_LEFT: &str = "┗";
    /// Thick bottom-right corner: ┛
    pub const THICK_BOTTOM_RIGHT: &str = "┛";
    /// Thick vertical-right junction: ┣
    pub const THICK_VERTICAL_RIGHT: &str = "┣";
    /// Thick vertical-left junction: ┫
    pub const THICK_VERTICAL_LEFT: &str = "┫";
    /// Thick horizontal-down junction: ┳
    pub const THICK_HORIZONTAL_DOWN: &str = "┳";
    /// Thick horizontal-up junction: ┻
    pub const THICK_HORIZONTAL_UP: &str = "┻";
    /// Thick cross junction: ╋
    pub const THICK_CROSS: &str = "╋";

    /// Double vertical line: ║
    pub const DOUBLE_VERTICAL: &str = "║";
    /// Double horizontal line: ═
    pub const DOUBLE_HORIZONTAL: &str = "═";
    /// Double top-left corner: ╔
    pub const DOUBLE_TOP_LEFT: &str = "╔";
    /// Double top-right corner: ╗
    pub const DOUBLE_TOP_RIGHT: &str = "╗";
    /// Double bottom-left corner: ╚
    pub const DOUBLE_BOTTOM_LEFT: &str = "╚";
    /// Double bottom-right corner: ╝
    pub const DOUBLE_BOTTOM_RIGHT: &str = "╝";
    /// Double vertical-right junction: ╠
    pub const DOUBLE_VERTICAL_RIGHT: &str = "╠";
    /// Double vertical-left junction: ╣
    pub const DOUBLE_VERTICAL_LEFT: &str = "╣";
    /// Double horizontal-down junction: ╦
    pub const DOUBLE_HORIZONTAL_DOWN: &str = "╦";
    /// Double horizontal-up junction: ╩
    pub const DOUBLE_HORIZONTAL_UP: &str = "╩";
    /// Double cross junction: ╬
    pub const DOUBLE_CROSS: &str = "╬";

    /// Rounded top-left corner: ╭
    pub const ROUNDED_TOP_LEFT: &str = "╭";
    /// Rounded top-right corner: ╮
    pub const ROUNDED_TOP_RIGHT: &str = "╮";
    /// Rounded bottom-left corner: ╰
    pub const ROUNDED_BOTTOM_LEFT: &str = "╰";
    /// Rounded bottom-right corner: ╯
    pub const ROUNDED_BOTTOM_RIGHT: &str = "╯";
}

/// Block drawing symbols.
pub mod block {
    /// Full block: █
    pub const FULL: &str = "█";
    /// Seven-eighths block: ▉
    pub const SEVEN_EIGHTHS: &str = "▉";
    /// Three-quarters block: ▊
    pub const THREE_QUARTERS: &str = "▊";
    /// Five-eighths block: ▋
    pub const FIVE_EIGHTHS: &str = "▋";
    /// Half block: ▌
    pub const HALF: &str = "▌";
    /// Three-eighths block: ▍
    pub const THREE_EIGHTHS: &str = "▍";
    /// Quarter block: ▎
    pub const QUARTER: &str = "▎";
    /// One-eighth block: ▏
    pub const ONE_EIGHTH: &str = "▏";

    /// Upper half block: ▀
    pub const UPPER_HALF: &str = "▀";
    /// Lower half block: ▄
    pub const LOWER_HALF: &str = "▄";
    /// Left half block: ▌
    pub const LEFT_HALF: &str = "▌";
    /// Right half block: ▐
    pub const RIGHT_HALF: &str = "▐";

    /// Light shade: ░
    pub const LIGHT_SHADE: &str = "░";
    /// Medium shade: ▒
    pub const MEDIUM_SHADE: &str = "▒";
    /// Dark shade: ▓
    pub const DARK_SHADE: &str = "▓";
}

/// Bar drawing symbols for charts and graphs.
pub mod bar {
    /// Empty bar
    pub const EMPTY: &str = " ";
    /// One-eighth bar: ▁
    pub const ONE_EIGHTH: &str = "▁";
    /// Quarter bar: ▂
    pub const QUARTER: &str = "▂";
    /// Three-eighths bar: ▃
    pub const THREE_EIGHTHS: &str = "▃";
    /// Half bar: ▄
    pub const HALF: &str = "▄";
    /// Five-eighths bar: ▅
    pub const FIVE_EIGHTHS: &str = "▅";
    /// Three-quarters bar: ▆
    pub const THREE_QUARTERS: &str = "▆";
    /// Seven-eighths bar: ▇
    pub const SEVEN_EIGHTHS: &str = "▇";
    /// Full bar: █
    pub const FULL: &str = "█";

    /// Set of all vertical bars from empty to full
    pub const VERTICAL_BARS: [&str; 9] = [
        EMPTY,
        ONE_EIGHTH,
        QUARTER,
        THREE_EIGHTHS,
        HALF,
        FIVE_EIGHTHS,
        THREE_QUARTERS,
        SEVEN_EIGHTHS,
        FULL,
    ];

    /// Left one-eighth bar: ▏
    pub const HORIZONTAL_ONE_EIGHTH: &str = "▏";
    /// Left quarter bar: ▎
    pub const HORIZONTAL_QUARTER: &str = "▎";
    /// Left three-eighths bar: ▍
    pub const HORIZONTAL_THREE_EIGHTHS: &str = "▍";
    /// Left half bar: ▌
    pub const HORIZONTAL_HALF: &str = "▌";
    /// Left five-eighths bar: ▋
    pub const HORIZONTAL_FIVE_EIGHTHS: &str = "▋";
    /// Left three-quarters bar: ▊
    pub const HORIZONTAL_THREE_QUARTERS: &str = "▊";
    /// Left seven-eighths bar: ▉
    pub const HORIZONTAL_SEVEN_EIGHTHS: &str = "▉";

    /// Set of all horizontal bars from empty to full
    pub const HORIZONTAL_BARS: [&str; 9] = [
        EMPTY,
        HORIZONTAL_ONE_EIGHTH,
        HORIZONTAL_QUARTER,
        HORIZONTAL_THREE_EIGHTHS,
        HORIZONTAL_HALF,
        HORIZONTAL_FIVE_EIGHTHS,
        HORIZONTAL_THREE_QUARTERS,
        HORIZONTAL_SEVEN_EIGHTHS,
        FULL,
    ];
}

/// Dot symbols for scatter plots and braille patterns.
pub mod dot {
    /// Braille blank pattern (no dots)
    pub const BRAILLE_BLANK: char = '⠀';
    /// Braille full pattern (all dots)
    pub const BRAILLE_FULL: char = '⣿';

    /// Small dot: ·
    pub const SMALL: &str = "·";
    /// Medium dot: •
    pub const MEDIUM: &str = "•";
    /// Large dot: ●
    pub const LARGE: &str = "●";
}

/// Arrow symbols.
pub mod arrow {
    /// Up arrow: ↑
    pub const UP: &str = "↑";
    /// Down arrow: ↓
    pub const DOWN: &str = "↓";
    /// Left arrow: ←
    pub const LEFT: &str = "←";
    /// Right arrow: →
    pub const RIGHT: &str = "→";

    /// Double up arrow: ⇑
    pub const DOUBLE_UP: &str = "⇑";
    /// Double down arrow: ⇓
    pub const DOUBLE_DOWN: &str = "⇓";
    /// Double left arrow: ⇐
    pub const DOUBLE_LEFT: &str = "⇐";
    /// Double right arrow: ⇒
    pub const DOUBLE_RIGHT: &str = "⇒";

    /// Up-down arrow: ↕
    pub const UP_DOWN: &str = "↕";
    /// Left-right arrow: ↔
    pub const LEFT_RIGHT: &str = "↔";
}

/// Special symbols.
pub mod special {
    /// Bullet: •
    pub const BULLET: &str = "•";
    /// Checkbox unchecked: ☐
    pub const CHECKBOX_UNCHECKED: &str = "☐";
    /// Checkbox checked: ☑
    pub const CHECKBOX_CHECKED: &str = "☑";
    /// Checkbox crossed: ☒
    pub const CHECKBOX_CROSSED: &str = "☒";

    /// Radio button unchecked: ○
    pub const RADIO_UNCHECKED: &str = "○";
    /// Radio button checked: ◉
    pub const RADIO_CHECKED: &str = "◉";

    /// Ellipsis: …
    pub const ELLIPSIS: &str = "…";
    /// Vertical ellipsis: ⋮
    pub const VERTICAL_ELLIPSIS: &str = "⋮";

    /// Star: ★
    pub const STAR: &str = "★";
    /// Empty star: ☆
    pub const STAR_EMPTY: &str = "☆";
}

/// Spinners and progress indicators.
pub mod spinner {
    /// Braille spinner frames
    pub const BRAILLE: [&str; 8] = ["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];

    /// Dots spinner frames
    pub const DOTS: [&str; 4] = ["⠋", "⠙", "⠹", "⠸"];

    /// Line spinner frames
    pub const LINE: [&str; 4] = ["-", "\\", "|", "/"];

    /// Simple spinner frames
    pub const SIMPLE: [&str; 4] = ["◴", "◷", "◶", "◵"];

    /// Block spinner frames
    pub const BLOCK: [&str; 4] = ["▖", "▘", "▝", "▗"];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_symbols() {
        assert_eq!(line::VERTICAL, "│");
        assert_eq!(line::HORIZONTAL, "─");
        assert_eq!(line::TOP_LEFT, "┌");
        assert_eq!(line::THICK_VERTICAL, "┃");
        assert_eq!(line::DOUBLE_VERTICAL, "║");
        assert_eq!(line::ROUNDED_TOP_LEFT, "╭");
    }

    #[test]
    fn test_block_symbols() {
        assert_eq!(block::FULL, "█");
        assert_eq!(block::HALF, "▌");
        assert_eq!(block::LIGHT_SHADE, "░");
    }

    #[test]
    fn test_bar_symbols() {
        assert_eq!(bar::FULL, "█");
        assert_eq!(bar::HALF, "▄");
        assert_eq!(bar::VERTICAL_BARS.len(), 9);
        assert_eq!(bar::HORIZONTAL_BARS.len(), 9);
    }

    #[test]
    fn test_arrow_symbols() {
        assert_eq!(arrow::UP, "↑");
        assert_eq!(arrow::DOWN, "↓");
        assert_eq!(arrow::LEFT, "←");
        assert_eq!(arrow::RIGHT, "→");
    }

    #[test]
    fn test_special_symbols() {
        assert_eq!(special::BULLET, "•");
        assert_eq!(special::CHECKBOX_CHECKED, "☑");
        assert_eq!(special::STAR, "★");
    }

    #[test]
    fn test_spinner_symbols() {
        assert_eq!(spinner::BRAILLE.len(), 8);
        assert_eq!(spinner::DOTS.len(), 4);
        assert_eq!(spinner::LINE.len(), 4);
    }
}
