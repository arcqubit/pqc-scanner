#!/bin/bash

# Script to create GitHub issues for PQC Scanner Implementation Plan
# Based on: docs/implementation/IMPLEMENTATION_PLAN.md

set -e

REPO="arcqubit/pqc-scanner"

echo "🚀 Creating GitHub Issues for PQC Scanner Implementation Plan..."
echo ""

# ============================================================================
# Q1 2026: CLI Tool & Developer Experience
# ============================================================================

echo "📋 Creating Q1 2026 - CLI Tool Issues..."

# Epic: CLI Tool Development
gh issue create --repo "$REPO" \
  --title "Epic: CLI Tool Development" \
  --body "## Epic: CLI Tool & Developer Experience

**Goal**: Provide powerful command-line interface for developers
**Timeline**: Q1 2026 (Sprints 1-3)
**Team Size**: 3 developers, 1 QA

### Features
- Global CLI installation via NPM
- Interactive scanning mode with auto-fix prompts
- Watch mode for continuous scanning
- CLI configuration file management

### Success Metrics
- Installation time < 30 seconds
- Scan speed: 1000 LOC in < 5 seconds
- Memory usage < 100MB
- User satisfaction: 4.5+ stars

### Dependencies
- Rust WASM core (✅ Complete)
- NPM package structure (✅ Complete)

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md
- Vision: docs/vision/ROADMAP_2025-2030.md" \
  --label "cli,priority-critical,points-21" \
  --milestone "Q1 2026 - CLI & Dev Experience"

# Feature 1: Global CLI Installation
gh issue create --repo "$REPO" \
  --title "Feature: Global CLI Installation via NPM" \
  --body "## Feature: Global CLI Installation

**As a developer**
**I want to install the PQC scanner globally via NPM**
**So that I can scan projects from any directory**

### Story Points: 5

### Acceptance Criteria

