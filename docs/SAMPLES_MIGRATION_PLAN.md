# PQC Scanner Samples Migration Plan

## Overview

**Migration Date**: 2025-11-17
**Source Repository**: `arcqubit/pqc-scanner`
**Target Repository**: `arcqubit/pqc-scanner-samples` (https://github.com/arcqubit/pqc-scanner-samples.git)

This document outlines the complete plan for separating sample repositories into a dedicated repository while maintaining all references, documentation, and CI/CD integration.

---

## Table of Contents

1. [Rationale](#rationale)
2. [Current State Analysis](#current-state-analysis)
3. [Migration Steps](#migration-steps)
4. [New Repository Structure](#new-repository-structure)
5. [Updates Required in Main Repo](#updates-required-in-main-repo)
6. [CI/CD Integration](#cicd-integration)
7. [Documentation Updates](#documentation-updates)
8. [Rollback Plan](#rollback-plan)
9. [Validation Checklist](#validation-checklist)

---

## Rationale

### Why Separate the Samples?

1. **Repository Size**: `samples/legacy-banking/` alone is 35MB (99% due to node_modules)
2. **Cleaner Main Repo**: Focus on core scanner functionality
3. **Independent Versioning**: Samples can evolve independently
4. **Easier Discovery**: Dedicated repo is more discoverable for users
5. **Simplified CI/CD**: Reduce main repo CI complexity
6. **Better Organization**: Samples become a showcase/tutorial resource

### Benefits

- ✅ Smaller main repository (removes ~35MB)
- ✅ Faster git operations (clone, fetch, pull)
- ✅ Independent sample evolution
- ✅ Clearer separation of concerns
- ✅ Easier for users to fork/customize samples
- ✅ Reduced security scanning surface (fewer dependencies to track)

---

## Current State Analysis

### Directory Structure

```
samples/
├── README.md                    (182 lines - overview)
├── GETTING_STARTED.md           (143 lines - quick start)
├── .samples-summary.txt         (summary data)
├── crypto-messenger/            (~32K - Python Flask)
│   ├── app/
│   ├── tests/
│   ├── requirements.txt
│   └── README.md
├── iot-device/                  (~40K - C++ embedded)
│   ├── src/
│   ├── CMakeLists.txt
│   └── README.md
├── legacy-banking/              (~35MB - JavaScript/Node.js)
│   ├── src/
│   ├── tests/
│   ├── package.json
│   ├── node_modules/           (35MB - should be gitignored!)
│   ├── VULNERABILITIES.md
│   └── README.md
├── old-web-framework/           (~44K - Java/Spring)
│   ├── src/
│   └── README.md
└── polyglot-app/                (~32K - Multi-language)
    ├── backend/
    ├── frontend/
    ├── legacy-service/
    ├── lib-crypto/
    ├── microservices/
    └── README.md
```

### Total Samples Data

- **5 sample applications**
- **33,859 lines of code**
- **139 files** (excluding node_modules)
- **94+ intentional vulnerabilities**
- **~35MB total size** (mostly node_modules)

### References in Main Repo

#### Workflows
1. `.github/workflows/codeql.yml:55` - npm install for legacy-banking

#### Configuration Files
1. `.github/dependabot.yml:121` - NPM monitoring for legacy-banking
2. `.github/dependabot.yml:141` - Python monitoring for crypto-messenger
3. `.trufflehog.yml:23` - Excludes samples/ from secret scanning
4. `trivy.yaml:47` - Excludes samples/ from security scanning

#### Documentation
1. `docs/SAMPLE_REPOSITORIES.md` - Sample repository guidelines
2. `docs/vulnerability-remediation-phase2.md` - References samples
3. `docs/security-scanning.md` - Mentions samples in skip-files
4. `CONTRIBUTING.md` - May reference samples for testing
5. `mcp/README.md` - May reference sample usage
6. `samples/README.md` - Main samples documentation
7. `samples/GETTING_STARTED.md` - Quick start guide

#### Code References
- May be referenced in examples/
- May be used in integration tests
- May be mentioned in README.md

---

## Migration Steps

### Phase 1: Prepare New Repository (pqc-scanner-samples)

**Duration**: 1 hour

1. **Initialize New Repository**
   ```bash
   # Clone the new empty repo
   git clone https://github.com/arcqubit/pqc-scanner-samples.git
   cd pqc-scanner-samples

   # Initialize with main branch
   git checkout -b main
   ```

2. **Copy Samples Directory**
   ```bash
   # From pqc-scanner repo
   cd /path/to/pqc-scanner
   cp -r samples/* /path/to/pqc-scanner-samples/
   ```

3. **Create Root Documentation**
   - Move `samples/README.md` → `README.md`
   - Move `samples/GETTING_STARTED.md` → `GETTING_STARTED.md`
   - Create `CONTRIBUTING.md` (samples-specific)
   - Create `LICENSE` (MIT for samples)
   - Create `.gitignore` (comprehensive)

4. **Add Metadata Files**
   - `VULNERABILITY_INDEX.md` - Comprehensive vuln catalog
   - `TESTING.md` - How to test samples
   - `BENCHMARKS.md` - Performance benchmarks
   - `.github/workflows/validate-samples.yml` - CI validation

5. **Clean Up**
   ```bash
   # Remove node_modules (should never be committed)
   find . -name "node_modules" -type d -prune -exec rm -rf {} +

   # Add proper .gitignore
   cat > .gitignore <<EOF
   # Dependencies
   node_modules/
   __pycache__/
   *.pyc
   .venv/
   venv/

   # Build artifacts
   target/
   build/
   dist/
   *.class
   *.o

   # IDE
   .vscode/
   .idea/
   *.swp

   # OS
   .DS_Store
   Thumbs.db

   # Test artifacts
   coverage/
   .pytest_cache/
   junit.xml
   EOF
   ```

6. **Initial Commit**
   ```bash
   git add .
   git commit -m "feat: Initialize PQC Scanner samples repository

   Migrated from arcqubit/pqc-scanner to dedicated repository.

   Includes 5 intentionally vulnerable sample applications:
   - crypto-messenger (Python/Flask)
   - iot-device (C++ embedded)
   - legacy-banking (JavaScript/Node.js)
   - old-web-framework (Java/Spring)
   - polyglot-app (Multi-language)

   Total: 94+ vulnerabilities across 33,859 LOC

   See README.md for complete documentation."

   git push -u origin main
   ```

### Phase 2: Update Main Repository (pqc-scanner)

**Duration**: 1 hour

**Branch**: `refactor/separate-samples-repo` (already created)

1. **Remove Samples Directory**
   ```bash
   # In pqc-scanner repo
   git rm -r samples/
   git commit -m "refactor: Move samples to dedicated repository

   Samples have been moved to:
   https://github.com/arcqubit/pqc-scanner-samples

   This reduces main repo size by ~35MB and improves organization."
   ```

2. **Update Dependabot Configuration**

   Edit `.github/dependabot.yml`:
   - **Remove** NPM monitoring for `samples/legacy-banking` (lines 118-140)
   - **Remove** Python monitoring for `samples/crypto-messenger` (lines 141-164)

3. **Update CodeQL Workflow**

   Edit `.github/workflows/codeql.yml`:
   - **Remove** step: "Install dependencies (Samples)" (lines 53-56)
   - Update comment to reflect only MCP scanning

4. **Update Security Scanning Configuration**

   Edit `.trufflehog.yml`:
   - **Remove**: `- samples/` from exclude_paths

   Edit `trivy.yaml`:
   - **Remove**: `- "**/samples/**"` from skip-files

5. **Update Documentation**

   **README.md**:
   - Add "Samples" section linking to new repo
   - Update quick start to reference external samples

   **CONTRIBUTING.md**:
   - Update testing section to reference samples repo
   - Add instructions for cloning samples separately

   **docs/SAMPLE_REPOSITORIES.md**:
   - Update to point to new repository
   - Add migration notes

   **docs/vulnerability-remediation-phase2.md**:
   - Update sample references to new repo URL

   **docs/security-scanning.md**:
   - Remove samples/ skip-files mentions

6. **Create Samples Reference Document**

   Create `docs/SAMPLES.md`:
   ```markdown
   # Sample Vulnerable Applications

   PQC Scanner includes sample vulnerable applications for testing and demonstration.

   ## Samples Repository

   All samples have been moved to a dedicated repository:

   **Repository**: https://github.com/arcqubit/pqc-scanner-samples

   ### Quick Start

   ```bash
   # Clone samples repository
   git clone https://github.com/arcqubit/pqc-scanner-samples.git

   # Scan a sample
   cargo run -- scan samples/legacy-banking/src/
   ```

   ### Available Samples

   - **legacy-banking**: JavaScript/Node.js banking app (15 vulns)
   - **crypto-messenger**: Python/Flask messenger (12 vulns)
   - **old-web-framework**: Java/Spring framework (18 vulns)
   - **iot-device**: C++ embedded device (14 vulns)
   - **polyglot-app**: Multi-language app (35+ vulns)

   See the [samples repository](https://github.com/arcqubit/pqc-scanner-samples) for complete documentation.
   ```

7. **Update Examples (if applicable)**

   Check and update any code examples that reference `samples/`:
   ```bash
   # Find references in examples/
   grep -r "samples/" examples/

   # Update to use relative or absolute paths
   ```

8. **Update MCP Documentation**

   Edit `mcp/README.md`, `mcp/QUICKSTART.md`:
   - Update sample paths to reference external repo

### Phase 3: Create New Repository Assets

**Duration**: 1 hour

In `pqc-scanner-samples` repo:

1. **Create Comprehensive README.md**
   - Overview of all samples
   - Quick start guide
   - Vulnerability index
   - Contributing guidelines
   - License information

2. **Create CONTRIBUTING.md**
   - How to add new samples
   - Sample requirements
   - Testing guidelines
   - Vulnerability documentation standards

3. **Create .github/workflows/validate-samples.yml**
   ```yaml
   name: Validate Samples

   on:
     push:
       branches: [ main ]
     pull_request:
       branches: [ main ]

   jobs:
     validate:
       name: Validate Sample Structure
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v4

         - name: Check sample structure
           run: |
             # Ensure each sample has required files
             for sample in crypto-messenger iot-device legacy-banking old-web-framework polyglot-app; do
               echo "Validating $sample..."
               test -f "$sample/README.md" || exit 1
             done

         - name: Verify no secrets committed
           run: |
             # Check for accidentally committed secrets
             if grep -r "sk-" . --exclude-dir=.git; then
               echo "Potential API key found!"
               exit 1
             fi
   ```

4. **Create VULNERABILITY_INDEX.md**
   - Comprehensive catalog of all vulnerabilities
   - Organized by type (quantum-vulnerable, deprecated, weak)
   - Cross-referenced with samples

5. **Create BENCHMARKS.md**
   - Performance benchmarks for each sample
   - Expected scan times
   - Memory usage
   - Detection accuracy

---

## New Repository Structure

```
pqc-scanner-samples/
├── README.md                       (Main documentation)
├── CONTRIBUTING.md                 (How to contribute samples)
├── GETTING_STARTED.md              (Quick start guide)
├── VULNERABILITY_INDEX.md          (Complete vulnerability catalog)
├── BENCHMARKS.md                   (Performance data)
├── TESTING.md                      (How to test samples)
├── LICENSE                         (MIT License)
├── .gitignore                      (Comprehensive ignores)
├── .github/
│   └── workflows/
│       └── validate-samples.yml    (CI validation)
├── crypto-messenger/               (Python Flask messenger)
│   ├── app/
│   ├── tests/
│   ├── requirements.txt
│   ├── README.md
│   └── VULNERABILITIES.md
├── iot-device/                     (C++ embedded device)
│   ├── src/
│   ├── CMakeLists.txt
│   ├── README.md
│   └── VULNERABILITIES.md
├── legacy-banking/                 (JavaScript/Node.js banking)
│   ├── src/
│   ├── tests/
│   ├── package.json
│   ├── README.md
│   └── VULNERABILITIES.md
├── old-web-framework/              (Java/Spring framework)
│   ├── src/
│   ├── README.md
│   └── VULNERABILITIES.md
└── polyglot-app/                   (Multi-language microservices)
    ├── backend/
    ├── frontend/
    ├── legacy-service/
    ├── lib-crypto/
    ├── microservices/
    ├── README.md
    └── VULNERABILITIES.md
```

---

## Updates Required in Main Repo

### Files to Modify

| File | Action | Lines | Description |
|------|--------|-------|-------------|
| `samples/` | **DELETE** | - | Entire directory |
| `.github/dependabot.yml` | **EDIT** | 118-164 | Remove samples NPM/Python monitoring |
| `.github/workflows/codeql.yml` | **EDIT** | 53-56 | Remove samples npm install |
| `.trufflehog.yml` | **EDIT** | 23 | Remove samples exclude |
| `trivy.yaml` | **EDIT** | 47 | Remove samples skip-files |
| `README.md` | **EDIT** | TBD | Add samples repo link |
| `CONTRIBUTING.md` | **EDIT** | TBD | Update testing references |
| `docs/SAMPLE_REPOSITORIES.md` | **EDIT** | All | Update to point to new repo |
| `docs/vulnerability-remediation-phase2.md` | **EDIT** | TBD | Update sample references |
| `docs/security-scanning.md` | **EDIT** | TBD | Remove samples mentions |
| `docs/SAMPLES.md` | **CREATE** | - | New samples reference doc |
| `mcp/README.md` | **EDIT** | TBD | Update sample paths |
| `mcp/QUICKSTART.md` | **EDIT** | TBD | Update sample paths |

### Total Changes Estimate
- **1 directory removed**: `samples/` (~35MB, ~139 files)
- **12 files modified**: Documentation and configuration
- **1 file created**: `docs/SAMPLES.md`
- **Net reduction**: ~35MB repository size

---

## CI/CD Integration

### Main Repository (pqc-scanner)

**Simplified Workflows**:
- ✅ Remove samples NPM dependency monitoring
- ✅ Remove samples Python dependency monitoring
- ✅ Remove samples from CodeQL scanning
- ✅ Remove samples from TruffleHog/Trivy exclusions

**Result**: Faster CI/CD, fewer false positives

### New Samples Repository (pqc-scanner-samples)

**New Workflows**:
1. **validate-samples.yml**: Structure validation, no secrets
2. **dependency-updates.yml**: Dependabot for each sample
3. **security-scan.yml**: Intentional vulns vs. actual issues

---

## Documentation Updates

### In pqc-scanner

1. **README.md**
   ```markdown
   ## Sample Applications

   PQC Scanner includes sample vulnerable applications for testing and demonstration.

   **Samples Repository**: https://github.com/arcqubit/pqc-scanner-samples

   Includes 5 intentionally vulnerable applications with 94+ known vulnerabilities.
   See the samples repository for complete documentation and quick start guide.
   ```

2. **docs/SAMPLES.md** (NEW)
   - Complete reference to samples repository
   - Quick start guide
   - Integration instructions

3. **CONTRIBUTING.md**
   - Update testing section
   - Reference samples repo for test data

4. **docs/SAMPLE_REPOSITORIES.md**
   - Redirect to new repository
   - Migration notes
   - Historical context

### In pqc-scanner-samples

1. **README.md** - Complete samples documentation
2. **CONTRIBUTING.md** - Sample contribution guidelines
3. **GETTING_STARTED.md** - Quick start guide
4. **VULNERABILITY_INDEX.md** - Vulnerability catalog
5. **BENCHMARKS.md** - Performance data
6. **Individual sample READMEs** - Detailed per-sample docs

---

## Rollback Plan

If migration causes issues:

### Quick Rollback (< 1 hour)

1. **Revert Main Repo Changes**
   ```bash
   # In pqc-scanner repo
   git checkout main
   git revert <migration-commit-sha>
   git push
   ```

2. **Re-add Samples** (if needed)
   ```bash
   # Copy samples back from new repo
   git clone https://github.com/arcqubit/pqc-scanner-samples.git temp-samples
   cp -r temp-samples/* samples/
   git add samples/
   git commit -m "revert: Restore samples directory"
   git push
   ```

3. **Restore Configuration**
   - Revert `.github/dependabot.yml`
   - Revert `.github/workflows/codeql.yml`
   - Revert security scanning configs

### Full Rollback (if new repo has issues)

1. Archive `pqc-scanner-samples` repository
2. Keep samples in main repo long-term
3. Document decision for future reference

---

## Validation Checklist

### Pre-Migration

- [ ] Backup current repository state
- [ ] Document all sample references
- [ ] Test current CI/CD pipelines
- [ ] Review all workflows for samples references

### During Migration

- [ ] New repo created and initialized
- [ ] Samples copied to new repo
- [ ] New repo documentation complete
- [ ] .gitignore prevents node_modules
- [ ] Initial commit pushed successfully

### Post-Migration (pqc-scanner)

- [ ] Samples directory removed
- [ ] Dependabot config updated
- [ ] CodeQL workflow updated
- [ ] Security scanning configs updated
- [ ] README updated with samples link
- [ ] CONTRIBUTING.md updated
- [ ] docs/SAMPLES.md created
- [ ] All documentation references updated
- [ ] CI/CD pipelines pass
- [ ] Repository size reduced
- [ ] No broken links

### Post-Migration (pqc-scanner-samples)

- [ ] README.md comprehensive
- [ ] CONTRIBUTING.md complete
- [ ] VULNERABILITY_INDEX.md accurate
- [ ] All samples have READMEs
- [ ] .gitignore working correctly
- [ ] No node_modules committed
- [ ] CI validation workflow working
- [ ] Repository publicly accessible

### Integration Testing

- [ ] Clone both repos fresh
- [ ] Run scanner against samples
- [ ] Verify all vulns detected
- [ ] Check performance benchmarks
- [ ] Test documentation links
- [ ] Verify CI/CD integration

---

## Timeline

### Estimated Duration: 3-4 hours

| Phase | Duration | Tasks |
|-------|----------|-------|
| **Phase 1**: New Repo Setup | 1 hour | Initialize, copy, document, commit |
| **Phase 2**: Main Repo Updates | 1 hour | Remove samples, update configs, docs |
| **Phase 3**: New Repo Assets | 1 hour | Complete docs, CI/CD, validation |
| **Validation & Testing** | 1 hour | Full integration testing |

### Recommended Schedule

**Day 1**:
- Phase 1: New repository setup (morning)
- Phase 2: Main repository updates (afternoon)

**Day 2**:
- Phase 3: New repository assets (morning)
- Validation & testing (afternoon)

**Day 3**:
- PR review and merge
- Announcement and documentation

---

## Success Criteria

✅ **Primary Goals**:
1. New `pqc-scanner-samples` repository is functional and documented
2. Main `pqc-scanner` repository reduced by ~35MB
3. All CI/CD pipelines pass
4. No broken documentation links
5. Scanner still works with external samples

✅ **Secondary Goals**:
1. Improved main repository organization
2. Clearer separation of concerns
3. Easier sample discovery and usage
4. Better contributor experience

---

## Post-Migration Actions

### Immediate (Within 1 week)

1. **Announcement**:
   - GitHub Discussion post
   - Update project README
   - Social media announcement (if applicable)

2. **Update External References**:
   - Update any blog posts
   - Update external documentation
   - Update package registry descriptions

3. **Monitor**:
   - Watch for broken links
   - Monitor CI/CD failures
   - Track community feedback

### Short-term (Within 1 month)

1. **Enhance Samples Repo**:
   - Add more samples
   - Improve documentation
   - Add video tutorials

2. **Integration Examples**:
   - Create integration examples
   - Add to main repo examples/
   - Document common use cases

3. **Community Engagement**:
   - Encourage sample contributions
   - Create sample request template
   - Build sample showcase

---

## Questions & Decisions

### Open Questions

1. **Q**: Should we keep any samples in main repo for quick testing?
   **A**: No - use examples/ directory for lightweight code snippets instead.

2. **Q**: Should samples repo have its own CI/CD for running scanner?
   **A**: Yes - add workflow to scan samples weekly and verify detection.

3. **Q**: How to handle sample versioning?
   **A**: Use git tags (v1.0.0, v1.1.0) for major sample updates.

4. **Q**: Should samples repo have releases?
   **A**: Yes - create releases for scanner version compatibility.

### Decisions Made

✅ Separate samples into dedicated repository
✅ Use MIT license for samples (educational use)
✅ Remove node_modules from git history (should never be committed)
✅ Create comprehensive vulnerability index
✅ Add CI validation for sample structure

---

## References

- **New Repository**: https://github.com/arcqubit/pqc-scanner-samples.git
- **Main Repository**: https://github.com/arcqubit/pqc-scanner.git
- **Migration Branch**: `refactor/separate-samples-repo`
- **Related Issues**: TBD
- **Related PRs**: TBD

---

**Migration Plan Created**: 2025-11-17
**Document Version**: 1.0
**Status**: Ready for Execution
