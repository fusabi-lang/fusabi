// TUI Color Types
// Represents terminal colors

type Color =
    | Black
    | White
    | Red
    | Green
    | Blue
    | Yellow
    | Cyan
    | Magenta
    | Rgb of int * int * int
    | Indexed of int

// Color constructors
let black = Black
let white = White
let red = Red
let green = Green
let blue = Blue
let yellow = Yellow
let cyan = Cyan
let magenta = Magenta
let rgb r g b = Rgb (r, g, b)
let indexed i = Indexed i
