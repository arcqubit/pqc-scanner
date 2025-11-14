# OpenSSF Scorecard & Linux Foundation CII Best Practices Research

**Research Date:** 2025-11-11
**Project:** PQC Scanner (Rust/WASM Post-Quantum Cryptography)
**Goal:** Linux Foundation OSS Community Contribution

---

## Executive Summary

This document compiles comprehensive research on OpenSSF Scorecard requirements, Linux Foundation CII Best Practices Badge criteria, and security best practices for Rust/WASM projects targeting Linux Foundation community standards.

**Target Metrics:**
- OpenSSF Scorecard Score: 8.0+ out of 10
- CII Best Practices Badge: Passing level (minimum)
- Test Coverage: 80%+
- Vulnerability Response: ≤14 days

---

## 1. OpenSSF Scorecard Checks (20 Total)

### Critical Checks for Rust/WASM Projects

1. **Branch-Protection** (High Risk)
2. **Code-Review** (High Risk)
3. **Pinned-Dependencies** (Medium Risk)
4. **Security-Policy** (Medium Risk)
5. **Token-Permissions** (Medium Risk)
6. **Signed-Releases** (High Risk)
7. **Dependency-Update-Tool** (High Risk)
8. **SAST** (Medium Risk)
9. **Vulnerabilities** (High Risk)

### Complete Check Reference

#### 1. Binary-Artifacts
- **Risk Level:** High
- **Purpose:** Verifies no executable binaries in repository
- **Requirement:** Build exclusively from source code
- **Remediation:** Remove any checked-in binaries
- **Allowed Exceptions:** Reviewable source files, generated code from build tools (bison, yacc, flex, lex), generated documentation

#### 2. Branch-Protection
- **Risk Level:** High (vulnerable to code injection)
- **Purpose:** Ensures branch protection rules enabled on default/release branches
- **Scoring Tiers:**
  - **Tier 1 (3pts):** Prevent force push + deletion
  - **Tier 2 (6pts):** Minimum 1 reviewer required
  - **Tier 3 (8pts):** Status checks required
  - **Tier 4 (9pts):** 2 reviewers + code owner approval
  - **Tier 5 (10pts):** Stale review dismissal + admin enforcement

**Requirements:**
- Prevent force pushes
- Prevent branch deletion
- Require 2+ code reviewers (minimum 1 for lower score)
- Require status checks before merge
- Code owner approval required
- Dismiss stale reviews
- Apply rules to administrators

**Implementation:** GitHub Settings → Branches → Branch protection rules

#### 3. CI-Tests
- **Risk Level:** Low
- **Purpose:** Confirms tests run before PR merge
- **Detected Systems:** GitHub Actions, Travis CI, CircleCI, Jenkins, BuildKite, AppVeyor
- **Remediation:** Integrate with CI/CD platform (GitHub Actions recommended)

#### 4. CII-Best-Practices
- **Risk Level:** Low
- **Purpose:** Verifies OpenSSF Best Practices Badge earned
- **Scoring:**
  - Gold badge: 10 points
  - Silver badge: 7 points
  - Passing badge: 5 points
  - In progress: 2 points
- **Website:** https://www.bestpractices.dev/

#### 5. Code-Review
- **Risk Level:** High (unintentional vulnerabilities or malicious code)
- **Purpose:** Ensures human review before merge
- **Critical:** Bot reviews DON'T count - must be human
- **Detection Methods:**
  - GitHub approval system
  - Prow labels ("lgtm" or "approved")
  - Gerrit tags ("Reviewed-on" and "Reviewed-by")
  - Different committer/merger (implicit review)
- **Scoring Penalties:**
  - Unreviewed single human change: -7 points
  - Multiple unreviewed changes: additional -3 points each
  - Unreviewed bot changes: -3 points

#### 6. Contributors
- **Risk Level:** Low
- **Purpose:** Assesses organizational diversity
- **Requirement:** 3+ contributors from different organizations, each with 5+ commits in last 30 commits
- **Detection:** Uses GitHub user profile "Company" field

#### 7. Dangerous-Workflow
- **Risk Level:** Critical (repository compromise)
- **Purpose:** Detects dangerous GitHub Actions patterns
- **Patterns to Avoid:**
  - `pull_request_target` + explicit PR checkout (dangerous with write access)
  - `workflow_run` trigger misuse
  - Script injection via untrusted context variables (`github.event.issue.title`, etc.)
