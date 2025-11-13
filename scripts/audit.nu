#!/usr/bin/env nu
# Security audit for dependencies

def main [] {
    print "🔒 Running security audit..."

    # Check if cargo-audit is installed
    if (which cargo-audit | is-empty) {
        print "⚠️  cargo-audit not found. Install with:"
        print "   cargo install cargo-audit"
        print "   Skipping audit..."
        return
    }

    cargo audit

    if $env.LAST_EXIT_CODE != 0 {
        print "⚠️  Security vulnerabilities found!"
        print "   Review the issues above and update dependencies"
        exit 1
    }

    print "✅ No known security vulnerabilities"
}

# Entry point
main
