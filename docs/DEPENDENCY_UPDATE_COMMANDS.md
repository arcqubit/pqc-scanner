# Dependency Update - Command Reference

Quick reference for executing the dependency update plan.

---

## Phase 1: Verification & Tooling

```bash
# Install security tools
cargo install cargo-audit cargo-outdated

# Run security scans
cargo audit
cd mcp && npm audit --audit-level=moderate

# Trivy scan
docker run --rm -v $(pwd):/project aquasec/trivy fs /project \
  --severity CRITICAL,HIGH \
  --scanners vuln,secret \
  --skip-dirs node_modules,target \
  --format json \
  --output trivy-baseline.json

# Create baseline
git checkout -b baseline/pre-dependency-updates
cargo test --verbose 2>&1 | tee baseline-rust-tests.txt
cd mcp && npm test 2>&1 | tee baseline-mcp-tests.txt
cargo bench --no-fail-fast 2>&1 | tee baseline-benchmarks.txt

# Document WASM size
wasm-pack build --target bundler
ls -lh pkg/*.wasm | tee baseline-wasm-size.txt

# Tag baseline
git tag baseline-v2025.11.0
git checkout main
```

---

## Phase 2: Remove Express (PR #1)

```bash
# Create branch
git checkout -b chore/remove-unused-express

# Verify Express is unused
cd mcp
grep -r "express" src/ --exclude-dir=node_modules
grep -r "app\\.use\|app\\.listen\|app\\.get" src/

# Remove dependencies
npm uninstall express cors

# Test
npm test
npm run validate
npm start &
sleep 2
kill $!

# Commit
git add mcp/package.json mcp/package-lock.json
git commit -m "chore(mcp): remove unused Express and CORS dependencies

- Express was not used anywhere in MCP server (only uses stdio transport)
- Reduces attack surface and bundle size
- All tests pass, MCP validation succeeds

Closes #<issue-number>"

# Push and create PR
git push -u origin chore/remove-unused-express
gh pr create --title "chore(mcp): Remove unused Express dependency" \
  --body "Removes Express and CORS dependencies that are not used by the MCP server.

## Changes
- Remove express and cors from package.json
- MCP server only uses @modelcontextprotocol/sdk stdio transport

## Testing
- [x] All tests pass
- [x] MCP validation succeeds
- [x] No runtime errors

## Security Impact
- Reduces attack surface
- Smaller dependency tree
- Fewer potential vulnerabilities"
```

---

## Phase 3: Update @types/node (PR #2)

```bash
# Create branch
git checkout main
git pull
git checkout -b chore/update-types-node

# Check current Node version
node --version

# Update to Node 22 LTS types
cd mcp
npm install --save-dev @types/node@^22.0.0

# Test
npm test
npm run validate

# Check for deprecated API usage
grep -r "crypto.DEFAULT_ENCODING\|util.print\|require.extensions" src/

# Commit
git add mcp/package.json mcp/package-lock.json
git commit -m "chore(mcp): update @types/node to v22 (Node.js 22 LTS)

- Update TypeScript type definitions for Node.js 22 LTS
- No breaking changes detected
- All tests pass

Closes #<issue-number>"

# Push and create PR
git push -u origin chore/update-types-node
gh pr create --title "chore(mcp): Update @types/node to v22" \
  --body "Updates TypeScript type definitions for Node.js 22 LTS.

## Changes
- Update @types/node from v20 to v22

## Testing
- [x] All tests pass
- [x] No TypeScript compilation errors
- [x] No deprecated API usage detected

## Breaking Changes
None - Node.js 22 LTS is backward compatible with Node 18+"
```

---

## Phase 4: Update Rust Dependencies (PR #3)

