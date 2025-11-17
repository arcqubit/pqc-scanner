# Dependency Updates - Documentation Index

Complete documentation suite for systematic, safe dependency updates for PQC Scanner.

**Generated**: 2025-11-17
**Total Documents**: 5
**Total Size**: ~68KB
**Estimated Read Time**: 45 minutes (all docs)

---

## Quick Navigation

### 🚀 NEW TO THIS? START HERE
**File**: `START_HERE.md` (7.5KB)
- 5-minute quick start guide
- Prerequisites check
- Phase 1 execution commands
- What to do first

### 📋 EXECUTIVE SUMMARY (5 min read)
**File**: `DEPENDENCY_UPDATE_SUMMARY.md` (5.6KB)
- One-page overview
- Risk assessment matrix
- Timeline and effort
- Key decisions

### 📖 COMPLETE REFERENCE (30 min read)
**File**: `DEPENDENCY_UPDATE_PLAN.md` (19KB)
- Full 5-phase execution plan
- Detailed risk analysis
- Testing strategy
- Rollback procedures
- Success criteria
- Breaking changes reference

### 💻 COMMAND REFERENCE (Copy-Paste)
**File**: `DEPENDENCY_UPDATE_COMMANDS.md` (11KB)
- Ready-to-execute commands
- Phase-by-phase instructions
- Security scan commands
- Rollback commands
- Monitoring commands
- Bash aliases

### 📊 VISUAL GUIDE (15 min read)
**File**: `DEPENDENCY_UPDATE_VISUAL.md` (25KB)
- Flowcharts and diagrams
- Risk vs Impact matrix
- Testing pipeline visualization
- Timeline with milestones
- Before/After comparison
- Phase completion checklists

---

## Document Purpose Matrix

| Document | Purpose | Audience | When to Use |
|----------|---------|----------|-------------|
| START_HERE.md | Quick start | Everyone | First time reading |
| SUMMARY.md | Overview | Leadership, PM | Decision making |
| PLAN.md | Complete guide | Developers | During execution |
| COMMANDS.md | Quick reference | Developers | Copy-paste commands |
| VISUAL.md | Understanding | Visual learners | Clarification |

---

## Recommended Reading Order

### For Developers (Full Execution)
1. `START_HERE.md` (understand context)
2. `DEPENDENCY_UPDATE_PLAN.md` (read Phase 1 fully)
3. `DEPENDENCY_UPDATE_COMMANDS.md` (keep open while executing)
4. `DEPENDENCY_UPDATE_VISUAL.md` (reference as needed)

### For Team Leads (Planning)
1. `DEPENDENCY_UPDATE_SUMMARY.md` (get overview)
2. `DEPENDENCY_UPDATE_VISUAL.md` (understand flow)
3. `DEPENDENCY_UPDATE_PLAN.md` (detailed review)

### For Quick Reference (During Work)
1. `DEPENDENCY_UPDATE_COMMANDS.md` (commands)
2. `DEPENDENCY_UPDATE_VISUAL.md` (checklists)

---

## Key Sections by Document

### START_HERE.md
- ✅ Quick Start (5 minutes)
- ✅ Prerequisites check
- ✅ What to do next
- ✅ Phase execution order
- ✅ FAQ

### DEPENDENCY_UPDATE_SUMMARY.md
- ✅ Executive summary
- ✅ Current state analysis
- ✅ Risk assessment
- ✅ Timeline estimate
- ✅ Success criteria
- ✅ Key decisions

