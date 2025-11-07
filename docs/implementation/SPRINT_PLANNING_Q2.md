# Sprint Planning - Q2 2026 (Sprints 5-9)

**Quarter**: Q2 2026 (April - June)
**Epic**: #6 - VS Code Extension
**Total Story Points**: 68 points
**Timeline**: 10 weeks (5 sprints × 2 weeks)
**Team Size**: 4 developers (1 extension lead, 2 dev, 1 QA)
**Target Velocity**: 13-14 points per sprint

---

## 🎯 Q2 Goal: Professional IDE Integration

**Mission**: Launch VS Code and JetBrains extensions that provide real-time quantum-safe cryptography scanning directly in developers' IDEs with instant feedback, quick fixes, and seamless workflow integration.

**Success Criteria**:
- ✅ VS Code extension published to marketplace
- ✅ Real-time highlighting of vulnerabilities
- ✅ One-click quick fixes for common issues
- ✅ Scanner panel with detailed reports
- ✅ JetBrains plugin (IntelliJ, PyCharm, WebStorm)
- ✅ 100K+ installs by Q2 end
- ✅ 4.5+ star rating

---

## 📅 Sprint Overview

| Sprint | Dates | Issues | Points | Focus Area |
|--------|-------|--------|--------|------------|
| **Sprint 5** | Week 9-10 | #7, #8 (partial) | 14 | VS Code Foundation |
| **Sprint 6** | Week 11-12 | #8 (complete), #9 | 13 | Real-Time + Quick Fixes |
| **Sprint 7** | Week 13-14 | #10 | 13 | Scanner Panel |
| **Sprint 8** | Week 15-16 | #11 (partial) | 14 | JetBrains Plugin Start |
| **Sprint 9** | Week 17-18 | #11 (complete) | 14 | JetBrains Polish + Launch |

**Total**: 68 story points across 5 sprints

---

## 🚀 Sprint 5: VS Code Foundation

**Status**: ⏳ Pending Q1 Completion
**Timeline**: Week 9-10 (Apr 1-14, 2026)
**Story Points**: 14
**Team**: Full team (4 developers)
**Dependencies**: Q1 CLI tool complete

### Issues

#### Issue #7: VS Code Publishing (5 points, Critical)
**Goal**: Extension published to VS Code Marketplace

**Tasks**:
- [ ] Setup VS Code extension project (Yeoman generator)
- [ ] Configure extension manifest (package.json)
- [ ] Create extension activation logic
- [ ] Add command palette commands
- [ ] Setup build pipeline (webpack/esbuild)
- [ ] Create marketplace listing (description, screenshots)
- [ ] Setup Azure DevOps for publishing
- [ ] Publish initial version (v0.1.0)

**Acceptance Criteria** (Gherkin):
```gherkin
Scenario: Extension installation from marketplace
  Given I open VS Code
  When I search for "PQC Scanner" in extensions marketplace
  Then I should find the extension
  When I click "Install"
  Then the extension should install successfully
  And I should see the PQC Scanner icon in the activity bar
```

**Definition of Done**:
- ✅ Extension published to VS Code Marketplace
- ✅ Installation works on VS Code 1.80+
- ✅ Basic command palette integration
- ✅ Extension icon and branding
- ✅ Marketplace listing complete with screenshots
- ✅ Automated publishing pipeline

**Key Features**:
- **Command Palette**: "PQC: Scan File", "PQC: Scan Workspace"
- **Activity Bar**: PQC Scanner icon
- **Status Bar**: Scan status indicator
- **Settings**: Basic configuration options

---

#### Issue #8: Real-Time Highlighting (13 points, Critical) - **Part 1** (9 points this sprint)
**Goal**: Inline vulnerability highlighting as you code

**Sprint 5 Tasks** (Foundation):
- [ ] Integrate CLI scanner as embedded library
- [ ] Setup language server protocol (LSP) architecture
- [ ] Create diagnostic provider for VS Code
- [ ] Implement file parsing and AST analysis
- [ ] Basic syntax highlighting for vulnerabilities
- [ ] Setup diagnostic update on file save

