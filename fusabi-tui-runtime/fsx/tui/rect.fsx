// TUI Rect Types
// Represents rectangular areas in the terminal

// Note: Since Fusabi doesn't support record types yet, we use DUs
// Rect represented as x * y * width * height
type Rect =
    | Rect of int * int * int * int

// Rect constructors and utilities
let createRect x y width height = Rect (x, y, width, height)
let emptyRect = Rect (0, 0, 0, 0)

let areaOf rect =
    match rect with
    | Rect (_, _, w, h) -> w * h

let leftOf rect =
    match rect with
    | Rect (x, _, _, _) -> x

let rightOf rect =
    match rect with
    | Rect (x, _, w, _) -> x + w

let topOf rect =
    match rect with
    | Rect (_, y, _, _) -> y

let bottomOf rect =
    match rect with
    | Rect (_, y, _, h) -> y + h

let innerRect margin rect =
    match rect with
    | Rect (x, y, w, h) ->
        let newW = if w > margin * 2 then w - margin * 2 else 0 in
        let newH = if h > margin * 2 then h - margin * 2 else 0 in
        Rect (x + margin, y + margin, newW, newH)
