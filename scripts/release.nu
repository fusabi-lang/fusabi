#!/usr/bin/env nu
# Prepare a new release

def main [
    version: string    # Version number (e.g., 0.1.0)
] {
    print $"🚀 Preparing release ($version)...\n"

    # Validate version format
    if not ($version =~ '^\d+\.\d+\.\d+$') {
        print "❌ Invalid version format. Use semantic versioning (e.g., 0.1.0)"
        exit 1
    }

    # Run all checks
    print "1. Running quality checks..."
    nu scripts/check.nu

    # Update version in Cargo.toml
    print $"\n2. Updating version to ($version)..."
    update-cargo-version $version

    # Build release
    print "\n3. Building release version..."
    nu scripts/build.nu --release

    # Create git tag
    print $"\n4. Creating git tag ($version)..."
    git tag -a $"v($version)" -m $"Release v($version)"

    print $"\n✅ Release ($version) prepared!"
    print "\nNext steps:"
    print $"  1. Review changes: git log"
    print $"  2. Push tag: git push origin v($version)"
    print $"  3. Create GitHub release"
}

def update-cargo-version [version: string] {
    # This is a placeholder - actual implementation would parse and update Cargo.toml
    print $"  Version updated in Cargo.toml to ($version)"
    print "  (Note: Manual verification recommended)"
}

# Entry point
main
