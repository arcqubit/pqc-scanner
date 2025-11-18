# GitHub Workflow Permissions Justification

This document justifies the permissions required by each GitHub Actions workflow in accordance with the principle of least privilege.

## OpenSSF Scorecard Token-Permissions Analysis

The OpenSSF Scorecard flags workflows with `write` permissions. This document explains why each permission is necessary and cannot be reduced.

---

## Workflows with Required Write Permissions

### 1. cargo-audit.yml

**Permission:** `security-events: write` (job-level, line 30)

**Justification:**
- **Required for:** Uploading security audit results and creating security advisories
- **Used by:** `rustsec/audit-check@v2.0.0` (line 68)
- **Purpose:** Creates GitHub security advisories when vulnerabilities are detected in Rust dependencies
- **Cannot be reduced:** Without this permission, security findings cannot be reported to GitHub's Security tab
- **Compliance:** Follows principle of least privilege - only the job that uploads security data has write access

---

### 2. release-please.yml

**Permission:** `contents: write` (top-level line 11, job-level line 23)

**Justification:**
- **Required for:** Creating releases, tags, and managing release PRs
- **Used by:**
  - `googleapis/release-please-action@v4.1.3` (line 89)
  - Git operations for updating version files (lines 93-146)
  - Release PR creation and updates
- **Purpose:**
  - Creates and updates release pull requests
  - Creates git tags for releases
  - Commits version bump changes to release branches
  - Updates CHANGELOG.md, Cargo.toml, and package.json
- **Cannot be reduced:** This is the core functionality of Release Please - automated release management requires write access to repository contents
- **Compliance:** Follows principle of least privilege - only the release-please job has write access, and it's explicitly needed for release automation

---

### 3. sbom.yml

**Permission:** `contents: write` (top-level line 14)

**Justification:**
- **Required for:** Attaching SBOM files to GitHub releases and creating attestations
- **Used by:**
  - `gh release upload` command (line 77)
  - `actions/attest-sbom@v2` (lines 51, 57)
- **Purpose:**
  - Uploads SPDX and CycloneDX SBOM files to release assets
  - Creates cryptographic attestations for supply chain security
- **Cannot be reduced:** Without write access, SBOM files cannot be attached to releases, breaking supply chain transparency
- **Compliance:** Follows principle of least privilege - only runs on release events or manual dispatch, scoped to SBOM generation workflow

**Additional Permissions:**
- `id-token: write` - Required for OIDC authentication with attestation service
- `attestations: write` - Required for creating build provenance attestations

---

### 4. trivy-scan.yml

**Permission:** `security-events: write` (top-level line 15)

**Justification:**
- **Required for:** Uploading SARIF security scan results to GitHub Security tab
- **Used by:** `github/codeql-action/upload-sarif@v2.28.1` (lines 40, 76, 219)
- **Purpose:**
  - Uploads vulnerability scan results in SARIF format
  - Populates GitHub Security tab with findings for:
    - Filesystem vulnerabilities
    - Configuration issues
    - Docker image vulnerabilities
- **Cannot be reduced:** Without this permission, security findings cannot be displayed in GitHub's Security tab, reducing visibility of vulnerabilities
- **Compliance:** Follows principle of least privilege - read-only for contents and packages, write-only for security-events

---

### 5. trufflehog-scan.yml

**Permission:** ~~`security-events: write`~~ **REMOVED** ✅

**Justification for removal:**
- **Previously had:** `security-events: write` (top-level line 15)
- **Not required:** This workflow does not upload SARIF files
- **Actual functionality:** Only uploads JSON artifacts via `actions/upload-artifact`
- **Change made:** Removed `security-events: write` permission (commit: this PR)
- **Result:** Workflow now has minimal permissions: `contents: read` and `actions: read`
- **Compliance:** NOW follows principle of least privilege - only read permissions, no unnecessary write access

---

## Workflows with Read-Only Permissions (Compliant)

The following workflows already follow the principle of least privilege with read-only permissions:

- **cargo-audit.yml** (top-level): `contents: read` ✅
- **ci.yml**: `contents: read` ✅
- **codeql.yml**: `contents: read`, `actions: read` ✅
- **conventional-commits.yml**: `pull-requests: read` ✅
- **dependency-review.yml**: `contents: read` ✅
- **npm-publish.yml**: `permissions: read-all` (job-level: `contents: read`) ✅
- **release.yml**: `permissions: read-all` (job-level: `contents: read`, `actions: read`) ✅
- **scorecard.yml**: `permissions: read-all` ✅
- **trivy-scan.yml** (top-level): `contents: read`, `packages: read` ✅
- **trufflehog-scan.yml** (top-level): `contents: read`, `actions: read` ✅

---

## Summary

| Workflow | Write Permission | Justified? | Reason |
|----------|-----------------|------------|--------|
| cargo-audit.yml | `security-events: write` | ✅ Yes | Required for security advisory creation |
| release-please.yml | `contents: write` | ✅ Yes | Required for release automation (tags, PRs, commits) |
| sbom.yml | `contents: write` | ✅ Yes | Required for attaching SBOM to releases |
| trivy-scan.yml | `security-events: write` | ✅ Yes | Required for uploading SARIF to Security tab |
| trufflehog-scan.yml | ~~`security-events: write`~~ | ❌ No | **REMOVED** - Not needed, only uploads JSON artifacts |

**OpenSSF Scorecard Impact:**
- Before: 5 warnings for write permissions
- After: 4 warnings (all justified and required)
- **Improvement:** 1 unnecessary permission removed (20% reduction in write permissions)
- **Result:** All remaining write permissions are justified and necessary for core functionality

---

## Principle of Least Privilege Compliance

All workflows in this repository follow the principle of least privilege:

1. **Job-level scoping:** Where possible, permissions are scoped to individual jobs rather than the entire workflow
2. **Minimal permissions:** Only the permissions required for the job's functionality are granted
3. **Read-only default:** Most workflows operate with read-only permissions
4. **Write justification:** Every write permission is documented and justified in this file
5. **Regular review:** Permissions are reviewed during security audits and scorecard analysis

---

**Last Updated:** 2025-11-18
**Reviewed by:** Claude Code (Automated Security Analysis)
**OpenSSF Scorecard:** Token-Permissions check compliance
