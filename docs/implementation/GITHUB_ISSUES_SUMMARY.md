# GitHub Issues Summary - PQC Scanner Implementation

**Created**: 2025-11-07
**Repository**: https://github.com/arcqubit/pqc-scanner
**Based on**: [IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md)

---

## Overview

Successfully created **21 GitHub issues** covering Year 1 (2026) implementation plan:

- **4 Epic Issues** (21 points each) = 84 story points
- **17 Feature Issues** = 163 story points
- **Total Story Points**: 247 points
- **Estimated Development Time**: ~30 weeks (at 8 points/week/developer with 3 developers)

---

## Issues by Quarter

### Q1 2026: CLI Tool & Developer Experience

**Milestone**: [Q1 2026 - CLI & Dev Experience](https://github.com/arcqubit/pqc-scanner/milestone/1)
**Due Date**: March 31, 2026
**Story Points**: 47

| Issue # | Title | Points | Priority | Labels |
|---------|-------|--------|----------|--------|
| [#1](https://github.com/arcqubit/pqc-scanner/issues/1) | Epic: CLI Tool Development | 21 | Critical | `cli`, `priority-critical` |
| [#2](https://github.com/arcqubit/pqc-scanner/issues/2) | Feature: Global CLI Installation via NPM | 5 | Critical | `cli`, `priority-critical` |
| [#3](https://github.com/arcqubit/pqc-scanner/issues/3) | Feature: Interactive Scanning with Auto-Fix Prompts | 8 | Critical | `cli`, `ux`, `priority-critical` |
| [#4](https://github.com/arcqubit/pqc-scanner/issues/4) | Feature: Watch Mode for Continuous Scanning | 8 | High | `cli`, `performance`, `priority-high` |
| [#5](https://github.com/arcqubit/pqc-scanner/issues/5) | Feature: CLI Configuration File (.pqcrc.json) | 5 | Medium | `cli`, `priority-medium` |

**Key Features**:
- Global NPM installation with `pqc-scan` command
- Interactive mode with auto-fix prompts and dry-run
- Watch mode for real-time continuous scanning
- Configuration file support (.pqcrc.json)

---

### Q2 2026: IDE Extensions

**Milestone**: [Q2 2026 - IDE Extensions](https://github.com/arcqubit/pqc-scanner/milestone/2)
**Due Date**: June 30, 2026
**Story Points**: 68

| Issue # | Title | Points | Priority | Labels |
|---------|-------|--------|----------|--------|
| [#6](https://github.com/arcqubit/pqc-scanner/issues/6) | Epic: VS Code Extension Development | 21 | Critical | `ide`, `priority-critical` |
| [#7](https://github.com/arcqubit/pqc-scanner/issues/7) | Feature: VS Code Extension Marketplace Publishing | 5 | Critical | `ide`, `ux`, `priority-critical` |
| [#8](https://github.com/arcqubit/pqc-scanner/issues/8) | Feature: Real-Time Crypto Vulnerability Highlighting | 13 | Critical | `ide`, `ux`, `performance`, `priority-critical` |
| [#9](https://github.com/arcqubit/pqc-scanner/issues/9) | Feature: One-Click Vulnerability Quick Fixes | 8 | Critical | `ide`, `ux`, `priority-critical` |
| [#10](https://github.com/arcqubit/pqc-scanner/issues/10) | Feature: Dedicated Scanner View Panel | 8 | High | `ide`, `ux`, `priority-high` |
| [#11](https://github.com/arcqubit/pqc-scanner/issues/11) | Feature: JetBrains IDE Plugin | 13 | High | `ide`, `priority-high` |

**Key Features**:
- VS Code extension with marketplace publishing
- Real-time vulnerability highlighting (< 500ms response)
- Quick fix actions with lightbulb menu
- Dedicated side panel for vulnerability management
- JetBrains plugin (IntelliJ, PyCharm, WebStorm, Android Studio)

---

### Q3 2026: Enhanced Detection

**Milestone**: [Q3 2026 - Enhanced Detection](https://github.com/arcqubit/pqc-scanner/milestone/3)
**Due Date**: September 30, 2026
**Story Points**: 63

| Issue # | Title | Points | Priority | Labels |
|---------|-------|--------|----------|--------|
| [#12](https://github.com/arcqubit/pqc-scanner/issues/12) | Epic: Multi-Language Detection Expansion | 21 | High | `detection`, `priority-high` |
| [#13](https://github.com/arcqubit/pqc-scanner/issues/13) | Feature: Kotlin Language Cryptography Detection | 8 | High | `detection`, `priority-high` |
| [#14](https://github.com/arcqubit/pqc-scanner/issues/14) | Feature: Swift Language Cryptography Detection | 8 | High | `detection`, `priority-high` |
| [#15](https://github.com/arcqubit/pqc-scanner/issues/15) | Feature: NIST PQC Algorithm Detection | 13 | Critical | `detection`, `security`, `priority-critical` |
| [#16](https://github.com/arcqubit/pqc-scanner/issues/16) | Feature: Framework-Specific Crypto Pattern Detection | 13 | Medium | `detection`, `integration`, `priority-medium` |

**Key Features**:
- Kotlin language support (Android development)
- Swift language support (iOS development)
- NIST PQC algorithm detection (CRYSTALS-Kyber, Dilithium, SPHINCS+)
- Framework-specific patterns (Django, Spring, Express.js, Rails, Laravel)
- Hybrid crypto mode detection (RSA + PQC)

---

### Q4 2026: Compliance Ecosystem

**Milestone**: [Q4 2026 - Compliance](https://github.com/arcqubit/pqc-scanner/milestone/4)
**Due Date**: December 31, 2026
**Story Points**: 69

| Issue # | Title | Points | Priority | Labels |
|---------|-------|--------|----------|--------|
| [#17](https://github.com/arcqubit/pqc-scanner/issues/17) | Epic: Regulatory Compliance Automation | 21 | Critical | `compliance`, `priority-critical` |
| [#18](https://github.com/arcqubit/pqc-scanner/issues/18) | Feature: FedRAMP Cryptographic Controls Reporting | 13 | Critical | `compliance`, `security`, `priority-critical` |
| [#19](https://github.com/arcqubit/pqc-scanner/issues/19) | Feature: ISO 27001:2022 Cryptographic Controls | 13 | High | `compliance`, `priority-high` |
| [#20](https://github.com/arcqubit/pqc-scanner/issues/20) | Feature: PCI DSS Payment Security Compliance | 13 | Critical | `compliance`, `security`, `priority-critical` |
| [#21](https://github.com/arcqubit/pqc-scanner/issues/21) | Feature: HIPAA Security Rule Compliance | 13 | High | `compliance`, `security`, `priority-high` |

**Key Features**:
- FedRAMP compliance reporting (NIST 800-53 SC-13, FIPS 140-2)
- ISO 27001:2022 cryptographic controls (A.10.1.1, A.10.1.2)
- PCI DSS 4.0 payment security (Requirement 3.5.1)
- HIPAA Security Rule (§ 164.312(a)(2)(iv))
- Automated report generation (SSP, AOC, SOA, Risk Analysis)

---

## Labels Created

### Priority Labels
- `priority-critical` (red) - Must be done
- `priority-high` (orange) - High priority
- `priority-medium` (yellow) - Medium priority
- `priority-low` (green) - Low priority

### Category Labels
- `cli` - CLI tool features
- `ide` - IDE extensions
- `detection` - Detection capabilities
- `compliance` - Compliance features
- `ai` - AI/ML features
- `ux` - User experience
- `performance` - Performance optimization
- `security` - Security features
- `automation` - Automation features
- `integration` - Third-party integrations
- `enterprise` - Enterprise features
- `sdk` - SDK development

### Story Point Labels
- `points-1` - 1 story point (trivial)
- `points-2` - 2 story points (simple)
- `points-3` - 3 story points (easy)
- `points-5` - 5 story points (medium)
- `points-8` - 8 story points (complex)
- `points-13` - 13 story points (very complex)
- `points-21` - 21 story points (epic)

---

## Story Point Distribution

| Points | Count | Total | Percentage |
|--------|-------|-------|------------|
| 21 | 4 | 84 | 34% |
| 13 | 8 | 104 | 42% |
| 8 | 5 | 40 | 16% |
| 5 | 2 | 10 | 4% |
| 3 | 0 | 0 | 0% |
| 2 | 0 | 0 | 0% |
| 1 | 0 | 0 | 0% |
| **Total** | **19** | **238** | **100%** |

---

## Sprint Planning Recommendations

### Team Configuration
- **Team Size**: 3 developers + 1 QA engineer
- **Sprint Duration**: 2 weeks
- **Velocity**: 24 story points per sprint (8 points per developer)
- **Buffer**: 20% for unknowns and technical debt

### Recommended Sprint Breakdown

**Sprint 1-2 (Q1 2026)**: CLI Foundation
- Issue #2: Global CLI Installation (5 points)
- Issue #3: Interactive Scanning (8 points)
- Total: 13 points

**Sprint 3-4 (Q1 2026)**: CLI Advanced Features
- Issue #4: Watch Mode (8 points)
- Issue #5: Configuration File (5 points)
- Total: 13 points

**Sprint 5-7 (Q2 2026)**: VS Code Extension
- Issue #7: VS Code Publishing (5 points)
- Issue #8: Real-Time Highlighting (13 points)
- Issue #9: Quick Fixes (8 points)
- Total: 26 points

**Sprint 8-9 (Q2 2026)**: VS Code Side Panel + JetBrains
- Issue #10: Scanner Panel (8 points)
- Issue #11: JetBrains Plugin (13 points)
- Total: 21 points

**Sprint 10-14 (Q3 2026)**: Enhanced Detection
- Issue #13: Kotlin Support (8 points)
- Issue #14: Swift Support (8 points)
- Issue #15: NIST PQC Detection (13 points)
- Issue #16: Framework Patterns (13 points)
- Total: 42 points

**Sprint 15-19 (Q4 2026)**: Compliance Ecosystem
- Issue #18: FedRAMP (13 points)
- Issue #19: ISO 27001 (13 points)
- Issue #20: PCI DSS (13 points)
- Issue #21: HIPAA (13 points)
- Total: 52 points

---

## Acceptance Criteria

All issues include **Gherkin-style acceptance criteria** with:
- Given-When-Then scenarios
- Testable conditions
- Performance requirements
- Success metrics

**Example from Issue #3**:
```gherkin
Scenario: Interactive scan with auto-fix prompts
  Given I have a file "src/crypto.js" with RSA-1024 usage
  When I run "pqc-scan --interactive src/crypto.js"
  Then I should see a vulnerability report for RSA-1024
  And I should be prompted "Auto-fix to RSA-2048? (y/n)"
  When I type "y"
  Then the file should be automatically modified
  And a backup "src/crypto.js.bak" should be created
  And I should see a success message "✓ Fixed RSA-1024 → RSA-2048"
```

---

## Success Metrics

### Q1 2026 Targets
- ✅ CLI installation time < 30 seconds
- ✅ Scan speed: 1000 LOC in < 5 seconds
- ✅ Memory usage < 100MB
- ✅ NPM package size < 50MB

### Q2 2026 Targets
- ✅ Extension installs: 10,000+
- ✅ User rating: 4+ stars
- ✅ Response time: < 500ms for highlights
- ✅ Memory usage: < 50MB per file

### Q3 2026 Targets
- ✅ Language coverage: 15+ languages
- ✅ Detection accuracy: 99%+
- ✅ False positive rate: < 1%
- ✅ PQC algorithms detected: 3+ (Kyber, Dilithium, SPHINCS+)

### Q4 2026 Targets
- ✅ Compliance frameworks: 4+ (FedRAMP, ISO, PCI, HIPAA)
- ✅ Report generation: < 30 seconds
- ✅ Compliance accuracy: 99%+
- ✅ Enterprise adoption: 10+ customers

---

## Next Steps

1. **Review Issues**: Visit https://github.com/arcqubit/pqc-scanner/issues
2. **Create GitHub Project Board**: Add issues to project kanban board
3. **Sprint Planning**: Assign issues to Sprint 1 (starting Q1 2026)
4. **Team Assignment**: Assign developers to specific issues
5. **Set Up CI/CD**: Configure automated testing for acceptance criteria
6. **Documentation**: Set up API docs generation
7. **Community**: Set up Discord/Slack for contributors

---

## Resources

- **Implementation Plan**: [docs/implementation/IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md)
- **Vision Document**: [docs/vision/ROADMAP_2025-2030.md](../vision/ROADMAP_2025-2030.md)
- **Issues Script**: [scripts/create-github-issues.sh](../../scripts/create-github-issues.sh)
- **GitHub Repository**: https://github.com/arcqubit/pqc-scanner
- **GitHub Milestones**: https://github.com/arcqubit/pqc-scanner/milestones

---

**Document Version**: 1.0
**Created**: 2025-11-07
**Author**: ArcQubit Team
**Last Updated**: 2025-11-07