- **Remediation:** Avoid dangerous trigger patterns, use proper input sanitization

#### 8. Dependency-Update-Tool
- **Risk Level:** High (vulnerable to known flaws)
- **Purpose:** Confirms automated dependency update tool usage
- **Supported Tools:**
  - Dependabot
  - Renovate bot
  - PyUp (Python)
- **Remediation:** Enable Dependabot in repository settings

#### 9. Fuzzing
- **Risk Level:** Medium
- **Purpose:** Identifies use of fuzz testing
- **Methods:**
  - OSS-Fuzz integration
  - ClusterFuzzLite deployment
  - Language-specific fuzzing (cargo-fuzz for Rust)
- **Remediation:** Integrate with OSS-Fuzz following official guidelines

#### 10. License
- **Risk Level:** Low
- **Purpose:** Verifies published software license
- **Files:** LICENSE, LICENCE, COPYING, COPYRIGHT (with .txt, .md, .html extensions)
- **Locations:** Repository root or LICENSES/ directory
- **Scoring:**
  - License file detected: 6 points
  - Top-level location: 3 points
  - FSF or OSI-approved: 1 point
- **SPDX Support:** Use SPDX identifiers in filename or LICENSES/ directory

#### 11. Maintained
- **Risk Level:** High (unpatched vulnerabilities)
- **Purpose:** Assesses active maintenance
- **Requirement:** 1+ commit per week in last 90 days for highest score
- **Alternative:** Issue activity from collaborators/members/owners

#### 12. Packaging
- **Risk Level:** Medium (users missing security updates)
- **Purpose:** Confirms project distribution as package
- **Formats:** Language-specific package managers (crates.io for Rust), OS package managers, container images
- **Remediation:** Publish via GitHub Packages or language-specific hub

#### 13. Pinned-Dependencies
- **Risk Level:** Medium (compromised dependencies)
- **Purpose:** Verifies dependencies pinned to specific hashes
- **Scope:** Dockerfiles, shell scripts, GitHub workflows

**Rust/Cargo Specific:**
- Commit `Cargo.lock` for reproducible builds
- Use semver ranges in `Cargo.toml`, not exact pins
- Pin git dependencies to commit SHAs
- Use Dependabot/Renovate for automated updates

**GitHub Actions:**
- Pin to full commit SHA (not tags)
- Format: `uses: actions/checkout@<full-sha>`
- Include version comment for reference
- Tools: Dependabot, Renovate, StepSecurity online tool

#### 14. SAST
- **Risk Level:** Medium (unknown bugs)
- **Purpose:** Detects Static Application Security Testing
- **Recognized Tools:**
  - CodeQL (github-code-scanning)
  - SonarCloud
- **Remediation:** Enable CodeQL via GitHub Actions

#### 15. SBOM
- **Risk Level:** Medium (inaccurate vulnerability reporting)
- **Purpose:** Confirms Software Bill of Materials
- **Formats:** CycloneDX, SPDX
- **Scoring:**
  - SBOM exists: 5 points
  - Published as release artifact: 5 points (preferred)
- **Tools:** GitHub/GitLab native SBOM generation, CycloneDX/SPDX tool centers

#### 16. Security-Policy
- **Risk Level:** Medium (insecure vulnerability reporting)
- **Purpose:** Verifies published vulnerability disclosure policy
- **File:** SECURITY.md in repository root
- **Scoring:**
  - Email or HTTPS contact: 6 points
  - Policy text: 3 points
  - Keywords + timeline: 1 point

**Required Content:**
- Supported versions receiving security updates
- How to report vulnerabilities (email or secure URL)
- Response timeline (≤14 days recommended)
- Disclosure process
- Vulnerability terminology

#### 17. Signed-Releases
- **Risk Level:** High (malicious releases)
- **Purpose:** Confirms cryptographic signing of releases
- **Formats:**
  - *.minisig (Minisign)
  - *.asc (PGP/GPG)
  - *.sig, *.sign
  - *.sigstore, *.sigstore.json
  - *.intoto.jsonl (SLSA provenance)
- **Scoring:**
  - Signatures in last 5 releases: 8 points
  - SLSA provenance: 10 points
- **Limitation:** Check detects but doesn't verify signatures

