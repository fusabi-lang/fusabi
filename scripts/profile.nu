#!/usr/bin/env nu
# Profile script execution performance

def main [
    script: string     # Script to profile
    --flamegraph       # Generate flamegraph
] {
    print $"🔥 Profiling ($script)...\n"

    # Check if script exists
    if not ($script | path exists) {
        print $"❌ Script not found: ($script)"
        exit 1
    }

    if $flamegraph {
        generate-flamegraph $script
    } else {
        basic-profiling $script
    }
}

def basic-profiling [script: string] {
    print "Running basic performance profiling..."

    # Check if host binary exists
    let binary = find-host-binary

    if ($binary | is-empty) {
        print "❌ Host binary not found. Run 'just build' first."
        exit 1
    }

    # Run with time profiling
    let start = (date now)
    ^$binary run $script
    let end = (date now)

    let duration = ($end - $start)
    print $"\n⏱️  Execution time: ($duration)"
}

def generate-flamegraph [script: string] {
    print "Generating flamegraph..."

    # Check if cargo-flamegraph is installed
    if (which cargo-flamegraph | is-empty) {
        print "❌ cargo-flamegraph not found. Install with:"
        print "   cargo install cargo-flamegraph"
        exit 1
    }

    # Generate flamegraph
    cargo flamegraph --bin fsrs-host -- run $script

    if $env.LAST_EXIT_CODE == 0 {
        print "✅ Flamegraph generated: flamegraph.svg"
    } else {
        print "❌ Flamegraph generation failed"
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
        null
    }
}

# Entry point
main
