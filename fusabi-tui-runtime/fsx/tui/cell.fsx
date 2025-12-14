#load "color.fsx"
#load "style.fsx"

// TUI Cell Types
// Represents a single terminal cell with character and style

// Note: Since Fusabi doesn't support record types yet, we use DUs
// Cell represented as symbol * style
type Cell =
    | Cell of string * Style

// Cell constructors and utilities
let emptyCell = Cell (" ", emptyStyle)

let createCell symbol style = Cell (symbol, style)

let cellFromChar ch = Cell (String.ofChar ch, emptyStyle)

let styledCell symbol style = Cell (symbol, style)
