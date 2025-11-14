# Repository Cleanup Plan for Publishing

**Branch:** `cleanup-for-publish`
**Objective:** Remove all traces of AI-assisted development and non-essential files to create a clean, professional repository showing only the core PDQ Scanner functionality.

---

## Phase 1: Delete Claude/AI Infrastructure

### Remove entire directories (11 directories, 100+ files):

```bash
# Claude/AI development infrastructure
rm -rf .claude/                 # All agent definitions and commands
rm -rf .claude-flow/            # Metrics and performance data
rm -rf .swarm/                  # Memory databases (*.db, *.db-wal, *.db-shm)
rm -rf .hive-mind/              # Hive database
rm -rf memory/                  # Claude flow data and sessions
rm -rf coordination/            # Orchestration artifacts
rm -f claude-flow               # Binary executable (if exists)
```

---

## Phase 2: Delete Non-Technical Documentation

### Development/Planning Documentation:

```bash
# Claude-specific configuration
rm -f CLAUDE.md

# PhotonIQ/Ollama integration (14 files)
rm -rf docs/photoniq/

# Planning documents
rm -rf docs/plans/

# Research documents
rm -rf docs/research/

# Implementation planning
rm -rf docs/implementation/

# Marketing materials
rm -rf docs/marketing/

# Vision/roadmap
rm -rf docs/vision/
```

### Maintainer Documentation:

```bash
rm -f docs/REVIEW_GUIDE.md
rm -f docs/RELEASE_CHECKLIST.md
rm -f docs/PHASE2_BUILD_REPORT.md
rm -f docs/PHASE_2_SUMMARY.md
rm -f docs/PHASE_3_DEPLOYMENT.md
rm -f CONTRIBUTING.md              # If exists
```

### Test Artifacts:

```bash
# Test output files
rm -rf test/results/

# Generated reports (if uncommitted)
rm -rf reports/

# Release artifacts (if exists)
rm -rf release-artifacts/

# Misplaced TypeScript declaration
rm -f rust_wasm_app.d.ts          # Should be in pkg/
```

### Project Management Scripts:

```bash
rm -f scripts/create-github-issues.sh
rm -f scripts/create-project-board.sh
rm -f scripts/populate-project-board.sh
rm -f scripts/organize-project-board.sh
```

---

## Phase 3: Modify Files to Remove AI References

### Cargo.toml

**Remove lines 26-39:**
```toml
# =============================================
# Agent-Booster Integration
# =============================================
# These dependencies support agentic patterns, tool coordination,
# and distributed workflows as part of Claude-Flow, Claude Swarm,
# and Hive-Mind MCP servers. They enable:
#
# - Multi-agent task orchestration
# - Shared memory and state management
# - Tool chaining and composition
# - Parallel execution pipelines
# - Event-driven automation
# =============================================
```

