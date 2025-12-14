// Fusabi Async Tokio Demo
// Demonstrates Tokio-backed real async operations

let log msg =
    printfn (sprintf "[Async] %s" [msg])

// Example 1: Simple async sleep
let example1 () =
    log "Example 1: Async.sleep"
    let task = Async.sleep 1000
    log "Task created, now awaiting..."
    let result = Async.await task
    log "Sleep completed!"
    result

// Example 2: Parallel execution
let example2 () =
    log "Example 2: Async.parallel"

    // Create multiple sleep tasks
    let task1 = Async.sleep 500
    let task2 = Async.sleep 300
    let task3 = Async.sleep 400

    // Run them in parallel
    let tasks = [task1; task2; task3]
    let parallel_task = Async.parallel tasks

    log "Waiting for all tasks to complete..."
    let results = Async.await parallel_task
    log "All tasks completed!"
    results

// Example 3: Timeout handling
let example3 () =
    log "Example 3: Async.withTimeout"

    // Create a long-running task
    let long_task = Async.sleep 5000

    // Apply a 1-second timeout
    let timeout_task = Async.withTimeout 1000 long_task

    log "Waiting with 1-second timeout on 5-second task..."
    let result = Async.await timeout_task

    match result with
    | Some(_) -> log "Task completed in time"
    | None -> log "Task timed out!"

    result

// Example 4: Error handling
let example4 () =
    log "Example 4: Async.catch"

    // For demonstration, we'll use a successful task
    // In real usage, this would be a task that might fail
    let task = Async.sleep 100
    let caught_task = Async.catch task

    log "Awaiting task with error handling..."
    let result = Async.await caught_task

    match result with
    | Ok(v) ->
        log "Task succeeded"
        v
    | Error(e) ->
        log (sprintf "Task failed: %s" [e])
        ()

// Example 5: Task cancellation
let example5 () =
    log "Example 5: Async.cancel"

    // Create a long task
    let task = Async.sleep 3000

    // Poll to check status
    let status = Async.poll task
    log (sprintf "Initial status: %s" [status])

    // Cancel the task
    let _ = Async.cancel task
    log "Task cancelled"

    // Check status again
    let status2 = Async.poll task
    log (sprintf "Status after cancel: %s" [status2])
    ()

// Run all examples
log "========================================="
log "Starting Tokio Async Demonstrations"
log "========================================="

let _ = example1()
let _ = example2()
let _ = example3()
let _ = example4()
let _ = example5()

log "========================================="
log "All examples completed!"
log "========================================="
