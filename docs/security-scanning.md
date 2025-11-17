# Security Scanning with TruffleHog and Trivy

## Overview

This project uses comprehensive automated security scanning to detect vulnerabilities, secrets, misconfigurations, and compliance issues.

**Scanning Tools**:
- **TruffleHog**: Secret and credential detection (750+ secret types)
- **Trivy**: Vulnerability, secret, misconfiguration, and license scanning

---

## TruffleHog Secret Scanning

### What TruffleHog Scans

TruffleHog detects **750+ types of secrets** including:

- **Cloud Providers**: AWS, Azure, GCP credentials
- **Version Control**: GitHub, GitLab, Bitbucket tokens
- **Databases**: PostgreSQL, MySQL, MongoDB credentials
- **APIs**: Stripe, Twilio, SendGrid, Slack keys
- **Generic**: API keys, passwords, private keys, JWT tokens

### Scan Types

#### 1. **Filesystem Scan** (Current Files)
- Scans all files in the repository
- Detects secrets in current codebase
- Runs on: Every push, PR, daily at 3:00 UTC

#### 2. **Git History Scan** (All Commits)
- Scans entire git history
- Detects secrets in past commits
- **Critical**: Finds secrets even if removed
- Runs on: Every push, PR, daily at 3:00 UTC

#### 3. **SARIF Upload** (GitHub Security Tab)
- Uploads findings to GitHub Security
- Integrates with Security Overview
- Provides remediation guidance

### Configuration

**File**: `.trufflehog.yml`

```yaml
# Exclude paths from scanning
exclude_paths:
  - target/
  - node_modules/
  - docs/
  - samples/  # Test fixtures may have dummy secrets

# Only scan verified secrets (reduces false positives)
only_verified: true

# Concurrent scanning for speed
scan_options:
  concurrency: 4
```

### Usage

#### Manual Scan

```bash
# Install TruffleHog
curl -sSfL https://raw.githubusercontent.com/trufflesecurity/trufflehog/main/scripts/install.sh | sh -s -- -b /usr/local/bin

# Scan filesystem
trufflehog filesystem . --json --only-verified

# Scan git history
trufflehog git file://. --json --only-verified --since-commit=HEAD~100

# Scan specific directory
trufflehog filesystem ./src --json --only-verified
```

#### CI Integration

**Workflow**: `.github/workflows/trufflehog-scan.yml`

Runs automatically on:
- Every push to main/develop
- Every pull request
- Daily at 3:00 UTC
- Manual workflow dispatch

### Handling Findings

#### If Secrets Are Detected

1. **Immediate Action**:
   ```bash
   # Revoke the exposed credential immediately
   # (e.g., rotate API key, invalidate token)
   ```

2. **Remove from Git History**:
   ```bash
   # Use git-filter-repo to clean history
   pip install git-filter-repo
   git filter-repo --path <file-with-secret> --invert-paths
   ```

3. **Force Push** (⚠️ Coordinate with team):
   ```bash
   git push --force-with-lease
   ```

4. **Verify Removal**:
   ```bash
   trufflehog git file://. --json --only-verified
   ```

#### False Positives

Add to `.trufflehog.yml`:
```yaml
exclude_paths:
  - path/to/false/positive.txt
```

---

## Trivy Security Scanning

### What Trivy Scans

Trivy provides **comprehensive security scanning**:

1. **Vulnerabilities** (CVE database)
   - Operating system packages
   - Language-specific dependencies (Rust, NPM, Python)
   - Container images

2. **Secrets** (120+ secret types)
   - API keys, passwords, tokens
   - Credentials in configuration files

3. **Misconfigurations** (IaC/Config)
   - Dockerfiles
   - Kubernetes manifests
   - Terraform/CloudFormation
   - GitHub Actions workflows

4. **License Compliance**
   - OSS license detection
   - Forbidden license blocking (GPL, AGPL)
   - Restricted license warnings (MPL, EPL)

### Scan Types

#### 1. **Filesystem Scan**
```bash
trivy fs . --severity CRITICAL,HIGH,MEDIUM
```
Scans all files for vulnerabilities, secrets, and misconfigurations.

#### 2. **Configuration Scan**
```bash
trivy config . --severity CRITICAL,HIGH
```
Scans IaC and configuration files for misconfigurations.

