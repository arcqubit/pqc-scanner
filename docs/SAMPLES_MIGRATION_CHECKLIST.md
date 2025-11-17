# PQC Scanner Samples Migration - Quick Checklist

This is a condensed checklist for executing the samples migration. See [SAMPLES_MIGRATION_PLAN.md](SAMPLES_MIGRATION_PLAN.md) for complete details.

---

## Phase 1: New Repository Setup (pqc-scanner-samples)

### 1.1 Initialize Repository
```bash
- [ ] git clone https://github.com/arcqubit/pqc-scanner-samples.git
- [ ] cd pqc-scanner-samples
- [ ] git checkout -b main
```

### 1.2 Copy Samples
```bash
- [ ] cp -r /path/to/pqc-scanner/samples/* .
- [ ] mv README.md ROOT_README.md
- [ ] mv GETTING_STARTED.md ROOT_GETTING_STARTED.md
```

### 1.3 Create Root Files
```bash
- [ ] Create README.md (from samples/README.md)
- [ ] Create CONTRIBUTING.md
- [ ] Create LICENSE (MIT)
- [ ] Create .gitignore
```

### 1.4 Clean Up
```bash
- [ ] find . -name "node_modules" -type d -prune -exec rm -rf {} +
- [ ] find . -name "__pycache__" -type d -prune -exec rm -rf {} +
- [ ] find . -name ".venv" -type d -prune -exec rm -rf {} +
```

### 1.5 Commit and Push
```bash
- [ ] git add .
- [ ] git commit -m "feat: Initialize PQC Scanner samples repository"
- [ ] git push -u origin main
```

---

## Phase 2: Update Main Repository (pqc-scanner)

### 2.1 Remove Samples Directory
```bash
- [ ] cd /path/to/pqc-scanner
- [ ] git checkout -b refactor/separate-samples-repo
- [ ] git rm -r samples/
```

### 2.2 Update Configuration Files

#### .github/dependabot.yml
```bash
- [ ] Remove lines 118-140 (NPM: samples/legacy-banking)
- [ ] Remove lines 141-164 (Python: samples/crypto-messenger)
```

#### .github/workflows/codeql.yml
```bash
- [ ] Remove lines 53-56 (Install dependencies - Samples)
- [ ] Update comments to reflect MCP-only scanning
```

#### .trufflehog.yml
```bash
- [ ] Remove line 23 (- samples/)
```

#### trivy.yaml
```bash
- [ ] Remove line 47 (- "**/samples/**")
```

### 2.3 Update Documentation

#### README.md
```bash
- [ ] Add "Sample Applications" section
- [ ] Link to https://github.com/arcqubit/pqc-scanner-samples
```

#### CONTRIBUTING.md
```bash
- [ ] Update testing section
- [ ] Reference external samples repo
```

#### docs/SAMPLE_REPOSITORIES.md
```bash
- [ ] Update all references to new repo
- [ ] Add migration notes
```

#### docs/vulnerability-remediation-phase2.md
```bash
- [ ] Update sample references to new repo URL
```

#### docs/security-scanning.md
```bash
- [ ] Remove samples/ skip-files mentions
```

#### docs/SAMPLES.md (NEW)
```bash
- [ ] Create new file
- [ ] Add complete samples repository reference
- [ ] Include quick start guide
```

#### mcp/README.md & mcp/QUICKSTART.md
```bash
- [ ] Update sample paths to external repo
```

### 2.4 Commit Changes
```bash
- [ ] git add .
- [ ] git commit -m "refactor: Move samples to dedicated repository"
```

---

## Phase 3: New Repository Assets (pqc-scanner-samples)

### 3.1 Create Documentation
```bash
- [ ] README.md (comprehensive)
- [ ] CONTRIBUTING.md (sample-specific)
- [ ] GETTING_STARTED.md (quick start)
- [ ] VULNERABILITY_INDEX.md (vuln catalog)
- [ ] BENCHMARKS.md (performance data)
- [ ] TESTING.md (testing guide)
```

### 3.2 Create CI/CD
```bash
- [ ] .github/workflows/validate-samples.yml
- [ ] .github/dependabot.yml (per-sample monitoring)
```

### 3.3 Add Vulnerability Documentation
```bash
- [ ] crypto-messenger/VULNERABILITIES.md
- [ ] iot-device/VULNERABILITIES.md
- [ ] legacy-banking/VULNERABILITIES.md (already exists)
- [ ] old-web-framework/VULNERABILITIES.md
- [ ] polyglot-app/VULNERABILITIES.md
```

