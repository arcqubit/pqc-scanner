# Auto-Remediation Enhancement Plan

## Executive Summary

This plan outlines the enhancement of the PQC Scanner's auto-remediation capabilities, transforming it from basic template-based string replacement to an intelligent, multi-language code transformation system with advanced pattern recognition, context-aware fixes, and integration with compliance frameworks.

**Version**: 2025.11.1 (CalVer)
**Status**: Planning Phase
**Estimated Effort**: 3-4 weeks
**Risk Level**: Medium (existing remediation functionality must remain backward compatible)

---

## Current State Analysis

### ✅ Currently Implemented (Base Features)

The existing remediation system (`src/remediation.rs`) provides:

1. **Basic Template-Based Fixes**:
   - MD5 → SHA-256 (85% confidence, auto-applicable)
   - SHA-1 → SHA-256 (90% confidence, auto-applicable)
   - RSA weak keys → RSA-2048 with PQC migration warning (70% confidence, manual review)
   - DES/3DES → AES-256 (75% confidence, manual review)

2. **Fix Classification**:
   - `auto_applicable`: Can be applied automatically
   - `manual_review_required`: Needs human verification
   - Confidence scoring (0.0 - 1.0)

3. **Metadata Tracking**:
   - File path, line number, column
   - Old code → New code
   - Explanation and algorithm type
   - Remediation summary statistics

4. **Supported Languages** (detection only):
   - Rust, JavaScript, TypeScript, Python, Java, Go, C++, C#

5. **Detected Crypto Types**:
   - RSA, ECDSA, ECDH, DSA, Diffie-Hellman
   - SHA-1, MD5
   - DES, 3DES, RC4

### ❌ Current Limitations

1. **Limited Remediation Coverage**:
   - Only 4 crypto types have remediation (MD5, SHA-1, RSA, DES/3DES)
   - Missing: ECDSA, ECDH, DSA, DH, RC4 (5 algorithms)
   - No language-specific patterns (uses generic string replacement)

2. **Naive Pattern Matching**:
   - Simple string replacement (`.replace("md5", "sha256")`)
   - No AST (Abstract Syntax Tree) analysis
   - Misses context (e.g., variable names, imports, function calls)
   - Can break code structure

3. **No Multi-File Support**:
   - Only processes single files
   - Can't update related imports across files
   - No dependency tracking

4. **Limited Post-Quantum Guidance**:
   - Generic warnings about CRYSTALS-Dilithium/Kyber
   - No concrete migration templates
   - No PQC library detection

5. **No Compliance Integration**:
   - Remediation doesn't reference NIST/ITSG-33 requirements
   - No CCCS algorithm approval status in fix suggestions
   - Missing CMVP validation guidance

6. **No Diff/Patch Generation**:
   - Returns plain text old/new code
   - No unified diff format
   - No integration with version control

---

## Enhancement Goals

### Phase 1: Enhanced Pattern Recognition (Week 1)

**Objective**: Improve accuracy of fixes through language-aware pattern matching

#### 1.1 Language-Specific Template Engine

Create language-specific remediation templates:

```rust
pub struct RemediationTemplate {
    pub language: Language,
    pub crypto_type: CryptoType,
    pub patterns: Vec<Pattern>,
    pub replacement_template: String,
    pub imports_required: Vec<String>,
    pub confidence_boost: f32, // +0.1 for exact pattern match
}

pub struct Pattern {
    pub regex: Regex,
    pub match_group: String, // capture group to replace
    pub context_required: Vec<String>, // e.g., ["hashlib", "import"]
}
```

**Example Templates**:

| Language | Algorithm | Pattern | Replacement | Confidence |
|----------|-----------|---------|-------------|------------|
| Python | MD5 | `hashlib\.md5\((.*?)\)` | `hashlib.sha256($1)` | 0.95 |
| JavaScript | MD5 | `crypto\.createHash\(['"]md5['"]\)` | `crypto.createHash('sha256')` | 0.95 |
| Java | MD5 | `MessageDigest\.getInstance\("MD5"\)` | `MessageDigest.getInstance("SHA-256")` | 0.90 |
| Go | MD5 | `md5\.New\(\)` | `sha256.New()` | 0.90 |
| Rust | MD5 | `Md5::new\(\)` | `Sha256::new()` | 0.95 |

#### 1.2 Import Management

Automatically update/add required imports:

```rust
pub struct ImportFix {
    pub remove: Vec<String>,  // e.g., ["import hashlib.md5"]
    pub add: Vec<String>,     // e.g., ["from hashlib import sha256"]
    pub language: Language,
}
```