#### 3. **Repository Scan**
```bash
trivy repo . --severity CRITICAL,HIGH,MEDIUM,LOW
```
Comprehensive scan including licenses.

#### 4. **Container Image Scan**
```bash
trivy image pqc-scanner:latest --severity CRITICAL,HIGH
```
Scans Docker images for OS and application vulnerabilities.

#### 5. **SBOM Generation**
```bash
trivy fs . --format spdx-json --output sbom-spdx.json
trivy fs . --format cyclonedx --output sbom-cyclonedx.json
```
Generates Software Bill of Materials in industry-standard formats.

### Configuration

**File**: `trivy.yaml`

```yaml
scan:
  security-checks:
    - vuln       # Vulnerabilities
    - secret     # Secrets
    - config     # Misconfigurations
    - license    # License compliance

vulnerability:
  severity:
    - CRITICAL
    - HIGH
    - MEDIUM
    - LOW

license:
  forbidden:
    - GPL-2.0
    - GPL-3.0
    - AGPL-1.0
    - AGPL-3.0

ignore-unfixed: false  # Report unfixed vulnerabilities
```

**Ignore File**: `.trivyignore`

```
# Ignore specific CVEs with justification
CVE-2024-12345  # Dev dependency, not in production
```

### Usage

#### Manual Scan

```bash
# Install Trivy
# macOS
brew install trivy

# Linux
wget -qO - https://aquasecurity.github.io/trivy-repo/deb/public.key | sudo apt-key add -
echo "deb https://aquasecurity.github.io/trivy-repo/deb $(lsb_release -sc) main" | sudo tee -a /etc/apt/sources.list.d/trivy.list
sudo apt-get update
sudo apt-get install trivy

# Scan filesystem
trivy fs . --severity CRITICAL,HIGH

# Scan with config file
trivy fs . --config trivy.yaml

# Generate SARIF for GitHub
trivy fs . --format sarif --output trivy-results.sarif
```

#### CI Integration

**Workflows**:
- **`.github/workflows/trivy-scan.yml`**: Comprehensive daily scans
- **`.github/workflows/ci.yml`**: Quick scan on every push

**CI Quick Scan**: Runs on every push/PR
- Scans for CRITICAL and HIGH vulnerabilities only
- Fast feedback (< 2 minutes)

**Full Scan**: Runs daily at 4:00 UTC
- All severity levels
- SBOM generation
- Container image scanning
- Uploads to GitHub Security tab

### Handling Findings

#### Vulnerabilities

1. **Review Finding**:
   - Check severity (CRITICAL, HIGH, MEDIUM, LOW)
   - Review CVE description
   - Assess exploitability

2. **Remediation**:
   ```bash
   # Update dependencies
   cargo update <package>
   npm update <package>
   pip install --upgrade <package>
   ```

3. **If No Fix Available**:
   - Document in `.trivyignore` with justification
   - Implement compensating controls
   - Monitor for updates

#### Secrets

Same process as TruffleHog findings (see above).

#### Misconfigurations

1. **Review Recommendation**:
   - Trivy provides specific fix guidance
   - Links to security best practices

2. **Apply Fix**:
   ```yaml
   # Example: Fix Dockerfile
   # Before:
   FROM ubuntu:latest

   # After:
   FROM ubuntu:24.04  # Use specific version
   ```

3. **Verify**:
   ```bash
   trivy config Dockerfile
   ```

---

## Workflow Integration

### CI/CD Pipeline

```
┌─────────────────────────────────────────────────────────┐
│                   Every Push/PR                         │
├─────────────────────────────────────────────────────────┤
│ 1. Trivy Quick Scan (CRITICAL/HIGH only)                │
│ 2. Unit Tests                                           │
│ 3. Linting                                              │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                 Daily Scheduled Scans                    │
├─────────────────────────────────────────────────────────┤
│ 3:00 UTC - TruffleHog (Secrets)                         │
│ 4:00 UTC - Trivy (Full Scan + SBOM)                     │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                    Release Workflow                      │
├─────────────────────────────────────────────────────────┤
│ 1. Full Trivy scan                                      │
│ 2. TruffleHog git history scan                          │
│ 3. SBOM generation                                      │
│ 4. Container image scan (if applicable)                 │
│ 5. Sign artifacts with Sigstore                         │
└─────────────────────────────────────────────────────────┘
```

