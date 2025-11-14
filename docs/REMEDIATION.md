# Auto-Remediation Module

The PQC Scanner includes an intelligent auto-remediation engine that provides template-based code fixes for cryptographic vulnerabilities.

## Overview

The remediation module analyzes audit results and generates actionable fix suggestions with:

- **Confidence Scores**: Each fix includes a confidence level (0.0-1.0)
- **Auto-Apply Detection**: Identifies which fixes can be safely auto-applied
- **Template-Based**: Uses language-aware templates for code replacement
- **WASM Compatible**: Fully functional in WebAssembly environments

## Supported Remediations

### Hash Functions

#### MD5 → SHA-256
- **Auto-Apply**: ✅ Yes
- **Confidence**: 85%
- **Example**:
  ```python
  # Before
  hash = hashlib.md5(data).hexdigest()

  # After
  hash = hashlib.sha256(data).hexdigest()
  ```

#### SHA-1 → SHA-256
- **Auto-Apply**: ✅ Yes
- **Confidence**: 90%
- **Example**:
  ```javascript
  // Before
  const hash = crypto.createHash('sha1');

  // After
  const hash = crypto.createHash('sha256');
  ```

### Public Key Cryptography

#### RSA-1024 → RSA-2048 (Interim)
- **Auto-Apply**: ⚠️ Manual Review Required
- **Confidence**: 70%
- **Example**:
  ```python
  # Before
  key = RSA.generate(1024)

  # After (Interim - Plan PQC Migration)
  key = RSA.generate(2048)
  ```
- **Note**: This is an interim fix. RSA is quantum-vulnerable. Plan migration to CRYSTALS-Dilithium (signatures) or CRYSTALS-Kyber (encryption).

#### RSA-2048+ → PQC Migration Warning
- **Auto-Apply**: ⚠️ Manual Review Required
- **Confidence**: 50%
- **Recommendation**: Migrate to post-quantum algorithms
  - **Signatures**: CRYSTALS-Dilithium
  - **Encryption**: CRYSTALS-Kyber

### Symmetric Encryption

#### DES/3DES → AES-256-GCM
- **Auto-Apply**: ⚠️ Manual Review Required
- **Confidence**: 75%
- **Example**:
  ```python
  # Before
  cipher = DES.new(key, DES.MODE_ECB)

  # After
  cipher = AES.new(key, AES.MODE_GCM)
  ```
- **Note**: Requires proper key management and authenticated encryption mode (GCM)

## API Usage

### Rust

```rust
use rust_wasm_app::{analyze, generate_remediations};

fn main() {
    let source = r#"
    import hashlib
    hash = hashlib.md5(data)
    "#;

    // Run audit
    let audit_result = analyze(source, "python").unwrap();

    // Generate remediations
    let remediation = generate_remediations(&audit_result, "crypto.py");

    // Check summary
    println!("Total vulnerabilities: {}", remediation.summary.total_vulnerabilities);
    println!("Auto-fixable: {}", remediation.summary.auto_fixable);
    println!("Manual review: {}", remediation.summary.manual_review_required);
    println!("Avg confidence: {:.1}%", remediation.summary.average_confidence * 100.0);

    // Process fixes
    for fix in &remediation.fixes {
        println!("\nFix: {}", fix.algorithm);
        println!("  File: {}:{}", fix.file_path, fix.line);
        println!("  Old: {}", fix.old_code);
        println!("  New: {}", fix.new_code);
        println!("  Confidence: {:.0}%", fix.confidence * 100.0);
        println!("  Auto-apply: {}", fix.auto_applicable);
        println!("  Explanation: {}", fix.explanation);
    }
}
```

### WASM/JavaScript

```javascript
import { generate_remediation } from './pkg/rust_wasm_app.js';

const source = `
import hashlib
hash = hashlib.md5(data)
`;

const remediation = generate_remediation(source, 'python', 'crypto.py');

console.log(`Total vulnerabilities: ${remediation.summary.total_vulnerabilities}`);
console.log(`Auto-fixable: ${remediation.summary.auto_fixable}`);

for (const fix of remediation.fixes) {
    console.log(`\nFix: ${fix.algorithm}`);
    console.log(`  ${fix.old_code} → ${fix.new_code}`);
    console.log(`  Confidence: ${(fix.confidence * 100).toFixed(0)}%`);
    if (fix.auto_applicable) {
        console.log('  ✅ Safe to auto-apply');
    } else {
        console.log('  ⚠️  Manual review required');
    }
}
```