#### 1.3 Context-Aware Variable Renaming

Detect and update variable/function names:

```python
# Before
md5_hash = hashlib.md5(data)
verify_md5(md5_hash)

# After (with variable renaming)
sha256_hash = hashlib.sha256(data)
verify_sha256(sha256_hash)
```

### Phase 2: Expand Algorithm Coverage (Week 2)

**Objective**: Add remediation for all 10 detected crypto types

#### 2.1 ECDSA Remediation

```
ECDSA → Ed25519 (interim) or CRYSTALS-Dilithium (PQC)
Confidence: 0.65 (manual review)
```

**Templates**:
- Python: `ecdsa.SigningKey` → `ed25519.SigningKey` or `dilithium.SigningKey`
- JavaScript: `crypto.createECDH` → PQC library
- Java: `KeyPairGenerator.getInstance("EC")` → `"Ed25519"`

#### 2.2 ECDH Remediation

```
ECDH → X25519 (interim) or CRYSTALS-Kyber (PQC)
Confidence: 0.65 (manual review)
```

#### 2.3 DSA Remediation

```
DSA → RSA-2048 (interim) or Ed25519 or CRYSTALS-Dilithium
Confidence: 0.70 (manual review)
```

#### 2.4 Diffie-Hellman Remediation

```
DH → ECDH P-256 (interim) or CRYSTALS-Kyber (PQC)
Confidence: 0.65 (manual review)
```

#### 2.5 RC4 Remediation

```
RC4 → AES-256-GCM or ChaCha20-Poly1305
Confidence: 0.80 (manual review - mode selection required)
```

### Phase 3: Post-Quantum Migration Assistant (Week 2-3)

**Objective**: Provide concrete PQC migration templates

#### 3.1 PQC Library Detection

Detect and recommend PQC libraries per language:

| Language | PQC Library | Algorithms |
|----------|-------------|------------|
| Python | `pqcrypto` | Dilithium, Kyber, SPHINCS+ |
| JavaScript | `noble-post-quantum` | Dilithium, Kyber |
| Rust | `pqcrypto` | All NIST PQC candidates |
| Java | `BouncyCastle` | Dilithium, Kyber |
| Go | `circl` | Dilithium, Kyber |

#### 3.2 Hybrid Crypto Patterns

Generate hybrid classical + PQC solutions:

```python
# Hybrid RSA + Kyber for key exchange
def hybrid_key_exchange():
    # Classical key exchange
    rsa_key = RSA.generate(2048)

    # PQC key exchange
    kyber_public, kyber_secret = kyber.keypair()

    # Combine both for quantum-safe transition
    return (rsa_key, kyber_public, kyber_secret)
```

#### 3.3 Migration Roadmap Generation

For each file, generate a migration roadmap:

```markdown
## PQC Migration Roadmap: crypto_utils.py

### Immediate Actions (High Priority)
1. Replace MD5 → SHA-256 (Line 42) - Auto-applicable
2. Replace SHA-1 → SHA-256 (Line 55) - Auto-applicable

### Short-term (< 6 months)
3. Upgrade RSA 1024 → RSA 2048 (Line 78) - Manual review
4. Replace DES → AES-256-GCM (Line 92) - Manual review

### Long-term PQC Migration (< 2 years)
5. RSA signatures → CRYSTALS-Dilithium (Lines 78-85)
6. ECDH key exchange → CRYSTALS-Kyber (Lines 110-120)

### Required Dependencies
- `pip install pqcrypto`
- Update: `cryptography >= 41.0.0`
```

### Phase 4: Compliance-Aware Remediation (Week 3)

**Objective**: Integrate NIST/ITSG-33/CCCS requirements into fix suggestions

#### 4.1 NIST 800-53 SC-13 Alignment

Tag each fix with NIST requirements:

```rust
pub struct ComplianceFix {
    pub code_fix: CodeFix,
    pub nist_control: String,           // "NIST 800-53 SC-13"
    pub nist_requirement: String,       // "Use FIPS-approved algorithms"
    pub cccs_approval: CCCSApprovalStatus, // Approved, Prohibited, etc.
    pub itsp_reference: Option<String>, // "ITSP.40.111 Section 5.3"
}
```

#### 4.2 Classification-Aware Fixes

Provide fixes based on Canadian security classification:

```rust
// For Protected B systems, suggest stronger algorithms
if classification == SecurityClassification::ProtectedB {
    // SHA-256 is insufficient, use SHA-384
    remediation.new_code = old_code.replace("sha256", "sha384");
    remediation.explanation = "Protected B requires SHA-384 minimum per ITSP.40.111";
}
```

#### 4.3 CMVP Validation Warnings

Warn about CMVP requirements:

```
⚠ CMVP Validation Required for Protected A
The suggested fix uses OpenSSL. Ensure you're using a CMVP-validated version:
- OpenSSL FIPS Object Module (Cert #4282)
- Minimum version: OpenSSL 3.0.0-fips
```

### Phase 5: Advanced Features (Week 4)

#### 5.1 Diff/Patch Generation

Generate unified diff format:

```diff
--- a/crypto_utils.py
+++ b/crypto_utils.py
@@ -1,5 +1,5 @@
 import hashlib

 def hash_password(password):
-    return hashlib.md5(password.encode()).hexdigest()
+    return hashlib.sha256(password.encode()).hexdigest()
```

#### 5.2 Batch File Remediation

Process entire directories:

```rust
pub fn generate_batch_remediations(
    directory: &Path,
    classification: SecurityClassification,
) -> BatchRemediationResult {
    // Scan all files
    // Generate fixes
    // Track dependencies
    // Create migration plan
}
```

#### 5.3 Interactive Remediation Mode

```
Found 5 vulnerabilities. Apply fixes?

[1] MD5 → SHA-256 (test.py:42) - Auto-fix? [Y/n/s(kip)]
[2] SHA-1 → SHA-256 (test.py:55) - Auto-fix? [Y/n/s]
[3] RSA-1024 → RSA-2048 (test.py:78) - Requires review [r(eview)/s]
```

#### 5.4 Confidence Scoring Algorithm

Improve confidence based on:

```rust
pub fn calculate_confidence(
    pattern_match_quality: f32,    // 0.0-1.0
    context_available: bool,        // +0.1
    imports_verified: bool,         // +0.1
    language_specific: bool,        // +0.15
    tested_pattern: bool,           // +0.1
    manual_review_required: bool,   // -0.3
) -> f32 {
    // Weighted scoring
}
```

#### 5.5 Remediation Testing

Generate unit tests for applied fixes:

```python
# Auto-generated test
def test_sha256_hash_migration():
    """Test migrated hash function from MD5 to SHA-256"""
    data = b"test data"

    # Old behavior (MD5)
    # hash_old = hashlib.md5(data).hexdigest()

    # New behavior (SHA-256)
    hash_new = hashlib.sha256(data).hexdigest()

    assert len(hash_new) == 64  # SHA-256 produces 64 hex chars
    assert hash_new != hashlib.md5(data).hexdigest()
```

---

## Technical Architecture

### New Module Structure

```
src/
├── remediation/
│   ├── mod.rs                  # Public API (re-export)
│   ├── core.rs                 # Core remediation logic (current)
│   ├── templates.rs            # Language-specific templates
│   ├── patterns.rs             # Pattern matching engine
│   ├── pqc_migration.rs        # Post-quantum migration templates
│   ├── compliance_integration.rs # NIST/CCCS integration
│   ├── diff_generator.rs       # Diff/patch generation
│   ├── batch_processor.rs      # Multi-file remediation
│   └── confidence_scorer.rs    # Advanced confidence scoring
│
data/
├── remediation_templates/
│   ├── python_templates.json
│   ├── javascript_templates.json
│   ├── java_templates.json
│   ├── rust_templates.json
│   ├── go_templates.json
│   └── pqc_libraries.json
│
tests/
├── remediation/
│   ├── test_templates.rs
│   ├── test_patterns.rs
│   ├── test_pqc_migration.rs
│   └── test_batch_processing.rs
```

### New Data Types

```rust
// Enhanced remediation result with compliance
pub struct EnhancedRemediationResult {
    pub fixes: Vec<ComplianceFix>,
    pub summary: RemediationSummary,
    pub warnings: Vec<String>,
    pub migration_roadmap: Option<MigrationRoadmap>,
    pub pqc_recommendations: Vec<PqcRecommendation>,
    pub diff_patches: HashMap<String, String>, // file_path -> unified diff
}

// PQC recommendation
pub struct PqcRecommendation {
    pub crypto_type: CryptoType,
    pub interim_solution: String,
    pub pqc_algorithm: String,
    pub library_recommendations: Vec<LibraryRecommendation>,
    pub migration_complexity: MigrationComplexity,
}

// Migration roadmap
pub struct MigrationRoadmap {
    pub immediate_actions: Vec<ComplianceFix>,
    pub short_term_actions: Vec<ComplianceFix>,
    pub long_term_pqc_migration: Vec<PqcRecommendation>,
    pub estimated_effort: Duration,
}
```

