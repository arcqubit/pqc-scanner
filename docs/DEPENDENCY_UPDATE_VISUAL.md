# Dependency Update Plan - Visual Guide

## Current Dependency Tree

```
PQC Scanner Project
├── Rust Dependencies (Cargo.toml)
│   ├── wasm-bindgen: 0.2.105 ✅ Current
│   ├── serde: 1.0.228 ✅ Current
│   ├── serde-wasm-bindgen: 0.6.5 ✅ Current
│   ├── serde_json: 1.0.145 ✅ Current
│   ├── regex: 1.12.2 ✅ Current
│   ├── lazy_static: 1.5.0 ✅ Current
│   ├── thiserror: 1.0.69 ✅ Current
│   ├── once_cell: 1.21.3 ✅ Current
│   ├── uuid: 1.18.1 ✅ Current
│   └── chrono: 0.4.42 ✅ Current
│
└── NPM Dependencies (mcp/package.json)
    ├── @modelcontextprotocol/sdk: 1.22.0 ✅ Current
    ├── express: 4.21.2 ⚠️ UNUSED - REMOVE
    ├── cors: 2.8.5 ⚠️ UNUSED - REMOVE
    └── @types/node: 20.19.25 📦 Update to 22.x
```

---

## Update Flow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         PHASE 1: SETUP                          │
│                       (Week 1, 2 hours)                         │
├─────────────────────────────────────────────────────────────────┤
│  Install Tools    │  Security Scan  │  Create Baseline          │
│  cargo-audit      │  Trivy          │  Tag: baseline-v2025.11.0 │
│  cargo-outdated   │  cargo audit    │  Save test results        │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                   PHASE 2: REMOVE EXPRESS                       │
│                   (Week 1-2, 4 hours) → PR #1                   │
├─────────────────────────────────────────────────────────────────┤
│  Branch: chore/remove-unused-express                            │
│                                                                 │
│  1. npm uninstall express cors                                 │
│  2. Test: npm test && npm run validate                         │
│  3. Verify: MCP server starts via stdio                        │
│  4. PR Review → Merge                                           │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                 PHASE 3: UPDATE @types/node                     │
│                   (Week 2, 2 hours) → PR #2                     │
├─────────────────────────────────────────────────────────────────┤
│  Branch: chore/update-types-node                                │
│                                                                 │
│  1. npm install --save-dev @types/node@^22.0.0                 │
│  2. Test: npm test && npm run validate                         │
│  3. Check for deprecated APIs                                   │
│  4. PR Review → Merge                                           │
└────────────────────────────┬────────────────────────────────────┘
                             │
         ┌───────────────────┴───────────────────┐
         │                                       │
         ▼                                       ▼
┌─────────────────────────┐         ┌───────────────────────────┐
│  PHASE 4: RUST DEPS     │         │  PHASE 5: MCP SDK         │
│  (Week 3, 3 hours)      │         │  (Week 3, 3 hours)        │
│  → PR #3                │         │  → PR #4                  │
├─────────────────────────┤         ├───────────────────────────┤
│  Branch: chore/update-  │         │  Branch: chore/update-    │
│          rust-deps      │         │          mcp-sdk          │
│                         │         │                           │
│  1. cargo update        │         │  1. npm update            │
│  2. cargo test          │         │     @modelcontext...      │
│  3. cargo audit         │         │  2. npm run validate      │
│  4. wasm-pack build     │         │  3. Test tool calls       │
│  5. PR Review → Merge   │         │  4. PR Review → Merge     │
└─────────────────────────┘         └───────────────────────────┘
         │                                       │
         └───────────────────┬───────────────────┘
                             ▼
                    ┌────────────────────┐
                    │  COMPLETE ✅       │
                    │  All deps updated  │
                    │  Security validated│
                    └────────────────────┘
```

---

## Risk vs Impact Matrix

```
High Impact │
            │
            │                              ┌─────────────┐
            │                              │ Express 5.x │
            │                              │  AVOIDED!   │
            │                              └─────────────┘
            │
            │
            │    ┌──────────────┐
Medium      │    │  MCP SDK     │
            │    │  Update      │
            │    │   Phase 5    │
            │    └──────────────┘
            │
            │                        ┌──────────────┐
            │                        │ @types/node  │
