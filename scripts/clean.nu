#!/usr/bin/env nu
# Clean build artifacts and caches

def main [
    --all              # Deep clean including caches
] {
    if $all {
        deep-clean
    } else {
        clean-build
    }
}

def clean-build [] {
    print "🧹 Cleaning build artifacts..."

    # Clean Rust build artifacts
    cargo clean

    # Clean transpiled files
    if ("target/transpiled" | path exists) {
        rm -rf target/transpiled
        print "  Removed target/transpiled"
    }

    # Clean coverage reports
    if ("target/coverage" | path exists) {
        rm -rf target/coverage
        print "  Removed target/coverage"
    }

    print "✅ Build artifacts cleaned"
}

def deep-clean [] {
    print "🧹 Deep cleaning (including caches)..."

    # Run regular clean
    clean-build

    # Clean Cargo cache (registry and git)
    let cargo_home = ($env.CARGO_HOME? | default $"($env.HOME)/.cargo")

    print $"  Cleaning Cargo registry cache..."
    if ($"($cargo_home)/registry" | path exists) {
        rm -rf $"($cargo_home)/registry"
    }

    # Clean .NET caches if applicable
    if (which dotnet | is-not-empty) {
        print "  Cleaning .NET caches..."
        dotnet nuget locals all --clear
    }

    print "✅ Deep clean complete"
}

# Entry point
main
