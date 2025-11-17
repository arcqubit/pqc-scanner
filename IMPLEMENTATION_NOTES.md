# Auto-Remediation Implementation Notes

## Updated Approach (Based on User Feedback)

### Key Changes from Original Plan

1. **NO AUTO-APPLY**: Remediation is NEVER automatically applied
   - All fixes generate reports and patch files only
   - User must manually review and apply changes
   - Safety-first approach

2. **Feature Flag**: `--auto-remediate`
   - **Default**: `false` (disabled)
   - When `true`: Generates patch files
   - When `false`: Report-only mode

3. **Focus**: Breadth-first (template-based for all 8 languages)
   - AST-based remediation deferred to future phase
   - Language-specific templates for comprehensive coverage

### Updated API Design

```rust
pub struct RemediationOptions {
    pub classification: Option<SecurityClassification>,
    pub generate_patches: bool,              // NEW: controlled by --auto-remediate
    pub include_pqc_recommendations: bool,
    pub include_migration_roadmap: bool,
    pub language_specific_templates: bool,
    pub confidence_threshold: f32,
}

// Default behavior: report only
impl Default for RemediationOptions {
    fn default() -> Self {
        Self {
            classification: None,
            generate_patches: false,  // NO patches by default
            include_pqc_recommendations: true,
            include_migration_roadmap: true,
            language_specific_templates: true,
            confidence_threshold: 0.5,  // Show all fixes, user decides
        }
    }
}
```

### CLI Integration

```bash
# Default: Report only
pqc-scanner scan src/ --compliance-report

# With remediation report (no patches)
pqc-scanner scan src/ --remediation-report

# Generate patch files (requires explicit flag)
pqc-scanner scan src/ --auto-remediate=true

# Full workflow with classification
pqc-scanner scan src/ --auto-remediate=true --classification=protected-b
```

### Updated Field Names

```rust
pub struct CodeFix {
    // Remove misleading field
    // pub auto_applicable: bool,  // REMOVED

    // Add new fields
    pub patch_available: bool,        // Can generate patch
    pub requires_manual_review: bool, // Always true (everything needs review)
    pub review_notes: Vec<String>,    // What to check before applying
}
```

### Patch File Format

```diff
# File: remediation_patches/crypto_utils.py.patch
# Generated: 2025-11-17T16:30:00Z
# Algorithm: MD5 → SHA-256
# Confidence: 0.95
# NIST: 800-53 SC-13
# CCCS: ITSP.40.111 Section 5.3 (MD5 PROHIBITED)
# Review: Verify all MD5 usages are for hashing, not checksums

--- a/crypto_utils.py
+++ b/crypto_utils.py
@@ -1,5 +1,5 @@
 import hashlib

 def hash_password(password):
-    return hashlib.md5(password.encode()).hexdigest()
+    return hashlib.sha256(password.encode()).hexdigest()
```

### Report Structure

```json
{
  "metadata": {
    "scan_date": "2025-11-17T16:30:00Z",
    "remediation_enabled": false,
    "patches_generated": false
  },
  "summary": {
    "total_vulnerabilities": 10,
    "remediable": 8,
    "patch_available": 8,
    "manual_only": 2
  },
  "fixes": [
    {
      "file_path": "crypto_utils.py",
      "line": 42,
      "algorithm": "MD5 → SHA-256",
      "confidence": 0.95,
      "patch_available": true,
      "requires_manual_review": true,
      "patch_file": "remediation_patches/crypto_utils.py_001.patch",
      "review_notes": [
        "Verify MD5 is used for hashing, not file checksums",
        "Update unit tests to expect SHA-256 output",
        "Check for any stored MD5 hashes in database"
      ]
    }
  ]
}
```

## Implementation Order

### Week 1: Core Infrastructure
1. Remove `auto_applicable` field
2. Add `--auto-remediate` CLI flag
3. Implement patch file generation
4. Create template engine
5. Add language-specific patterns

### Week 2: Algorithm Coverage
1. ECDSA, ECDH, DSA, DH, RC4 remediation
2. 50+ language-specific templates
3. Review notes for each fix type

### Week 3: PQC + Compliance
1. PQC migration guidance
2. NIST/CCCS integration
3. Migration roadmap

### Week 4: Polish + Testing
1. Comprehensive test suite
2. Documentation
3. Examples
4. Beta release

## Safety Features

1. **Patch Preview**: Always show diff before any action
2. **Review Checklist**: Each fix includes review notes
3. **Confidence Threshold**: User can filter by confidence
4. **Dry Run**: Default mode is always dry run
5. **Explicit Opt-in**: Must use `--auto-remediate=true`
