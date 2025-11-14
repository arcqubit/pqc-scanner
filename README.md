# Rust WASM Quantum-Safe Crypto Auditor

[![CI](https://github.com/arcqubit/pdq-scanner/actions/workflows/ci.yml/badge.svg)](https://github.com/arcqubit/pdq-scanner/actions/workflows/ci.yml)
[![Security Audit](https://github.com/arcqubit/pdq-scanner/actions/workflows/cargo-audit.yml/badge.svg)](https://github.com/arcqubit/pdq-scanner/actions/workflows/cargo-audit.yml)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/arcqubit/pdq-scanner/badge)](https://securityscorecards.dev/viewer/?uri=github.com/arcqubit/pdq-scanner)
[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/11462/badge)](https://www.bestpractices.dev/projects/11462)

A high-performance Rust-based auditor for detecting quantum-vulnerable cryptographic algorithms in source code, compiled to WebAssembly for multi-platform deployment.

## Features

- **Multi-language Support**: Rust, JavaScript, TypeScript, Python, Java, Go, C++, C#
- **10 Crypto Detection Patterns**: RSA, ECDSA, ECDH, DSA, DH, MD5, SHA-1, DES, 3DES, RC4
- **NIST 800-53 SC-13 Compliance Reports**: Automated assessment reports with data-driven evidence
- **OSCAL JSON Output**: Machine-readable compliance reports in OSCAL 1.1.2 format
- **WASM Compilation**: <500KB gzipped, runs in browser/Node.js/Deno
- **High Performance**: 28x faster than target (0.35ms for 1000 LOC)
- **Comprehensive Testing**: 19 tests passing, >90% code coverage
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

## Project Structure

```
rust-wasm-app/
├── src/
│   ├── lib.rs          # WASM entry point & public API
│   ├── types.rs        # Shared types, OSCAL schemas, and errors
│   ├── audit.rs        # Core audit logic
│   ├── compliance.rs   # NIST 800-53 SC-13 & OSCAL reporting
│   ├── remediation.rs  # Auto-remediation engine
│   ├── parser.rs       # Multi-language parsing
│   └── detector.rs     # Pattern detection
├── tests/
│   ├── integration_tests.rs
│   ├── remediation_test.rs
│   └── fixtures/       # Test files
├── examples/
│   ├── generate_compliance_report.rs
│   └── remediation_example.rs
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

The scanner includes **intelligent auto-remediation** with template-based code fixes:

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

## Model Context Protocol (MCP) Integration

The PQC Scanner includes an MCP server for AI assistant integration:

```bash
# Setup MCP server
cd mcp
npm install

# Configure Claude Desktop
# Add to ~/Library/Application Support/Claude/claude_desktop_config.json:
{
  "mcpServers": {
    "pqc-scanner": {
      "command": "node",
      "args": ["/path/to/pqc-scanner/mcp/src/index.js"]
    }
  }
}
```

**Available Tools:**
- `scan_code` - Scan directories for crypto vulnerabilities
- `analyze_file` - Analyze single source files
- `get_remediation` - Get migration recommendations
- `validate_compliance` - Validate NIST 800-53 SC-13 compliance

See [mcp/README.md](mcp/README.md) for complete documentation.

## Phase 1 Status: ✅ COMPLETE

- ✅ Rust core implementation
- ✅ Multi-language parser (8 languages)
- ✅ Pattern detector (10 crypto algorithms)
- ✅ NIST 800-53 SC-13 compliance reporting
- ✅ OSCAL 1.1.2 JSON output
- ✅ 19 tests passing (100% success rate)
- ✅ Performance benchmarks (all targets met)

## Phase 2 Status: ✅ COMPLETE

- ✅ Agent-booster integration for auto-remediation
- ✅ Intelligent code transformation engine
- ✅ Multi-language fix generation
- ✅ Sample vulnerable repositories
- ✅ Verification and validation workflow
- ✅ Before/after comparison reports
- ✅ Context-aware suggestions

## Phase 3 Status: ✅ COMPLETE

- ✅ NPM package publishing (`@arcqubit/pdq-scanner`)
- ✅ GitHub release automation workflow
- ✅ Distroless container image (ghcr.io/arcqubit/pdq-scanner)
- ✅ GitHub Action for CI/CD integration
- ✅ Complete deployment ecosystem

See [docs/PHASE_3_DEPLOYMENT.md](docs/PHASE_3_DEPLOYMENT.md) for details.

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

See [docs/PHASE_3_DEPLOYMENT.md](docs/PHASE_3_DEPLOYMENT.md) for complete deployment guide.

## License

MIT