#### 18. Token-Permissions
- **Risk Level:** Medium
- **Purpose:** Validates GitHub Actions token permissions
- **Requirement:** GITHUB_TOKEN uses least-privilege access

**Best Practices:**
- Set default to read-only
- Use `permissions:` key for granular control
- Specify only required scopes
- Limit third-party action permissions

**Available Scopes:**
- actions, attestations, checks, contents, deployments
- id-token, issues, discussions, packages, pages
- pull-requests, security-events, statuses

#### 19. Vulnerabilities
- **Risk Level:** High
- **Purpose:** Scans for known security vulnerabilities in dependencies
- **Remediation:** Address all medium/high vulnerabilities within 60 days

#### 20. Webhooks
- **Risk Level:** Medium
- **Purpose:** Checks webhook configuration security
- **Requirement:** Secure webhook configuration

---

## 2. Linux Foundation CII Best Practices Badge

### Passing Level (67 Criteria)

**Basics:**
- Website describing software purpose
- FLOSS license posted in repository
- Basic and reference documentation
- HTTPS support
- Public version-controlled repository

**Change Control:**
- Public version-controlled repository
- Unique version identifiers
- Human-readable release notes

**Reporting:**
- Bug-reporting process with public archive
- Vulnerability report process with ≤14 day response time

**Quality:**
- Automated test suite with execution documentation
- Compiler warnings enabled and addressed
- 1+ developer understands secure design

**Security:**
- Use publicly reviewed cryptographic protocols only
- Deliver via HTTPS/SSH
- No unpatched medium/high vulnerabilities >60 days old
- No leaked credentials in repositories

**Analysis:**
- Static code analysis on major releases
- Dynamic analysis recommended for memory-unsafe languages

### Silver Level (55 Criteria)

**Prerequisites:** Must achieve Passing badge

**Governance:**
- Developer Certificate of Origin
- Documented governance model
- Code of conduct
- Succession planning

**Documentation:**
- One-year roadmap
- Architecture documentation
- Security requirements
- Quick-start guide

**Quality:**
- Identified coding standards with automated enforcement
- Build system honors environment variables
- 80% statement coverage in automated tests
- Regression tests for ≥50% of bugs (last 6 months)

**Security:**
- Secure design principles implemented
- Cryptographically signed releases with verification docs
- Input validation using allowlist approach
- Security requirements assurance case

### Gold Level (23 Criteria)

**Prerequisites:** Must achieve Silver badge

**Governance:**
- Bus factor of 2+ (survives loss of any single person)
- 2+ unassociated significant contributors

**Quality:**
- Code review standards documented
- 50% of changes reviewed before release
- Reproducible builds (bit-for-bit identical)
- 90% statement coverage + 80% branch coverage
- Continuous integration on all changes

**Security:**
- Security review within last 5 years
- Hardening mechanisms mandatory in produced software
- Insecure protocols (FTP, HTTP, SSLv3) disabled by default
- Dynamic analysis on all major releases

---

## 3. Required Repository Files

### SECURITY.md

**Location:** Repository root

**Required Sections:**
1. **Supported Versions** - Which versions receive security updates
2. **Reporting Instructions** - How to confidentially report vulnerabilities
3. **Contact Information** - Email or secure URL for reporting
4. **Response Timeline** - When reporters can expect acknowledgment (≤14 days)
5. **Disclosure Process** - How vulnerabilities are disclosed publicly
6. **Definitions** - What constitutes a security vulnerability

**Templates:**
- https://github.com/dec0dOS/amazing-github-template
- https://github.com/svt/open-source-project-template

**Example Structure:**
```markdown
# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

To report a security vulnerability, please email security@project.org
with a description of the issue, steps to reproduce, affected versions,
and any suggested fixes.

You will receive acknowledgment within 14 days and regular updates on
progress toward resolution.

## Disclosure Policy

We follow coordinated vulnerability disclosure. Vulnerabilities will be
publicly disclosed after a fix is available.
```

### CONTRIBUTING.md

**Location:** Repository root, docs/, or .github/

**Required Sections:**
1. **How to Report Bugs**
2. **How to Submit Pull Requests**
3. **Development Setup**
4. **Testing Procedures**
5. **Code Style Guidelines**
6. **Code of Conduct Reference**
7. **Communication Channels**
8. **Recognition Model**