\`\`\`gherkin
Scenario: Install CLI globally via NPM
  Given Node.js v18+ is installed
  And NPM is available in the system PATH
  When I run \"npm install -g @arcqubit/pqc-scanner\"
  Then the installation should complete successfully
  And the \"pqc-scan\" command should be available globally
  And running \"pqc-scan --version\" should display version \"1.3.0\"
  And the installation size should be less than 50MB

Scenario: Verify CLI binary permissions
  Given the CLI is installed globally
  When I check the file permissions of \"pqc-scan\"
  Then the binary should be executable
  And the binary should work without sudo privileges
  And the binary should be in the system PATH

Scenario: Uninstall CLI cleanly
  Given the CLI is installed globally
  When I run \"npm uninstall -g @arcqubit/pqc-scanner\"
  Then all CLI files should be removed
  And the \"pqc-scan\" command should no longer be available
  And no orphaned configuration files should remain
\`\`\`

### Technical Notes
- Create bin/ directory with CLI entry point
- Add shebang: #!/usr/bin/env node
- Configure package.json bin field
- Test on Linux, macOS, Windows (WSL)

### Testing Requirements
- Unit tests for CLI argument parsing
- Integration tests for installation process
- Cross-platform compatibility tests
- Performance: installation < 30 seconds

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 1)" \
  --label "cli,priority-critical,points-5" \
  --milestone "Q1 2026 - CLI & Dev Experience"

# Feature 2: Interactive Scanning Mode
gh issue create --repo "$REPO" \
  --title "Feature: Interactive Scanning with Auto-Fix Prompts" \
  --body "## Feature: Interactive Code Scanning

**As a security engineer**
**I want to scan code interactively with real-time feedback**
**So that I can quickly identify and fix vulnerabilities**

### Story Points: 8

### Acceptance Criteria

\`\`\`gherkin
Scenario: Interactive scan with auto-fix prompts
  Given I have a file \"src/crypto.js\" with RSA-1024 usage
  When I run \"pqc-scan --interactive src/crypto.js\"
  Then I should see a vulnerability report for RSA-1024
  And I should be prompted \"Auto-fix to RSA-2048? (y/n)\"
  When I type \"y\"
  Then the file should be automatically modified
  And a backup \"src/crypto.js.bak\" should be created
  And I should see a success message \"✓ Fixed RSA-1024 → RSA-2048\"

Scenario: Batch interactive fixes
  Given I have 10 files with MD5 hash usage
  When I run \"pqc-scan --interactive src/ --auto-fix\"
  Then I should see a summary \"Found 10 fixable vulnerabilities\"
  And I should be prompted \"Fix all? (y/n/review)\"
  When I type \"review\"
  Then I should see each vulnerability one-by-one
  And I can choose \"y\", \"n\", or \"skip\" for each

Scenario: Dry-run mode
  Given I have files with crypto vulnerabilities
  When I run \"pqc-scan --interactive --dry-run src/\"
  Then I should see what changes would be made
  But no files should be modified
  And I should see a summary \"Would fix: 5, Would skip: 2\"

Scenario: Interactive scan performance
  Given I have a project with 1000 source files
  When I run \"pqc-scan --interactive .\"
  Then the initial scan should complete in under 5 seconds
  And interactive prompts should appear immediately
  And memory usage should stay under 100MB
\`\`\`

### Technical Implementation
- Use \`inquirer\` or \`prompts\` for CLI prompts
- Implement backup strategy (.bak files)
- Create interactive UI with colored output
- Add progress indicators for large scans

### UX Requirements
- Clear, color-coded vulnerability severity
- Helpful error messages
- Undo/rollback capability
- Keyboard shortcuts (y/n/q/?)

### Testing Requirements
- Unit tests for interactive prompt logic
- Integration tests with mocked user input
- Performance tests with 1000+ files
- User acceptance testing

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 2)" \
  --label "cli,ux,priority-critical,points-8" \
  --milestone "Q1 2026 - CLI & Dev Experience"

# Feature 3: Watch Mode
gh issue create --repo "$REPO" \
  --title "Feature: Watch Mode for Continuous Scanning" \
  --body "## Feature: Watch Mode for Real-Time Scanning

**As a developer**
**I want the scanner to watch my files for changes**
**So that I get immediate feedback when I write vulnerable code**

### Story Points: 8

### Acceptance Criteria

\`\`\`gherkin
Scenario: Start watch mode on a directory
  Given I am in a project root directory
  When I run \"pqc-scan --watch src/\"
  Then I should see \"👁️  Watching src/ for changes...\"
  And the scanner should run in the background
  When I modify \"src/auth.js\" to use MD5
  Then within 1 second I should see a warning
  And the warning should display the file path and line number

Scenario: Auto-remediation in watch mode
  Given watch mode is running with \"--auto-remediate\"
  When I save a file with SHA-1 usage
  Then the scanner should detect it immediately
  And the file should be auto-fixed to SHA-256
  And I should see a notification \"🔧 Auto-fixed: SHA-1 → SHA-256\"
  And my editor should reload the file with changes

Scenario: Watch mode with file filters
  Given I run \"pqc-scan --watch src/ --include '*.js,*.ts'\"
  When I modify a Python file \"src/crypto.py\"
  Then the scanner should ignore it
  When I modify a JavaScript file \"src/crypto.js\"
  Then the scanner should scan it immediately

Scenario: Watch mode stability
  Given watch mode is running
  When 100 files are modified rapidly
  Then the scanner should queue scans efficiently
  And the process should not crash
  And memory usage should not exceed 200MB
  And CPU usage should stay under 30%
\`\`\`

### Technical Implementation
- Use \`chokidar\` for file watching
- Implement debouncing (300ms delay)
- Queue system for batch updates
- Graceful shutdown on SIGINT/SIGTERM

### Performance Requirements
- File change detection: < 100ms
- Scan trigger: < 1 second
- Memory leak prevention
- CPU throttling for large changes

### Testing Requirements
- Unit tests for file watcher logic
- Stress tests with 100+ rapid changes
- Memory leak tests (run for 1+ hours)
- Cross-platform file watcher tests

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 3)" \
  --label "cli,performance,priority-high,points-8" \
  --milestone "Q1 2026 - CLI & Dev Experience"

# Feature 4: CLI Configuration
gh issue create --repo "$REPO" \
  --title "Feature: CLI Configuration File (.pqcrc.json)" \
  --body "## Feature: CLI Configuration Management

**As a team lead**
**I want to define scanning rules in a config file**
**So that all team members use consistent settings**

### Story Points: 5

### Acceptance Criteria

\`\`\`gherkin
Scenario: Create default configuration
  Given I am in a project root
  When I run \"pqc-scan --init\"
  Then a file \".pqcrc.json\" should be created
  And it should contain default scanning rules
  And it should include severity thresholds
  And it should include auto-fix preferences

Scenario: Use custom configuration
  Given I have a \".pqcrc.json\" with custom rules:
    {
      \"severity\": \"high\",
      \"autoFix\": true,
      \"ignore\": [\"test/**\", \"vendor/**\"],
      \"rules\": {
        \"RSA-1024\": \"error\",
        \"MD5\": \"warning\"
      }
    }
  When I run \"pqc-scan src/\"
  Then files in \"test/\" should be ignored
  And RSA-1024 findings should cause exit code 1
  And MD5 findings should not cause failures

Scenario: Validate configuration syntax
  Given I have an invalid \".pqcrc.json\"
  When I run \"pqc-scan src/\"
  Then I should see \"❌ Invalid configuration file\"
  And I should see specific error messages
  And the exit code should be 2
\`\`\`

### Configuration Schema
\`\`\`json
{
  \"$schema\": \"https://pqc-scanner.arcqubit.io/schema.json\",
  \"severity\": \"medium\",
  \"autoFix\": false,
  \"ignore\": [],
  \"include\": [],
  \"rules\": {},
  \"compliance\": [],
  \"output\": \"text\"
}
\`\`\`

### Technical Implementation
- JSON schema validation
- Support .pqcrc.json, .pqcrc.yaml
- Hierarchical config (project > user > global)
- Environment variable overrides

### Testing Requirements
- Unit tests for config parsing
- Integration tests with various configs
- Schema validation tests
- Error message clarity tests

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 3)" \
  --label "cli,priority-medium,points-5" \
  --milestone "Q1 2026 - CLI & Dev Experience"

echo "✅ Q1 2026 CLI issues created"
echo ""

# ============================================================================
# Q2 2026: IDE Extensions
# ============================================================================

echo "📋 Creating Q2 2026 - IDE Extension Issues..."

# Epic: VS Code Extension
gh issue create --repo "$REPO" \
  --title "Epic: VS Code Extension Development" \
  --body "## Epic: VS Code Extension

**Goal**: Bring PQC scanning into VS Code with real-time feedback
**Timeline**: Q2 2026 (Sprints 4-7)
**Team Size**: 2 developers, 1 UX designer, 1 QA

### Features
- VS Code Marketplace publishing
- Real-time crypto vulnerability highlighting
- Quick fix actions (lightbulb menu)
- Dedicated scanner side panel
- Problems panel integration

### Success Metrics
- Extension installs: 10,000+ in Q2
- User rating: 4+ stars
- Response time: < 500ms for highlights
- Memory usage: < 50MB

### Dependencies
- Rust WASM core (✅ Complete)
- Language Server Protocol implementation
- VS Code Extension API

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md
- Vision: docs/vision/ROADMAP_2025-2030.md" \
  --label "ide,priority-critical,points-21" \
  --milestone "Q2 2026 - IDE Extensions"

# Feature 5: VS Code Extension Installation
gh issue create --repo "$REPO" \
  --title "Feature: VS Code Extension Marketplace Publishing" \
  --body "## Feature: VS Code Extension Installation

**As a VS Code user**
**I want to install the PQC Scanner extension from the marketplace**
**So that I can scan code without leaving my editor**

### Story Points: 5

### Acceptance Criteria

\`\`\`gherkin
Scenario: Install from VS Code Marketplace
  Given I have VS Code open
  When I search for \"ArcQubit PQC Scanner\" in extensions
  Then I should see the extension in search results
  And it should have 4+ star rating display
  And it should show \"Quantum-Safe Crypto Auditor\" description
  When I click \"Install\"
  Then the extension should install in under 30 seconds
  And I should see a welcome notification

Scenario: First-time setup wizard
  Given the extension is installed for the first time
  When VS Code reloads
  Then I should see a setup wizard
  And I should be asked \"Scan workspace now? (Yes/No)\"
  And I should be asked \"Enable auto-fix? (Yes/No/Ask)\"
  When I complete the wizard
  Then my preferences should be saved
\`\`\`

### Technical Implementation
- Create VS Code extension with vsce
- Implement activation events
- Add welcome walkthrough
- Configure extension manifest
- Set up CI/CD for marketplace publishing

### Marketplace Requirements
- Icon (128x128 PNG)
- README with screenshots
- CHANGELOG
- LICENSE
- Keywords for discoverability

### Testing Requirements
- Install tests on Windows, macOS, Linux
- First-run experience testing
- Marketplace submission validation

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 4)" \
  --label "ide,ux,priority-critical,points-5" \
  --milestone "Q2 2026 - IDE Extensions"

# Feature 6: Real-Time Highlighting
gh issue create --repo "$REPO" \
  --title "Feature: Real-Time Crypto Vulnerability Highlighting" \
  --body "## Feature: Inline Code Diagnostics

**As a developer**
**I want to see crypto vulnerabilities highlighted in my code**
**So that I can fix them while writing code**

### Story Points: 13

### Acceptance Criteria

\`\`\`gherkin
Scenario: Highlight vulnerable crypto usage
  Given I open a file \"src/auth.js\"
  And the file contains: const hash = crypto.createHash('md5');
  Then I should see a red squiggly underline under 'md5'
  When I hover over 'md5'
  Then I should see a tooltip:
    \"🛑 MD5 is cryptographically broken
    Severity: High
    Recommendation: Use SHA-256 or SHA-3
    [Quick Fix] [View Details]\"

Scenario: Color-coded severity levels
  Given I have a file with multiple vulnerabilities
  Then RSA-1024 should have a red underline (critical)
  And SHA-1 should have an orange underline (high)
  And RSA-2048 should have a yellow underline (medium)
  And the gutter should show icons for each severity

Scenario: Real-time scanning as I type
  Given I am editing \"crypto.js\"
  When I type \"hashlib.md5(\"
  Then within 500ms I should see a warning highlight
  And the Problems panel should update immediately
  When I change it to \"hashlib.sha256(\"
  Then the warning should disappear within 500ms

Scenario: Performance with large files
  Given I open a 5000-line JavaScript file
  When the extension scans the file
  Then the scan should complete in under 2 seconds
  And VS Code should remain responsive
  And the extension should use less than 50MB RAM
\`\`\`

### Technical Implementation
- Implement Language Server Protocol (LSP)
- Use Diagnostic API for squiggles
- Implement incremental parsing
- Add debouncing for typing events

### Performance Requirements
- Scan latency: < 500ms
- Memory usage: < 50MB per file
- No UI blocking
- Efficient diff-based rescanning

### Testing Requirements
- Unit tests for diagnostic creation
- Performance tests with large files
- User interaction tests
- Memory leak tests

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 4-5)" \
  --label "ide,ux,performance,priority-critical,points-13" \
  --milestone "Q2 2026 - IDE Extensions"

# Feature 7: Quick Fix Actions
gh issue create --repo "$REPO" \
  --title "Feature: One-Click Vulnerability Quick Fixes" \
  --body "## Feature: One-Click Vulnerability Fixes

**As a developer**
**I want to fix vulnerabilities with a single click**
**So that I can remediate issues quickly**

### Story Points: 8

### Acceptance Criteria

\`\`\`gherkin
Scenario: Apply quick fix from lightbulb menu
  Given I have \"crypto.createHash('md5')\" in my code
  And the cursor is on the 'md5' string
  When I click the yellow lightbulb icon
  Then I should see quick fix options:
    \"🔧 Replace MD5 with SHA-256 (recommended)
    🔧 Replace MD5 with SHA-3-256
    📖 Learn more about MD5 vulnerabilities
    🚫 Ignore this warning\"
  When I select \"Replace MD5 with SHA-256\"
  Then the code should change to \"crypto.createHash('sha256')\"
  And I should see a success message
  And the file should be marked as modified

Scenario: Batch fix all similar issues
  Given I have 10 occurrences of MD5 in my file
  When I right-click on one MD5 warning
  Then I should see \"Fix all MD5 → SHA-256 in file\"
  When I click it
  Then all 10 occurrences should be fixed
  And I should see \"✓ Fixed 10 MD5 instances\"

Scenario: Preview changes before applying
  Given I select a quick fix
  When I hold \"Ctrl\" while clicking the fix
  Then I should see a diff preview
  And I can review the before/after code
  And I can choose \"Apply\" or \"Cancel\"
\`\`\`

### Technical Implementation
- Implement CodeActionProvider
- Use WorkspaceEdit for modifications
- Add diff preview UI
- Batch operations support

### UX Requirements
- Clear, descriptive action titles
- Safety confirmations for batch operations
- Undo support (Ctrl+Z)
- Preview diffs for complex changes

### Testing Requirements
- Unit tests for code actions
- Integration tests for batch fixes
- Undo/redo tests
- Edge case handling (read-only files, etc.)

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 5-6)" \
  --label "ide,ux,priority-critical,points-8" \
  --milestone "Q2 2026 - IDE Extensions"

# Feature 8: Scanner Side Panel
gh issue create --repo "$REPO" \
  --title "Feature: Dedicated Scanner View Panel" \
  --body "## Feature: Dedicated Scanner View Panel

**As a security engineer**
**I want a dedicated panel for vulnerability management**
**So that I can see all issues and track remediation progress**

### Story Points: 8

### Acceptance Criteria

\`\`\`gherkin
Scenario: Open scanner panel
  Given VS Code is open
  When I click the PQC Scanner icon in the activity bar
  Then a side panel should open
  And I should see tabs: \"Vulnerabilities\", \"Compliance\", \"Settings\"

Scenario: Vulnerabilities list view
  Given the scanner panel is open on \"Vulnerabilities\" tab
  Then I should see a grouped list:
    \"🔴 Critical (3)
      - RSA-1024 in auth.js:45
      - DES encryption in db.js:120
    🟠 High (7)
      - MD5 in utils.js:12
      - SHA-1 in hasher.js:34
    🟡 Medium (5)
      - RSA-2048 (PQC migration recommended) in api.js:67\"
  When I click \"RSA-1024 in auth.js:45\"
  Then the file should open at line 45
  And the vulnerable code should be highlighted

Scenario: Compliance score dashboard
  Given the scanner panel is open on \"Compliance\" tab
  Then I should see:
    \"NIST 800-53 SC-13 Compliance
    Score: 67/100
    Status: ⚠️ Partially Compliant
    Findings: 15 total
    - 3 Critical
    - 7 High
    - 5 Medium
    [Generate Report] [Export OSCAL JSON]\"
\`\`\`

### Technical Implementation
- Create TreeView with TreeDataProvider
- Implement WebviewView for dashboard
- Add navigation to source locations
- Real-time updates on file changes

### UX Requirements
- Collapsible groups by severity
- Search/filter functionality
- Sort by file, severity, type
- Export capabilities

### Testing Requirements
- Unit tests for tree view provider
- Integration tests for navigation
- UI tests with multiple vulnerabilities
- Performance tests with 100+ issues

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 6-7)" \
  --label "ide,ux,priority-high,points-8" \
  --milestone "Q2 2026 - IDE Extensions"

# Feature 9: JetBrains Plugin
gh issue create --repo "$REPO" \
  --title "Feature: JetBrains IDE Plugin (IntelliJ, PyCharm, WebStorm)" \
  --body "## Feature: JetBrains IDE Integration

**As a JetBrains user**
**I want the same PQC scanning features in my IDE**
**So that I can use my preferred development environment**

### Story Points: 13

### Acceptance Criteria

\`\`\`gherkin
Scenario: Install from JetBrains Marketplace
  Given I have IntelliJ IDEA open
  When I go to Settings > Plugins > Marketplace
  And I search for \"ArcQubit PQC Scanner\"
  Then I should find the plugin
  When I click \"Install\"
  Then the plugin should install successfully
  And I should see \"Restart IDE\" button

Scenario: Inspection integration
  Given the plugin is installed in IntelliJ
  When I open a Java file with RSA-1024 usage
  Then I should see an inspection warning
  And it should appear in the Problems tool window
  And I should be able to apply quick fixes
  And the behavior should match VS Code extension

Scenario: Multi-IDE support
  Given the plugin is built with IntelliJ Platform
  Then it should work in IntelliJ IDEA
  And it should work in PyCharm
  And it should work in WebStorm
  And it should work in Android Studio
  And settings should sync across IDEs
\`\`\`

### Technical Implementation
- Use IntelliJ Platform SDK
- Implement LocalInspectionTool
- Create IntentionAction for quick fixes
- Build with Gradle
- Use IntelliJ Platform Plugin Template

### IDE Support
- IntelliJ IDEA (2023.1+)
- PyCharm (2023.1+)
- WebStorm (2023.1+)
- Android Studio (Flamingo+)
- PhpStorm (2023.1+)

### Testing Requirements
- Unit tests with IntelliJ test framework
- Integration tests across IDE versions
- UI tests for inspections
- Compatibility tests

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 8-9)" \
  --label "ide,priority-high,points-13" \
  --milestone "Q2 2026 - IDE Extensions"

echo "✅ Q2 2026 IDE Extension issues created"
echo ""

# ============================================================================
# Q3 2026: Enhanced Detection
# ============================================================================

echo "📋 Creating Q3 2026 - Enhanced Detection Issues..."

# Epic: Multi-Language Expansion
gh issue create --repo "$REPO" \
  --title "Epic: Multi-Language Detection Expansion" \
  --body "## Epic: Multi-Language Expansion

**Goal**: Support 15+ programming languages
**Timeline**: Q3 2026 (Sprints 10-14)
**Team Size**: 4 developers, 2 QA

### Features
- Kotlin language support
- Swift language support
- NIST PQC algorithm detection (Kyber, Dilithium, SPHINCS+)
- Framework-specific pattern detection
- Binary analysis capabilities

### Success Metrics
- Language coverage: 15+ languages
- Detection accuracy: 99%+
- False positive rate: < 1%
- New crypto patterns: 20+ additional patterns

### Dependencies
- Rust parser infrastructure (✅ Complete)
- Test corpus for new languages

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md
- Vision: docs/vision/ROADMAP_2025-2030.md" \
  --label "detection,priority-high,points-21" \
  --milestone "Q3 2026 - Enhanced Detection"

# Feature 10: Kotlin Support
gh issue create --repo "$REPO" \
  --title "Feature: Kotlin Language Cryptography Detection" \
  --body "## Feature: Kotlin Cryptography Detection

**As an Android developer**
**I want to scan Kotlin code for crypto vulnerabilities**
**So that my mobile apps are quantum-safe**

### Story Points: 8

### Acceptance Criteria

\`\`\`gherkin
Scenario: Detect Kotlin crypto imports
  Given I have a Kotlin file with:
    import java.security.KeyPairGenerator
    fun generateRSAKey() {
      val keyGen = KeyPairGenerator.getInstance(\"RSA\")
      keyGen.initialize(1024)  // Vulnerable!
    }
  When I scan the file
  Then I should detect \"RSA-1024\" vulnerability
  And severity should be \"critical\"
  And line number should be correct

Scenario: Detect Kotlin Crypto API usage
  Given I have Kotlin code using Android Keystore
  When the code uses KeyGenParameterSpec with 128-bit AES
  Then I should detect \"AES-128\" usage
  And recommendation should be \"Use AES-256 for better security\"

Scenario: Kotlin-specific auto-remediation
  Given I have vulnerable Kotlin crypto code
  When I apply auto-fix
  Then the code should use Kotlin idioms
  And it should preserve nullability annotations
  And it should maintain code formatting
\`\`\`

### Technical Implementation
- Implement Kotlin AST parser
- Add Kotlin crypto pattern matchers
- Support Java interop patterns
- Handle Kotlin-specific syntax (data classes, extension functions)

### Testing Requirements
- Unit tests with Kotlin test corpus
- Android-specific crypto patterns
- Kotlin DSL detection
- Cross-language tests (Kotlin + Java)

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 10)" \
  --label "detection,priority-high,points-8" \
  --milestone "Q3 2026 - Enhanced Detection"

# Feature 11: Swift Support
gh issue create --repo "$REPO" \
  --title "Feature: Swift Language Cryptography Detection" \
  --body "## Feature: Swift Cryptography Detection

**As an iOS developer**
**I want to scan Swift code for crypto vulnerabilities**
**So that my iOS apps are secure**

### Story Points: 8

### Acceptance Criteria

\`\`\`gherkin
Scenario: Detect CommonCrypto usage
  Given I have a Swift file using CommonCrypto MD5
  When I scan the file
  Then I should detect \"MD5\" vulnerability
  And I should see \"CommonCrypto MD5 is deprecated\"

Scenario: Detect CryptoKit weak algorithms
  Given I have Swift code using CryptoKit
  When the code uses insecure algorithms
  Then the scanner should detect them
  And recommendations should reference CryptoKit best practices

Scenario: Auto-fix to CryptoKit
  Given I have CommonCrypto MD5 usage
  When I apply auto-fix
  Then it should suggest CryptoKit SHA256
\`\`\`

### Technical Implementation
- Implement Swift AST parser
- Support CommonCrypto patterns
- Support CryptoKit patterns
- Handle Swift-specific syntax

### Testing Requirements
- Unit tests with Swift test corpus
- iOS-specific crypto patterns
- SwiftUI integration tests
- Cross-platform tests (iOS/macOS)

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 11)" \
  --label "detection,priority-high,points-8" \
  --milestone "Q3 2026 - Enhanced Detection"

# Feature 12: NIST PQC Algorithm Detection
gh issue create --repo "$REPO" \
  --title "Feature: NIST Post-Quantum Cryptography Algorithm Detection" \
  --body "## Feature: Post-Quantum Cryptography Standards Detection

**As a forward-looking developer**
**I want to detect usage of NIST-approved PQC algorithms**
**So that I can track quantum-safe migration progress**

### Story Points: 13

### Acceptance Criteria

\`\`\`gherkin
Scenario: Detect CRYSTALS-Kyber usage
  Given I have code using CRYSTALS-Kyber:
    from pqcrypto.kem.kyber512 import generate_keypair
    public_key, secret_key = generate_keypair()
  When I scan the file
  Then I should see \"✅ CRYSTALS-Kyber detected\"
  And it should be marked as \"quantum-safe\"
  And compliance score should increase

Scenario: Detect CRYSTALS-Dilithium signatures
  Given I have code using Dilithium digital signatures
  When I scan the file
  Then I should see \"✅ CRYSTALS-Dilithium detected\"
  And it should be reported in compliance section

Scenario: Detect SPHINCS+ signatures
  Given I have code using SPHINCS+
  When I scan the file
  Then I should see \"✅ SPHINCS+ detected\"
  And it should be marked as \"hash-based signature (quantum-safe)\"

Scenario: Mixed crypto detection
  Given I have code using both RSA and CRYSTALS-Kyber
  When I scan the file
  Then I should see \"⚠️ Hybrid crypto mode detected\"
  And it should show \"RSA-2048 (transitional) + Kyber-512 (quantum-safe)\"
  And it should recommend \"Consider migrating fully to PQC\"
\`\`\`

### NIST PQC Standards (2024)
- **KEM**: CRYSTALS-Kyber (ML-KEM)
- **Signatures**: CRYSTALS-Dilithium (ML-DSA), SPHINCS+ (SLH-DSA), Falcon
- **Future**: BIKE, HQC, Classic McEliece (under consideration)

### Technical Implementation
- Add PQC library detection patterns
- Support multiple PQC implementations
- Detect hybrid crypto schemes
- Track PQC adoption metrics

### Testing Requirements
- Unit tests for each PQC algorithm
- Hybrid crypto detection tests
- Library version detection
- Compliance impact tests

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 12)
- NIST PQC Standards: https://csrc.nist.gov/Projects/post-quantum-cryptography" \
  --label "detection,security,priority-critical,points-13" \
  --milestone "Q3 2026 - Enhanced Detection"

# Feature 13: Framework Detection
gh issue create --repo "$REPO" \
  --title "Feature: Framework-Specific Crypto Pattern Detection" \
  --body "## Feature: Detect Crypto Usage in Popular Frameworks

**As a full-stack developer**
**I want framework-specific crypto pattern detection**
**So that I catch vulnerabilities in framework abstractions**

### Story Points: 13

### Acceptance Criteria

\`\`\`gherkin
Scenario: Django crypto detection
  Given I have a Django application
  When I use weak password hashers in settings:
    PASSWORD_HASHERS = [
      'django.contrib.auth.hashers.MD5PasswordHasher',  # Vulnerable!
    ]
  Then I should detect \"Django MD5PasswordHasher\"
  And recommendation should be \"Use Argon2PasswordHasher\"

Scenario: Spring Security crypto detection
  Given I have a Spring Boot application
  When I configure weak encryption:
    @Bean
    public TextEncryptor textEncryptor() {
      return Encryptors.text(\"password\", \"salt\");  // Weak!
    }
  Then I should detect \"Spring Encryptors with weak config\"

Scenario: Express.js crypto detection
  Given I have an Express.js app using crypto
  When I use DES cipher in a route
  Then I should detect \"DES cipher in Express.js route\"
  And I should see the route path
\`\`\`

### Framework Support
- **Python**: Django, Flask, FastAPI
- **Java**: Spring Boot, Spring Security
- **JavaScript**: Express.js, NestJS
- **Ruby**: Rails
- **PHP**: Laravel, Symfony
- **Go**: Gin, Echo
- **.NET**: ASP.NET Core

### Technical Implementation
- Framework-specific AST patterns
- Configuration file parsing
- Route/endpoint detection
- ORM/database crypto detection

### Testing Requirements
- Integration tests with real frameworks
- Version compatibility tests
- Framework upgrade detection
- Best practice recommendations

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 13-14)" \
  --label "detection,integration,priority-medium,points-13" \
  --milestone "Q3 2026 - Enhanced Detection"

echo "✅ Q3 2026 Enhanced Detection issues created"
echo ""

# ============================================================================
# Q4 2026: Compliance Ecosystem
# ============================================================================

echo "📋 Creating Q4 2026 - Compliance Ecosystem Issues..."

# Epic: Regulatory Compliance
gh issue create --repo "$REPO" \
  --title "Epic: Regulatory Compliance Automation" \
  --body "## Epic: Regulatory Compliance Automation

**Goal**: Support FedRAMP, ISO 27001, PCI DSS, HIPAA
**Timeline**: Q4 2026 (Sprints 15-19)
**Team Size**: 3 developers, 1 compliance expert, 1 QA

### Features
- FedRAMP compliance reporting
- ISO 27001 cryptographic controls
- PCI DSS payment security
- HIPAA healthcare compliance
- Automated report generation

### Success Metrics
- Compliance frameworks supported: 4+
- Report generation time: < 30 seconds
- Compliance accuracy: 99%+
- Enterprise adoption: 10+ customers

### Dependencies
- OSCAL JSON implementation (✅ Complete)
- NIST 800-53 mapping (✅ Complete)

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md
- Vision: docs/vision/ROADMAP_2025-2030.md" \
  --label "compliance,priority-critical,points-21" \
  --milestone "Q4 2026 - Compliance"

# Feature 14: FedRAMP Compliance
gh issue create --repo "$REPO" \
  --title "Feature: FedRAMP Cryptographic Controls Reporting" \
  --body "## Feature: FedRAMP Compliance Reporting

**As a federal contractor**
**I want automated FedRAMP compliance checks**
**So that I can achieve FedRAMP authorization faster**

### Story Points: 13

### Acceptance Criteria

\`\`\`gherkin
Scenario: Generate FedRAMP compliance report
  Given I have scanned my application codebase
  When I run \"pqc-scan --compliance fedramp\"
  Then I should receive a FedRAMP assessment report
  And it should map findings to NIST 800-53 controls:
    \"SC-13: Cryptographic Protection
      Status: ⚠️ Partially Compliant
      Findings: 5 non-compliant instances
      - RSA-1024 in auth.js (non-FIPS)
      - MD5 in utils.js (deprecated)
    SC-12: Cryptographic Key Management
      Status: ✅ Compliant
      No findings\"

Scenario: FIPS 140-2 validation check
  Given my code uses OpenSSL
  When I scan with FedRAMP profile
  Then the scanner should verify FIPS mode
  And provide remediation guidance

Scenario: Export FedRAMP SSP documentation
  Given I have completed a scan
  When I run \"pqc-scan --export fedramp-ssp\"
  Then I should receive System Security Plan documentation
  And it should include crypto inventory and compliance matrix
\`\`\`

### FedRAMP Requirements
- NIST 800-53 SC-13 (Cryptographic Protection)
- NIST 800-53 SC-12 (Key Management)
- FIPS 140-2 validated crypto modules
- Continuous monitoring

### Technical Implementation
- FedRAMP control mapping
- FIPS mode detection
- SSP template generation
- Automated evidence collection

### Testing Requirements
- Compliance mapping tests
- FIPS validation tests
- Report format validation
- Integration with OSCAL

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 15-16)
- FedRAMP: https://www.fedramp.gov/" \
  --label "compliance,security,priority-critical,points-13" \
  --milestone "Q4 2026 - Compliance"

# Feature 15: ISO 27001 Compliance
gh issue create --repo "$REPO" \
  --title "Feature: ISO 27001:2022 Cryptographic Controls" \
  --body "## Feature: ISO 27001:2022 Compliance

**As an information security manager**
**I want automated ISO 27001 cryptographic control checks**
**So that I can maintain certification**

### Story Points: 13

### Acceptance Criteria

\`\`\`gherkin
Scenario: Map findings to ISO 27001 controls
  Given I have scanned my application
  When I generate an ISO 27001 report
  Then findings should map to controls:
    \"A.10.1.1: Cryptographic Controls
      Status: ⚠️ Non-compliant
      Gap: 3 instances of weak cryptography
      Findings:
      - RSA-1024: Does not meet current standards
      - MD5 hashing: Cryptographically broken
      Remediation:
      - Migrate RSA-1024 → RSA-2048 (transitional)
      - Replace MD5 → SHA-256 minimum
    A.10.1.2: Key Management
      Status: ✅ Compliant\"

Scenario: Generate Statement of Applicability
  Given I have crypto scan results
  When I export ISO 27001 documentation
  Then I should receive a Statement of Applicability
  And it should list all cryptographic controls
  And it should show implementation status
\`\`\`

### ISO 27001:2022 Controls
- A.8.24: Use of cryptography
- A.10.1.1: Cryptographic controls
- A.10.1.2: Key management
- A.18.1.5: Cryptographic controls in contracts

### Technical Implementation
- ISO 27001 control mapping
- Statement of Applicability generation
- Risk assessment integration
- Audit evidence collection

### Testing Requirements
- Control mapping accuracy
- SOA format validation
- Multi-framework tests
- Auditor review

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 16-17)
- ISO/IEC 27001:2022: https://www.iso.org/standard/27001" \
  --label "compliance,priority-high,points-13" \
  --milestone "Q4 2026 - Compliance"

# Feature 16: PCI DSS Payment Security
gh issue create --repo "$REPO" \
  --title "Feature: PCI DSS Payment Security Compliance" \
  --body "## Feature: PCI DSS Cryptographic Requirements

**As a payment processor**
**I want to ensure PCI DSS cryptographic compliance**
**So that I can handle payment card data securely**

### Story Points: 13

### Acceptance Criteria

\`\`\`gherkin
Scenario: Detect weak encryption for payment data
  Given I have code handling credit card numbers
  When I use DES encryption
  Then I should see a critical violation:
    \"🚨 PCI DSS 4.0 Violation
    Requirement 3.5.1: Strong cryptography required for cardholder data
    Finding: DES encryption (weak) used for payment data
    Risk: Cardholder data at risk
    Required Action: Replace with AES-256-GCM immediately\"

Scenario: Verify TLS configuration for payment APIs
  Given I have payment API endpoints
  When I scan TLS configurations
  Then the scanner should verify TLS 1.2+ usage
  And strong cipher suites
  And no weak ciphers (RC4, DES)

Scenario: Generate PCI DSS Attestation of Compliance
  Given I have remediated all findings
  When I run \"pqc-scan --compliance pci-dss --export aoc\"
  Then I should receive compliance attestation
  And it should be suitable for QSA review
\`\`\`

### PCI DSS 4.0 Requirements
- Requirement 3.5.1: Strong cryptography
- Requirement 4.2: TLS configuration
- Requirement 6.5.3: Cryptographic failures
- Requirement 12.3: Crypto key management

### Technical Implementation
- PCI DSS control mapping
- Payment data detection
- TLS/SSL configuration analysis
- AOC template generation

### Testing Requirements
- Payment-specific pattern tests
- TLS configuration tests
- QSA review simulation
- E-commerce integration tests

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 17-18)
- PCI DSS 4.0: https://www.pcisecuritystandards.org/" \
  --label "compliance,security,priority-critical,points-13" \
  --milestone "Q4 2026 - Compliance"

# Feature 17: HIPAA Healthcare Compliance
gh issue create --repo "$REPO" \
  --title "Feature: HIPAA Security Rule Compliance" \
  --body "## Feature: HIPAA Security Rule Compliance

**As a healthcare software developer**
**I want to ensure HIPAA cryptographic compliance**
**So that protected health information (PHI) is secure**

### Story Points: 13

### Acceptance Criteria

\`\`\`gherkin
Scenario: Detect weak PHI encryption
  Given I have code handling patient records
  When I use DES encryption for ePHI
  Then I should see a compliance violation:
    \"⚠️ HIPAA Security Rule Violation
    § 164.312(a)(2)(iv): Encryption and Decryption
    Finding: DES encryption insufficient for ePHI
    Requirement: Use NIST-approved algorithms (AES-256)
    Risk Level: High - PHI at risk of breach\"

Scenario: Generate HIPAA risk analysis report
  Given I have scanned a healthcare application
  When I export HIPAA documentation
  Then I should receive:
    - Risk analysis summary
    - Encryption inventory
    - Remediation timeline
    - Breach risk assessment
\`\`\`

### HIPAA Requirements
- § 164.312(a)(2)(iv): Encryption and decryption
- § 164.312(e)(2)(ii): Transmission security
- § 164.308(a)(4): Technical safeguards
- § 164.316(b)(2): Documentation

### Technical Implementation
- HIPAA Security Rule mapping
- ePHI detection heuristics
- Risk analysis automation
- Breach notification guidance

### Testing Requirements
- Healthcare-specific pattern tests
- ePHI detection accuracy
- Risk scoring validation
- OCR audit simulation

### Reference
- Implementation Plan: docs/implementation/IMPLEMENTATION_PLAN.md (Sprint 19)
- HIPAA Security Rule: https://www.hhs.gov/hipaa/for-professionals/security/" \
  --label "compliance,security,priority-high,points-13" \
  --milestone "Q4 2026 - Compliance"

echo "✅ Q4 2026 Compliance Ecosystem issues created"
echo ""

echo "======================================================================"
echo "✅ GitHub Issues Created Successfully!"
echo "======================================================================"
echo ""
echo "Summary:"
echo "- Epic Issues: 4 (CLI, IDE, Detection, Compliance)"
echo "- Feature Issues: 17+"
echo "- Total Story Points: 200+"
echo ""
echo "Next Steps:"
echo "1. Review issues at: https://github.com/arcqubit/pqc-scanner/issues"
echo "2. Add issues to GitHub Projects"
echo "3. Begin sprint planning for Q1 2026"
echo ""
echo "🚀 Ready to start development!"