## Data Structures

### CodeFix

```rust
pub struct CodeFix {
    pub file_path: String,        // File containing the vulnerability
    pub line: usize,               // Line number
    pub column: usize,             // Column number
    pub old_code: String,          // Original vulnerable code
    pub new_code: String,          // Suggested replacement
    pub confidence: f32,           // Confidence score (0.0-1.0)
    pub algorithm: String,         // Algorithm transformation
    pub explanation: String,       // Detailed explanation
    pub auto_applicable: bool,     // Can be auto-applied?
}
```

### RemediationResult

```rust
pub struct RemediationResult {
    pub fixes: Vec<CodeFix>,           // List of fixes
    pub summary: RemediationSummary,   // Summary statistics
    pub warnings: Vec<String>,         // Warnings/notes
}
```

### RemediationSummary

```rust
pub struct RemediationSummary {
    pub total_vulnerabilities: usize,      // Total analyzed
    pub auto_fixable: usize,               // Safe for auto-apply
    pub manual_review_required: usize,     // Needs manual review
    pub average_confidence: f32,           // Average confidence
}
```

## JSON Export

```json
{
  "fixes": [
    {
      "file_path": "crypto.py",
      "line": 3,
      "column": 7,
      "old_code": "hashlib.md5(data)",
      "new_code": "hashlib.sha256(data)",
      "confidence": 0.85,
      "algorithm": "MD5 → SHA-256",
      "explanation": "Replaced deprecated MD5 hash with SHA-256. For cryptographic security, consider using SHA-3 or BLAKE2.",
      "auto_applicable": true
    }
  ],
  "summary": {
    "total_vulnerabilities": 1,
    "auto_fixable": 1,
    "manual_review_required": 0,
    "average_confidence": 0.85
  },
  "warnings": []
}
```

## Examples

See `/examples/remediation_example.rs` for comprehensive examples including:
- MD5 hash remediation
- SHA-1 remediation
- RSA weak key upgrades
- DES/3DES cipher migration
- Multiple vulnerability handling
- JSON export

Run the example:
```bash
cargo run --example remediation_example
```

## Testing

The remediation module includes comprehensive tests:

```bash
# Run unit tests
cargo test --lib remediation

# Run integration tests
cargo test --test remediation_test

# Run all tests
cargo test
```

**Test Coverage**: 21 tests covering all remediation types and edge cases.

## Limitations

### Unsupported Algorithms

The following algorithms do not yet have automatic remediations:
- ECDSA/ECDH
- DSA
- Diffie-Hellman
- RC4

For these, the module returns warnings:
```
"No automatic remediation available for ECDSA at line 42"
```

### Manual Review Required

Some remediations require manual review because:
1. **Quantum Vulnerability**: RSA is fundamentally vulnerable to quantum attacks
2. **Configuration Changes**: AES requires proper mode (GCM) and key management
3. **Architectural Changes**: PQC migration may require API changes
4. **Testing Required**: All crypto changes should be thoroughly tested

## Best Practices

1. **Always Review**: Even auto-applicable fixes should be reviewed
2. **Test Thoroughly**: Run comprehensive tests after applying fixes
3. **Plan Migration**: For RSA/ECDSA, plan post-quantum migration
4. **Update Dependencies**: Ensure crypto libraries support modern algorithms
5. **Follow Standards**: Align with NIST 800-53 SC-13 requirements
6. **Document Changes**: Maintain audit trail of all changes

## Future Enhancements

Planned features:
- [ ] ECDSA/ECDH → PQC migration templates
- [ ] RC4 → ChaCha20 remediation
- [ ] Multi-line code transformations
- [ ] Import statement updates
- [ ] Dependency version recommendations
- [ ] Git commit generation with fixes
- [ ] Interactive remediation mode

## Contributing

To add support for new remediation types:

1. Add remediation function in `src/remediation.rs`
2. Add test cases in `src/remediation.rs` and `tests/remediation_test.rs`
3. Update documentation
4. Ensure WASM compatibility

See existing remediation functions for examples:
- `remediate_md5()`
- `remediate_sha1()`
- `remediate_rsa()`
- `remediate_des_3des()`

## References

- [NIST 800-53 SC-13](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-53r5.pdf)
- [NIST PQC Standardization](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [CRYSTALS-Kyber](https://pq-crystals.org/kyber/)
- [CRYSTALS-Dilithium](https://pq-crystals.org/dilithium/)