Low Impact  │  ┌────────────┐       │   Phase 3    │
            │  │Rust Deps   │       └──────────────┘
            │  │ Phase 4    │
            │  └────────────┘
            │
            └───────────────────────────────────────────
              Low Risk    Medium Risk    High Risk
```

**Strategy**: We've eliminated the high-risk/high-impact Express 5.x upgrade by removing it entirely!

---

## Testing Pipeline per Phase

```
┌──────────────────┐
│  Code Changes    │
└────────┬─────────┘
         │
         ▼
┌──────────────────────────────────────────┐
│         Unit Tests                       │
│  • cargo test --verbose                  │
│  • npm test                              │
└────────┬─────────────────────────────────┘
         │ PASS ✅
         ▼
┌──────────────────────────────────────────┐
│      Integration Tests                   │
│  • WASM builds (bundler, nodejs, web)    │
│  • CLI execution                         │
│  • MCP validation                        │
└────────┬─────────────────────────────────┘
         │ PASS ✅
         ▼
┌──────────────────────────────────────────┐
│      Security Scans                      │
│  • Trivy (CRITICAL, HIGH)                │
│  • cargo audit --deny warnings           │
│  • npm audit --audit-level=high          │
└────────┬─────────────────────────────────┘
         │ CLEAN ✅
         ▼
┌──────────────────────────────────────────┐
│      Performance Tests                   │
│  • cargo bench --no-fail-fast            │
│  • Compare with baseline                 │
│  • Check WASM size < 512KB               │
└────────┬─────────────────────────────────┘
         │ NO REGRESSION ✅
         ▼
┌──────────────────────────────────────────┐
│      CI Pipeline (GitHub Actions)        │
│  • Linux tests                           │
│  • Windows tests                         │
│  • Clippy lints                          │
│  • Format checks                         │
└────────┬─────────────────────────────────┘
         │ ALL GREEN ✅
         ▼
┌──────────────────────────────────────────┐
│      Code Review & PR Approval           │
│  • 1 reviewer required                   │
│  • Security checklist verified           │
│  • Changelog updated                     │
└────────┬─────────────────────────────────┘
         │ APPROVED ✅
         ▼
┌──────────────────────────────────────────┐
│         MERGE TO MAIN                    │
│  • Squash commits                        │
│  • Delete branch                         │
│  • Deploy to production                  │
└──────────────────────────────────────────┘
```

---

## Security Validation Flow

```
                        ┌─────────────────┐
                        │  Update Deps    │
                        └────────┬────────┘
                                 │
                    ┌────────────┴────────────┐
                    ▼                         ▼
           ┌─────────────────┐      ┌─────────────────┐
           │  Rust Security  │      │  NPM Security   │
           │  cargo audit    │      │  npm audit      │
           └────────┬────────┘      └────────┬────────┘
                    │                         │
                    └────────────┬────────────┘
                                 ▼
                        ┌─────────────────┐
                        │  Trivy Scan     │
                        │  (Container     │
                        │   + Filesystem) │
                        └────────┬────────┘
                                 │
                    ┌────────────┴────────────┐
                    ▼                         ▼
           ┌─────────────────┐      ┌─────────────────┐
           │ CRITICAL/HIGH   │      │   Secrets       │
           │ Vulnerabilities │      │   Detection     │
           └────────┬────────┘      └────────┬────────┘
                    │                         │
                    └────────────┬────────────┘
                                 │
                        ┌────────▼────────┐
                        │   CLEAN? ✅     │
                        └────────┬────────┘
                                 │
                    ┌────────────┴────────────┐
                    ▼                         ▼
           ┌─────────────────┐      ┌─────────────────┐
           │  YES: Continue  │      │  NO: Block PR   │
           │  to merge       │      │  Fix issues     │
           └─────────────────┘      └─────────────────┘
```

---

## Timeline with Milestones

```
Week 1
├── Mon-Tue: Phase 1 (Setup & Tooling) ✅
│   └── Milestone: Baseline created
├── Wed-Fri: Phase 2 (Remove Express) 🔄
│   └── Milestone: PR #1 merged

