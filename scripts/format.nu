#!/usr/bin/env nu
# Code formatting script

def main [
    --check            # Check formatting without making changes
] {
    if $check {
        print "🔍 Checking code formatting..."
        check-format
    } else {
        print "✨ Formatting code..."
        format-code
    }
}

def format-code [] {
    # Format Rust code
    print "  Formatting Rust code..."
    cargo fmt

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Rust formatting failed"
        exit 1
    }

    # Format F# code if fantomas is available
    if not (which fantomas | is-empty) {
        print "  Formatting F# code..."
        let fsharp_files = (ls **/*.fs **/*.fsx? | get name)

        if not ($fsharp_files | is-empty) {
            for file in $fsharp_files {
                fantomas $file
            }
        }
    }

    print "✅ Code formatted"
}

def check-format [] {
    # Check Rust formatting
    print "  Checking Rust formatting..."
    cargo fmt --check

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Rust code is not formatted. Run 'just fmt' to fix."
        exit 1
    }

    print "✅ Code formatting is correct"
}

# Entry point
main
