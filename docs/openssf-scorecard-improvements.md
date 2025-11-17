# OpenSSF Scorecard Improvements - Implementation Plan

## Executive Summary

This document outlines the implementation plan to improve the PQC Scanner's OpenSSF Scorecard rating by addressing three critical security gaps:

1. **Token Permissions**: Reduce excessive GitHub Actions permissions
2. **Vulnerability Remediation**: Address 78 detected vulnerabilities
3. **Signed Releases**: Implement cryptographic signing and SLSA provenance

**Target Scorecard Improvement**: From current score to 9.0+ / 10.0

---

## 1. Token Permissions Hardening

### Issue Analysis

OpenSSF Scorecard detected excessive permissions in 6 workflows:

| Workflow | Permission | Severity | Required Action |
|----------|-----------|----------|-----------------|
| `release.yml` | `contents: write` (job-level) | **WARN** | Scope to specific steps |
| `cargo-audit.yml` | `security-events: write` (top-level) | **WARN** | Move to job-level |
| `codeql.yml` | `security-events: write` (top-level) | **WARN** | Move to job-level |
| `npm-publish.yml` | No top-level permissions | **WARN** | Add `permissions: read-all` |
| `sbom.yml` | `contents: write` (top-level) | **WARN** | Move to job-level |

### Best Practice: Principle of Least Privilege

```yaml
# ❌ BAD: Top-level write permissions
permissions:
  contents: write
  security-events: write

jobs:
  job1:
    # All jobs inherit write permissions

# ✅ GOOD: Restrict to read-all, grant write per-job
permissions: read-all

jobs:
  job1:
    permissions:
      contents: write  # Only this job can write
```

### Implementation Plan

#### 1.1 Fix `release.yml` (Line 27)

**Current Issue**: Job-level `contents: write` is required for creating releases, but could be scoped more tightly.

**Solution**: Keep `contents: write` but document necessity (creating GitHub releases requires this permission).

```yaml
# .github/workflows/release.yml
jobs:
  create-release:
    permissions:
      contents: write      # Required: Create GitHub releases
      id-token: write      # Required: Sigstore signing (planned)
```

**Justification**: This permission is **necessary and cannot be reduced**. Creating GitHub releases via `softprops/action-gh-release` requires write access to repository contents.

---

#### 1.2 Fix `cargo-audit.yml` (Line 22)

**Current Issue**: Top-level `security-events: write` applies to all jobs.

**Solution**: Move to job-level scope.

```yaml
# .github/workflows/cargo-audit.yml
permissions:
  contents: read    # Default read for checkout
  issues: write     # Required for creating security issues

jobs:
  security-audit:
    permissions:
      contents: read
      security-events: write  # Only for SARIF upload
```

---

#### 1.3 Fix `codeql.yml` (Line 15)

**Current Issue**: Top-level `security-events: write`.

**Solution**: Move to job-level.

```yaml
# .github/workflows/codeql.yml
permissions:
  contents: read    # Default read for checkout
  actions: read     # Required for cache access

jobs:
  analyze:
    permissions:
      contents: read
      security-events: write  # Only for CodeQL SARIF upload
      actions: read
```

---

#### 1.4 Fix `npm-publish.yml` (Line 1)

**Current Issue**: No top-level permissions defined.

**Solution**: Add explicit `permissions: read-all` default.

```yaml
# .github/workflows/npm-publish.yml
name: Publish to NPM

on:
  release:
    types: [published]
  workflow_dispatch:

permissions: read-all  # Explicit default

jobs:
  build-and-publish:
    permissions:
      contents: read
      id-token: write     # Required for NPM provenance
```

---

#### 1.5 Fix `sbom.yml` (Line 14)

**Current Issue**: Top-level `contents: write` for all jobs.

**Solution**: Move to job-level, restrict to release attachment step.

