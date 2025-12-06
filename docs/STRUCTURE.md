# Documentation Structure

This document describes the required structure and organization of Fusabi documentation.

## Directory Layout

```
docs/
├── STRUCTURE.md           # This file - documentation structure guide
├── RELEASE.md            # Release process documentation
├── versions/             # Versioned documentation snapshots
│   ├── v0.12.0/         # Docs for version 0.12.0
│   ├── v0.13.0/         # Docs for version 0.13.0
│   └── vNEXT/           # Upcoming release documentation
├── 01-overview.md        # Current: Language overview
├── 02-language-spec.md   # Current: Language specification
├── 03-vm-design.md       # Current: VM design document
├── STDLIB_REFERENCE.md   # Current: Standard library API reference
├── design/              # Design documents and RFCs
│   ├── ABI.md
│   ├── bytecode-format.md
│   ├── embedding-guide.md
│   ├── host-interop.md
│   ├── module_system.md
│   ├── package-management.md
│   ├── RFC-002-ASYNC-CE.md
│   ├── RFD-001-MCP-DSL.md
│   └── SECURITY.md
├── meta/                # Project meta-documentation
│   ├── BRANDING.md
│   ├── ci-cd.md
│   ├── development.md
│   ├── OMAKASE.md
│   ├── roadmap.md
│   ├── setup.md
│   ├── testing.md
│   └── toc.md
├── cookbook/            # Code examples and recipes
├── packages/            # Package management documentation
├── stdlib/              # Standard library detailed docs
└── workstreams/         # Development workstream tracking

```

## Required Sections

### Root Level Documentation

1. **01-overview.md** - High-level language overview
   - What is Fusabi?
   - Key features
   - Quick start guide
   - Links to detailed documentation

2. **02-language-spec.md** - Complete language specification
   - Syntax and semantics
   - Type system
   - Supported F# features
   - Language limitations

3. **03-vm-design.md** - Virtual Machine architecture
   - Bytecode format
   - Execution model
   - Performance characteristics

4. **STDLIB_REFERENCE.md** - Standard library API reference
   - Generated from source code
   - Must be kept in sync with implementation
   - Updated via `nu scripts/gen-docs.nu`

### Design Documents (design/)

Design documents cover architectural decisions and major features:
- Must include motivation and design rationale
- Should reference related issues/PRs
- RFCs follow RFC-XXX-TITLE.md naming
- RFDs (Requests for Discussion) follow RFD-XXX-TITLE.md naming

### Meta Documentation (meta/)

Project-level documentation:
- **BRANDING.md** - Visual identity and brand guidelines
- **ci-cd.md** - CI/CD pipeline documentation
- **development.md** - Development workflow
- **roadmap.md** - Product roadmap
- **setup.md** - Development environment setup
- **testing.md** - Testing strategy
- **toc.md** - Documentation table of contents

### Versioned Documentation (versions/)

Each release must have a snapshot of documentation:
- Copy current docs to `versions/vX.Y.Z/` on release
- `vNEXT/` contains upcoming release documentation
- Allows users to reference docs for their installed version

## Documentation Standards

### Markdown Requirements
- All documentation must be valid Markdown
- Follow `.markdownlint.json` rules
- Use relative links for internal references
- Include code examples where appropriate

### File Naming
- Use lowercase with hyphens for multi-word files
- RFCs: `RFC-XXX-TITLE.md`
- RFDs: `RFD-XXX-TITLE.md`
- Numbered guides: `NN-title.md` (01-overview.md)

### Content Guidelines
1. **Be Concise** - Get to the point quickly
2. **Show Examples** - Include runnable code samples
3. **Stay Current** - Keep docs in sync with code
4. **Link Appropriately** - Reference related docs
5. **No AI Notes** - Remove process notes, keep distilled content

## Forbidden Content

The following should NEVER be committed to the repository:
- AI assistant notes or conversation logs
- Process/orchestration instructions
- Review checklists meant for AI
- Temporary work-in-progress notes
- Personal development notes

Keep only distilled, user-facing documentation.

## CI Checks

Documentation freshness is enforced in CI:
1. **Stdlib Reference** - Must match generated output from source
2. **Markdown Lint** - All .md files must pass linting
3. **Structure Check** - Validates required files exist

See `.github/workflows/ci.yml` for implementation details.

## Updating Documentation

### For New Features
1. Update relevant spec documents
2. Add examples to cookbook/
3. Update STDLIB_REFERENCE.md (if applicable)
4. Regenerate: `nu scripts/gen-docs.nu`
5. Update roadmap.md to mark feature as complete

### For Breaking Changes
1. Document in migration guide (create if needed)
2. Update language spec
3. Update examples
4. Note in RELEASE.md for next version

### For Releases
1. Copy current docs to `versions/vX.Y.Z/`
2. Update version references
3. Generate fresh STDLIB_REFERENCE.md
4. Update changelog
5. Verify all CI checks pass

## Migration Guides

When breaking changes occur, create migration guides:
- Location: `docs/migrations/vX-to-vY.md`
- Include: What changed, why, how to migrate
- Link from RELEASE.md

## Maintenance

### Regular Tasks
- Review and update roadmap quarterly
- Audit documentation for accuracy on each release
- Remove outdated design docs (archive or delete)
- Keep examples directory in sync with language features

### Automation
- STDLIB_REFERENCE.md generation: `nu scripts/gen-docs.nu`
- CI enforces freshness checks
- Pre-commit hooks can validate markdown

## Questions?

For documentation questions or suggestions, open an issue with the `documentation` label.