**Best Practices:**
- Include table of contents for accessibility
- Provide step-by-step instructions
- Link to code of conduct
- Specify commit message format
- Document PR review process

### CODE_OF_CONDUCT.md

**Location:** Repository root

**Recommended Template:** Contributor Covenant (used by 40,000+ projects)

**Required Sections:**
1. **Our Pledge** - Commitment to inclusive environment
2. **Our Standards** - Expected and unacceptable behavior
3. **Enforcement Responsibilities** - Who enforces and how
4. **Scope** - Where code of conduct applies
5. **Enforcement** - Reporting process and consequences
6. **Attribution** - Source of code of conduct

**Templates:**
- Contributor Covenant: https://www.contributor-covenant.org/
- Django Code of Conduct
- Citizen Code of Conduct

**Timing:** Establish as early as possible, ideally at project creation

### LICENSE

**Location:** Repository root or LICENSES/ directory

**Formats:**
- LICENSE, LICENCE, COPYING, COPYRIGHT
- Extensions: .txt, .md, .html

**Requirements:**
- FSF or OSI-approved license
- SPDX identifier in filename (e.g., LICENSE-Apache-2.0.txt)
- Or use LICENSES/ directory with SPDX-named files

**For Rust Projects:** Common choices are MIT, Apache-2.0, or dual MIT/Apache-2.0

---

## 4. GitHub Branch Protection Settings

### Critical Settings

**Navigate to:** Settings → Branches → Branch protection rules

**Required Protections:**
1. ☑ Require a pull request before merging
   - ☑ Require approvals: **2**
   - ☑ Dismiss stale pull request approvals when new commits are pushed
   - ☑ Require review from Code Owners
2. ☑ Require status checks to pass before merging
   - ☑ Require branches to be up to date before merging
3. ☑ Require conversation resolution before merging
4. ☑ Require signed commits
5. ☑ Require linear history
6. ☑ Do not allow bypassing the above settings (apply to administrators)
7. ☑ Restrict who can push to matching branches
8. ☑ Allow force pushes: **Never**
9. ☑ Allow deletions: **No**

### Alternative: Repository Rulesets

GitHub now offers Repository Rules as a more flexible alternative to branch protection rules.

**Navigate to:** Settings → Rules → Rulesets

---

## 5. Rust/Cargo Best Practices

### Cargo.lock Management

**Recommendation:** Always commit `Cargo.lock` to version control

**Rationale:**
- Ensures reproducible builds across environments
- Prevents CI/CD from using moving target dependencies
- Allows tracking exact dependency versions
- Critical for security audits

**Previous Guidance (Outdated):** Only commit for binaries, not libraries
**Current Guidance (2023+):** Commit for all projects as starting point

**Update Strategy:**
```bash
# Update specific crate to exact version
cargo update -p crate-name --precise X.Y.Z

# Update all dependencies
cargo update

# Use Dependabot or Renovate for automated PR-based updates
```

### Dependency Specification in Cargo.toml

**Best Practice:** Use semver ranges, not exact pins

```toml
# ✅ Good - allows patch and minor updates
serde = "1.0"

# ✅ Good - allows patch updates only
serde = "1.0.100"

# ❌ Avoid - prevents all updates including security fixes
serde = "=1.0.100"
```

**Git Dependencies:** Pin to commit SHAs for stability

```toml
[dependencies]
my-crate = { git = "https://github.com/user/repo", rev = "abc123def456" }
```

### CI/CD Best Practices

**Caching:**
```yaml
- uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/bin/
      ~/.cargo/registry/index/
      ~/.cargo/registry/cache/
      ~/.cargo/git/db/
      target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

**Environment Variables:**
```bash
# Ensure resolver doesn't limit dependencies by Rust version
CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback
```

**Matrix Testing:**
```yaml
strategy:
  matrix:
    rust: [stable, beta, nightly]
    os: [ubuntu-latest, windows-latest, macos-latest]
```

---

## 6. GitHub Actions Security

### Action Pinning

**Requirement:** Pin to full commit SHA, not tags

**Rationale:**
- Tags can be moved to point to different commits
- SHAs are immutable
- Prevents supply chain attacks

**Format:**
```yaml
# ✅ Correct - pinned to SHA with version comment
- uses: actions/checkout@a81bbbf8298c0fa03ea29cdc473d45769f953675 # v4.1.1