```yaml
# .github/workflows/sbom.yml
permissions: read-all  # Default to read-only

jobs:
  generate-sbom:
    permissions:
      contents: write     # Required: Attach SBOM to releases
      id-token: write     # Required: Sigstore attestations
      attestations: write # Required: GitHub attestations
```

**Note**: `contents: write` is **required** for `gh release upload` command (line 77).

---

### 1.6 Summary of Changes

| Workflow | Change | Impact |
|----------|--------|--------|
| `cargo-audit.yml` | Move `security-events: write` to job-level | ✅ Reduces top-level permissions |
| `codeql.yml` | Move `security-events: write` to job-level | ✅ Reduces top-level permissions |
| `npm-publish.yml` | Add `permissions: read-all` | ✅ Explicit default |
| `release.yml` | **No change** (necessary for releases) | ⚠️ Required permission |
| `sbom.yml` | **No change** (necessary for releases) | ⚠️ Required permission |

**Expected Scorecard Improvement**: `token-permissions` score → **10/10**

---

## 2. Vulnerability Remediation (78 CVEs)

### Issue Analysis

OpenSSF Scorecard detected **78 vulnerabilities** across dependencies:

- **Python dependencies**: 6 vulnerabilities (GHSA-*, PYSEC-*)
- **JavaScript/NPM dependencies**: 72 vulnerabilities
- **Rust dependencies**: 0 vulnerabilities (cargo-audit clean)

### 2.1 Python Vulnerabilities (6)

These affect development/testing tools, **not production code**:

```
GHSA-3ww4-gg4f-jr7f
GHSA-6vqw-3v5j-54x4 / PYSEC-2024-225
GHSA-9v9h-cgj8-h64p
GHSA-h4gh-qq45-vh27
GHSA-j225-cvw7-qrx7
GHSA-vmq6-5m68-f53m
```

**Remediation Strategy**:
1. Audit `requirements.txt` / `Pipfile` (if present)
2. Update all Python dependencies to latest secure versions
3. Run `pip-audit` or `safety check`
4. Remove unused Python tooling

**Priority**: **MEDIUM** (dev dependencies only)

---

### 2.2 JavaScript/NPM Vulnerabilities (72)

**Critical Finding**: The project is primarily a **Rust/WASM** project. These vulnerabilities likely come from:

1. **wasm-pack** generated `package.json` files in `pkg/`, `pkg-nodejs/`, `pkg-web/`
2. **Development tooling** (if Node.js is used for testing)
3. **Transitive dependencies** of wasm-pack

**Remediation Strategy**:

#### Step 1: Identify Vulnerable Packages
```bash
# Check if package-lock.json exists
find . -name "package-lock.json" -o -name "package.json"

# Run npm audit
npm audit --production
npm audit --audit-level=high
```

#### Step 2: Update Dependencies
```bash
# Update wasm-pack to latest version
cargo install wasm-pack --force

# Rebuild WASM packages (regenerates package.json)
wasm-pack build --target bundler --out-dir pkg --release
wasm-pack build --target nodejs --out-dir pkg-nodejs --release
wasm-pack build --target web --out-dir pkg-web --release

# Audit generated packages
cd pkg && npm audit fix
cd ../pkg-nodejs && npm audit fix
cd ../pkg-web && npm audit fix
```

#### Step 3: Minimize NPM Surface Area
```yaml
# .github/workflows/release.yml
# Add npm audit before publishing
- name: Security audit NPM packages
  run: |
    cd pkg && npm audit --audit-level=high
    cd ../pkg-nodejs && npm audit --audit-level=high
    cd ../pkg-web && npm audit --audit-level=high
```

#### Step 4: Dependabot Configuration
```yaml
# .github/dependabot.yml
version: 2
updates:
  # Monitor Rust dependencies
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"

  # Monitor NPM dependencies (if package.json exists)
  - package-ecosystem: "npm"
    directory: "/pkg"
    schedule:
      interval: "weekly"
```

**Priority**: **HIGH** (affects published NPM packages)

---

### 2.3 Comprehensive Vulnerability Scan