```bash
# Create branch
git checkout main
git pull
git checkout -b chore/update-rust-deps

# Check for outdated dependencies
cargo outdated --root-deps-only

# Update dependencies (semver-compatible)
cargo update

# Run comprehensive tests
cargo test --verbose
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check

# Security audit
cargo audit --deny warnings

# Build WASM (all targets)
wasm-pack build --target bundler --out-dir pkg
wasm-pack build --target nodejs --out-dir pkg-nodejs
wasm-pack build --target web --out-dir pkg-web

# Check WASM size (must be < 512KB)
ls -lh pkg/*.wasm

# Run benchmarks
cargo bench --no-fail-fast 2>&1 | tee benchmarks-updated.txt

# Compare with baseline
diff baseline-benchmarks.txt benchmarks-updated.txt

# Check dependency tree for duplicates
cargo tree --duplicates

# Commit
git add Cargo.toml Cargo.lock
git commit -m "chore(deps): update Rust dependencies to latest compatible versions

- Update all dependencies via cargo update
- All tests pass
- No security vulnerabilities
- WASM size remains < 512KB
- No performance regressions

Updates:
$(cargo tree --depth 1 | head -20)

Closes #<issue-number>"

# Push and create PR
git push -u origin chore/update-rust-deps
gh pr create --title "chore(deps): Update Rust dependencies" \
  --body "Updates Rust dependencies to latest semver-compatible versions.

## Changes
- Run \`cargo update\` to update all dependencies
- See commit message for detailed update list

## Testing
- [x] All tests pass (Linux + Windows)
- [x] Clippy checks pass
- [x] Format checks pass
- [x] WASM builds succeed (bundler, nodejs, web)
- [x] WASM size < 512KB
- [x] Cargo audit clean
- [x] No performance regressions

## Security
- [x] No new vulnerabilities
- [x] Cargo audit passes with --deny warnings"
```

---

## Phase 5: Update MCP SDK (PR #4)

```bash
# Create branch
git checkout main
git pull
git checkout -b chore/update-mcp-sdk

# Check for updates
cd mcp
npm outdated @modelcontextprotocol/sdk

# View available versions
npm view @modelcontextprotocol/sdk versions --json

# Check changelog
open "https://github.com/modelcontextprotocol/sdk/releases"

# Update SDK
npm update @modelcontextprotocol/sdk

# Test
npm test
npm run validate --target 2025-11

# Test MCP server loading
node -e "
import('./src/index.js').then(() => {
  console.log('MCP server loaded successfully');
  process.exit(0);
}).catch(e => {
  console.error('Load failed:', e);
  process.exit(1);
});
"

# Test tool execution
npm start &
MCP_PID=$!
sleep 2
kill $MCP_PID

# Commit
git add mcp/package.json mcp/package-lock.json
git commit -m "chore(mcp): update @modelcontextprotocol/sdk to latest version

- Update MCP SDK to v$(npm list @modelcontextprotocol/sdk --depth=0 | grep @modelcontextprotocol/sdk | awk '{print $2}')
- All tests pass
- MCP validation succeeds
- No protocol breaking changes

Closes #<issue-number>"

# Push and create PR
git push -u origin chore/update-mcp-sdk
gh pr create --title "chore(mcp): Update MCP SDK" \
  --body "Updates @modelcontextprotocol/sdk to latest version.

## Changes
- Update @modelcontextprotocol/sdk

## Testing
- [x] All tests pass
- [x] MCP validation succeeds
- [x] Tool schemas valid
- [x] Stdio transport functional
- [x] No protocol compatibility issues

## Breaking Changes
None detected"
```

---

## Security Validation Commands

```bash
# Full security scan
docker run --rm -v $(pwd):/project aquasec/trivy fs /project \
  --severity CRITICAL,HIGH \
  --scanners vuln,secret \
  --skip-dirs node_modules,target \
  --exit-code 1

# Cargo security audit
cargo audit --deny warnings

# NPM security audit
cd mcp && npm audit --audit-level=high

# Check for known vulnerabilities in dependencies
cargo tree | grep -E "RUSTSEC-"

# Check Dependabot alerts
gh api repos/arcqubit/pqc-scanner/dependabot/alerts \
  --jq '.[] | select(.state=="open") | {number, severity: .security_advisory.severity, summary: .security_advisory.summary}'
```

