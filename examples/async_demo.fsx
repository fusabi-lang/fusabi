let log msg =
    let t = Time.format "%H:%M:%S" (Time.now())
    printfn (sprintf "[%s] %s" [t; msg])

let sleep ms = async {
    log (sprintf "Sleeping for %d ms..." [ms])
    let _ = Process.runShell "echo 'zzzzzzz'"
    log "Woke up!"
    return ms
}

let fetchData url = async {
    log (sprintf "Fetching %s..." [url])
    do! sleep 100
    return (sprintf "Content of %s" [url])
}

let main = async {
    log "Starting workflow..."
    
    let! d1 = fetchData "url1"
    log (sprintf "Got: %s" [d1])
    
    return "Done"
}

let res = Async.RunSynchronously main
log (sprintf "Result: %s" [res])