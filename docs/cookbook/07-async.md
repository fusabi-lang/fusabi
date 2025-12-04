# Async

Fusabi supports async operations through computation expressions, providing a clean syntax for asynchronous workflows.

## Basic Async Block

Use the `async { }` syntax to create async computations:

```fsharp
// examples/async_demo.fsx
let log msg =
    print msg
    print "\n"

let main = async {
    log "Inside async"
    return "Done"
}

log "Calling RunSync"
let _ = Async.RunSynchronously main
log "Done"
```

## Running Async Computations

### Synchronous Execution

Use `Async.RunSynchronously` to block until completion:

```fsharp
let computation = async {
    return 42
}

let result = Async.RunSynchronously computation
// result = 42
```

## Return Values

The `return` keyword wraps a value in the async context:

```fsharp
let getNumber = async {
    return 42
}

let getMessage = async {
    return "Hello from async!"
}
```

## Sequencing Operations

Operations in an async block execute in order:

```fsharp
let workflow = async {
    print "Step 1\n"
    print "Step 2\n"
    print "Step 3\n"
    return "Complete"
}

Async.RunSynchronously workflow
// Output:
// Step 1
// Step 2
// Step 3
```

## Combining with Let

Use let bindings inside async blocks:

```fsharp
let compute = async {
    let x = 10
    let y = 20
    let sum = x + y
    return sum
}

let result = Async.RunSynchronously compute  // 30
```

## Side Effects in Async

Async blocks can perform I/O and other side effects:

```fsharp
let loggedComputation = async {
    print "Starting computation...\n"
    let result = 2 + 2
    print "Computation complete!\n"
    return result
}
```

## Practical Example: Timed Operation

```fsharp
let timedOperation = async {
    let start = Time.now ()
    // Perform some work
    let result = List.map (fun x -> x * 2) [1; 2; 3; 4; 5]
    let elapsed = Time.now () - start
    print "Elapsed: "
    print elapsed
    print "ms\n"
    return result
}
```

## Error Handling Pattern

Combine async with Result for error handling:

```fsharp
let safeComputation input = async {
    if input > 0 then
        return Ok (input * 2)
    else
        return Error "Input must be positive"
}

let result = Async.RunSynchronously (safeComputation 5)
// result = Ok 10
```

## Async with System Operations

Combine async with system modules:

```fsharp
let shellCommand = async {
    let result = Process.runShell "echo 'Hello from shell'"
    return result.stdout
}

let output = Async.RunSynchronously shellCommand
```

## Pattern: Async Wrapper

Wrap synchronous operations in async for consistent interfaces:

```fsharp
let asyncRead filename = async {
    // In the future, this could be truly async
    let content = File.read filename
    return content
}

let asyncWrite filename content = async {
    File.write filename content
    return ()
}
```

## Computation Expression Syntax

The async block is a computation expression that desugars to builder method calls:

```fsharp
// This async block:
async {
    let x = 1
    let y = 2
    return x + y
}

// Is equivalent to builder calls like:
// Async.Bind(..., fun x ->
//   Async.Bind(..., fun y ->
//     Async.Return(x + y)))
```

## When to Use Async

Use async when:

- Performing I/O operations (file, network)
- Running shell commands
- Operations that may take time
- Building composable workflows

Don't use async for:

- Pure computations
- Simple synchronous operations
- Performance-critical tight loops

## Next Steps

Learn about [I/O operations](08-io.md) for file and terminal interaction.
