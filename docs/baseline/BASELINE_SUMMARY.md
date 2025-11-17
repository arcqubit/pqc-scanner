# Dependency Update Baseline - Phase 1

**Date**: 2025-11-17
**Version**: 2025.11.0-beta.1
**Branch**: refactor/separate-samples-repo

## Security Audit Results

### Rust (cargo audit)
✅ **CLEAN** - No vulnerabilities found in 91 crate dependencies

### NPM (npm audit)
✅ **CLEAN** - found 0 vulnerabilities

### GitHub Dependabot
✅ **ALL FIXED** - 5 alerts, all marked as "fixed":
- #5: postgresql SQL Injection (CRITICAL) - fixed
- #4: postgresql TemporaryFolder (MEDIUM) - fixed
- #3: spring-boot RCE (CRITICAL) - fixed
- #2: flask fallback key (LOW) - fixed
- #1: cryptography OpenSSL (LOW) - fixed

## Test Results

### Rust Tests
✅ **62 tests passed, 0 failed**
- Unit tests: 43 passed
- Integration tests: 9 passed
- Remediation tests: 10 passed
- Doc tests: 0 passed
- Total time: ~0.24s

### MCP Tests
⚠️ No test suite exists (MCP server uses stdio transport, manual verification only)

## Binary Sizes

### Release Binaries
- `pqc-scanner`: **2.0 MB**
- `pqc-scanner-cli`: **2.0 MB**

### WASM (not measured in baseline)
- Will measure after Phase 2 changes

## Dependencies Overview

### Rust (Cargo.toml)
```toml
wasm-bindgen = "0.2"
serde = "1.0"
serde-wasm-bindgen = "0.6"
serde_json = "1.0"
regex = "1.10"
lazy_static = "1.4"
thiserror = "1.0"
once_cell = "1.19"
uuid = "1.6"
chrono = "0.4"
```

### NPM (mcp/package.json)
```json
{
  "@modelcontextprotocol/sdk": "^1.0.0",
  "express": "^4.18.2",  ← UNUSED, will remove in Phase 2
  "cors": "^2.8.5"       ← UNUSED, will remove in Phase 2
}
```

## Phase 1 Complete ✅

All baseline metrics captured. Ready for Phase 2: Express removal.

### Next Steps (Phase 2)
1. Remove `express` and `cors` from mcp/package.json
2. Remove unused imports from mcp/src/index.js (if any)
3. Run `npm install` to update package-lock.json
4. Verify MCP server still works
5. Run security audits again
6. Create PR

### Success Criteria for Phase 2
- ✅ npm audit: 0 vulnerabilities (maintained)
- ✅ MCP server starts successfully
- ✅ Dependencies reduced from 3 → 1
- ✅ No functionality regression
