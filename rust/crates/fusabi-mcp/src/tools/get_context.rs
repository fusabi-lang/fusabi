use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::protocol::{CallToolResponse, ToolContent};

pub async fn execute(args: HashMap<String, Value>) -> Result<CallToolResponse> {
    let context_type = args
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("all");

    let content = match context_type {
        "syntax" => get_syntax_info(),
        "examples" => get_examples(),
        "stdlib" => get_stdlib_info(),
        "all" => {
            format!(
                "{}\n\n{}\n\n{}",
                get_syntax_info(),
                get_examples(),
                get_stdlib_info()
            )
        }
        _ => {
            return Ok(CallToolResponse {
                content: vec![ToolContent::Error {
                    error: format!("Unknown context type: {}", context_type),
                }],
            })
        }
    };

    Ok(CallToolResponse {
        content: vec![ToolContent::Text { text: content }],
    })
}

fn get_syntax_info() -> String {
    r#"# Fusabi Language Syntax

## Basic Types
- Numbers: `42`, `3.14`, `1.5e10`
- Strings: `"hello"`, `'world'`
- Booleans: `true`, `false`
- Null: `null`

## Variables
```fusabi
let x = 10;
const PI = 3.14159;
x = 20; // OK for let
// PI = 3; // Error for const
```

## Functions
```fusabi
fn add(a, b) {
    return a + b;
}

// Arrow functions
let multiply = (a, b) => a * b;

// Anonymous functions
let divide = fn(a, b) { a / b };
```

## Control Flow
```fusabi
// If expressions
let result = if x > 0 { "positive" } else { "non-positive" };

// Match expressions
let description = match value {
    0 => "zero",
    1..10 => "small",
    _ => "large"
};

// Loops
for i in 0..10 {
    print(i);
}

while condition {
    // ...
}

loop {
    if done { break; }
}
```

## Data Structures
```fusabi
// Arrays
let arr = [1, 2, 3];
arr[0]; // 1

// Objects
let obj = { name: "Fusabi", version: 1 };
obj.name; // "Fusabi"

// Tuples
let tuple = (1, "hello", true);
let (a, b, c) = tuple; // Destructuring
```

## Pattern Matching
```fusabi
match value {
    Some(x) => print(x),
    None => print("nothing"),
    [first, ...rest] => print(first),
    { x, y } => print(x + y),
}
```

## Modules
```fusabi
// Define module
module math {
    export fn sqrt(x) { /* ... */ }
    export const PI = 3.14159;
}

// Import
import { sqrt, PI } from math;
import * as math from math;
```

## Type System (with inference)
```fusabi
// Types are inferred
let x = 10; // inferred as Int
let y = 3.14; // inferred as Float

// Function with inferred types
fn add(a, b) { a + b } // Types inferred from usage
```"#.to_string()
}

fn get_examples() -> String {
    r#"# Fusabi Code Examples

## Hello World
```fusabi
print("Hello, World!");
```

## Fibonacci
```fusabi
fn fibonacci(n) {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

print(fibonacci(10)); // 55
```

## FizzBuzz
```fusabi
for i in 1..=100 {
    let output = match (i % 3 == 0, i % 5 == 0) {
        (true, true) => "FizzBuzz",
        (true, false) => "Fizz",
        (false, true) => "Buzz",
        _ => i.to_string()
    };
    print(output);
}
```

## Quick Sort
```fusabi
fn quicksort(arr) {
    if arr.length <= 1 {
        return arr;
    }

    let pivot = arr[arr.length / 2];
    let less = arr.filter(|x| x < pivot);
    let equal = arr.filter(|x| x == pivot);
    let greater = arr.filter(|x| x > pivot);

    return [...quicksort(less), ...equal, ...quicksort(greater)];
}

let sorted = quicksort([3, 1, 4, 1, 5, 9, 2, 6]);
print(sorted); // [1, 1, 2, 3, 4, 5, 6, 9]
```

## Classes and Objects
```fusabi
class Person {
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }

    greet() {
        print("Hi, I'm " + this.name);
    }
}

let alice = Person("Alice", 30);
alice.greet(); // Hi, I'm Alice
```

## Async/Await
```fusabi
async fn fetchData(url) {
    let response = await fetch(url);
    return await response.json();
}

async fn main() {
    let data = await fetchData("https://api.example.com/data");
    print(data);
}
```"#.to_string()
}

