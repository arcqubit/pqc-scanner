# PQC Scanner - Systematic Dependency Update Plan

**Status**: All 5 Dependabot alerts are FIXED
**Generated**: 2025-11-17
**Branch**: `refactor/separate-samples-repo`
**Review Required**: Yes - Major breaking changes identified

---

## Executive Summary

This plan outlines a systematic, risk-mitigated approach to updating dependencies for the PQC Scanner project. The primary concern is the **Express 4.x → 5.x migration**, which contains breaking changes. All security alerts are currently resolved.

### Risk Assessment
- **HIGH**: Express 4.21.2 → 5.1.0 (breaking changes)
- **MEDIUM**: @types/node 20.x → 24.x (major version jump)
- **LOW**: Rust dependencies (all appear current)
- **SECURITY**: No active vulnerabilities ✅

---

## Current State Analysis

### NPM Dependencies (mcp/ directory)

```json
Current versions:
- express: 4.21.2 (latest: 5.1.0) - MAJOR UPDATE AVAILABLE
- @types/node: 20.19.25 (latest: 24.10.1) - MAJOR UPDATE AVAILABLE
- @modelcontextprotocol/sdk: 1.22.0 (current)
- cors: 2.8.5 (current)
```

**Express Usage**: MCP server does NOT actually use Express HTTP server features - only uses MCP SDK's StdioServerTransport. The Express dependency appears to be unused or legacy.

### Rust Dependencies (Cargo.toml)

```toml
All dependencies appear current:
- wasm-bindgen: 0.2.105
- serde: 1.0.228
- regex: 1.12.2
- chrono: 0.4.42
- thiserror: 1.0.69
- uuid: 1.18.1
```

**Cargo Audit**: Not installed (recommended to add)

### GitHub Actions
- All actions use pinned SHAs (security best practice) ✅
- Dependabot configured for weekly updates ✅

---

## Update Strategy: Phased Approach

### Phase 1: Verification & Tooling (Week 1)
**Risk: LOW** | **Priority: HIGH** | **Effort: 2 hours**

Install security audit tools and verify current state before changes.

#### Actions:
1. **Install cargo-audit**
   ```bash
   cargo install cargo-audit
   cargo audit
   ```

2. **Install cargo-outdated**
   ```bash
   cargo install cargo-outdated
   cargo outdated --root-deps-only
   ```

3. **Run comprehensive security scans**
   ```bash
   # Trivy scan
   docker run --rm -v $(pwd):/project aquasec/trivy fs /project \
     --severity CRITICAL,HIGH \
     --scanners vuln,secret \
     --skip-dirs node_modules,target

   # NPM audit
   cd mcp && npm audit --audit-level=moderate
   ```

4. **Document baseline metrics**
   ```bash
   # Test suite baseline
   cargo test --verbose > baseline-rust-tests.log 2>&1
   cd mcp && npm test > baseline-mcp-tests.log 2>&1

   # Build size baseline
   cargo build --release
   ls -lh target/release/pqc-scanner
   wasm-pack build --target bundler
   ls -lh pkg/*.wasm
   ```

#### Success Criteria:
- ✅ No CRITICAL or HIGH vulnerabilities in Trivy scan
- ✅ Baseline test results documented
- ✅ All security tooling installed and functional

#### Rollback:
Not applicable (read-only operations).

---

### Phase 2: Express Investigation & Removal (Week 1-2)
**Risk: LOW-MEDIUM** | **Priority: HIGH** | **Effort: 4 hours**

**Investigation Findings**: The MCP server (mcp/src/index.js) does NOT use Express anywhere. It only uses:
- `@modelcontextprotocol/sdk` for MCP protocol
- `StdioServerTransport` for stdio communication
- Node.js built-ins (fs, child_process, path)

**Recommendation**: REMOVE Express entirely instead of upgrading.

#### Actions:

1. **Verify Express is unused**
   ```bash
   cd mcp
   grep -r "express" src/ --exclude-dir=node_modules
   grep -r "app\\.use\|app\\.listen\|app\\.get" src/
   ```