---

## Rollback Commands

```bash
# Emergency rollback (any phase)
git checkout main
git branch -D <branch-name>

# Selective rollback (Rust)
git checkout main -- Cargo.toml Cargo.lock
cargo build --release
cargo test

# Selective rollback (NPM)
cd mcp
git checkout main -- package.json package-lock.json
npm install
npm test

# Production hotfix (revert merged PR)
git revert -m 1 <merge-commit-sha>
git checkout -b hotfix/revert-dependency-update
cargo test && cd mcp && npm test
gh pr create --title "Revert: dependency updates" \
  --body "Reverting due to production issue"
```

---

## Testing Commands

```bash
# Rust tests
cargo test --verbose
cargo test --all-features
cargo test --no-default-features
cargo test --doc

# Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt -- --check

# WASM tests
wasm-pack test --node
wasm-pack test --headless --firefox

# Benchmarks
cargo bench --no-fail-fast

# MCP tests
cd mcp
npm test
npm run validate --target 2025-11

# E2E MCP test
npm start &
MCP_PID=$!
sleep 2
ps -p $MCP_PID || echo "MCP server failed to start"
kill $MCP_PID

# Integration test (CLI)
cargo build --release
./target/release/pqc-scanner --help
./target/release/pqc-scanner scan --path ./samples/java-rsa --format sc13
```

---

## Monitoring Commands

```bash
# Check dependency versions
cargo tree --depth 1
cd mcp && npm ls --depth=0

# Check for outdated dependencies
cargo outdated --root-deps-only
cd mcp && npm outdated

# Check dependency tree for duplicates
cargo tree --duplicates

# Check WASM size
wasm-pack build --target bundler
ls -lh pkg/*.wasm
du -sh pkg/

# Check build size
cargo build --release
ls -lh target/release/pqc-scanner

# Performance comparison
cargo bench --no-fail-fast 2>&1 | tee bench-current.txt
diff bench-baseline.txt bench-current.txt
```

---

## CI/CD Commands

```bash
# Trigger CI manually
gh workflow run ci.yml

# View CI status
gh run list --limit 5

# View specific run
gh run view <run-id>

# View workflow logs
gh run view <run-id> --log

# Check workflow files
gh workflow list
gh workflow view ci.yml
```

---

## Useful Aliases (Add to ~/.bashrc or ~/.zshrc)

```bash
# PQC Scanner aliases
alias pqc-test='cargo test && cd mcp && npm test && cd ..'
alias pqc-audit='cargo audit && cd mcp && npm audit && cd ..'
alias pqc-wasm='wasm-pack build --target bundler && ls -lh pkg/*.wasm'
alias pqc-bench='cargo bench --no-fail-fast'
alias pqc-trivy='docker run --rm -v $(pwd):/project aquasec/trivy fs /project --severity CRITICAL,HIGH'
```

---

## Quick Reference: Update Sequence

1. **Phase 1**: Install tools → `cargo install cargo-audit cargo-outdated`
2. **Phase 2**: Remove Express → `npm uninstall express cors`
3. **Phase 3**: Update @types/node → `npm install --save-dev @types/node@^22.0.0`
4. **Phase 4**: Update Rust → `cargo update`
5. **Phase 5**: Update MCP SDK → `npm update @modelcontextprotocol/sdk`

After each phase:
- Run tests: `cargo test && cd mcp && npm test`
- Run security scans: `cargo audit && npm audit`
- Create PR with detailed description

---

## Emergency Contacts

If issues arise during updates:

1. **Rollback immediately** using commands above
2. **Check CI logs**: `gh run view --log`
3. **Review error messages** carefully
4. **Consult changelogs** for breaking changes
5. **Test in isolation** to identify root cause

For critical production issues:
- Revert PR immediately
- Tag issue as `priority: critical`
- Notify team via GitHub issue