**Acceptance Criteria** (Sprint 5):
```gherkin
Scenario: Basic highlighting on file save
  Given I have a JavaScript file open in VS Code
  When I add code using RSA-1024
  And I save the file
  Then I should see a red squiggle under RSA-1024
  And hovering shows "Insecure: RSA key size below 2048 bits"
```

**Definition of Done** (Sprint 5):
- ✅ Diagnostics appear on file save
- ✅ Red squiggles for critical issues
- ✅ Yellow squiggles for warnings
- ✅ Hover tooltips show details
- ✅ Performance: <500ms diagnostic update

---

### Sprint 5 Deliverables

**At Sprint End**:
- ✅ VS Code extension live on marketplace
- ✅ Basic real-time highlighting working (on save)
- ✅ 100+ installs from early adopters
- ✅ Foundation for Sprint 6 enhancements
- ✅ Architecture documented

**Demo Script**:
```bash
# Install from marketplace
# Open VS Code > Extensions > Search "PQC Scanner"

# Open a JavaScript file
# Add RSA-1024 usage
# Save file
# See red squiggle and hover tooltip
```

---

## ⚡ Sprint 6: Real-Time + Quick Fixes

**Status**: ⏳ Pending Sprint 5
**Timeline**: Week 11-12 (Apr 15-28, 2026)
**Story Points**: 13
**Team**: Full team (4 developers)
**Dependencies**: Sprint 5 complete

### Issues

#### Issue #8: Real-Time Highlighting (13 points, Critical) - **Part 2** (4 points this sprint)
**Goal**: True real-time highlighting without save

**Sprint 6 Tasks** (Real-Time):
- [ ] Implement incremental parsing (TextDocument.onDidChangeTextDocument)
- [ ] Add debouncing for performance (300ms)
- [ ] Optimize AST diffing for changed regions only
- [ ] Add code lens with inline metrics
- [ ] Performance optimization for large files (>5000 lines)
- [ ] Add diagnostic severity levels (error, warning, info)

**Acceptance Criteria** (Sprint 6):
```gherkin
Scenario: Real-time highlighting without save
  Given I have a JavaScript file open in VS Code
  When I type code using RSA-1024
  Then within 300ms I should see a red squiggle
  And I should NOT need to save the file
  When I fix the issue
  Then the squiggle should disappear within 300ms
```

**Definition of Done** (Sprint 6):
- ✅ Real-time highlighting without save
- ✅ <300ms latency for diagnostic updates
- ✅ Incremental parsing for performance
- ✅ Code lens showing vulnerability count
- ✅ Performance tested with 10,000+ line files

---

#### Issue #9: Quick Fixes (8 points, Critical)
**Goal**: One-click fixes for common vulnerabilities

**Tasks**:
- [ ] Implement CodeActionProvider interface
- [ ] Create quick fix transformations (RSA upgrade, AES migration, etc.)
- [ ] Add "Fix All" command for entire file
- [ ] Implement preview before applying fixes
- [ ] Add undo support
- [ ] Create quick fix snippets for common patterns
- [ ] Add telemetry for fix success rates
- [ ] Write integration tests

**Acceptance Criteria** (Gherkin):
```gherkin
Scenario: Quick fix for RSA key size
  Given I have code with RSA-1024
  And I see a red squiggle
  When I click the lightbulb icon
  Then I should see "Upgrade to RSA-2048"
  When I click the quick fix
  Then the code should be automatically updated to RSA-2048
  And the squiggle should disappear

Scenario: Fix all vulnerabilities in file
  Given I have 5 vulnerabilities in a file
  When I run "PQC: Fix All in File"
  Then I should see a preview of all changes
  When I confirm
  Then all 5 vulnerabilities should be fixed
  And I can undo all changes with Ctrl+Z
```

**Definition of Done**:
- ✅ Quick fixes for 10+ vulnerability types
- ✅ "Fix All" command working
- ✅ Preview before applying changes
- ✅ Undo/redo support
- ✅ Success rate >90% for common patterns
- ✅ Integration tests for all quick fixes