---

## Implementation Phases

### ✅ Phase 1: Enhanced Patterns (Week 1)

**Tasks**:
1. Create template engine with language-specific patterns
2. Implement import management
3. Add context-aware variable renaming
4. Update existing MD5, SHA-1, RSA, DES remediations
5. Add 50+ new language-specific templates
6. Unit tests for each template

**Deliverables**:
- `src/remediation/templates.rs`
- `src/remediation/patterns.rs`
- `data/remediation_templates/*.json`
- 95% confidence for language-specific patterns

### ✅ Phase 2: Algorithm Coverage (Week 2)

**Tasks**:
1. Implement ECDSA remediation
2. Implement ECDH remediation
3. Implement DSA remediation
4. Implement DH remediation
5. Implement RC4 remediation
6. Integration tests for all algorithms

**Deliverables**:
- Complete coverage for all 10 crypto types
- 100+ remediation templates
- Comprehensive test suite

### ✅ Phase 3: PQC Migration (Week 2-3)

**Tasks**:
1. Create PQC library database
2. Implement hybrid crypto patterns
3. Build migration roadmap generator
4. Add PQC-specific recommendations
5. Generate migration documentation

**Deliverables**:
- `src/remediation/pqc_migration.rs`
- `data/remediation_templates/pqc_libraries.json`
- PQC migration examples

### ✅ Phase 4: Compliance Integration (Week 3)

**Tasks**:
1. Add NIST 800-53 SC-13 alignment
2. Implement classification-aware fixes
3. Add CMVP validation warnings
4. Integrate with CCCS algorithm database
5. Generate compliance-aware recommendations

**Deliverables**:
- `src/remediation/compliance_integration.rs`
- ITSG-33/NIST-aligned remediation
- CMVP validation guidance

### ✅ Phase 5: Advanced Features (Week 4)

**Tasks**:
1. Implement diff/patch generation
2. Build batch file processor
3. Create interactive mode
4. Enhance confidence scoring
5. Add remediation testing generator

**Deliverables**:
- `src/remediation/diff_generator.rs`
- `src/remediation/batch_processor.rs`
- CLI interactive mode
- Test generation

---

## Testing Strategy

### Unit Tests

- **Target Coverage**: >95%
- **Test Count**: 150+ tests (vs current 15)
- Per-language template tests
- Per-algorithm remediation tests
- Edge case handling

### Integration Tests

- End-to-end remediation workflows
- Multi-file batch processing
- Compliance integration
- PQC migration paths

### Regression Tests

- Ensure backward compatibility
- All current tests must pass
- No breaking API changes

### Performance Benchmarks

- Remediation generation: <5ms per vulnerability
- Batch processing: <100ms per file
- Memory usage: <100MB for 1000 files

---

## API Changes (Backward Compatible)

### Existing API (Preserved)

```rust
// Current API remains unchanged
pub fn generate_remediations(
    audit_result: &AuditResult,
    file_path: &str
) -> RemediationResult
```

### New Enhanced API

```rust
// New enhanced API with additional parameters
pub fn generate_enhanced_remediations(
    audit_result: &AuditResult,
    file_path: &str,
    options: RemediationOptions,
) -> EnhancedRemediationResult

pub struct RemediationOptions {
    pub classification: Option<SecurityClassification>,
    pub include_pqc_recommendations: bool,
    pub include_diff_patches: bool,
    pub include_migration_roadmap: bool,
    pub language_specific_templates: bool, // default: true
    pub confidence_threshold: f32,          // default: 0.5
}
```

---

## Documentation Updates

### User Documentation

- **docs/auto-remediation.md**: Complete remediation guide
- **docs/pqc-migration.md**: PQC migration handbook
- **examples/enhanced_remediation_example.rs**: Comprehensive examples

### API Documentation

- Rustdoc comments for all public APIs
- Template format specification
- Pattern matching guide

### Migration Guide

- Upgrading from basic to enhanced remediation
- Using PQC recommendations
- Batch processing workflows

---

## Success Metrics

### Coverage Metrics

- ✅ **Algorithm Coverage**: 10/10 (100%)
- ✅ **Language-Specific Templates**: 50+ templates
- ✅ **PQC Algorithms**: 3+ (Dilithium, Kyber, SPHINCS+)
- ✅ **Test Coverage**: >95%