# ❌ Wrong - tag can be moved
- uses: actions/checkout@v4

# ❌ Wrong - branch can change
- uses: actions/checkout@main
```

**Automation Tools:**
- Dependabot (automatic PR-based updates)
- Renovate (automatic PR-based updates)
- StepSecurity online tool (one-time conversion)

### Token Permissions

**Default Permission:** Read-only

**Implementation:**
```yaml
# Workflow-level (applies to all jobs)
permissions:
  contents: read

# Job-level (overrides workflow-level)
jobs:
  build:
    permissions:
      contents: read
      pull-requests: write
    steps:
      - uses: actions/checkout@<sha>
```

**Available Permission Scopes:**
- `actions` - GitHub Actions workflows
- `checks` - Check runs and suites
- `contents` - Repository contents
- `deployments` - Deployments
- `id-token` - OIDC token
- `issues` - Issues and labels
- `packages` - GitHub Packages
- `pages` - GitHub Pages
- `pull-requests` - Pull requests
- `security-events` - Security events (CodeQL)
- `statuses` - Commit statuses

**Principle:** Grant only the minimum required permissions

### Dangerous Patterns to Avoid

1. **pull_request_target Misuse**
```yaml
# ❌ Dangerous - runs untrusted PR code with write access
on: pull_request_target
steps:
  - uses: actions/checkout@<sha>
    with:
      ref: ${{ github.event.pull_request.head.sha }}
```

2. **Script Injection**
```yaml
# ❌ Dangerous - untrusted input in shell
- run: echo "Title: ${{ github.event.issue.title }}"

# ✅ Safe - use environment variable
- run: echo "Title: $TITLE"
  env:
    TITLE: ${{ github.event.issue.title }}
```

3. **Excessive Permissions**
```yaml
# ❌ Bad - unnecessary write access
permissions: write-all

# ✅ Good - minimal permissions
permissions:
  contents: read
```

---

## 7. Implementation Roadmap

### Phase 1: Foundation (Week 1)

**Community Health Files:**
- [ ] Create `SECURITY.md` with disclosure policy
- [ ] Create `CONTRIBUTING.md` with contribution guidelines
- [ ] Add `CODE_OF_CONDUCT.md` (Contributor Covenant recommended)
- [ ] Ensure `LICENSE` file present with SPDX identifier
- [ ] Commit `Cargo.lock` to repository

**Expected Scorecard Impact:** +2-3 points

### Phase 2: GitHub Settings (Week 1-2)

**Repository Configuration:**
- [ ] Enable branch protection on main branch
  - [ ] Require 2+ reviewers
  - [ ] Enable status checks requirement
  - [ ] Disable force push and deletion
  - [ ] Apply rules to administrators
- [ ] Enable Dependabot security updates
- [ ] Enable Dependabot version updates
- [ ] Configure code owners (CODEOWNERS file)

**Expected Scorecard Impact:** +3-4 points

### Phase 3: CI/CD Security (Week 2-3)

**GitHub Actions Hardening:**
- [ ] Pin all GitHub Actions to commit SHAs
- [ ] Set GITHUB_TOKEN to read-only by default
- [ ] Enable CodeQL scanning
- [ ] Add `cargo audit` to CI pipeline
- [ ] Implement automated testing in CI
- [ ] Add coverage reporting (target 80%+)
- [ ] Avoid dangerous workflow patterns

**Expected Scorecard Impact:** +2-3 points

### Phase 4: Advanced Security (Week 3-4)

**Additional Measures:**
- [ ] Generate and publish SBOM with releases
- [ ] Sign releases with GPG or Sigstore
- [ ] Integrate fuzzing (cargo-fuzz)
- [ ] Apply for OpenSSF Best Practices Badge (Passing level)
- [ ] Consider OSS-Fuzz integration
- [ ] Implement SLSA provenance (if applicable)

**Expected Scorecard Impact:** +2-3 points

**Total Expected Score:** 9-13 points improvement (target 8.0+)

---

## 8. Rust/WASM Specific Considerations

### WASM Build Security

**Pinning WASM Tools:**
```bash
# Pin wasm-pack version
cargo install wasm-pack --version 0.12.1