**Quick Fix Types**:
1. **RSA Key Size**: Upgrade 1024 → 2048 → 4096
2. **AES Mode**: ECB → CBC → GCM
3. **Hash Algorithm**: MD5/SHA1 → SHA256
4. **Random Number**: Math.random() → crypto.randomBytes()
5. **Deprecated APIs**: Old → New crypto APIs
6. **Key Derivation**: Weak → PBKDF2/Argon2
7. **Certificate Validation**: Disabled → Enabled
8. **TLS Version**: TLS 1.0/1.1 → TLS 1.3
9. **Cipher Suites**: Weak → Strong
10. **PQC Migration**: Classical → Quantum-Safe

---

### Sprint 6 Deliverables

**At Sprint End**:
- ✅ True real-time highlighting (no save needed)
- ✅ One-click quick fixes for 10+ vulnerability types
- ✅ "Fix All" command working
- ✅ <300ms latency for updates
- ✅ 1,000+ installs
- ✅ 4.0+ star rating

**Demo Script**:
```javascript
// Open VS Code with PQC Scanner extension

// Type vulnerable code in real-time
const crypto = require('crypto');
const key = crypto.generateKeyPairSync('rsa', {
  modulusLength: 1024, // ← RED SQUIGGLE appears instantly
});

// Click lightbulb → "Upgrade to RSA-2048"
// Code automatically updated

// Command Palette → "PQC: Fix All in File"
// Preview all fixes → Confirm
// All vulnerabilities fixed instantly
```

---

## 📊 Sprint 7: Scanner Panel

**Status**: ⏳ Pending Sprint 6
**Timeline**: Week 13-14 (Apr 29 - May 12, 2026)
**Story Points**: 13
**Team**: Full team (4 developers)
**Dependencies**: Sprint 6 complete

### Issues

#### Issue #10: Scanner Panel (13 points, High)
**Goal**: Comprehensive vulnerability dashboard in VS Code

**Tasks**:
- [ ] Create TreeView for vulnerabilities list
- [ ] Add grouping (by file, by severity, by type)
- [ ] Implement filtering and search
- [ ] Add vulnerability details pane
- [ ] Create statistics dashboard (charts)
- [ ] Add export functionality (JSON, SARIF, HTML)
- [ ] Implement click-to-navigate to code
- [ ] Add compliance reports view
- [ ] Write UI tests

**Acceptance Criteria** (Gherkin):
```gherkin
Scenario: View all vulnerabilities in panel
  Given I have a workspace with 10 files
  And 15 vulnerabilities detected
  When I open the PQC Scanner panel
  Then I should see all 15 vulnerabilities listed
  And they should be grouped by severity (5 critical, 7 high, 3 medium)
  When I click a vulnerability
  Then the editor should jump to that line
  And the code should be highlighted

Scenario: Filter and search vulnerabilities
  Given the PQC Scanner panel is open
  When I filter by "Critical" severity
  Then I should see only 5 vulnerabilities
  When I search for "RSA"
  Then I should see only RSA-related vulnerabilities

Scenario: Export compliance report
  Given I have scanned my workspace
  When I click "Export Report"
  And I select "SARIF" format
  Then a SARIF file should be downloaded
  And it should contain all vulnerability details
```

**Definition of Done**:
- ✅ TreeView with vulnerabilities listed
- ✅ Grouping by file, severity, type
- ✅ Search and filter working
- ✅ Click-to-navigate to code
- ✅ Statistics dashboard with charts
- ✅ Export to JSON, SARIF, HTML
- ✅ Compliance reports (NIST, FedRAMP preview)
- ✅ UI tests passing

**Panel Features**:

**1. Vulnerability List**:
```
📁 src/crypto.js (3 issues)
  ├─ 🔴 RSA-1024 detected (Line 42)
  ├─ 🟠 Weak cipher AES-ECB (Line 87)
  └─ 🟡 MD5 hash usage (Line 103)

📁 src/auth.js (2 issues)
  ├─ 🔴 Hardcoded credentials (Line 15)
  └─ 🟠 Insecure random (Line 56)
```

**2. Statistics Dashboard**:
- Total vulnerabilities: 15
- Critical: 5 (33%)
- High: 7 (47%)
- Medium: 3 (20%)
- Trend: ↓ 20% from last week
- Most common: RSA key size issues (6)