### 3.4 Commit and Push
```bash
- [ ] git add .
- [ ] git commit -m "docs: Add comprehensive documentation and CI/CD"
- [ ] git push
```

---

## Phase 4: Validation

### 4.1 Pre-Push Validation (Main Repo)
```bash
- [ ] No broken markdown links
- [ ] All workflows pass locally
- [ ] README renders correctly
- [ ] Documentation cross-references valid
```

### 4.2 Post-Push Validation (Main Repo)
```bash
- [ ] CI/CD pipelines pass
- [ ] No references to deleted samples/ directory
- [ ] Repository size reduced (~35MB)
- [ ] All documentation links work
```

### 4.3 New Repo Validation (Samples Repo)
```bash
- [ ] README renders correctly
- [ ] No node_modules committed
- [ ] CI validation passes
- [ ] All samples have READMEs
- [ ] VULNERABILITY_INDEX is accurate
```

### 4.4 Integration Testing
```bash
- [ ] Clone both repos fresh
- [ ] Run: cargo run -- scan ../pqc-scanner-samples/legacy-banking/src/
- [ ] Verify all 15 vulnerabilities detected
- [ ] Check performance matches benchmarks
- [ ] Test all documentation examples
```

---

## Phase 5: Create Pull Request

### 5.1 Main Repo PR
```bash
- [ ] git push -u origin refactor/separate-samples-repo
- [ ] Create PR with comprehensive description
- [ ] Add labels: refactor, documentation
- [ ] Link to samples repository
- [ ] Request review
```

### 5.2 PR Description Template
```markdown
## Summary

This PR migrates sample vulnerable applications to a dedicated repository.

**New Repository**: https://github.com/arcqubit/pqc-scanner-samples

## Changes

- ✅ Removed samples/ directory (~35MB)
- ✅ Updated Dependabot configuration
- ✅ Updated CodeQL workflow
- ✅ Updated security scanning configs
- ✅ Created docs/SAMPLES.md reference
- ✅ Updated all documentation

## Benefits

- Reduced repository size by ~35MB
- Cleaner main repo organization
- Independent sample versioning
- Easier sample discovery
- Simplified CI/CD

## Validation

- [x] CI/CD passes
- [x] No broken links
- [x] Scanner works with external samples
- [x] Documentation complete

See [SAMPLES_MIGRATION_PLAN.md](docs/SAMPLES_MIGRATION_PLAN.md) for complete migration details.
```

---

## Phase 6: Post-Merge

### 6.1 Immediate Actions
```bash
- [ ] Create GitHub Discussion announcement
- [ ] Update README.md badges (if needed)
- [ ] Monitor CI/CD for issues
```

### 6.2 Short-term Actions (1 week)
```bash
- [ ] Update external references
- [ ] Monitor community feedback
- [ ] Fix any broken links reported
```

### 6.3 Long-term Actions (1 month)
```bash
- [ ] Add more samples to new repo
- [ ] Create integration examples
- [ ] Build sample showcase
- [ ] Encourage community contributions
```

---

## Rollback Procedure (If Needed)

### Quick Rollback
```bash
- [ ] git checkout main
- [ ] git revert <migration-commit-sha>
- [ ] git push
```

### Full Rollback
```bash
- [ ] Clone pqc-scanner-samples
- [ ] Copy samples back: cp -r ../pqc-scanner-samples/* samples/
- [ ] git add samples/
- [ ] git commit -m "revert: Restore samples directory"
- [ ] Revert all configuration changes
- [ ] git push
```

---

## Time Estimates

- **Phase 1**: 1 hour (New repo setup)
- **Phase 2**: 1 hour (Main repo updates)
- **Phase 3**: 1 hour (New repo assets)
- **Phase 4**: 1 hour (Validation)
- **Phase 5**: 30 minutes (PR creation)
- **Total**: 4.5 hours

---

## Success Criteria

✅ New pqc-scanner-samples repo is functional
✅ Main repo reduced by ~35MB
✅ All CI/CD pipelines pass
✅ No broken documentation links
✅ Scanner works with external samples
✅ Community announcement posted

---

## Notes

- Keep this checklist updated as you progress
- Check off items as completed
- Document any deviations from plan
- Note any issues encountered

---

**Created**: 2025-11-17
**Status**: Ready for Execution
**Branch**: refactor/separate-samples-repo
