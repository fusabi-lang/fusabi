#!/usr/bin/env nu
# Script execution runner

def main [
    script: string     # F# script file to run
    ...args: string    # Arguments to pass to the script
] {
    print $"🚀 Running ($script)..."

    # Check if script exists
    if not ($script | path exists) {
        print $"❌ Script not found: ($script)"
        exit 1
    }

    # Check if host binary exists
    let binary = find-host-binary

    # Transpile if needed
    let transpiled = ensure-transpiled $script

    # Run in host
    ^$binary run $transpiled ...$args

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Script execution failed"
        exit 1
    }
}

def find-host-binary [] {
    let release_bin = "target/release/fsrs-host"
    let debug_bin = "target/debug/fsrs-host"

    if ($release_bin | path exists) {
        $release_bin
    } else if ($debug_bin | path exists) {
        $debug_bin
    } else {
        print "❌ Host binary not found. Run 'just build' first."
        exit 1
    }
}

def ensure-transpiled [script: string] {
    let transpiled = $"target/transpiled/($script | path basename | str replace '.fsx' '.rs' | str replace '.fs' '.rs')"

    # Check if transpiled version exists and is newer
    if ($transpiled | path exists) {
        let script_time = (ls $script | get modified | first)
        let transpiled_time = (ls $transpiled | get modified | first)

        if $transpiled_time > $script_time {
            return $transpiled
        }
    }

    # Transpile
    print "  Transpiling script..."
    nu scripts/transpile.nu $script

    $transpiled
}

# Entry point
main
