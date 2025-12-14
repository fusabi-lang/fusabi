#load "../tui.fsx"

// Create a simple buffer and draw text
let area = createRect 0 0 40 10 in
let buffer = createBuffer area in

// Draw a styled message
let style = emptyStyle |> withFg green |> withBold in
let buffer2 = setBufferString 5 5 "Hello, Fusabi TUI!" style buffer in

// Get buffer dimensions
let dims = match area with | Rect (_, _, w, h) -> (w, h) in
let width = fst dims in
let height = snd dims in

// Print buffer dimensions
printfn "Buffer size: %dx%d" width height
