# PQC Scanner - Quantum-Safe Crypto Auditor

[![CI](https://github.com/arcqubit/pqc-scanner/actions/workflows/ci.yml/badge.svg)](https://github.com/arcqubit/pqc-scanner/actions/workflows/ci.yml)
[![Security Audit](https://github.com/arcqubit/pqc-scanner/actions/workflows/cargo-audit.yml/badge.svg)](https://github.com/arcqubit/pqc-scanner/actions/workflows/cargo-audit.yml)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/arcqubit/pqc-scanner/badge)](https://securityscorecards.dev/viewer/?uri=github.com/arcqubit/pqc-scanner)
[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/11462/badge)](https://www.bestpractices.dev/projects/11462)
[![CalVer](https://img.shields.io/badge/calver-2025.11.0--beta.1-22bfda.svg)](docs/CALVER.md)

A high-performance Rust-based auditor for detecting quantum-vulnerable cryptographic algorithms in source code, compiled to WebAssembly for multi-platform deployment.

**Version**: 2025.11.0-beta.1 ([CalVer](docs/CALVER.md))

## Features

- **Multi-language Support**: Rust, JavaScript, TypeScript, Python, Java, Go, C++, C#
- **10 Crypto Detection Patterns**: RSA, ECDSA, ECDH, DSA, DH, MD5, SHA-1, DES, 3DES, RC4
- **NIST 800-53 SC-13 Compliance Reports**: Automated assessment reports with data-driven evidence
- **Canadian CCCS/CSE Compliance**: ITSG-33 SC-13, ITSP.40.111, ITSP.40.062, and CMVP validation
- **Unified Compliance Reporting**: Combined NIST + Canadian compliance assessment
- **Security Classification Support**: Unclassified, Protected A/B/C with classification-specific requirements
- **OSCAL JSON Output**: Machine-readable compliance reports in OSCAL 1.1.2 format
- **WASM Compilation**: <500KB gzipped, runs in browser/Node.js/Deno
- **High Performance**: 28x faster than target (0.35ms for 1000 LOC)
- **Comprehensive Testing**: 62 tests passing, >90% code coverage
## Quick Start

```bash
# Install dependencies
make install

# Run tests
make test
# or: cargo test

# Build everything (Rust + WASM)
make build
# or: cargo build

# Build WASM packages
make wasm
# or: npm run build

# Run example
make example
# or: cargo run --example generate_compliance_report
```

### Development Scripts

The project includes comprehensive build and automation scripts:

**Makefile Targets:**
```bash
make build          # Build debug version
make test           # Run all tests
make wasm           # Build WASM (all targets)
make wasm-release   # Build optimized WASM
make lint           # Run clippy
make format         # Format code
make bench          # Run benchmarks
make example        # Run compliance report example
make clean          # Remove build artifacts
make help           # Show all targets
```

**Build Scripts:**
```bash
./scripts/build.sh [--release]    # Build Rust + WASM
./scripts/test.sh                 # Run full test suite
./scripts/release.sh <version>    # Prepare release
```

**NPM Scripts:**
```bash
npm run build           # Build all WASM targets
npm run build:bundler   # Build for bundler
npm run build:nodejs    # Build for Node.js
npm run build:web       # Build for web
npm test                # Run all tests
npm run clean           # Clean build artifacts
```
```

## Usage

```rust
use rust_wasm_app::{analyze, Language};

let source = r#"
    const rsa = crypto.generateKeyPairSync('rsa', { modulusLength: 2048 });
"#;

let result = analyze(source, "javascript").unwrap();

for vuln in result.vulnerabilities {
    println!("{}: {} at line {}", vuln.severity, vuln.message, vuln.line);
    println!("Recommendation: {}", vuln.recommendation);
}
```

## NIST 800-53 SC-13 Compliance Reporting

The auditor automatically generates NIST 800-53 SC-13 (Cryptographic Protection) compliance reports with data-driven evidence:

```rust
use rust_wasm_app::{analyze, generate_sc13_report, generate_oscal_json, export_sc13_json, export_oscal_json};

let source = r#"
    const rsa = crypto.generateKeyPairSync('rsa', { modulusLength: 2048 });
    const hash = crypto.createHash('md5');
"#;

// 1. Perform audit
let audit_result = analyze(source, "javascript").unwrap();

// 2. Generate NIST 800-53 SC-13 Assessment Report
let sc13_report = generate_sc13_report(&audit_result, Some("example.js"));

println!("Control Assessment: {:?}", sc13_report.control_assessment.assessment_status);
println!("Compliance Score: {}/100", sc13_report.summary.compliance_score);
println!("Total Findings: {}", sc13_report.findings.len());

// 3. Export as JSON
let json_report = export_sc13_json(&sc13_report).unwrap();
std::fs::write("sc13-report.json", json_report).unwrap();

// 4. Generate OSCAL-compliant Assessment Results
let oscal = generate_oscal_json(&sc13_report, Some("example.js"));
let oscal_json = export_oscal_json(&oscal).unwrap();
std::fs::write("templates/oscal-assessment-results.json", oscal_json).unwrap();
```

### SC-13 Report Structure

The assessment report includes:

- **Report Metadata**: Unique ID, timestamps, version info
- **Control Assessment**: Implementation status, assessment status, methods used
- **Summary Statistics**:
  - Lines scanned
  - Vulnerabilities found
  - Quantum-vulnerable algorithms detected
  - Deprecated algorithms detected
  - Weak key sizes
  - Compliance score (0-100)
  - Risk score (0-100)
- **Detailed Findings**: Per-crypto-type findings with:
  - Finding ID and control mapping
  - Implementation and assessment status
  - Description and risk level
  - Evidence collection with source locations
  - Remediation recommendations
- **Compliance Recommendations**: Actionable guidance aligned with NIST standards

### OSCAL JSON Output

The OSCAL (Open Security Controls Assessment Language) output conforms to NIST OSCAL 1.1.2 specification:

- Machine-readable assessment results
- Structured observations with evidence
- Findings mapped to SC-13 control objectives
- Compatible with OSCAL-based compliance tools
- Supports System Security Plan (SSP) integration

### Example WASM Usage

```javascript
import { generate_compliance_report, generate_oscal_report } from './pkg/rust_wasm_app.js';

const source = `
    const rsa = crypto.generateKeyPairSync('rsa', { modulusLength: 1024 });
`;

// Generate SC-13 compliance report
const sc13Report = generate_compliance_report(source, 'javascript', 'app.js');
console.log('Compliance Score:', sc13Report.summary.compliance_score);

// Generate OSCAL assessment results
const oscalReport = generate_oscal_report(source, 'javascript', 'app.js');
console.log('OSCAL Version:', oscalReport.oscal_version);
```

## Canadian CCCS/CSE Cryptographic Compliance (NEW in 2025.11.0-beta.1)

The scanner now provides comprehensive support for **Canadian Government cryptographic compliance standards**, enabling assessment against:

- **ITSG-33 SC-13**: Cryptographic Protection control (Canadian equivalent to NIST 800-53 SC-13)
- **ITSP.40.111**: Cryptographic Algorithms for UNCLASSIFIED, PROTECTED A, and PROTECTED B Information
- **ITSP.40.062**: Guidance on Securely Configuring Network Protocols
- **CMVP**: Cryptographic Module Validation Program (joint NIST/CCCS)

### Security Classification Support

The scanner supports all Canadian security classification levels with classification-specific cryptographic requirements:

| Classification | Min AES | Min RSA | Min ECC | Approved Hash | CMVP Required |
|----------------|---------|---------|---------|---------------|---------------|
| **Unclassified** | 128-bit | 2048-bit | 256-bit | SHA-256+ | No |
| **Protected A** | 128-bit | 2048-bit | 256-bit | SHA-256+ | Yes |
| **Protected B** | 256-bit | 3072-bit | 384-bit | SHA-384+ | Yes |
| **Protected C** | 256-bit | 4096-bit | 521-bit | SHA-512 | Yes |

### CCCS Algorithm Approval Status

The scanner validates algorithms against CCCS approval status per ITSP.40.111:

- **Approved**: AES (128/192/256), SHA-2 (256/384/512), SHA-3, HMAC
- **Conditionally Approved**: RSA (2048+), ECDSA, ECDH, DH - Quantum-vulnerable, sunset 2030
- **Deprecated**: 3DES (sunset 2023), DSA (sunset 2024)
- **Prohibited**: MD5, SHA-1, DES, RC4 - Immediate migration required
- **Under Review**: CRYSTALS-Kyber, CRYSTALS-Dilithium, SPHINCS+ (Post-Quantum)

### Canadian Compliance Usage

```rust
use pqc_scanner::{
    analyze,
    generate_itsg33_report,
    generate_unified_report,
    SecurityClassification,
    export_itsg33_json,
    export_unified_json
};

let source = r#"
    const hash = crypto.createHash('md5');
    const rsa = crypto.generateKeyPair('rsa', { modulusLength: 2048 });
"#;

// 1. Perform audit
let audit_result = analyze(source, "javascript").unwrap();

// 2. Generate ITSG-33 SC-13 Assessment Report for Protected A
let itsg33_report = generate_itsg33_report(
    &audit_result,
    SecurityClassification::ProtectedA,
    Some("example.js")
);

println!("ITSG-33 Control: {}", itsg33_report.control_assessment.control_id);
println!("Classification: {}", itsg33_report.control_assessment.security_classification);
println!("Compliance Score: {}/100", itsg33_report.summary.compliance_score);
println!("ITSP.40.111 Compliant: {}", itsg33_report.summary.itsp_40_111_compliant);
println!("CCCS Prohibited: {:?}", itsg33_report.summary.cccs_prohibited_algorithms);

// 3. Generate Unified NIST + Canadian Report
let unified_report = generate_unified_report(
    &audit_result,
    SecurityClassification::ProtectedB,
    Some("example.js")
);

println!("NIST Compliance: {}/100", unified_report.nist_summary.compliance_score);
println!("Canadian Compliance: {}/100", unified_report.canadian_summary.compliance_score);

// 4. Export to JSON
let json = export_itsg33_json(&itsg33_report).unwrap();
std::fs::write("itsg33-report.json", json).unwrap();

let unified_json = export_unified_json(&unified_report).unwrap();
std::fs::write("unified-compliance-report.json", unified_json).unwrap();
```

### WASM API for Canadian Compliance

```javascript
import init, {
    generate_itsg33_compliance_report,
    generate_unified_compliance_report
} from './pkg/pqc_scanner.js';

await init();

const source = `
    const hash = crypto.createHash('md5');
    const rsa = crypto.generateKeyPair('rsa', { modulusLength: 2048 });
`;

// Generate ITSG-33 report for Protected A
const itsg33Report = generate_itsg33_compliance_report(
    source,
    'javascript',
    'protected-a',
    'example.js'
);

console.log('ITSG-33 Compliance Score:', itsg33Report.summary.compliance_score);
console.log('ITSP.40.111 Compliant:', itsg33Report.summary.itsp_40_111_compliant);
console.log('Classification Compliant:', itsg33Report.summary.classification_compliant);
console.log('CCCS Prohibited Algorithms:', itsg33Report.summary.cccs_prohibited_algorithms);
console.log('CMVP Validated/Required:',
    `${itsg33Report.summary.cmvp_validated_count}/${itsg33Report.summary.cmvp_required_count}`);

// Generate unified NIST + Canadian report
const unifiedReport = generate_unified_compliance_report(
    source,
    'javascript',
    'protected-b',
    'example.js'
);

console.log('Control Mapping:', unifiedReport.control_mapping);
console.log('NIST Score:', unifiedReport.nist_summary.compliance_score);
console.log('Canadian Score:', unifiedReport.canadian_summary.compliance_score);
```

### ITSG-33 Report Structure

The ITSG-33 assessment report provides comprehensive Canadian compliance data:

- **Control Assessment**: ITSG-33 SC-13 implementation and assessment status
- **Security Classification**: Unclassified, Protected A/B/C with classification-specific validation
- **Canadian Summary**:
  - CCCS algorithm approval status (Approved/Deprecated/Prohibited)
  - CMVP validation tracking (validated vs. required count)
  - ITSP.40.111 and ITSP.40.062 compliance flags
  - Classification-specific compliance assessment
  - Quantum-vulnerable algorithm inventory
- **Canadian Findings**: Enhanced findings with:
  - CCCS approval status per algorithm
  - ITSP reference citations (e.g., "ITSP.40.111 Section 5.3")
  - Applicable security classifications
  - CMVP certificate validation
- **Recommendations**: Canadian-specific guidance with ITSP references

### Unified Compliance Report

The unified report combines both NIST 800-53 and ITSG-33 assessments:

- **Dual Framework Assessment**: Side-by-side NIST and Canadian compliance
- **Control Cross-Mapping**: SC-13 ↔ ITSG-33 SC-13 equivalence documentation
- **Consolidated Recommendations**: Unified guidance satisfying both frameworks
- **Shared Evidence**: Single evidence collection mapped to both frameworks

### CMVP Certificate Database

The scanner includes an offline database of CMVP-validated cryptographic modules:

- **OpenSSL FIPS Object Module** (Cert #4282)
- **Microsoft Cryptographic Primitives** (Cert #3966)
- **Bouncy Castle BC-FJA** (Cert #4118)
- **AWS-LC** (Cert #4536)
- **wolfCrypt FIPS** (Cert #4407)
- And more...

### Compliance Scoring

Canadian compliance scoring (0-100) uses penalty-based assessment:

- **Prohibited algorithms**: -40 points each (MD5, SHA-1, DES, RC4)
- **Deprecated algorithms**: -20 points each (3DES, DSA)
- **Weak key sizes**: -15 points each (below classification minimum)
- **Quantum-vulnerable**: -10 points per unique algorithm type

**Example:**
```
Initial Score: 100
- MD5 detected (prohibited): -40
- RSA 2048-bit on Protected B (weak): -15
- RSA detected (quantum-vulnerable): -10
= Final Score: 35 (Non-compliant)
```

### Documentation

For comprehensive Canadian compliance documentation, see:

- **[docs/canadian-compliance.md](docs/canadian-compliance.md)**: Complete usage guide
- **[CANADIAN_COMPLIANCE_IMPLEMENTATION.md](CANADIAN_COMPLIANCE_IMPLEMENTATION.md)**: Technical implementation details
- **[examples/canadian_compliance_example.rs](examples/canadian_compliance_example.rs)**: Working code examples

### Running Canadian Compliance Examples

```bash
# Run Canadian compliance example
cargo run --example canadian_compliance_example

# Expected output:
# - ITSG-33 SC-13 Assessment (Protected A)
# - ITSG-33 SC-13 Assessment (Protected B)
# - Unified NIST + Canadian Compliance
# - CCCS algorithm approval status
# - CMVP validation tracking
# - Classification-specific compliance
```

## Project Structure

```
pqc-scanner/
├── src/
│   ├── lib.rs                  # WASM entry point & public API
│   ├── types.rs                # Shared types, OSCAL schemas, Canadian types
│   ├── audit.rs                # Core audit logic
│   ├── compliance.rs           # NIST 800-53 SC-13 & OSCAL reporting
│   ├── canadian_compliance.rs  # ITSG-33 SC-13 & unified reporting
│   ├── algorithm_database.rs   # CCCS algorithm & CMVP validation
│   ├── remediation.rs          # Auto-remediation engine
│   ├── parser.rs               # Multi-language parsing
│   └── detector.rs             # Pattern detection
├── data/
│   ├── cccs_algorithms.json    # CCCS algorithm approval database
│   └── cmvp_certificates.json  # CMVP certificate database
├── tests/
│   ├── integration_tests.rs
│   ├── remediation_test.rs
│   └── fixtures/               # Test files
├── examples/
│   ├── generate_compliance_report.rs
│   ├── canadian_compliance_example.rs
│   └── remediation_example.rs
├── docs/
│   ├── canadian-compliance.md  # Canadian compliance guide
│   └── CALVER.md              # CalVer versioning guide
├── benches/
│   └── benchmarks.rs
└── Cargo.toml
```

## Performance Metrics

- **Parse 1000 LOC**: 0.35ms (target: <10ms) ✅ 28x faster
- **Pattern detection**: 0.53ms (target: <5ms) ✅ 9.5x faster
- **Memory usage**: 42MB (target: <50MB) ✅
- **Throughput**: 6,296 files/sec ✅

## Auto-Remediation

The scanner may include in the future **intelligent auto-remediation** with template-based code fixes:

```rust
use rust_wasm_app::{analyze, generate_remediations};

let source = r#"
import hashlib
hash = hashlib.md5(data)
"#;

let audit = analyze(source, "python").unwrap();
let remediation = generate_remediations(&audit, "crypto.py");

println!("Auto-fixable: {}", remediation.summary.auto_fixable);
println!("Manual review: {}", remediation.summary.manual_review_required);

for fix in &remediation.fixes {
    println!("Fix: {}", fix.algorithm);
    println!("  Old: {}", fix.old_code);
    println!("  New: {}", fix.new_code);
    println!("  Confidence: {:.0}%", fix.confidence * 100.0);
    println!("  Auto-apply: {}", fix.auto_applicable);
}
```

### Supported Remediations

| Vulnerability | Remediation | Auto-Apply | Confidence |
|---------------|-------------|------------|------------|
| MD5 | SHA-256 | ✅ Yes | 85% |
| SHA-1 | SHA-256 | ✅ Yes | 90% |
| RSA-1024 | RSA-2048 (interim) | ⚠️ Manual | 70% |
| RSA-2048+ | PQC migration warning | ⚠️ Manual | 50% |
| DES/3DES | AES-256-GCM | ⚠️ Manual | 75% |

### Remediation Output

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
    "total_vulnerabilities": 3,
    "auto_fixable": 2,
    "manual_review_required": 1,
    "average_confidence": 0.78
  },
  "warnings": []
}
```

### Run Examples

```bash
# Run remediation example
cargo run --example remediation_example

# Run with WASM
import { generate_remediation } from './pkg/rust_wasm_app.js';
const fixes = generate_remediation(source, 'python', 'crypto.py');
```

## Sample Repositories

Test the scanner against real-world vulnerable codebases:

```bash
# Clone sample repositories
git clone https://github.com/arcqubit/vulnerable-crypto-examples.git samples/

# Run automated scans
make scan-samples
```

**Available Sample Repos:**
- **legacy-banking**: Financial app with RSA-1024 and MD5
- **crypto-messenger**: Chat app with weak ECDH curves
- **old-web-framework**: Framework using SHA-1 and 3DES

See [docs/SAMPLE_REPOSITORIES.md](docs/SAMPLE_REPOSITORIES.md) for details.

## Installation & Distribution

### NPM Package

```bash
# Install globally
npm install -g @arcqubit/pdq-scanner

# Or as project dependency
npm install @arcqubit/pdq-scanner
```

### Docker Container

```bash
# Pull latest image
docker pull ghcr.io/arcqubit/pdq-scanner:latest

# Run scan
docker run --rm -v $(pwd):/data ghcr.io/arcqubit/pdq-scanner:latest
```

### GitHub Action

```yaml
name: Security Scan
on: [push, pull_request]
jobs:
  pdq-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: arcqubit/pdq-scanner@v1
        with:
          path: 'src/'
          fail-on-findings: true
          severity-threshold: 'high'
```


## License

MIT
