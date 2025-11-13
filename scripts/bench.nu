#!/usr/bin/env nu
# Performance benchmarking

def main [
    --filter: string   # Filter benchmarks by name
] {
    print "⚡ Running benchmarks..."

    let args = ["bench"]

    let args = if ($filter | is-not-empty) {
        $args | append $filter
    } else {
        $args
    }

    cargo ...$args

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Benchmarks failed"
        exit 1
    }

    print "✅ Benchmarks complete"
}

# Entry point
main