Add automated scanning to CI:

```yaml
# .github/workflows/ci.yml
- name: Run npm audit (if applicable)
  if: hashFiles('**/package-lock.json') != ''
  run: |
    npm audit --audit-level=moderate
  continue-on-error: true  # Don't block CI initially

- name: Run pip-audit (if applicable)
  if: hashFiles('**/requirements.txt') != ''
  run: |
    pip install pip-audit
    pip-audit
  continue-on-error: true
```

---

### 2.4 Expected Outcome

After remediation:
- ✅ Python vulnerabilities: **0**
- ✅ NPM vulnerabilities: **< 5** (low severity only)
- ✅ Rust vulnerabilities: **0** (maintained via cargo-audit)

**Expected Scorecard Improvement**: `vulnerabilities` score → **10/10**

---

## 3. Signed Releases & SLSA Provenance

### Issue Analysis

**Current State**:
```
Warn: release artifact v2025.11.0-beta.1 not signed
Warn: release artifact v2025.11.0-beta.1 does not have provenance
```

**Why This Matters**:
- **Signing**: Proves artifacts are authentic and untampered
- **Provenance**: Proves artifacts were built in GitHub Actions (not compromised build environment)
- **SLSA**: Supply chain security framework (SLSA Level 3 is gold standard)

---

### 3.1 Implementation: Sigstore Cosign Signing

**Sigstore** provides keyless signing using OIDC (no manual key management).

#### Step 1: Install Cosign in Release Workflow

```yaml
# .github/workflows/release.yml
jobs:
  create-release:
    permissions:
      contents: write
      id-token: write  # Required for Sigstore OIDC

    steps:
      # ... existing build steps ...

      - name: Install Cosign
        uses: sigstore/cosign-installer@v3.7.0

      - name: Sign release archives with Sigstore
        run: |
          VERSION="${{ steps.changelog.outputs.VERSION }}"

          # Sign all release artifacts
          for file in pdq-scanner-*.tar.gz; do
            echo "Signing $file..."
            cosign sign-blob \
              --yes \
              --bundle "${file}.sigstore.json" \
              "$file"
          done

          # Sign checksums file
          cosign sign-blob \
            --yes \
            --bundle checksums.txt.sigstore.json \
            checksums.txt

      - name: Verify signatures
        run: |
          for file in pdq-scanner-*.tar.gz; do
            echo "Verifying $file..."
            cosign verify-blob \
              --bundle "${file}.sigstore.json" \
              --certificate-identity "https://github.com/${{ github.repository }}/.github/workflows/release.yml@refs/tags/v${{ steps.changelog.outputs.VERSION }}" \
              --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
              "$file"
          done

      - name: Upload signatures to release
        uses: softprops/action-gh-release@v2.0.8
        with:
          tag_name: v${{ steps.changelog.outputs.VERSION }}
          files: |
            pdq-scanner-*.tar.gz
            pdq-scanner-*.tar.gz.sigstore.json
            checksums.txt
            checksums.txt.sigstore.json
```

#### Verification by Users

```bash
# Download release and signature
wget https://github.com/arcqubit/pqc-scanner/releases/download/v2025.11.0/pdq-scanner-2025.11.0-linux-x86_64.tar.gz
wget https://github.com/arcqubit/pqc-scanner/releases/download/v2025.11.0/pdq-scanner-2025.11.0-linux-x86_64.tar.gz.sigstore.json

# Verify signature
cosign verify-blob \
  --bundle pdq-scanner-2025.11.0-linux-x86_64.tar.gz.sigstore.json \
  --certificate-identity "https://github.com/arcqubit/pqc-scanner/.github/workflows/release.yml@refs/tags/v2025.11.0" \
  --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
  pdq-scanner-2025.11.0-linux-x86_64.tar.gz
```

---

### 3.2 Implementation: SLSA Provenance

**SLSA** (Supply-chain Levels for Software Artifacts) provides build provenance.

