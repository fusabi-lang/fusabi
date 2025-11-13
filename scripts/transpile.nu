#!/usr/bin/env nu
# F# to Rust transpilation script

def main [
    file: string       # F# source file (.fs or .fsx)
    --output: string   # Output directory (default: target/transpiled)
    --verbose (-v)     # Verbose output
] {
    print $"🔄 Transpiling ($file)..."

    # Check if file exists
    if not ($file | path exists) {
        print $"❌ File not found: ($file)"
        exit 1
    }

    # Check if dotnet and fable are available
    check-fable

    # Determine output directory
    let output_dir = if ($output | is-empty) {
        "target/transpiled"
    } else {
        $output
    }

    # Create output directory
    mkdir $output_dir

    # Run Fable transpiler
    transpile-with-fable $file $output_dir $verbose

    print $"✅ Transpiled to ($output_dir)"
}

def check-fable [] {
    if (which dotnet | is-empty) {
        print "❌ .NET SDK not found"
        print "   Install from https://dotnet.microsoft.com/download"
        exit 1
    }

    if (which fable | is-empty) {
        print "❌ Fable not found"
        print "   Install with: dotnet tool install -g fable"
        exit 1
    }
}

def transpile-with-fable [file: string, output_dir: string, verbose: bool] {
    let args = [
        $file,
        "--lang", "rust",
        "--outDir", $output_dir
    ]

    let args = if $verbose {
        $args | append "--verbose"
    } else {
        $args
    }

    # Run fable
    dotnet fable ...$args

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Transpilation failed"
        exit 1
    }
}

# Entry point
main
