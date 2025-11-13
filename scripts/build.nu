#!/usr/bin/env nu
# Build script for FSRS components

def main [
    --release          # Build in release mode
    --verbose (-v)     # Verbose output
] {
    print "🔨 Building FSRS..."

    let build_mode = if $release { "release" } else { "debug" }
    print $"Build mode: ($build_mode)"

    # Build Rust components
    build-rust $release $verbose

    # Build example scripts
    build-examples $verbose

    print "✅ Build complete!"
}

def build-rust [release: bool, verbose: bool] {
    print "\n🦀 Building Rust components..."

    let args = if $release {
        ["build", "--release"]
    } else {
        ["build"]
    }

    let args = if $verbose {
        $args | append "--verbose"
    } else {
        $args
    }

    # Build workspace
    cargo ...$args

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Rust build failed"
        exit 1
    }

    print "✅ Rust components built"
}

def build-examples [verbose: bool] {
    print "\n📚 Building example scripts..."

    # Check if dotnet is available
    if (which dotnet | is-empty) {
        print "⚠️  .NET not found, skipping example builds"
        return
    }

    # Find all .fsx files in examples
    let examples = (ls examples/*.fsx? | get name)

    if ($examples | is-empty) {
        print "  No example scripts found"
        return
    }

    for example in $examples {
        print $"  Transpiling ($example)..."
        nu scripts/transpile.nu $example
    }

    print "✅ Example scripts built"
}

# Entry point
main
