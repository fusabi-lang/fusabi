// Test Map support
// Note: Map functions take a list of (key, value) tuples
let entries = [("name", "Alice"); ("age", 30); ("city", "NYC")]
let m = Map.ofList entries

let name = Map.find "name" m
name