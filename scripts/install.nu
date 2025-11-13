#!/usr/bin/env nu
# Install/uninstall the FSRS host binary

def main [
    --uninstall        # Uninstall the binary
] {
    if $uninstall {
        uninstall-binary
    } else {
        install-binary
    }
}

def install-binary [] {
    print "📦 Installing FSRS host binary..."

    cargo install --path .

    if $env.LAST_EXIT_CODE != 0 {
        print "❌ Installation failed"
        exit 1
    }

    print "✅ FSRS host installed successfully"
    print "   Run 'fsrs-host --help' to get started"
}

def uninstall-binary [] {
    print "📦 Uninstalling FSRS host binary..."

    cargo uninstall fsrs-host

    if $env.LAST_EXIT_CODE != 0 {
        print "⚠️  Uninstall failed or binary not found"
        exit 1
    }

    print "✅ FSRS host uninstalled"
}

# Entry point
main