### Quality Metrics

- ✅ **Average Confidence**: >0.80 (vs 0.78 current)
- ✅ **Auto-Fixable Rate**: >60% (vs 50% current)
- ✅ **False Positive Rate**: <5%

### Performance Metrics

- ✅ **Remediation Speed**: <5ms per vulnerability
- ✅ **Batch Processing**: <100ms per file
- ✅ **Memory Efficiency**: <100MB for 1000 files

---

## Risk Assessment & Mitigation

### High Risks

**Risk**: Breaking existing remediation functionality
**Mitigation**:
- Preserve existing API
- Comprehensive regression testing
- Feature flags for new functionality

**Risk**: Incorrect code transformations causing bugs
**Mitigation**:
- Conservative confidence scoring
- Mandatory manual review for complex changes
- Extensive pattern testing

### Medium Risks

**Risk**: Language-specific templates are incomplete
**Mitigation**:
- Fallback to generic templates
- Community contribution process
- Continuous template refinement

**Risk**: PQC libraries not available for all languages
**Mitigation**:
- Document library availability
- Provide interim solutions
- Hybrid crypto patterns

### Low Risks

**Risk**: Performance degradation with enhanced patterns
**Mitigation**:
- Lazy template loading
- Pattern caching
- Benchmark-driven optimization

---

## Release Plan

### Version 2025.11.1-alpha.1 (End of Week 2)

- Phase 1 complete (enhanced patterns)
- Phase 2 partial (5 algorithms)

### Version 2025.11.1-beta.1 (End of Week 3)

- Phase 1-3 complete
- Full algorithm coverage
- PQC migration support

### Version 2025.11.1 (End of Week 4)

- All phases complete
- Full compliance integration
- Production-ready

---

## Future Enhancements (Post-MVP)

### Phase 6: AST-Based Remediation

- Use actual parsers (tree-sitter, syn, etc.)
- Type-aware transformations
- Control flow analysis

### Phase 7: Machine Learning

- Learn from user acceptances/rejections
- Pattern discovery from codebases
- Confidence tuning

### Phase 8: IDE Integration

- VS Code extension
- IntelliJ plugin
- Real-time remediation suggestions

### Phase 9: CI/CD Integration

- GitHub Actions workflow
- GitLab CI integration
- Automated PR generation

---

## Open Questions for Review

1. **Priority**: Should we focus on depth (AST-based for 3 languages) or breadth (template-based for all 8 languages)?

2. **PQC Libraries**: Which PQC libraries should be primary recommendations per language?

3. **Confidence Threshold**: What should be the default confidence threshold for auto-application? (Current thinking: 0.85)

4. **Breaking Changes**: Are there any breaking changes you'd accept for improved functionality?

5. **Interactive Mode**: Should interactive mode be CLI-only or also support a web UI?

6. **Batch Processing**: Should batch mode automatically create git commits or just generate patch files?

7. **Testing Strategy**: Should we auto-generate unit tests for every remediation or only on user request?

8. **Performance**: Is <5ms per vulnerability fast enough, or should we target <1ms?

---

## Resources Required

### Development

- **Primary Developer**: 1 full-time (4 weeks)
- **Code Reviewer**: 1 part-time (8 hours/week)
- **Documentation Writer**: 1 part-time (4 hours/week)

### Testing

- **QA Engineer**: 1 part-time (8 hours/week, weeks 3-4)
- **Security Reviewer**: 1 part-time (4 hours, final week)

### Infrastructure

- **CI/CD**: Existing GitHub Actions (no additional cost)
- **Test Data**: Create 100+ sample files for testing
- **PQC Libraries**: Research and document availability

---

## Conclusion

This enhancement plan transforms PQC Scanner's auto-remediation from a basic template system to a comprehensive, intelligent code transformation platform with:

- **100% algorithm coverage** (10/10 crypto types)
- **Multi-language support** with language-specific patterns
- **Post-quantum migration guidance** with concrete templates
- **Compliance integration** with NIST/ITSG-33/CCCS frameworks
- **Advanced features** like diff generation and batch processing

The phased approach ensures backward compatibility while delivering incremental value each week.

---

**Next Steps**: Please review this plan and provide feedback on:
- Overall approach and priorities
- Open questions (listed above)
- Timeline and resource allocation
- Any missing requirements or concerns

Once approved, development will begin with Phase 1 (Enhanced Pattern Recognition).