**3. Details Pane**:
```
🔴 RSA-1024 Detected

Location: src/crypto.js:42
Severity: Critical
Category: Cryptography / Key Size
CWE: CWE-326

Description:
RSA key size of 1024 bits is vulnerable to factorization
attacks and should be upgraded to at least 2048 bits.

Recommendation:
Use RSA-2048 or RSA-4096 for new keys.
Consider migrating to post-quantum algorithms (Kyber, Dilithium).

[Quick Fix] [Ignore] [Learn More]
```

**4. Export Formats**:
- **JSON**: Machine-readable full report
- **SARIF**: Static Analysis Results Interchange Format
- **HTML**: Printable report with charts
- **CSV**: Spreadsheet-compatible
- **PDF**: Executive summary (via HTML)

---

### Sprint 7 Deliverables

**At Sprint End**:
- ✅ Scanner panel with comprehensive vulnerability view
- ✅ Grouping, filtering, search working
- ✅ Statistics dashboard with visualizations
- ✅ Export to multiple formats
- ✅ Click-to-navigate functionality
- ✅ 5,000+ installs
- ✅ 4.5+ star rating

---

## 🔧 Sprint 8-9: JetBrains Plugin

**Status**: ⏳ Pending Sprint 7
**Timeline**: Week 15-18 (May 13 - Jun 8, 2026)
**Story Points**: 28 (14 per sprint)
**Team**: Full team (4 developers)
**Dependencies**: Sprint 7 complete

### Issues

#### Issue #11: JetBrains Plugin (13 points, High) - **Sprints 8-9**

**Goal**: Full-featured plugin for IntelliJ IDEA, PyCharm, WebStorm, and other JetBrains IDEs

---

### **Sprint 8**: Foundation (14 points)

**Tasks**:
- [ ] Setup IntelliJ Platform SDK project
- [ ] Create plugin descriptor (plugin.xml)
- [ ] Implement inspection tool interface
- [ ] Add real-time code analysis
- [ ] Create intention actions (quick fixes)
- [ ] Add settings panel
- [ ] Implement tool window (scanner panel)
- [ ] Basic testing on IntelliJ IDEA

**Acceptance Criteria** (Sprint 8):
```gherkin
Scenario: Plugin installation in IntelliJ IDEA
  Given I have IntelliJ IDEA 2023.1+
  When I go to Settings > Plugins > Marketplace
  And I search for "PQC Scanner"
  Then I should find the plugin
  When I click "Install"
  Then the plugin should install successfully
  And I should see the PQC Scanner tool window

Scenario: Real-time inspections
  Given I have a Java file open in IntelliJ
  When I write code using RSA-1024
  Then I should see a warning highlight
  And hovering shows "RSA key size below 2048 bits"
```

**Definition of Done** (Sprint 8):
- ✅ Plugin installable in IntelliJ IDEA
- ✅ Real-time inspections working
- ✅ Basic quick fixes implemented
- ✅ Settings panel functional
- ✅ Tool window with vulnerability list
- ✅ Tested on IntelliJ IDEA 2023.1+

---

### **Sprint 9**: Polish & Multi-IDE Support (14 points)

**Tasks**:
- [ ] Test and optimize for PyCharm
- [ ] Test and optimize for WebStorm
- [ ] Test and optimize for PhpStorm
- [ ] Add IDE-specific features (Java analysis for IntelliJ)
- [ ] Performance optimization for large projects
- [ ] Publish to JetBrains Marketplace
- [ ] Create documentation and screenshots
- [ ] Integration with JetBrains Security plugin
- [ ] Beta testing with community

**Acceptance Criteria** (Sprint 9):
```gherkin
Scenario: Multi-IDE compatibility
  Given the PQC Scanner plugin is installed
  When I open it in IntelliJ IDEA
  Then Java-specific crypto vulnerabilities are detected
  When I open it in PyCharm
  Then Python-specific crypto vulnerabilities are detected
  When I open it in WebStorm
  Then JavaScript/TypeScript vulnerabilities are detected

Scenario: Plugin marketplace listing
  Given I visit JetBrains Marketplace
  When I search for "PQC Scanner"
  Then I should see the plugin
  And it should show support for 5+ JetBrains IDEs
  And it should have a 4.5+ rating
```