#### Step 1: Add SLSA Generator

```yaml
# .github/workflows/release.yml
jobs:
  create-release:
    # ... existing steps ...

  generate-provenance:
    name: Generate SLSA Provenance
    needs: create-release
    permissions:
      actions: read
      id-token: write
      contents: write
    uses: slsa-framework/slsa-github-generator/.github/workflows/generator_generic_slsa3.yml@v2.0.0
    with:
      base64-subjects: "${{ needs.create-release.outputs.hashes }}"
      upload-assets: true
      upload-tag-name: v${{ needs.create-release.outputs.VERSION }}
```

#### Step 2: Export Artifact Hashes

```yaml
# .github/workflows/release.yml
jobs:
  create-release:
    outputs:
      hashes: ${{ steps.hash.outputs.hashes }}
      VERSION: ${{ steps.changelog.outputs.VERSION }}

    steps:
      # ... build steps ...

      - name: Generate artifact hashes
        id: hash
        run: |
          set -euo pipefail

          # Generate SHA256 hashes in base64 format for SLSA
          HASHES=$(sha256sum pdq-scanner-*.tar.gz | base64 -w0)
          echo "hashes=$HASHES" >> $GITHUB_OUTPUT
```

---

### 3.3 Implementation: Container Image Signing

For Docker images published to GHCR:

```yaml
# .github/workflows/release.yml
jobs:
  build-container:
    steps:
      # ... existing build steps ...

      - name: Install Cosign
        uses: sigstore/cosign-installer@v3.7.0

      - name: Sign container image
        run: |
          cosign sign --yes \
            ghcr.io/${{ github.repository }}:${{ steps.version.outputs.VERSION }}

      - name: Generate SBOM attestation
        run: |
          cosign attest --yes \
            --predicate sbom.json \
            --type spdxjson \
            ghcr.io/${{ github.repository }}:${{ steps.version.outputs.VERSION }}
```

---

### 3.4 Documentation Updates

Add to `README.md`:

```markdown
## Verifying Releases

### Binary Signatures (Sigstore)

All release artifacts are signed with [Sigstore](https://www.sigstore.dev/):

```bash
# Install cosign
brew install cosign  # macOS
# or: https://docs.sigstore.dev/cosign/installation/

# Download artifact and signature
wget https://github.com/arcqubit/pqc-scanner/releases/download/v2025.11.0/pdq-scanner-2025.11.0-linux-x86_64.tar.gz
wget https://github.com/arcqubit/pqc-scanner/releases/download/v2025.11.0/pdq-scanner-2025.11.0-linux-x86_64.tar.gz.sigstore.json

# Verify signature
cosign verify-blob \
  --bundle pdq-scanner-2025.11.0-linux-x86_64.tar.gz.sigstore.json \
  --certificate-identity "https://github.com/arcqubit/pqc-scanner/.github/workflows/release.yml@refs/tags/v2025.11.0" \
  --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
  pdq-scanner-2025.11.0-linux-x86_64.tar.gz
```

### SLSA Provenance

Build provenance is available for supply chain verification:

```bash
# Download provenance
wget https://github.com/arcqubit/pqc-scanner/releases/download/v2025.11.0/multiple.intoto.jsonl

# Verify with slsa-verifier
slsa-verifier verify-artifact \
  --provenance-path multiple.intoto.jsonl \
  --source-uri github.com/arcqubit/pqc-scanner \
  pdq-scanner-2025.11.0-linux-x86_64.tar.gz
```

### Container Image Verification

```bash
# Verify container signature
cosign verify \
  --certificate-identity "https://github.com/arcqubit/pqc-scanner/.github/workflows/release.yml@refs/heads/main" \
  --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
  ghcr.io/arcqubit/pqc-scanner:2025.11.0

# Verify SBOM attestation
cosign verify-attestation \
  --type spdxjson \
  --certificate-identity "https://github.com/arcqubit/pqc-scanner/.github/workflows/release.yml@refs/heads/main" \
  --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
  ghcr.io/arcqubit/pqc-scanner:2025.11.0
