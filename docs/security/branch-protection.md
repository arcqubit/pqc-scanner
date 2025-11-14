# Branch Protection Configuration

This document describes the required branch protection rules for the PQC Scanner project to meet OpenSSF Scorecard and Linux Foundation best practices.

## Main Branch Protection Rules

Navigate to: **Settings → Branches → Branch protection rules → Add rule**

### Branch Name Pattern
```
main
```

### Required Settings

#### Protect matching branches

- [x] **Require a pull request before merging**
  - [x] Require approvals: **1**
  - [x] Dismiss stale pull request approvals when new commits are pushed
  - [x] Require review from Code Owners
  - [ ] Restrict who can dismiss pull request reviews (optional)

- [x] **Require status checks to pass before merging**
  - [x] Require branches to be up to date before merging
  - Required status checks:
    - `test / Run tests`
    - `lint / Lint code`
    - `security-audit / Cargo Security Audit`
    - `dependency-review / Review Dependencies for Vulnerabilities` (for PRs)
    - `codeql / Analyze Code with CodeQL`

- [x] **Require conversation resolution before merging**

- [x] **Require signed commits** (recommended)

- [x] **Require linear history** (recommended)

- [ ] **Require deployments to succeed before merging** (if using deployments)

#### Rules applied to administrators

- [x] **Include administrators**
  - This ensures maintainers follow the same process

#### Restrictions

- [x] **Restrict who can push to matching branches**
  - Add: Maintainers team
  - This prevents direct commits; all changes must go through PRs

#### Additional Settings

- [x] **Allow force pushes**: **Disabled**
- [x] **Allow deletions**: **Disabled**

## Develop Branch Protection Rules (Optional)

For the `develop` branch (if using Git Flow):

### Branch Name Pattern
```
develop
```

### Required Settings

- [x] **Require a pull request before merging**
  - [x] Require approvals: **1**
  - [x] Dismiss stale pull request approvals when new commits are pushed

- [x] **Require status checks to pass before merging**
  - [x] Require branches to be up to date before merging
  - Required status checks:
    - `test / Run tests`
    - `lint / Lint code`
    - `security-audit / Cargo Security Audit`

- [x] **Require conversation resolution before merging**

- [ ] **Include administrators**: Optional for develop branch

- [x] **Allow force pushes**: **Disabled**
- [x] **Allow deletions**: **Disabled**

## Tag Protection Rules

Protect release tags from deletion:

Navigate to: **Settings → Tags → Tag protection rules → Add rule**

### Tag Name Pattern
```
v*
```

Settings:
- This protects all version tags (e.g., v1.0.0, v1.2.0)
- Tags cannot be deleted
- Only maintainers can create matching tags

## Verification

After configuring branch protection:

1. **Test the protection rules**:
   ```bash
   # Try to push directly to main (should fail)
   git checkout main
   git commit --allow-empty -m "Test commit"
   git push origin main
   # Expected: "remote: error: GH006: Protected branch update failed"
   ```

2. **Verify PR process**:
   - Create a test PR
   - Ensure status checks run
   - Verify approval is required
   - Check that merge is blocked until checks pass

3. **Check OpenSSF Scorecard**:
   ```bash
   # Run scorecard locally
   scorecard --repo=github.com/arcqubit/pqc-scanner --show-details | grep -A 10 "Branch-Protection"
   ```

## GitHub CLI Configuration

Alternatively, use GitHub CLI to configure branch protection:

```bash
# Install GitHub CLI
# https://cli.github.com/

# Configure main branch protection
gh api repos/arcqubit/pqc-scanner/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["test / Run tests","lint / Lint code","security-audit / Cargo Security Audit"]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{"required_approving_review_count":1,"dismiss_stale_reviews":true,"require_code_owner_reviews":true}' \
  --field restrictions=null \
  --field required_linear_history=true \
  --field allow_force_pushes=false \
  --field allow_deletions=false \
  --field required_conversation_resolution=true
```

## Impact on Workflow

With these protections in place:

1. **All changes require PR review**
   - No direct commits to main
   - At least 1 approval required
   - Code owners must approve changes to their areas

2. **All checks must pass**
   - Tests, linting, security scans
   - Prevents merging broken code
   - Catches security issues early

3. **Transparent process**
   - All discussions are visible
   - Changes are documented in PRs
   - Easier to track project history

4. **Better security**
   - Prevents accidental force pushes
   - Protects against malicious commits
   - Ensures code review for security

## Exceptions and Overrides

In rare emergency situations (e.g., critical security fix, site outage):

1. **Contact a maintainer** with admin access
2. **Explain the emergency** and get approval
3. **Maintainer can override** protection temporarily
4. **Create follow-up PR** for documentation and review
5. **Re-enable protections** immediately after

## Compliance Checks

These settings help achieve:

- ✅ OpenSSF Scorecard: **Branch-Protection** (10/10)
- ✅ OpenSSF Scorecard: **Code-Review** (10/10)
- ✅ CII Best Practices: **Code Review** (required)
- ✅ Linux Foundation: **Governance** (required)

## References

- [GitHub Branch Protection Docs](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/about-protected-branches)
- [OpenSSF Scorecard Branch Protection](https://github.com/ossf/scorecard/blob/main/docs/checks.md#branch-protection)
- [CII Best Practices Badge](https://bestpractices.coreinfrastructure.org/en/criteria)

---

**Last Updated**: 2025-11-11
**Next Review**: 2026-05-11