fn get_stdlib_info() -> String {
    r#"# Fusabi Standard Library

## Core Functions
- `print(value)` - Output a value
- `input(prompt?)` - Read user input
- `typeof(value)` - Get type of value
- `assert(condition, message?)` - Assert condition is true

## Math Module
- `math.abs(x)` - Absolute value
- `math.sqrt(x)` - Square root
- `math.pow(x, y)` - x to the power of y
- `math.sin(x)`, `math.cos(x)`, `math.tan(x)` - Trigonometric functions
- `math.floor(x)`, `math.ceil(x)`, `math.round(x)` - Rounding
- `math.random()` - Random number [0, 1)
- `math.PI`, `math.E` - Mathematical constants

## String Methods
- `str.length` - String length
- `str.charAt(index)` - Character at index
- `str.indexOf(search)` - Find substring
- `str.slice(start, end?)` - Extract substring
- `str.split(separator)` - Split into array
- `str.replace(search, replace)` - Replace occurrences
- `str.toUpper()`, `str.toLower()` - Case conversion
- `str.trim()` - Remove whitespace

## Array Methods
- `arr.length` - Array length
- `arr.push(item)` - Add to end
- `arr.pop()` - Remove from end
- `arr.shift()` - Remove from start
- `arr.unshift(item)` - Add to start
- `arr.slice(start, end?)` - Extract subarray
- `arr.map(fn)` - Transform elements
- `arr.filter(fn)` - Filter elements
- `arr.reduce(fn, initial?)` - Reduce to value
- `arr.forEach(fn)` - Iterate elements
- `arr.find(fn)` - Find first match
- `arr.findIndex(fn)` - Find first index
- `arr.includes(item)` - Check if contains
- `arr.sort(compareFn?)` - Sort in place
- `arr.reverse()` - Reverse in place

## Object Methods
- `Object.keys(obj)` - Get property names
- `Object.values(obj)` - Get property values
- `Object.entries(obj)` - Get [key, value] pairs
- `Object.assign(target, ...sources)` - Merge objects
- `Object.freeze(obj)` - Make immutable
- `Object.hasOwnProperty(obj, prop)` - Check property

## File System (io module)
- `io.readFile(path)` - Read file contents
- `io.writeFile(path, content)` - Write file
- `io.appendFile(path, content)` - Append to file
- `io.deleteFile(path)` - Delete file
- `io.exists(path)` - Check if exists
- `io.mkdir(path)` - Create directory

## Network (http module)
- `http.get(url)` - GET request
- `http.post(url, data)` - POST request
- `http.put(url, data)` - PUT request
- `http.delete(url)` - DELETE request

## JSON
- `JSON.parse(string)` - Parse JSON string
- `JSON.stringify(value)` - Convert to JSON string

## Time
- `time.now()` - Current timestamp
- `time.sleep(ms)` - Delay execution
- `time.setTimeout(fn, ms)` - Delayed execution
- `time.setInterval(fn, ms)` - Repeated execution"#.to_string()
}

pub fn get_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "type": {
                "type": "string",
                "enum": ["syntax", "examples", "stdlib", "all"],
                "description": "Type of context to retrieve",
                "default": "all"
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_context() {
        let args = HashMap::new();
        let result = execute(args).await.unwrap();

        assert_eq!(result.content.len(), 1);
        match &result.content[0] {
            ToolContent::Text { text } => {
                assert!(text.contains("Fusabi Language Syntax"));
                assert!(text.contains("Fusabi Code Examples"));
                assert!(text.contains("Fusabi Standard Library"));
            }
            _ => panic!("Expected text result"),
        }
    }

    #[tokio::test]
    async fn test_get_specific_context() {
        let mut args = HashMap::new();
        args.insert("type".to_string(), json!("syntax"));

        let result = execute(args).await.unwrap();
        assert_eq!(result.content.len(), 1);

        match &result.content[0] {
            ToolContent::Text { text } => {
                assert!(text.contains("Fusabi Language Syntax"));
                assert!(!text.contains("Fusabi Code Examples"));
            }
            _ => panic!("Expected text result"),
        }
    }
}