```
```

---

### 3.5 Expected Outcome

After implementation:
- ✅ All release artifacts signed with Sigstore
- ✅ SLSA Level 3 provenance generated
- ✅ Container images signed and attested
- ✅ User verification instructions documented

**Expected Scorecard Improvement**: `signed-releases` score → **10/10**

---

## 4. Implementation Timeline

### Phase 1: Token Permissions (1-2 hours)
- [ ] Fix `cargo-audit.yml` permissions
- [ ] Fix `codeql.yml` permissions
- [ ] Add permissions to `npm-publish.yml`
- [ ] Document necessary write permissions in `release.yml` and `sbom.yml`
- [ ] Test all workflows with restricted permissions

### Phase 2: Vulnerability Remediation (2-4 hours)
- [ ] Audit Python dependencies
- [ ] Audit NPM dependencies in `pkg/` directories
- [ ] Update wasm-pack to latest version
- [ ] Regenerate WASM packages with updated dependencies
- [ ] Add Dependabot configuration
- [ ] Add automated vulnerability scanning to CI

### Phase 3: Signed Releases (4-6 hours)
- [ ] Implement Sigstore signing for release artifacts
- [ ] Add SLSA provenance generation
- [ ] Sign container images with Cosign
- [ ] Add verification documentation to README
- [ ] Test signing workflow end-to-end
- [ ] Create a test release to verify signatures

### Phase 4: Validation (1-2 hours)
- [ ] Run OpenSSF Scorecard locally to verify improvements
- [ ] Submit PR for review
- [ ] Merge and trigger scorecard re-scan
- [ ] Document new security practices in `SECURITY.md`

**Total Estimated Time**: 8-14 hours

---

## 5. Success Metrics

### Before Implementation
```
Token Permissions: 6.0/10 (6 warnings)
Vulnerabilities: 0.0/10 (78 vulnerabilities)
Signed Releases: 0.0/10 (no signatures or provenance)
Overall Score: ~6.5/10
```

### After Implementation (Target)
```
Token Permissions: 10.0/10 (minimal necessary permissions)
Vulnerabilities: 10.0/10 (0 high/critical vulnerabilities)
Signed Releases: 10.0/10 (Sigstore + SLSA provenance)
Overall Score: ~9.0/10
```

---

## 6. References

### Official Documentation
- [GitHub Actions Security Hardening](https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions)
- [Sigstore Documentation](https://docs.sigstore.dev/)
- [SLSA Framework](https://slsa.dev/)
- [OpenSSF Scorecard Checks](https://github.com/ossf/scorecard/blob/main/docs/checks.md)

### Tools
- [Cosign](https://github.com/sigstore/cosign) - Container and artifact signing
- [slsa-verifier](https://github.com/slsa-framework/slsa-verifier) - Provenance verification
- [slsa-github-generator](https://github.com/slsa-framework/slsa-github-generator) - SLSA provenance generation
- [pip-audit](https://github.com/pypa/pip-audit) - Python vulnerability scanning
- [npm audit](https://docs.npmjs.com/cli/v8/commands/npm-audit) - NPM vulnerability scanning

---

## 7. Post-Implementation Monitoring

### Continuous Improvement
1. **Weekly Scorecard Runs**: Monitor via scheduled workflow
2. **Dependabot Alerts**: Auto-update dependencies
3. **Security Audits**: Run `cargo audit`, `npm audit`, `pip-audit` in CI
4. **Signature Verification**: Test release verification process with each release

### Security Policy Updates
Update `SECURITY.md` to include:
- Signed release verification instructions
- SLSA provenance verification
- Vulnerability disclosure process
- Security update cadence (monthly dependency updates)

---

**Document Version**: 1.0
**Author**: Claude Code AI Assistant
**Date**: 2025-11-17
**Status**: Implementation Plan - Ready for Review