**Keep:** All actual dependencies (they're standard Rust crates)

### Makefile

**Line 96:** Change `"PhotonIQ PDQ Scanner"` → `"PDQ Scanner"`

```makefile
# Before:
@echo "Building PhotonIQ PDQ Scanner (debug)..."

# After:
@echo "Building PDQ Scanner (debug)..."
```

### docs/openssf-research-findings.md

**Remove:**
- Line 1: `**Research Agent:** Claude Code Researcher`
- Line 6: `**Memory Key:** `coordination:ossf/requirements``

**Keep:** All technical OpenSSF research content

---

## Phase 4: Keep Comprehensive Technical Documentation

### Files to Preserve:

**Root Documentation:**
- `README.md`
- `CHANGELOG.md`
- `LICENSE`

**Technical Documentation:**
- `docs/ASCII_LOGO.txt` ✓ (Keep for CLI branding)
- `docs/REMEDIATION.md`
- `docs/AUTO_REMEDIATION.md`
- `docs/SAMPLE_REPOSITORIES.md`
- `docs/OSSF_IMPLEMENTATION.md`
- `docs/openssf-research-findings.md` (after modifications)
- `docs/architecture/` - All architecture documents
- `docs/security/` - All security documents

**Source Code:**
- `src/*.rs` - All Rust source files
- `tests/` - Test fixtures and integration tests
- `benches/` - Benchmarks
- `examples/` - Example code

**Build System:**
- `Cargo.toml` (after modifications)
- `Cargo.lock`
- `Makefile` (after modifications)
- `scripts/build.sh`
- `scripts/test.sh`
- `scripts/release.sh`

**CI/CD:**
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`
- `.github/workflows/cargo-audit.yml` (if exists)
- `action.yml` - GitHub Action definition

---

## Phase 5: Update .gitignore

### Add Missing Entries:

```gitignore
# Claude/AI development infrastructure
.claude/
.claude-flow/
claude-flow

# Development docs (if regenerated)
docs/photoniq/
```

**Note:** The following are already in .gitignore:
- `.swarm/`
- `.hive-mind/`
- `memory/`
- `coordination/`
- `*.db*`

---

## Phase 6: Verification Steps

### 1. Search for Remaining Claude References:

```bash
# Check source code and docs
grep -r "Claude\|claude-flow\|agent-booster\|swarm\|hive-mind" src/ docs/ \
  --exclude-dir=photoniq \
  --exclude-dir=plans \
  --exclude-dir=target \
  --exclude-dir=.git

# Expected result: No matches (or only false positives like "swarm" in comments)
```

### 2. Check for Secrets/API Keys:

```bash
# Scan for potential secrets
grep -r "sk-\|api_key\|secret\|token" src/ \
  --exclude-dir=target \
  --exclude-dir=.git

# Expected result: Only test fixtures or environment variable names
```

### 3. Test Build:

```bash
# Clean build
make clean
make build

# Run tests
make test

# Verify WASM build
make wasm

# Expected result: All builds succeed, all tests pass
```

### 4. Check Documentation Links:

```bash
# Ensure no broken links to deleted docs
grep -r "\[.*\](docs/photoniq\|docs/plans\|docs/marketing)" *.md docs/

# Expected result: No matches
```

---

## Expected Final State

### Directories That Should Exist:
```
pqc-scanner/
├── .github/workflows/        # CI/CD only
├── benches/                  # Benchmarks
├── docs/
│   ├── architecture/         # Technical architecture
│   ├── security/             # Security documentation
│   ├── ASCII_LOGO.txt        # CLI branding
│   ├── AUTO_REMEDIATION.md
│   ├── OSSF_IMPLEMENTATION.md
│   ├── REMEDIATION.md
│   ├── SAMPLE_REPOSITORIES.md
│   └── openssf-research-findings.md (cleaned)
├── examples/                 # Example code
├── scripts/
│   ├── build.sh              # Build automation
│   ├── test.sh               # Test automation
│   └── release.sh            # Release automation
├── src/
│   ├── bin/                  # CLI binary
│   └── *.rs                  # Rust modules
├── tests/                    # Test fixtures
├── .gitignore
├── action.yml
├── Cargo.lock
├── Cargo.toml (cleaned)
├── CHANGELOG.md
├── LICENSE
├── Makefile (cleaned)
└── README.md
```

### Directories That Should NOT Exist:
- `.claude/`
- `.claude-flow/`
- `.swarm/`
- `.hive-mind/`
- `memory/`
- `coordination/`
- `docs/photoniq/`
- `docs/plans/`
- `docs/research/`
- `docs/implementation/`
- `docs/marketing/`
- `docs/vision/`
- `test/results/`
- `reports/` (empty directory is OK, gitignored)

### Files That Should NOT Exist:
- `CLAUDE.md`
- `claude-flow` (binary)
- `rust_wasm_app.d.ts` (root level)
- `docs/REVIEW_GUIDE.md`
- `docs/RELEASE_CHECKLIST.md`
- `docs/PHASE*.md`
- `scripts/create-*.sh`
- `scripts/populate-*.sh`
- `scripts/organize-*.sh`

---

## Summary Statistics

- **Directories to delete:** 11
- **Files to delete:** 100+
- **Files to modify:** 3 (Cargo.toml, Makefile, openssf-research-findings.md)
- **Files to preserve:** All core source code, tests, CI/CD, comprehensive technical documentation

---

## Execution Commands

### Quick Delete Script:

```bash
#!/bin/bash
# cleanup.sh - Execute repository cleanup

# Phase 1: Delete AI infrastructure
git rm -rf .claude .claude-flow .swarm .hive-mind memory coordination 2>/dev/null
rm -f claude-flow

# Phase 2: Delete non-technical docs
git rm -f CLAUDE.md
git rm -rf docs/photoniq docs/plans docs/research docs/implementation docs/marketing docs/vision
git rm -f docs/REVIEW_GUIDE.md docs/RELEASE_CHECKLIST.md
git rm -f docs/PHASE2_BUILD_REPORT.md docs/PHASE_2_SUMMARY.md docs/PHASE_3_DEPLOYMENT.md
git rm -f CONTRIBUTING.md 2>/dev/null

# Phase 2: Delete test artifacts
git rm -rf test/results 2>/dev/null
rm -f rust_wasm_app.d.ts

# Phase 2: Delete project management scripts
git rm -f scripts/create-github-issues.sh scripts/create-project-board.sh
git rm -f scripts/populate-project-board.sh scripts/organize-project-board.sh

echo "✓ Cleanup complete - proceed to Phase 3 (file modifications)"
```

---

## Post-Cleanup Commit Message

```
chore: Clean repository for public release

Remove development infrastructure and non-technical documentation:
- Deleted AI/Claude development tools and artifacts
- Removed internal planning and research documents
- Removed maintainer-specific documentation
- Cleaned up AI references in Cargo.toml and Makefile
- Updated documentation to focus on technical content only

This prepares the repository for public release showing only the
core PDQ Scanner functionality: quantum-safe cryptography auditing
with NIST compliance reporting.
```

---

## Notes

- This cleanup is **non-destructive** to core functionality
- All deleted content is preserved in git history
- The main branch remains unchanged until PR is merged
- Build and test processes remain fully functional
- All technical documentation is preserved
