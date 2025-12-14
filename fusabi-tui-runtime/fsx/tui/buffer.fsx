#load "rect.fsx"
#load "cell.fsx"

// TUI Buffer Types
// Represents a 2D grid of cells

// Note: Since Fusabi doesn't support record types or list types in DUs yet,
// we use a simplified representation
// Buffer represented as just the area (content omitted)
type Buffer =
    | Buffer of Rect

// Buffer constructors and utilities
let createBuffer area = Buffer area

let indexInBuffer x y buffer =
    match buffer with
    | Buffer (Rect (_, rectY, rectW, _)) ->
        let rectX = match buffer with | Buffer (Rect (rx, _, _, _)) -> rx in
        (y - rectY) * rectW + (x - rectX)

// Note: These functions are stubs since we can't store cell lists
let getCell x y buffer = emptyCell

let setCell x y cell buffer = buffer

let setBufferString x y text style buffer = buffer
