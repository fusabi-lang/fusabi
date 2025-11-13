# FSRS - F#-to-Rust Script Engine
# Just command runner for development tasks

# Default recipe to display help
default:
    @just --list

# Setup: Install all dependencies and prepare development environment
setup:
    @nu scripts/setup.nu

# Build: Compile all components (host, runtime, examples)
build:
    @nu scripts/build.nu

# Build in release mode
build-release:
    @nu scripts/build.nu --release

# Test: Run all test suites
test:
    @nu scripts/test.nu

# Test with specific filter
test-filter filter:
    @nu scripts/test.nu --filter {{filter}}

# Test: Run only unit tests
test-unit:
    @nu scripts/test.nu --unit

# Test: Run only integration tests
test-integration:
    @nu scripts/test.nu --integration

# Coverage: Generate test coverage report
coverage:
    @nu scripts/test.nu --coverage

# Dev: Start development mode with watch and hot-reload
dev:
    @nu scripts/dev.nu watch

# Transpile: Convert F# script to Rust
transpile file:
    @nu scripts/transpile.nu {{file}}

# Run: Execute an F# script in the host runtime
run script *args:
    @nu scripts/run.nu {{script}} {{args}}

# Example: Run a specific example by name
example name *args:
    @nu scripts/run.nu examples/{{name}}.fsx {{args}}

# Format: Auto-format all code (Rust + F#)
fmt:
    @nu scripts/format.nu

# Format check: Verify formatting without changes
fmt-check:
    @nu scripts/format.nu --check

# Lint: Run all linters (clippy, etc.)
lint:
    @nu scripts/lint.nu

# Lint: Run with auto-fix where possible
lint-fix:
    @nu scripts/lint.nu --fix

# Check: Run all quality checks (format, lint, test)
check:
    @nu scripts/check.nu

# Audit: Security audit of dependencies
audit:
    @nu scripts/audit.nu

# Clean: Remove all build artifacts
clean:
    @nu scripts/clean.nu

# Clean: Remove all build artifacts and caches (deep clean)
clean-all:
    @nu scripts/clean.nu --all

# Docs: Generate and open documentation
docs:
    @nu scripts/docs.nu

# Docs: Build documentation without opening
docs-build:
    @nu scripts/docs.nu --build-only

# Bench: Run performance benchmarks
bench:
    @nu scripts/bench.nu

# Bench: Run specific benchmark
bench-filter filter:
    @nu scripts/bench.nu --filter {{filter}}

# Watch: Watch files and rebuild on changes
watch:
    @nu scripts/dev.nu watch

# Watch: Watch and run tests on changes
watch-test:
    @nu scripts/dev.nu watch-test

# Install: Install the host runtime binary
install:
    @nu scripts/install.nu

# Uninstall: Remove installed binary
uninstall:
    @nu scripts/install.nu --uninstall

# Update: Update dependencies
update:
    @nu scripts/update.nu

# Version: Display version information
version:
    @nu scripts/version.nu

# Release: Prepare a new release
release version:
    @nu scripts/release.nu {{version}}

# CI: Run CI checks locally
ci:
    @nu scripts/ci.nu

# Init: Initialize a new F# script project
init name:
    @nu scripts/init.nu {{name}}

# REPL: Start interactive REPL for testing scripts
repl:
    @nu scripts/repl.nu

# Profile: Profile script execution performance
profile script:
    @nu scripts/profile.nu {{script}}

# Flamegraph: Generate flamegraph for profiling
flamegraph script:
    @nu scripts/profile.nu --flamegraph {{script}}
