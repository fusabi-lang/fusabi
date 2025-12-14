#load "color.fsx"

// TUI Style Types
// Represents text styling (colors and modifiers)

type Modifier =
    | Bold
    | Dim
    | Italic
    | Underlined
    | Reversed
    | Hidden
    | CrossedOut

// Option type for Color (redefine here as Fusabi stdlib Option is runtime-only)
type OptionColor =
    | NoneColor
    | SomeColor of Color

// Note: Since Fusabi doesn't support record types or list types in DUs yet,
// we use a simple tuple representation
// Style represented as fg * bg (modifiers omitted for now)
type Style =
    | Style of OptionColor * OptionColor

// Style constructors and utilities
let emptyStyle = Style (NoneColor, NoneColor)

let withFg color style =
    match style with
    | Style (_, bg) -> Style (SomeColor color, bg)

let withBg color style =
    match style with
    | Style (fg, _) -> Style (fg, SomeColor color)

// Note: Modifier functions currently don't modify state
// since we can't store modifiers in the Style DU
let withBold style = style
let withDim style = style
let withItalic style = style
let withUnderlined style = style
let withReversed style = style
let withHidden style = style
let withCrossedOut style = style