Week 2
├── Mon-Tue: Phase 3 (@types/node) 🔄
│   └── Milestone: PR #2 merged
└── Wed-Fri: Buffer time / Start Phase 4 🔄

Week 3
├── Mon-Wed: Phase 4 (Rust deps) 🔄
│   └── Milestone: PR #3 merged
├── Thu-Fri: Phase 5 (MCP SDK) 🔄
│   └── Milestone: PR #4 merged
└── End: Final validation ✅
    └── Milestone: All updates complete

Legend:
✅ = Complete
🔄 = In Progress
⏸️ = Blocked
❌ = Failed (rollback)
```

---

## Dependency Before/After

### Before (Current State)

```json
// mcp/package.json
{
  "dependencies": {
    "@modelcontextprotocol/sdk": "^1.0.0",
    "express": "^4.18.2",           // ❌ Unused - 5.1.0 available
    "cors": "^2.8.5"                // ❌ Unused
  },
  "devDependencies": {
    "@types/node": "^20.0.0"        // ⚠️ 24.10.1 available
  }
}
```

### After (Target State)

```json
// mcp/package.json
{
  "dependencies": {
    "@modelcontextprotocol/sdk": "^1.0.0"  // ✅ Latest
  },
  "devDependencies": {
    "@types/node": "^22.0.0"               // ✅ Node 22 LTS
  }
}
```

**Result**:
- ✅ 2 dependencies removed (express, cors)
- ✅ 1 dependency updated (@types/node)
- ✅ Reduced attack surface
- ✅ Smaller bundle size
- ✅ Maintained functionality

---

## Express Removal Justification

### Why Express Was Added
```javascript
// Likely initial intent (never implemented):
// import express from 'express';
// const app = express();
// app.use(cors());
// app.listen(3000);
```

### What Actually Happened
```javascript
// mcp/src/index.js (actual implementation):
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';

// NO express import ❌
// NO cors import ❌
// NO HTTP server ❌

// Uses stdio transport instead ✅
const transport = new StdioServerTransport();
await server.connect(transport);
```

### Conclusion
Express and CORS were likely added as placeholders for a future HTTP transport that was never implemented. The MCP server exclusively uses stdio (standard input/output) communication, making Express completely unnecessary.

**Impact of Removal**:
- ✅ Security: Reduced attack surface (no HTTP server)
- ✅ Performance: Smaller bundle, faster install
- ✅ Maintenance: Fewer dependencies to update
- ✅ Simplicity: Clearer dependency tree

---

## Success Metrics Dashboard

```
╔════════════════════════════════════════════════════════════╗
║              DEPENDENCY UPDATE SUCCESS METRICS              ║
╠════════════════════════════════════════════════════════════╣
║                                                            ║
║  Security Status                                           ║
║  ├── Dependabot Alerts:        0 / 0 ✅                   ║
║  ├── Trivy CRITICAL:           0 ✅                        ║
║  ├── Trivy HIGH:               0 ✅                        ║
║  ├── Cargo Audit:              PASS ✅                     ║
║  └── NPM Audit:                PASS ✅                     ║
║                                                            ║
║  Test Coverage                                             ║
║  ├── Rust Unit Tests:          PASS ✅                     ║
║  ├── MCP Tests:                PASS ✅                     ║
║  ├── WASM Builds:              PASS ✅                     ║
║  ├── CI Pipeline:              GREEN ✅                    ║
║  └── Performance:              NO REGRESSION ✅            ║
║                                                            ║
║  Bundle Metrics                                            ║
║  ├── NPM Dependencies:         1 (was 3) ↓ 67% ✅        ║
║  ├── WASM Size:                < 512KB ✅                  ║
║  ├── Build Time:               NO INCREASE ✅              ║
║  └── Install Time:             FASTER ↓ ✅                ║
║                                                            ║
║  Code Quality                                              ║
║  ├── Clippy Warnings:          0 ✅                        ║
║  ├── Format Check:             PASS ✅                     ║
║  ├── TypeScript Errors:        0 ✅                        ║
║  └── Dead Code:                REMOVED ✅                  ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝

