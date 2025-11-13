#!/usr/bin/env nu
# Display version information

def main [] {
    print "📋 FSRS Version Information\n"

    # Project version
    print "Project: FSRS (F#-to-Rust Script Engine)"
    print "Repository: https://github.com/raibid-labs/fsrs\n"

    # Rust toolchain
    print "🦀 Rust Toolchain:"
    rustc --version
    cargo --version
    print ""

    # F# toolchain
    if not (which dotnet | is-empty) {
        print "📘 F# Toolchain:"
        dotnet --version

        if not (which fable | is-empty) {
            print $"Fable: (fable --version)"
        }
        print ""
    }

    # Nushell
    print "🐚 Shell:"
    print $"Nushell: (version | get version)"
    print ""

    # Just
    if not (which just | is-empty) {
        print "⚙️  Build Tools:"
        just --version
    }
}

# Entry point
main
