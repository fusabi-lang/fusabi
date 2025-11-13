#!/usr/bin/env nu
# Test runner for FSRS

def main [
    --unit             # Run only unit tests
    --integration      # Run only integration tests
    --coverage         # Generate coverage report
    --filter: string   # Filter tests by name
    --verbose (-v)     # Verbose output
] {
    print "🧪 Running FSRS tests..."

    if $coverage {
        run-coverage $verbose
    } else if $unit {
        run-unit-tests $filter $verbose
    } else if $integration {
        run-integration-tests $filter $verbose
    } else {
        run-all-tests $filter $verbose
    }
}

def run-all-tests [filter: string, verbose: bool] {
    print "\n📦 Running all tests..."

    let args = build-test-args $filter $verbose
    cargo test ...$args

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Tests failed"
        exit 1
    }

    print "✅ All tests passed"
}

def run-unit-tests [filter: string, verbose: bool] {
    print "\n🔬 Running unit tests..."

    let args = build-test-args $filter $verbose
    let args = $args | append "--lib"

    cargo test ...$args

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Unit tests failed"
        exit 1
    }

    print "✅ Unit tests passed"
}

def run-integration-tests [filter: string, verbose: bool] {
    print "\n🔗 Running integration tests..."

    let args = build-test-args $filter $verbose
    let args = $args | append "--test" | append "*"

    cargo test ...$args

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Integration tests failed"
        exit 1
    }

    print "✅ Integration tests passed"
}

def run-coverage [verbose: bool] {
    print "\n📊 Generating coverage report..."

    # Check if cargo-tarpaulin is installed
    if (which cargo-tarpaulin | is-empty) {
        print "❌ cargo-tarpaulin not found. Install with:"
        print "   cargo install cargo-tarpaulin"
        exit 1
    }

    let args = [
        "tarpaulin",
        "--out", "Html",
        "--output-dir", "target/coverage"
    ]

    let args = if $verbose {
        $args | append "--verbose"
    } else {
        $args
    }

    cargo ...$args

    if $env.LAST_EXIT_CODE == 0 {
        print "✅ Coverage report generated: target/coverage/index.html"
    } else {
        print "❌ Coverage generation failed"
        exit 1
    }
}

def build-test-args [filter: string, verbose: bool] {
    let args = ["test"]

    let args = if ($filter | is-not-empty) {
        $args | append $filter
    } else {
        $args
    }

    let args = if $verbose {
        $args | append "--verbose"
    } else {
        $args | append "--quiet"
    }

    $args
}

# Entry point
main