### DEPENDENCY_UPDATE_PLAN.md
- ✅ Phase 1: Verification & Tooling
- ✅ Phase 2: Remove Express (PR #1)
- ✅ Phase 3: Update @types/node (PR #2)
- ✅ Phase 4: Update Rust Dependencies (PR #3)
- ✅ Phase 5: Update MCP SDK (PR #4)
- ✅ Testing strategy
- ✅ Security validation
- ✅ Rollback procedures
- ✅ Breaking changes reference

### DEPENDENCY_UPDATE_COMMANDS.md
- ✅ Phase 1: Verification commands
- ✅ Phase 2: Remove Express commands
- ✅ Phase 3: @types/node commands
- ✅ Phase 4: Rust dependencies commands
- ✅ Phase 5: MCP SDK commands
- ✅ Security validation commands
- ✅ Rollback commands
- ✅ Testing commands
- ✅ Monitoring commands
- ✅ Bash aliases

### DEPENDENCY_UPDATE_VISUAL.md
- ✅ Dependency tree diagram
- ✅ Update flow diagram
- ✅ Risk vs Impact matrix
- ✅ Testing pipeline diagram
- ✅ Security validation flow
- ✅ Timeline with milestones
- ✅ Before/After comparison
- ✅ Express removal justification
- ✅ Success metrics dashboard
- ✅ Decision tree
- ✅ Rollback decision matrix
- ✅ Phase completion checklists

---

## File Locations

All files are in `/mnt/c/Users/bowma/Projects/_aq_/pqc-scanner/docs/`:

```
docs/
├── DEPENDENCY_UPDATES_INDEX.md        ← You are here
├── START_HERE.md                       ← Start here
├── DEPENDENCY_UPDATE_SUMMARY.md        ← Overview
├── DEPENDENCY_UPDATE_PLAN.md           ← Main plan
├── DEPENDENCY_UPDATE_COMMANDS.md       ← Commands
└── DEPENDENCY_UPDATE_VISUAL.md         ← Diagrams
```

---

## Plan Overview

### What Are We Updating?

**NPM Dependencies (mcp/ directory)**:
- ❌ Express 4.21.2 → REMOVE (unused)
- ❌ CORS 2.8.5 → REMOVE (unused)
- ✅ @types/node 20.x → 22.x (Node.js 22 LTS)
- ✅ @modelcontextprotocol/sdk (check for updates)

**Rust Dependencies (Cargo.toml)**:
- ✅ All dependencies (cargo update for patches)

### Why This Matters

**Security**:
- All 5 Dependabot alerts are currently FIXED ✅
- Keep dependencies up-to-date to prevent future vulnerabilities
- Reduce attack surface by removing unused dependencies

**Maintenance**:
- Avoid technical debt from outdated dependencies
- Stay compatible with modern tooling
- Easier to apply future security patches

**Performance**:
- Bug fixes and performance improvements
- Smaller bundle size (removing Express/CORS)

---

## Execution Timeline

| Week | Phase | Duration | Deliverable |
|------|-------|----------|-------------|
| 1 | Phase 1: Setup | 2h | Baseline + Tools |
| 1-2 | Phase 2: Remove Express | 4h | PR #1 merged |
| 2 | Phase 3: @types/node | 2h | PR #2 merged |
| 3 | Phase 4: Rust deps | 3h | PR #3 merged |
| 3 | Phase 5: MCP SDK | 3h | PR #4 merged |

**Total**: 3 weeks, 14 hours effort

---

## Risk Summary

### Eliminated Risks ✅
- **Express 5.x breaking changes**: AVOIDED (removing instead)
- **HTTP server vulnerabilities**: ELIMINATED (no HTTP server)

### Remaining Risks
- **@types/node compatibility**: LOW (LTS version)
- **Rust dependency conflicts**: LOW (semver updates)
- **MCP SDK breaking changes**: MEDIUM (test thoroughly)

### Mitigation Strategy
- ✅ One change at a time (separate PRs)
- ✅ Comprehensive testing per phase
- ✅ Clear rollback procedures
- ✅ Security validation at each step

---

## Success Criteria

The update plan succeeds when ALL of these are true:

✅ All tests pass (Rust + MCP + WASM)
✅ No security vulnerabilities (CRITICAL/HIGH)
✅ CI checks green on all platforms
✅ WASM size < 512KB
✅ No performance regressions
✅ MCP validation passes
✅ Zero Dependabot alerts

---

## Key Decisions Made

1. **Remove Express** instead of upgrading to v5.x
   - Rationale: Not used anywhere in codebase
   - Impact: Reduces attack surface, smaller bundle

2. **Update to Node 22 LTS types** (not Node 24)
   - Rationale: v22 is LTS, v24 is too new
   - Impact: Stability over bleeding edge

3. **Use cargo update** (not manual bumps)
   - Rationale: Semver-compatible, safer
   - Impact: Only patch/minor updates

4. **One PR per phase**
   - Rationale: Easy rollback, clear history
   - Impact: More PRs, but safer

5. **Sequential execution**
   - Rationale: Isolate issues, clear cause/effect
   - Impact: Takes longer, but much safer

---

## Breaking Changes Reference

### Express 4.x → 5.x
**Status**: NOT APPLICABLE (being removed)

### @types/node 20.x → 24.x
**Our approach**: Update to 22.x (LTS) only
**Breaking changes in 22.x**: Minimal (backward compatible with Node 18+)

### Rust Dependencies
**Approach**: cargo update (semver-compatible only)
**Blocked**: serde 1.x→2.x (per dependabot.yml)

---

## Testing Checklist

Every phase must pass:

### Unit Tests ✅
- `cargo test --verbose`
- `npm test`

### Integration Tests ✅
- WASM builds (bundler, nodejs, web)
- CLI execution
- MCP server validation

### Security Scans ✅
- Trivy (CRITICAL, HIGH)
- cargo audit --deny warnings
- npm audit --audit-level=high

### Performance Tests ✅
- cargo bench (no regressions)
- WASM size < 512KB

### CI Pipeline ✅
- Linux tests
- Windows tests
- Clippy lints
- Format checks

---

## Rollback Procedures

### Emergency Rollback
```bash
git checkout main
git branch -D <branch-name>
```

### Selective Rollback (Rust)
```bash
git checkout main -- Cargo.toml Cargo.lock
cargo build --release
```

### Selective Rollback (NPM)
```bash
cd mcp
git checkout main -- package.json package-lock.json
npm install
```

### Production Hotfix
```bash
git revert -m 1 <merge-commit-sha>
gh pr create --title "Revert: dependency updates"
```

---

## Monitoring & Validation

### After Each PR Merge
```bash
# Full test suite
cargo test --all-features
cd mcp && npm test

# Security validation
cargo audit --deny warnings
npm audit --audit-level=high
trivy fs . --severity CRITICAL,HIGH --exit-code 1

# Performance check
cargo bench --no-fail-fast
ls -lh pkg/*.wasm  # < 512KB
```

### Continuous Monitoring
- Dependabot: Auto-PRs weekly (Mondays 9am PT)
- GitHub Security Advisories: Email notifications
- Trivy: Scans on every push (CI)
- OpenSSF Scorecard: Security posture tracking

---

## Additional Resources

### External Documentation
- [Dependabot Config Docs](https://docs.github.com/en/code-security/dependabot)
- [Express 5.x Migration Guide](https://expressjs.com/en/guide/migrating-5.html) (NOT NEEDED)
- [Node.js Release Schedule](https://nodejs.org/en/about/previous-releases)
- [Rust Semver Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)
- [MCP SDK Releases](https://github.com/modelcontextprotocol/sdk/releases)

### Internal References
- GitHub Actions: `.github/workflows/ci.yml`
- Dependabot Config: `.github/dependabot.yml`
- Package Manifests: `Cargo.toml`, `mcp/package.json`

---

## FAQ

**Q: Why remove Express instead of upgrading?**
A: Express is not used anywhere in the codebase. The MCP server uses stdio transport, not HTTP. Removing it reduces attack surface and bundle size.

**Q: Can I skip phases?**
A: No. Phase 1 is mandatory (creates baseline). Phases 2-5 build on each other.

**Q: What if tests fail?**
A: Do not proceed. Investigate and fix failures, or rollback the change.

**Q: Can I do multiple phases in one day?**
A: Yes, but create separate PRs for each phase for easier rollback.

**Q: What about major version updates?**
A: Blocked for serde/tokio (per dependabot.yml). Others evaluated case-by-case.

**Q: How long will this take?**
A: 3 weeks calendar time, 14 hours effort (spread across phases).

---

## Contributions

This documentation was created to ensure safe, systematic dependency updates with:
- ✅ Clear execution steps
- ✅ Risk mitigation strategies
- ✅ Comprehensive testing
- ✅ Easy rollback procedures
- ✅ Security validation

Updates and improvements welcome via PR!

---

## Version History

- **v1.0** (2025-11-17): Initial creation
  - 5-phase plan created
  - All documentation generated
  - Ready for execution

---

## Next Steps

1. **Read** `START_HERE.md`
2. **Execute** Phase 1 (Quick Start)
3. **Follow** command reference for subsequent phases
4. **Monitor** progress using checklists
5. **Validate** success criteria

**Ready to begin?** Open `START_HERE.md` and follow the Quick Start guide!

---

**Status**: ✅ Documentation Complete - Ready for Execution
**Last Updated**: 2025-11-17
**Maintained By**: PQC Scanner Team
