#!/usr/bin/env nu
# Linting script for code quality checks

def main [
    --fix              # Automatically fix issues where possible
    --verbose (-v)     # Verbose output
] {
    print "🔍 Running linters..."

    if $fix {
        lint-with-fix $verbose
    } else {
        lint-check $verbose
    }
}

def lint-check [verbose: bool] {
    print "\n🦀 Running clippy..."

    let args = [
        "clippy",
        "--all-targets",
        "--all-features",
        "--",
        "-D", "warnings"
    ]

    let args = if $verbose {
        $args | append "--verbose"
    } else {
        $args
    }

    cargo ...$args

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Clippy found issues"
        exit 1
    }

    print "✅ No lint issues found"
}

def lint-with-fix [verbose: bool] {
    print "\n🔧 Running clippy with auto-fix..."

    let args = [
        "clippy",
        "--fix",
        "--allow-dirty",
        "--allow-staged",
        "--all-targets",
        "--all-features"
    ]

    let args = if $verbose {
        $args | append "--verbose"
    } else {
        $args
    }

    cargo ...$args

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Clippy fix failed"
        exit 1
    }

    print "✅ Issues fixed where possible"
}

# Entry point
main