2. **Remove Express dependency** (Create PR #1)
   ```bash
   git checkout -b chore/remove-unused-express
   cd mcp
   npm uninstall express cors
   ```

3. **Update package.json**
   ```json
   {
     "dependencies": {
       "@modelcontextprotocol/sdk": "^1.0.0"
     },
     "devDependencies": {
       "@types/node": "^20.0.0"
     }
   }
   ```

4. **Test MCP server functionality**
   ```bash
   # Start MCP server
   npm start &
   MCP_PID=$!

   # Test with validation
   npm run validate

   # Kill server
   kill $MCP_PID

   # Run test suite
   npm test
   ```

5. **Verify no regressions**
   ```bash
   # Test MCP tool calls
   echo '{"method":"tools/list"}' | npm start

   # CI pipeline check
   .github/workflows/ci.yml  # Ensure no Express-related tests
   ```

#### Success Criteria:
- ✅ All tests pass without Express
- ✅ MCP validation succeeds
- ✅ No import errors or runtime failures
- ✅ Package size reduced

#### Rollback Procedure:
```bash
git checkout main -- mcp/package.json mcp/package-lock.json
cd mcp && npm install
npm test
```

---

### Phase 3: @types/node Update (Week 2)
**Risk: LOW** | **Priority: MEDIUM** | **Effort: 2 hours**

Update TypeScript type definitions for Node.js.

#### Actions:

1. **Check Node.js engine compatibility**
   ```bash
   node --version  # Current runtime version
   grep "engines" mcp/package.json  # Required: >=18.0.0
   ```

2. **Update @types/node incrementally** (Create PR #2)
   ```bash
   git checkout -b chore/update-types-node
   cd mcp

   # Option A: Update to Node 22 LTS types
   npm install --save-dev @types/node@^22.0.0

   # Option B: Update to Node 24 (latest)
   # npm install --save-dev @types/node@^24.0.0
   ```

3. **Test for breaking changes**
   ```bash
   npm test
   npm run validate
   ```

4. **Check for deprecated APIs**
   ```bash
   grep -r "crypto.DEFAULT_ENCODING\|util.print\|require.extensions" src/
   ```

#### Breaking Changes to Watch:
- Node.js 22+: Deprecated APIs removed
- Buffer API changes
- Stream API updates
- Crypto module changes

#### Success Criteria:
- ✅ No TypeScript compilation errors
- ✅ All tests pass
- ✅ No runtime warnings about deprecated APIs

#### Rollback:
```bash
git checkout main -- mcp/package.json mcp/package-lock.json
cd mcp && npm install
```

---

### Phase 4: Rust Dependency Updates (Week 3)
**Risk: LOW** | **Priority: LOW** | **Effort: 3 hours**

Update Rust dependencies to latest compatible versions.

#### Actions:

1. **Generate update report**
   ```bash
   cargo outdated --root-deps-only
   cargo tree --duplicates  # Check for duplicate dependencies
   ```

2. **Update patch/minor versions** (Create PR #3)
   ```bash
   git checkout -b chore/update-rust-deps
   cargo update
   ```

3. **Run comprehensive tests**
   ```bash
   # Unit tests
   cargo test --verbose

   # Clippy checks
   cargo clippy --all-targets --all-features -- -D warnings

   # Format check
   cargo fmt -- --check

   # WASM builds
   wasm-pack build --target bundler
   wasm-pack build --target nodejs
   wasm-pack build --target web

   # Benchmarks
   cargo bench --no-fail-fast
   ```

4. **Security audit**
   ```bash
   cargo audit
   cargo audit --deny warnings
   ```

5. **Check WASM size**
   ```bash
   ls -lh pkg/*.wasm
   # Ensure < 512KB target
   ```

#### Dependencies to Monitor:
- `wasm-bindgen`: 0.2.x (stable)
- `serde`: 1.0.x (avoid major updates per dependabot.yml)
- `regex`: 1.x (stable)
- `chrono`: 0.4.x (watch for 0.5 breaking changes)

#### Success Criteria:
- ✅ All tests pass (Linux + Windows)
- ✅ No cargo audit warnings
- ✅ WASM builds succeed for all targets
- ✅ WASM size remains < 512KB
- ✅ No clippy warnings

#### Rollback:
```bash
git checkout main -- Cargo.toml Cargo.lock
cargo build --release
cargo test
```

---

### Phase 5: @modelcontextprotocol/sdk Update (Week 3)
**Risk: MEDIUM** | **Priority: MEDIUM** | **Effort: 3 hours**

Update MCP SDK to latest version (check for breaking changes).

#### Actions:

1. **Check for SDK updates**
   ```bash
   cd mcp
   npm outdated @modelcontextprotocol/sdk
   npm view @modelcontextprotocol/sdk versions --json
   ```

2. **Review changelog**
   ```bash
   # Visit: https://github.com/modelcontextprotocol/sdk/releases
   # Check for breaking changes in API
   ```

3. **Update SDK** (Create PR #4)
   ```bash
   git checkout -b chore/update-mcp-sdk
   cd mcp
   npm update @modelcontextprotocol/sdk
   ```

4. **Test MCP protocol compliance**
   ```bash
   npm run validate --target 2025-11
   npm test
   ```

5. **Verify tool schemas**
   ```bash
   node -e "
   import('./src/index.js').then(async () => {
     console.log('MCP server loaded successfully');
   }).catch(e => {
     console.error('Load failed:', e);
     process.exit(1);
   });
   "
   ```

#### Breaking Changes to Watch:
- Schema validation changes
- Transport protocol updates
- Tool definition format changes
- Request/response structure changes

#### Success Criteria:
- ✅ MCP validation passes
- ✅ All tool definitions valid
- ✅ Stdio transport functional
- ✅ No protocol compatibility issues

#### Rollback:
```bash
git checkout main -- mcp/package.json mcp/package-lock.json
cd mcp && npm install
```

---

## Testing Strategy

### Pre-Update Baseline
```bash
# Create baseline branch
git checkout -b baseline/pre-dependency-updates
git tag baseline-v2025.11.0

# Run full test suite
cargo test --verbose 2>&1 | tee baseline-rust-tests.txt
cd mcp && npm test 2>&1 | tee baseline-mcp-tests.txt

# Security baseline
trivy fs . --severity CRITICAL,HIGH --format json > baseline-trivy.json
cargo audit --json > baseline-cargo-audit.json 2>&1 || true
cd mcp && npm audit --json > baseline-npm-audit.json
```

### Per-Phase Testing

#### Unit Tests
```bash
# Rust
cargo test --all-features
cargo test --no-default-features
cargo test --doc

# MCP Server
cd mcp && npm test
```

#### Integration Tests
```bash
# WASM integration
wasm-pack test --node
wasm-pack test --headless --firefox

# CLI integration
./target/release/pqc-scanner --help
./target/release/pqc-scanner scan --path ./samples/java-rsa
```

#### E2E MCP Tests
```bash
cd mcp

# Test 1: scan_code tool
node -e "
import('./src/index.js').catch(console.error);
" <<EOF
{"method":"tools/call","params":{"name":"scan_code","arguments":{"path":"../samples/java-rsa"}}}
EOF

# Test 2: analyze_file tool
node src/test.js

# Test 3: MCP validation
npm run validate
```

#### Performance Tests
```bash
# Benchmark baseline
cargo bench 2>&1 | tee bench-baseline.txt

# After update
cargo bench 2>&1 | tee bench-updated.txt

# Compare
diff bench-baseline.txt bench-updated.txt
```

#### Security Validation
```bash
# Trivy scan
trivy fs . \
  --severity CRITICAL,HIGH \
  --scanners vuln,secret \
  --skip-dirs node_modules,target \
  --exit-code 1

# Cargo audit
cargo audit --deny warnings

# NPM audit
cd mcp && npm audit --audit-level=high
```

### Automated CI Validation

All updates must pass:
- ✅ GitHub Actions CI workflow
- ✅ Test suite (Ubuntu + Windows)
- ✅ WASM build (bundler, nodejs, web)
- ✅ Clippy lints
- ✅ Rustfmt checks
- ✅ Trivy security scan
- ✅ Benchmark performance (continue-on-error)

---

## Breaking Changes Reference

### Express 4.x → 5.x (NOT APPLICABLE - REMOVING)

**Status**: Express is unused and will be removed.

### @types/node 20.x → 24.x

**Breaking Changes**:
- Node.js 21+: V8 11.x (new JS features)
- Node.js 22 LTS: Stable API changes
- Node.js 23+: Experimental features

**Mitigation**:
- Stick to Node.js 22 LTS types (^22.0.0)
- Avoid experimental APIs
- Test on Node.js 18.x, 20.x, 22.x

### Potential Rust Updates

**wasm-bindgen 0.2.x**:
- Generally stable within 0.2.x
- Check for wasm-pack compatibility

**serde 1.x → 2.x**:
- DO NOT update (blocked by dependabot.yml)
- Major breaking changes expected

**regex 1.x → 2.x**:
- If available, check changelog carefully
- Likely syntax/API changes

---

## Rollback Procedures

### Emergency Rollback (Any Phase)

```bash
# Stop work immediately
git stash

# Return to main branch
git checkout main

# Verify working state
cargo test
cd mcp && npm test

# Delete update branch
git branch -D <update-branch-name>
```

### Selective Rollback (Single Dependency)

```bash
# Rust dependency
git checkout main -- Cargo.toml Cargo.lock
cargo update

# NPM dependency
cd mcp
git checkout main -- package.json package-lock.json
npm install
```

### Production Hotfix (If Issues Found Post-Merge)

```bash
# Revert the merge commit
git revert -m 1 <merge-commit-sha>

# Create hotfix branch
git checkout -b hotfix/revert-dependency-update

# Test revert
cargo test
cd mcp && npm test

# Create PR for revert
gh pr create --title "Revert: dependency updates" \
  --body "Reverting due to production issue: <description>"
```

---

## Security Validation Checklist

### Pre-Update
- [ ] Run `cargo audit` (install if needed)
- [ ] Run `npm audit` in mcp/
- [ ] Run Trivy scan
- [ ] Check GitHub Security Advisories
- [ ] Review Dependabot alerts (all fixed ✅)

### Per-Update
- [ ] Check CVE database for new vulnerabilities
- [ ] Review dependency changelogs for security fixes
- [ ] Verify no new transitive vulnerabilities
- [ ] Check RUSTSEC advisories
- [ ] Validate npm package signatures

### Post-Update
- [ ] Re-run all security scans
- [ ] Verify no new Dependabot alerts
- [ ] Check Trivy exit code = 0
- [ ] Run cargo audit --deny warnings
- [ ] Run npm audit --audit-level=moderate

### Ongoing Monitoring
- [ ] Enable GitHub security alerts
- [ ] Monitor Dependabot PRs weekly
- [ ] Subscribe to RUSTSEC advisory feed
- [ ] Review npm security advisories

---

## Timeline & Effort Estimate

| Phase | Duration | Effort | Risk | Can Parallelize? |
|-------|----------|--------|------|------------------|
| Phase 1: Verification & Tooling | Week 1 | 2h | LOW | No (prerequisite) |
| Phase 2: Remove Express | Week 1-2 | 4h | LOW | After Phase 1 |
| Phase 3: @types/node Update | Week 2 | 2h | LOW | After Phase 2 |
| Phase 4: Rust Dependencies | Week 3 | 3h | LOW | After Phase 1 |
| Phase 5: MCP SDK Update | Week 3 | 3h | MED | After Phase 3 |
| **TOTAL** | **3 weeks** | **14h** | **LOW-MED** | - |

**Phases 3-5 can partially overlap** if Phase 1-2 complete successfully.

---

## Success Criteria (Overall)

- ✅ All tests pass (Rust + MCP)
- ✅ No security vulnerabilities (CRITICAL/HIGH)
- ✅ All CI checks green
- ✅ WASM builds succeed (bundler, nodejs, web)
- ✅ WASM size < 512KB
- ✅ No performance regressions (benchmarks)
- ✅ MCP validation passes
- ✅ No Dependabot alerts
- ✅ Clean cargo audit
- ✅ Clean npm audit

---

## Pull Request Strategy

### PR Template for Each Phase

```markdown
## Description
[Phase X]: Update <dependency> from <old> to <new>

## Changes
- Update <dependency> in <file>
- [Any code changes needed]

## Testing
- [ ] Unit tests pass: `cargo test` / `npm test`
- [ ] Integration tests pass
- [ ] Security scan clean: `trivy`, `cargo audit`, `npm audit`
- [ ] CI pipeline green
- [ ] Performance benchmarks reviewed

## Breaking Changes
- None / [List breaking changes]

## Rollback Plan
```bash
git checkout main -- <files>
cargo/npm install
```

## Security Review
- [ ] No new vulnerabilities introduced
- [ ] Transitive dependencies checked
- [ ] Changelog reviewed for security fixes

## Related Issues
Closes #<issue-number> (if applicable)
```

### PR Merge Strategy

1. **Require approvals**: 1 reviewer minimum
2. **Require CI**: All checks must pass
3. **Squash commits**: Keep history clean
4. **Delete branch**: After merge
5. **Tag releases**: Semantic versioning

---

## Monitoring & Validation

### Post-Deployment Checks

```bash
# After each PR merges to main
git pull origin main

# Run full test suite
cargo test --all-features
cd mcp && npm test

# Security validation
trivy fs . --severity CRITICAL,HIGH --exit-code 1
cargo audit --deny warnings
cd mcp && npm audit --audit-level=high

# Performance check
cargo bench --no-fail-fast

# WASM validation
wasm-pack build --target bundler
ls -lh pkg/*.wasm
```

### Continuous Monitoring

- **Dependabot**: Auto-creates PRs weekly (Mondays 9am PT)
- **GitHub Security Advisories**: Email notifications
- **CI Pipeline**: Trivy scan on every push
- **OpenSSF Scorecard**: Tracks security posture

---

## Risk Mitigation Summary

### High-Risk Item: Express 5.x Migration
**Mitigation**: REMOVE Express entirely (unused dependency)
**Impact**: Reduced attack surface, smaller bundle

### Medium-Risk Item: @types/node Major Update
**Mitigation**: Update incrementally (20→22→24), test thoroughly
**Impact**: Better type safety, modern Node.js features

### Low-Risk Items: Rust Dependencies
**Mitigation**: Use `cargo update` (semver-compatible), test extensively
**Impact**: Bug fixes, security patches, performance improvements

---

## Additional Recommendations

### Add to CI Pipeline

```yaml
# .github/workflows/dependency-check.yml
name: Dependency Security Check
on:
  schedule:
    - cron: '0 9 * * 1'  # Weekly Monday 9am UTC
  workflow_dispatch:

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install cargo-audit
        run: cargo install cargo-audit

      - name: Rust security audit
        run: cargo audit --deny warnings

      - name: NPM security audit
        run: |
          cd mcp
          npm audit --audit-level=high

      - name: Trivy scan
        uses: aquasecurity/trivy-action@master
        with:
          scan-type: 'fs'
          severity: 'CRITICAL,HIGH'
          exit-code: '1'
```

### Add cargo-audit to CI

```yaml
# .github/workflows/ci.yml (add new job)
  security-audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install cargo-audit
        run: cargo install cargo-audit
      - name: Run cargo-audit
        run: cargo audit --deny warnings
```

### Create .nvmrc

```bash
# Specify Node.js version for consistency
echo "22.11.0" > .nvmrc
echo "20.18.1" >> .node-version  # LTS fallback
```

---

## Conclusion

This plan prioritizes:
1. **Security**: All phases include security validation
2. **Incremental Updates**: One major change at a time
3. **Testing**: Comprehensive testing at each stage
4. **Rollback Safety**: Clear rollback procedures for every phase
5. **Low Risk**: Removing unused Express instead of risky upgrade

**Estimated Total Time**: 3 weeks (14 hours effort)
**Risk Level**: LOW-MEDIUM (mostly LOW with proper testing)
**Security Impact**: POSITIVE (reduced dependencies, up-to-date security patches)

---

## Execution Checklist

- [ ] **Week 1, Day 1-2**: Phase 1 - Install tooling, baseline metrics
- [ ] **Week 1, Day 3-5**: Phase 2 - Remove Express, create PR #1
- [ ] **Week 2, Day 1-2**: Phase 3 - Update @types/node, create PR #2
- [ ] **Week 2-3**: Phase 4 - Update Rust deps, create PR #3
- [ ] **Week 3**: Phase 5 - Update MCP SDK, create PR #4
- [ ] **Week 3, End**: Final validation, monitoring setup

**Next Step**: Begin Phase 1 - Verification & Tooling
