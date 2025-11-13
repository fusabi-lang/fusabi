#!/usr/bin/env nu
# Start FSRS REPL for interactive F# script execution

def main [] {
    print "🎮 FSRS REPL - Interactive F# Script Execution"
    print "================================================\n"

    # Check if host binary exists
    let binary = find-host-binary

    if ($binary | is-empty) {
        print "❌ Host binary not found. Run 'just build' first."
        exit 1
    }

    print "Starting REPL..."
    print "Type F# expressions and press Enter to execute"
    print "Type 'exit' or press Ctrl+C to quit\n"

    # Start the REPL
    ^$binary --repl
}

def find-host-binary [] {
    let release_bin = "target/release/fsrs-host"
    let debug_bin = "target/debug/fsrs-host"

    if ($release_bin | path exists) {
        $release_bin
    } else if ($debug_bin | path exists) {
        $debug_bin
    } else {
        null
    }
}

# Entry point
main
