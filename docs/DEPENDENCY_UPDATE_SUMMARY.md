# Dependency Update Plan - Executive Summary

**Status**: Ready for execution
**Timeline**: 3 weeks (14 hours effort)
**Risk Level**: LOW-MEDIUM
**Security Impact**: POSITIVE

---

## Quick Overview

### What Needs Updating?

| Dependency | Current | Latest | Priority | Risk | Action |
|------------|---------|--------|----------|------|--------|
| **express** | 4.21.2 | 5.1.0 | HIGH | LOW | **REMOVE** (unused) |
| **@types/node** | 20.19.25 | 24.10.1 | MEDIUM | LOW | Update to 22.x LTS |
| **Rust deps** | Various | Current | LOW | LOW | `cargo update` |
| **MCP SDK** | 1.22.0 | Check | MEDIUM | MEDIUM | Update if available |

### Key Finding

**Express is NOT used by the MCP server** - it only uses stdio transport. We will REMOVE it instead of upgrading, reducing attack surface.

---

## 5-Phase Execution Plan

### Phase 1: Verification (Week 1, 2 hours)
- Install `cargo-audit` and `cargo-outdated`
- Run baseline security scans
- Document current test results and metrics
- **No code changes**

### Phase 2: Remove Express (Week 1-2, 4 hours) → PR #1
- Remove Express and CORS from package.json
- Test MCP server functionality
- **Impact**: Reduced dependencies, smaller bundle

### Phase 3: Update @types/node (Week 2, 2 hours) → PR #2
- Update to Node.js 22 LTS types
- Test for deprecated API usage
- **Impact**: Better type safety

### Phase 4: Update Rust (Week 3, 3 hours) → PR #3
- Run `cargo update` for semver-compatible updates
- Test all targets: Linux, Windows, WASM
- **Impact**: Bug fixes, security patches

### Phase 5: Update MCP SDK (Week 3, 3 hours) → PR #4
- Update @modelcontextprotocol/sdk
- Validate MCP protocol compliance
- **Impact**: Latest features, bug fixes

---

## Testing Strategy

Each phase requires:

✅ Unit tests pass (`cargo test`, `npm test`)
✅ Integration tests pass (WASM builds, CLI)
✅ Security scans clean (Trivy, cargo-audit, npm-audit)
✅ CI pipeline passes (GitHub Actions)
✅ No performance regressions (benchmarks)

---

## Security Posture

**Current State**:
- ✅ All 5 Dependabot alerts are FIXED
- ✅ No CRITICAL or HIGH vulnerabilities
- ✅ Trivy scans pass in CI

**After Updates**:
- ✅ Reduced attack surface (fewer dependencies)
- ✅ Latest security patches
- ✅ Maintained clean security status

---

## Rollback Safety

Every phase has a documented rollback procedure:

```bash
# Emergency rollback
git checkout main -- <files>
cargo/npm install
cargo test && npm test
```

All changes are isolated in separate PRs for easy revert.

---

## Breaking Changes Watch List

### Express 4→5 (NOT APPLICABLE)
We're removing Express entirely, not upgrading.

### @types/node 20→24
- Stick to v22 (Node.js 22 LTS) for stability
- Avoid v24 experimental features
- Test on Node 18.x, 20.x, 22.x

### Rust Dependencies
- `serde 1.x→2.x`: Blocked by dependabot.yml (avoid)
- `wasm-bindgen`: Stable within 0.2.x
- `regex 1.x→2.x`: Check changelog if available

---

## Success Criteria

The update plan succeeds when:

✅ All tests pass (Rust + MCP)
✅ No security vulnerabilities (CRITICAL/HIGH)
✅ CI checks green on all platforms
✅ WASM builds succeed (bundler, nodejs, web)
✅ WASM size < 512KB
✅ No performance regressions
✅ MCP validation passes
✅ Zero Dependabot alerts

---

## Resource Requirements

**Time**: 14 hours total (spread over 3 weeks)
**Approvals**: 1 reviewer per PR (4 PRs total)
**CI**: ~2 hours per PR (automated)

**Prerequisites**:
- Rust toolchain (stable)
- Node.js 18+ (recommend 22 LTS)
- Docker (for Trivy scans)
- GitHub CLI (`gh`)

---

## Next Steps

1. **Review** this plan with team
2. **Schedule** 3-week execution window
3. **Begin Phase 1**: Install tooling and create baseline
4. **Execute** phases sequentially with PR reviews
5. **Monitor** post-deployment for issues

---

## Documentation

**Full Details**: `/mnt/c/Users/bowma/Projects/_aq_/pqc-scanner/docs/DEPENDENCY_UPDATE_PLAN.md`
**Command Reference**: `/mnt/c/Users/bowma/Projects/_aq_/pqc-scanner/docs/DEPENDENCY_UPDATE_COMMANDS.md`
**This Summary**: `/mnt/c/Users/bowma/Projects/_aq_/pqc-scanner/docs/DEPENDENCY_UPDATE_SUMMARY.md`

---

## Risk Matrix

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Express upgrade breaks MCP | **ELIMINATED** | N/A | Removing instead |
| @types/node compatibility | Low | Low | Test on LTS versions |
| Rust dep conflicts | Low | Low | Semver-compatible updates |
| MCP SDK breaking changes | Medium | Medium | Review changelog, test thoroughly |
| CI pipeline failures | Low | Low | Comprehensive testing per phase |

---

## Key Decisions

1. **Remove Express** instead of upgrading to v5 (unused dependency)
2. **Update to Node 22 types** (LTS) not Node 24 (too new)
3. **Use cargo update** not manual version bumps (semver safety)
4. **One PR per phase** for easy rollback
5. **Sequential execution** to isolate issues

---

## Timeline Visual

```
Week 1:  [Phase 1: Tooling] → [Phase 2: Remove Express]
Week 2:  [Phase 3: @types/node] ─────────────────────────┐
Week 3:  [Phase 4: Rust deps] → [Phase 5: MCP SDK] ←─────┘
         └─ Can parallelize with Phase 3 after Phase 1
```

---

## Contact

For questions or issues during execution:

1. Consult full plan in `DEPENDENCY_UPDATE_PLAN.md`
2. Use command reference in `DEPENDENCY_UPDATE_COMMANDS.md`
3. Check CI logs: `gh run view --log`
4. Create GitHub issue with label `dependencies`

**Start Date**: TBD (after plan approval)
**Target Completion**: TBD + 3 weeks
**Review Date**: After Phase 2 (reassess if needed)