**Definition of Done** (Sprint 9):
- ✅ Plugin works in 5+ JetBrains IDEs
- ✅ Language-specific analysis for Java, Python, JS/TS, PHP
- ✅ Published to JetBrains Marketplace
- ✅ Performance optimized (<1s analysis per file)
- ✅ Complete documentation
- ✅ 1,000+ installs
- ✅ 4.5+ star rating

---

### Sprints 8-9 Deliverables

**At Sprint End**:
- ✅ JetBrains plugin published to marketplace
- ✅ Support for 5+ JetBrains IDEs
- ✅ Feature parity with VS Code extension
- ✅ Language-specific analysis
- ✅ 1,000+ installs
- ✅ 4.5+ star rating
- ✅ Community feedback incorporated

**Supported JetBrains IDEs**:
1. IntelliJ IDEA (Ultimate & Community)
2. PyCharm (Professional & Community)
3. WebStorm
4. PhpStorm
5. GoLand
6. Rider (.NET)
7. CLion (C/C++)

---

## 📊 Q2 Summary & Metrics

### Story Point Distribution

| Sprint | VS Code | JetBrains | Testing | Docs | Total |
|--------|---------|-----------|---------|------|-------|
| Sprint 5 | 14 | - | - | - | 14 |
| Sprint 6 | 13 | - | - | - | 13 |
| Sprint 7 | 13 | - | - | - | 13 |
| Sprint 8 | - | 14 | - | - | 14 |
| Sprint 9 | - | 10 | 2 | 2 | 14 |
| **Total** | **40** | **24** | **2** | **2** | **68** |

### Feature Completion

- **VS Code Extension**: Full-featured (#7, #8, #9, #10)
- **JetBrains Plugin**: Multi-IDE support (#11)
- **Test Coverage**: 85%+ target
- **Documentation**: Complete user guides
- **Marketplace**: Both platforms live

### Team Velocity

- **Target Velocity**: 13-14 points/sprint
- **Actual Velocity**: TBD (track during execution)
- **Sprint Duration**: 2 weeks per sprint
- **Total Duration**: 10 weeks (2.5 months)

### Success Metrics

**Technical**:
- ✅ <300ms real-time diagnostic latency
- ✅ 10+ quick fix types
- ✅ 85%+ test coverage
- ✅ <500ms panel load time
- ✅ Support for 5+ JetBrains IDEs

**Adoption** (Q2 End):
- Target VS Code: 100,000+ installs
- Target JetBrains: 10,000+ installs
- Target Combined: 4.5+ star rating
- Target Engagement: 50%+ active users

**Quality**:
- ✅ Zero critical bugs at release
- ✅ <5% crash rate
- ✅ 90%+ quick fix success rate

---

## 🚧 Risks & Mitigation

### Risk 1: VS Code API Changes
**Impact**: Medium
**Probability**: Low
**Mitigation**:
- Use stable APIs only
- Monitor VS Code insiders builds
- Automated compatibility testing

### Risk 2: Performance in Large Projects
**Impact**: High
**Probability**: Medium
**Mitigation**:
- Early performance testing
- Incremental analysis only
- Configurable scan depth
- Background processing

### Risk 3: JetBrains Platform Complexity
**Impact**: High
**Probability**: Medium
**Mitigation**:
- Start with IntelliJ IDEA (most popular)
- Reuse core scanning engine
- Community plugin expertise
- Extended beta period

### Risk 4: Marketplace Approval Delays
**Impact**: Medium
**Probability**: Low
**Mitigation**:
- Submit early in sprint
- Follow all guidelines strictly
- Have rollback plan
- Alternative distribution (GitHub releases)

---

## 🎯 Transition to Q3

### Sprint 9 Planning Tasks:
- [ ] Q2 retrospective meeting
- [ ] Q3 roadmap review
- [ ] Sprint 10 planning (Multi-language detection)
- [ ] Resource allocation for Q3
- [ ] Lessons learned documentation

### Q3 Preview:
**Epic #12**: Multi-Language Detection
**Timeline**: Q3 2026 (Sprints 10-14)
**Story Points**: 63 points
**Goal**: Expand support to Kotlin, Swift, and advanced framework detection

---

**Document Version**: 1.0
**Created**: 2025-11-07
**Author**: ArcQubit Engineering Team
**Status**: Pending Q1 completion
