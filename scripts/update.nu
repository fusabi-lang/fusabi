#!/usr/bin/env nu
# Update dependencies

def main [] {
    print "🔄 Updating dependencies..."

    # Update Rust dependencies
    update-rust-deps

    # Update F# tools
    update-fsharp-tools

    print "✅ Dependencies updated"
}

def update-rust-deps [] {
    print "\n🦀 Updating Rust dependencies..."

    # Check if cargo-edit is installed
    if (which cargo-upgrade | is-empty) {
        print "⚠️  cargo-edit not found. Install with:"
        print "   cargo install cargo-edit"
        return
    }

    cargo upgrade
    cargo update

    print "✅ Rust dependencies updated"
}

def update-fsharp-tools [] {
    if (which dotnet | is-empty) {
        print "⚠️  .NET not found, skipping F# tools update"
        return
    }

    print "\n📘 Updating F# tools..."

    # Update Fable
    dotnet tool update -g fable

    print "✅ F# tools updated"
}

# Entry point
main