### GitHub Security Tab

All findings are uploaded to **GitHub Security > Code Scanning**:

- **TruffleHog**: `trufflehog-secrets` category
- **Trivy Filesystem**: `trivy-filesystem` category
- **Trivy Config**: `trivy-config` category
- **Trivy Docker**: `trivy-docker-image` category

Access at: `https://github.com/arcqubit/pqc-scanner/security/code-scanning`

---

## SBOM (Software Bill of Materials)

### What is SBOM?

A complete inventory of all components in the software, including:
- Direct dependencies
- Transitive dependencies
- Versions
- Licenses
- Vulnerabilities

### SBOM Formats

**SPDX (Software Package Data Exchange)**:
- Industry standard
- ISO/IEC 5962:2021
- Human and machine-readable

**CycloneDX**:
- OWASP standard
- Focus on security use cases
- Lightweight JSON/XML

### SBOM Generation

```bash
# SPDX format
trivy fs . --format spdx-json --output sbom-spdx.json

# CycloneDX format
trivy fs . --format cyclonedx --output sbom-cyclonedx.json
```

**Automated**: Daily scans generate SBOMs and upload as artifacts (90-day retention).

### SBOM Usage

```bash
# View SBOM
jq . sbom-spdx.json

# Extract licenses
jq '.packages[].licenseConcluded' sbom-spdx.json | sort -u

# Find vulnerable packages
trivy sbom sbom-cyclonedx.json --severity CRITICAL,HIGH
```

---

## Best Practices

### Secret Management

1. **Never commit secrets** to version control
2. **Use environment variables** for sensitive data
3. **Use secret management tools**: AWS Secrets Manager, HashiCorp Vault
4. **Rotate credentials** regularly
5. **Scan before committing**:
   ```bash
   # Pre-commit hook
   trufflehog filesystem . --json --only-verified --fail
   ```

### Vulnerability Management

1. **Monitor findings** in GitHub Security tab
2. **Triage weekly**: Review new vulnerabilities
3. **Patch promptly**: Update dependencies within 30 days
4. **Document exceptions**: Use `.trivyignore` with justification
5. **Test updates**: Ensure patches don't break functionality

### Configuration Hardening

1. **Follow security best practices** for Dockerfiles, K8s, etc.
2. **Use specific versions** (not `latest`)
3. **Minimize attack surface**: Remove unnecessary tools
4. **Principle of least privilege**: Restrict permissions
5. **Regular audits**: Review configurations quarterly

---

## Troubleshooting

### TruffleHog Issues

**Problem**: False positive detections

**Solution**:
```yaml
# Add to .trufflehog.yml
exclude_paths:
  - path/to/file.txt
```

**Problem**: Scan too slow

**Solution**:
```yaml
# Increase concurrency
scan_options:
  concurrency: 8
```

### Trivy Issues

**Problem**: Too many LOW severity findings

**Solution**:
```bash
# Scan only CRITICAL and HIGH
trivy fs . --severity CRITICAL,HIGH
```

**Problem**: False positive CVE

**Solution**:
```
# Add to .trivyignore
CVE-2024-12345  # Not applicable: we don't use affected feature
```

**Problem**: Database update failures

**Solution**:
```bash
# Clear cache and retry
trivy image --clear-cache
trivy fs . --download-db-only
```

---

## References

### TruffleHog
- [Official Documentation](https://github.com/trufflesecurity/trufflehog)
- [Detector List](https://github.com/trufflesecurity/trufflehog/tree/main/pkg/detectors)
- [GitHub Action](https://github.com/trufflesecurity/trufflehog-actions-scan)

### Trivy
- [Official Documentation](https://aquasecurity.github.io/trivy/)
- [GitHub Action](https://github.com/aquasecurity/trivy-action)
- [Vulnerability Database](https://github.com/aquasecurity/trivy-db)
- [SBOM Guide](https://aquasecurity.github.io/trivy/latest/docs/supply-chain/sbom/)

### Standards
- [SPDX Specification](https://spdx.dev/specifications/)
- [CycloneDX Specification](https://cyclonedx.org/specification/overview/)
- [NIST SBOM Guidelines](https://www.nist.gov/itl/executive-order-14028-improving-nations-cybersecurity/software-security-supply-chains-software-1)

---

**Last Updated**: 2025-11-17
**Version**: 1.0