Overall Status: ✅ READY FOR PRODUCTION
```

---

## Quick Decision Tree

```
┌─────────────────────────────┐
│  Should I update this dep?  │
└──────────────┬──────────────┘
               │
               ▼
     ┌─────────────────────┐
     │ Is it used in code? │
     └─────────┬───────────┘
          NO ⟍  ⟋ YES
             ⟍⟋
    ┌─────────┴─────────┐
    ▼                   ▼
┌─────────┐      ┌──────────────────┐
│ REMOVE  │      │ Security update? │
│   IT    │      └────────┬─────────┘
└─────────┘          YES ⟍  ⟋ NO
                         ⟍⟋
                ┌─────────┴─────────┐
                ▼                   ▼
         ┌────────────┐      ┌────────────┐
         │ UPDATE NOW │      │ Breaking?  │
         │  (High Pri)│      └─────┬──────┘
         └────────────┘         NO ⟍  ⟋ YES
                                   ⟍⟋
                          ┌─────────┴─────────┐
                          ▼                   ▼
                   ┌────────────┐      ┌────────────┐
                   │UPDATE SAFE │      │TEST BRANCH │
                   │ (Low Risk) │      │REVIEW FIRST│
                   └────────────┘      └────────────┘

Examples:
• Express → REMOVE (not used)
• CORS → REMOVE (not used)
• @types/node → UPDATE SAFE (dev dependency)
• Rust deps → UPDATE SAFE (cargo update)
• MCP SDK → TEST BRANCH (potential breaking changes)
```

---

## Rollback Decision Matrix

```
Issue Severity │ When to Rollback
───────────────┼──────────────────────────────────────────
CRITICAL       │ • Tests failing after merge
               │ • Security vulnerability introduced
               │ • Production crashes
               │ → Immediate revert, hotfix PR
───────────────┼──────────────────────────────────────────
HIGH           │ • CI pipeline broken
               │ • WASM build failures
               │ • Performance regression > 20%
               │ → Revert PR, investigate offline
───────────────┼──────────────────────────────────────────
MEDIUM         │ • Warnings in CI
               │ • Clippy complaints
               │ • Minor performance impact < 10%
               │ → Fix forward in follow-up PR
───────────────┼──────────────────────────────────────────
LOW            │ • Documentation updates needed
               │ • Minor type warnings
               │ • Non-critical deprecations
               │ → Create issue, fix in next sprint
```

---

## Phase Completion Checklist

### Phase 1: Setup ✅
- [ ] cargo-audit installed
- [ ] cargo-outdated installed
- [ ] Trivy scan completed
- [ ] Baseline branch created
- [ ] Baseline tag created
- [ ] Test results saved
- [ ] WASM size documented

### Phase 2: Remove Express ✅
- [ ] Express uninstalled
- [ ] CORS uninstalled
- [ ] MCP tests pass
- [ ] Validation succeeds
- [ ] CI green
- [ ] PR #1 created
- [ ] PR #1 approved
- [ ] PR #1 merged

### Phase 3: @types/node ✅
- [ ] Updated to v22
- [ ] No TypeScript errors
- [ ] No deprecated APIs
- [ ] Tests pass
- [ ] CI green
- [ ] PR #2 created
- [ ] PR #2 approved
- [ ] PR #2 merged

### Phase 4: Rust Deps ✅
- [ ] cargo update run
- [ ] Tests pass (Linux)
- [ ] Tests pass (Windows)
- [ ] WASM builds succeed
- [ ] Cargo audit clean
- [ ] Clippy clean
- [ ] Benchmarks OK
- [ ] PR #3 created
- [ ] PR #3 approved
- [ ] PR #3 merged

### Phase 5: MCP SDK ✅
- [ ] SDK updated
- [ ] Changelog reviewed
- [ ] MCP validation passes
- [ ] Tool schemas valid
- [ ] Tests pass
- [ ] CI green
- [ ] PR #4 created
- [ ] PR #4 approved
- [ ] PR #4 merged

### Final Validation ✅
- [ ] All PRs merged
- [ ] Full test suite passes
- [ ] Security scans clean
- [ ] Performance baseline met
- [ ] Documentation updated
- [ ] Release notes created