# In CI, use specific version
- run: cargo install wasm-pack --version 0.12.1
```

**WASM Binary Verification:**
- WASM binaries should not be committed to repository
- Build from source in CI/CD
- Consider publishing to npm/crates.io for distribution

### Cryptography Considerations

**Post-Quantum Crypto Libraries:**
- Ensure dependencies are from trusted sources
- Pin git dependencies to specific commits
- Audit cryptographic implementations
- Document algorithm choices in SECURITY.md

**Security Disclosure:**
- Critical for cryptographic software
- Response time target: ≤7 days for critical issues
- Consider coordinated disclosure with crypto community

---

## 9. Monitoring and Maintenance

### Regular Tasks

**Weekly:**
- Review and merge Dependabot PRs
- Check for new security advisories
- Monitor CI/CD pipeline status

**Monthly:**
- Review OpenSSF Scorecard results
- Update documentation as needed
- Audit dependencies with `cargo audit`

**Quarterly:**
- Review and update security policy
- Assess test coverage improvements
- Consider new security tools/practices

### Automated Monitoring

**GitHub Security Features:**
- Dependabot security alerts (automatic)
- Dependabot version updates (automatic PRs)
- Code scanning alerts (CodeQL)
- Secret scanning

**External Tools:**
- OpenSSF Scorecard (run via GitHub Action)
- Cargo audit in CI/CD
- Coverage tracking (codecov.io or coveralls.io)

---

## 10. Resources and References

### Official Documentation

- **OpenSSF Scorecard:** https://github.com/ossf/scorecard
- **Scorecard Checks:** https://github.com/ossf/scorecard/blob/main/docs/checks.md
- **OpenSSF Best Practices:** https://www.bestpractices.dev/
- **Best Practices Criteria:** https://www.bestpractices.dev/en/criteria
- **GitHub Branch Protection:** https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches
- **GitHub Actions Security:** https://docs.github.com/en/actions/security-guides

### Templates and Tools

- **Amazing GitHub Template:** https://github.com/dec0dOS/amazing-github-template
- **Contributor Covenant:** https://www.contributor-covenant.org/
- **StepSecurity Actions Pinning:** https://www.stepsecurity.io/
- **Cargo Book:** https://doc.rust-lang.org/cargo/
- **Rust Security WG:** https://www.rust-lang.org/governance/wgs/wg-security

### Rust-Specific Resources

- **Cargo Security Advisory DB:** https://github.com/rustsec/advisory-db
- **cargo-audit:** https://github.com/rustsec/rustsec/tree/main/cargo-audit
- **cargo-fuzz:** https://github.com/rust-fuzz/cargo-fuzz
- **WASM Security:** https://webassembly.org/docs/security/

---

## 11. Quick Reference Checklist

### Immediate Actions (Day 1)

- [ ] Create SECURITY.md
- [ ] Create CONTRIBUTING.md
- [ ] Add CODE_OF_CONDUCT.md
- [ ] Commit Cargo.lock
- [ ] Verify LICENSE file

### GitHub Settings (Day 2)

- [ ] Enable branch protection (2 reviewers)
- [ ] Enable Dependabot
- [ ] Disable force push/deletion
- [ ] Apply rules to admins

### CI/CD Security (Week 1)

- [ ] Pin GitHub Actions to SHAs
- [ ] Set token permissions to read-only
- [ ] Enable CodeQL
- [ ] Add cargo audit

### Advanced (Week 2-4)

- [ ] Sign releases
- [ ] Publish SBOM
- [ ] Apply for CII badge
- [ ] Integrate fuzzing

---

## Conclusion

This research provides a comprehensive roadmap for achieving high security standards aligned with OpenSSF Scorecard and Linux Foundation CII Best Practices. The phased implementation approach allows for incremental improvements while maintaining development velocity.

**Key Success Factors:**
1. Commit to security as a core value
2. Automate security checks and updates
3. Document security processes clearly
4. Respond to vulnerabilities promptly (≤14 days)
5. Maintain high test coverage (80%+)
6. Engage with security community

**Expected Outcome:**
- OpenSSF Scorecard: 8.0+ score
- CII Best Practices: Passing badge
- Enhanced trust from Linux Foundation community
- Improved security posture for post-quantum cryptography users

---

**Document Version:** 1.0
**Last Updated:** 2025-11-11
**Memory Location:** `coordination:ossf/requirements`
**Research Agent:** Claude Code Researcher
