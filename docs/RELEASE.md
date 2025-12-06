# Release Process

This document describes the complete release process for Fusabi.

## Overview

Fusabi follows [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes to public APIs
- **MINOR**: New features, backwards compatible
- **PATCH**: Bug fixes, backwards compatible

Current version: **0.21.0** (check `rust/crates/*/Cargo.toml`)

## Release Checklist

### Pre-Release (1-2 days before)

- [ ] Ensure all planned features/fixes are merged to `main`
- [ ] All CI checks are passing on `main`
- [ ] Review and close issues targeted for this release
- [ ] Update version numbers in all Cargo.toml files:
  - `rust/crates/fusabi-vm/Cargo.toml`
  - `rust/crates/fusabi-frontend/Cargo.toml`
  - `rust/crates/fusabi/Cargo.toml`
  - `rust/crates/fusabi-pm/Cargo.toml`
  - `rust/crates/fusabi-lsp/Cargo.toml`
- [ ] Update README.md version reference if mentioned
- [ ] Regenerate documentation: `nu scripts/gen-docs.nu`
- [ ] Run full test suite: `just test`
- [ ] Run benchmarks to check for regressions: `just bench`
- [ ] Test release build: `just build-release`

### Documentation Snapshot

- [ ] Create version snapshot directory: `docs/versions/vX.Y.Z/`
- [ ] Copy current documentation to version directory:
  ```bash
  mkdir -p docs/versions/v0.X.Y
  cp docs/*.md docs/versions/v0.X.Y/
  cp -r docs/design docs/versions/v0.X.Y/
  cp -r docs/meta docs/versions/v0.X.Y/
  cp docs/STDLIB_REFERENCE.md docs/versions/v0.X.Y/
  ```
- [ ] Update `docs/versions/vNEXT/` with any new documentation
- [ ] Commit documentation snapshot

### Changelog

- [ ] Update `docs/meta/changelog.md` with release notes:
  - New features
  - Bug fixes
  - Breaking changes (if any)
  - Migration guide (if needed)
  - Performance improvements
  - Contributors
- [ ] Link to closed issues and PRs

### Release Execution

#### Option 1: Tag-based Release (Recommended)

1. **Create and push tag:**
   ```bash
   git tag -a v0.X.Y -m "Release v0.X.Y"
   git push origin v0.X.Y
   ```

2. **GitHub Actions automatically:**
   - Creates GitHub release (draft)
   - Builds binaries for all platforms
   - Publishes crates to crates.io (in order):
     1. `fusabi-vm`
     2. `fusabi-frontend`
     3. `fusabi` (CLI)

3. **Review draft release:**
   - Go to GitHub Releases
   - Edit the draft release
   - Add release notes from changelog
   - Add highlights and breaking changes
   - Publish the release

#### Option 2: Manual Release Dispatch

1. **Trigger workflow manually:**
   - Go to Actions > Release workflow
   - Click "Run workflow"
   - Enter version: `v0.X.Y`
   - Click "Run workflow"

2. **Follow same review process as above**

### Publishing Crates

The release workflow publishes crates in the correct order:

1. **fusabi-vm** (core VM, no dependencies on other crates)
2. **fusabi-frontend** (depends on fusabi-vm)
3. **fusabi** (CLI, depends on both)

**Important:** Crates.io has propagation delays. The workflow waits 30 seconds between publishes.

**Requirements:**
- `CARGO_TOKEN` secret must be set in GitHub repository settings
- Token must have publish permissions for all crates

### Post-Release

- [ ] Verify crates published successfully on crates.io:
  - https://crates.io/crates/fusabi-vm
  - https://crates.io/crates/fusabi-frontend
  - https://crates.io/crates/fusabi
- [ ] Verify GitHub release created with all binaries
- [ ] Test installation: `cargo install fusabi`
- [ ] Download and test platform binaries from GitHub release
- [ ] Update version in README badges if needed
- [ ] Announce release:
  - GitHub Discussions
  - Social media (if applicable)
  - Community channels
- [ ] Close release milestone (if used)
- [ ] Create next milestone for upcoming release

## Branch Protection

The `main` branch has the following protections:

- **Require pull request reviews**: At least 1 approval required
- **Require status checks**: Must pass CI before merge
- **Require branches to be up to date**: Ensures latest changes tested
- **Require linear history**: No merge commits (rebase or squash)
- **Dismiss stale reviews**: New commits reset approvals

**Release-related files require review from maintainers** (see `.github/CODEOWNERS`):
- `.github/workflows/release.yml`
- `rust/*/Cargo.toml`
- `docs/RELEASE.md`

## Minimum Supported Rust Version (MSRV)

**Current MSRV: 1.70.0** (Rust 2021 edition)

The MSRV is enforced in CI via the `msrv-check` job.

### Updating MSRV

When updating MSRV:
1. Update `rust-version` in all `Cargo.toml` files
2. Update `.github/workflows/ci.yml` MSRV job
3. Document in release notes as breaking change
4. Test on the new minimum version

## Troubleshooting Releases

### Release workflow failed

1. **Check workflow logs** in GitHub Actions
2. **Common issues:**
   - Missing `CARGO_TOKEN` secret
   - Crate version already published
   - Dependency version mismatch
   - CI checks not passing

### Crate publish failed

If a crate fails to publish mid-release:
1. Check which crate failed in workflow logs
2. Manually publish remaining crates:
   ```bash
   cd rust
   cargo publish -p <crate-name>
   ```
3. Note the issue for post-mortem

### Binary build failed

1. Check platform-specific build logs
2. May need to update cross-compilation dependencies
3. Can manually rebuild and upload to release

## Release Artifacts

Each release includes:

### GitHub Release Assets
- `fusabi-linux-x86_64` - Linux x86_64 binary
- `fusabi-linux-aarch64` - Linux ARM64 binary
- `fusabi-macos-x86_64` - macOS Intel binary
- `fusabi-macos-aarch64` - macOS Apple Silicon binary
- `fusabi-windows-x86_64.exe` - Windows x86_64 binary

### Crates.io Packages
- `fusabi-vm` - Virtual machine library
- `fusabi-frontend` - Parser and compiler
- `fusabi` - CLI tool and embedding library

### Documentation
- Version snapshot in `docs/versions/vX.Y.Z/`
- Updated API docs on docs.rs

## Version Compatibility

### Crate Versioning

All crates follow the same version number for clarity:
- Easier to communicate: "Use Fusabi v0.X.Y"
- Simplifies dependency management
- Clear compatibility story

### Breaking Changes

When introducing breaking changes:
1. Document in migration guide: `docs/migrations/vX-to-vY.md`
2. Update language spec with deprecation warnings
3. Provide migration examples
4. Consider deprecation period for major APIs

### Bytecode Compatibility

The `.fzb` bytecode format version is independent of release version:
- Current bytecode version in VM is tracked separately
- Breaking bytecode changes require migration tools
- Document in `docs/design/bytecode-format.md`

## Security Releases

For security vulnerabilities:
1. **Do not disclose publicly before fix**
2. Create fix in private fork or security branch
3. Follow standard release process
4. Add security advisory to GitHub
5. Publish release with security notice
6. Notify users through all channels

## Hotfix Releases

For critical bugs in production:
1. Branch from release tag: `git checkout -b hotfix/v0.X.Y v0.X.Y`
2. Apply minimal fix
3. Update PATCH version
4. Follow standard release process
5. Merge back to `main`

## Rolling Back a Release

If a release has critical issues:

1. **Yank from crates.io:**
   ```bash
   cargo yank --vers 0.X.Y fusabi
   cargo yank --vers 0.X.Y fusabi-vm
   cargo yank --vers 0.X.Y fusabi-frontend
   ```

2. **Mark GitHub release as pre-release** or delete

3. **Communicate issue** to users

4. **Prepare hotfix** and new release

## Release Schedule

Fusabi follows a **rolling release** model:
- Releases when features are ready
- No fixed schedule
- Aim for releases every 2-4 weeks during active development
- May slow down as project stabilizes

## Questions?

For questions about the release process:
- Open an issue with `release` label
- Contact maintainers
- Check GitHub Actions logs for automated releases

## Appendix: Release Commands

### Quick Release Checklist

```bash
# 1. Update versions in Cargo.toml files (manual edit)
# 2. Update documentation
nu scripts/gen-docs.nu
git add docs/STDLIB_REFERENCE.md

# 3. Create documentation snapshot
mkdir -p docs/versions/v0.X.Y
cp docs/*.md docs/versions/v0.X.Y/
cp -r docs/design docs/versions/v0.X.Y/
cp -r docs/meta docs/versions/v0.X.Y/
git add docs/versions/v0.X.Y

# 4. Update changelog (manual edit docs/meta/changelog.md)

# 5. Commit changes
git commit -m "chore: Prepare release v0.X.Y"

# 6. Create tag
git tag -a v0.X.Y -m "Release v0.X.Y"

# 7. Push to trigger release
git push origin main
git push origin v0.X.Y

# 8. Monitor GitHub Actions
# 9. Review and publish draft release on GitHub
```

### Manual Crate Publishing (if needed)

```bash
cd rust

# Publish in order
cargo publish -p fusabi-vm
sleep 30  # Wait for crates.io propagation

cargo publish -p fusabi-frontend
sleep 30

cargo publish -p fusabi
```
