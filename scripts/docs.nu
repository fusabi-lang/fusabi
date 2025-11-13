#!/usr/bin/env nu
# Documentation generation

def main [
    --build-only       # Build docs without opening browser
] {
    print "📚 Generating documentation..."

    # Generate Rust docs
    cargo doc --no-deps --all-features

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Documentation generation failed"
        exit 1
    }

    print "✅ Documentation generated"

    if not $build_only {
        print "  Opening documentation in browser..."
        cargo doc --no-deps --all-features --open
    } else {
        print $"  Documentation available at: target/doc/index.html"
    }
}

# Entry point
main
