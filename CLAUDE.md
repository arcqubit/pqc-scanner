# PQC Scanner - Claude Code Configuration

## 🔐 Project Overview

**PQC Scanner** is a quantum-safe cryptography auditor that detects vulnerable cryptographic algorithms and generates NIST compliance reports. Built in Rust with WASM support for cross-platform deployment.

**Key Capabilities:**
- Detect quantum-vulnerable algorithms (RSA, ECDSA, DSA, DH)
- Identify deprecated/weak crypto (MD5, SHA-1, DES, RC4, 3DES)
- Multi-language support (JavaScript, TypeScript, Python, Rust, Java, Go, C++, C#)
- Generate NIST 800-53 SC-13 compliance reports
- Export OSCAL 1.1.2 assessment results
- CLI tool for directory and remote repository scanning

## 🚨 CRITICAL: CONCURRENT EXECUTION & FILE MANAGEMENT

**ABSOLUTE RULES**:
1. ALL operations MUST be concurrent/parallel in a single message
2. **NEVER save working files, text/mds and tests to the root folder**
3. ALWAYS organize files in appropriate subdirectories
4. **USE CLAUDE CODE'S TASK TOOL** for spawning agents concurrently

### ⚡ GOLDEN RULE: "1 MESSAGE = ALL RELATED OPERATIONS"

**MANDATORY PATTERNS:**
- **TodoWrite**: ALWAYS batch ALL todos in ONE call (5-10+ todos minimum)
- **Task tool**: ALWAYS spawn ALL agents in ONE message with full instructions
- **File operations**: ALWAYS batch ALL reads/writes/edits in ONE message
- **Bash commands**: ALWAYS batch ALL terminal operations in ONE message

### 📁 Project File Organization

**NEVER save to root folder. Use these directories:**
- `/src` - Rust source code (lib.rs, modules)
- `/src/bin` - CLI binary implementations
- `/tests` - Integration and unit tests
- `/examples` - Example usage code
- `/docs` - Project documentation (when explicitly needed)
- `/reports` - Generated compliance reports (gitignored)
- `/samples` - Test samples for vulnerability detection
- `/benches` - Performance benchmarks

## 🛠️ Development Commands

### Rust Build Commands
```bash
# Development
make build              # Debug build
cargo build             # Direct cargo build

# Release (optimized)
make build-release      # Production build
cargo build --release   # Binary at: ./target/release/pqc-scanner

# Testing
make test              # Run all tests
make test-integration  # Integration tests only
cargo test --verbose   # Detailed test output

# Quality
make lint              # Run clippy linter
make format            # Auto-format code
make format-check      # Check formatting without changes
make bench             # Run benchmarks

# WASM
make wasm              # Build WASM (debug)
make wasm-release      # Build WASM (optimized)
make wasm-size         # Check WASM bundle sizes

# Cleanup
make clean             # Remove build artifacts

# Full workflow
make dev               # format + lint + test
make ci                # lint + format-check + test + bench
make release           # build-release + wasm-release
```

### CLI Usage
```bash
# Scan local directory
./target/release/pqc-scanner scan <directory> [OPTIONS]

# Scan remote repository
./target/release/pqc-scanner scan <repo-url> [OPTIONS]

# Options
--report-dir <dir>     # Output directory (default: reports)
--report-name <name>   # Base name for reports (default: dir/repo name)
--keep-clone           # Keep cloned repo after scan

# Examples
./target/release/pqc-scanner scan samples/vulnerable-app-1
./target/release/pqc-scanner scan https://github.com/user/repo.git
./target/release/pqc-scanner scan myapp --report-name security-audit
```

## 🔍 Cryptographic Scanning Domain Knowledge

### Vulnerability Detection Patterns

**Quantum-Vulnerable Algorithms:**
- RSA (all key sizes vulnerable to Shor's algorithm)
- ECDSA/ECDH (elliptic curve crypto)
- DSA (discrete logarithm problem)
- Diffie-Hellman key exchange

**Deprecated/Weak Algorithms:**
- MD5 (cryptographically broken, collision attacks)
- SHA-1 (deprecated, collision attacks demonstrated)
- DES (56-bit keys, too weak)
- 3DES (deprecated, 112-bit effective security)
- RC4 (broken stream cipher)

**Weak Key Sizes:**
- RSA < 2048 bits (CRITICAL)
- RSA < 4096 bits (HIGH)
- ECDSA curves < 256 bits

**Post-Quantum Safe Alternatives:**
- CRYSTALS-Kyber (key encapsulation)
- CRYSTALS-Dilithium (digital signatures)
- FALCON (compact signatures)
- NTRU (lattice-based)

### NIST Compliance Standards

**NIST 800-53 SC-13: Cryptographic Protection**
- Ensures FIPS-validated cryptography
- Prohibits deprecated algorithms
- Requires adequate key sizes
- Mandates proper key management

**OSCAL 1.1.2 Format:**
- Machine-readable security assessments
- Standardized compliance reporting
- Interoperable with GRC tools

## 🏗️ Rust/WASM Architecture

### Module Structure
```rust
src/
├── lib.rs              # Public API, WASM exports
├── audit.rs            # Core vulnerability scanning
├── detector.rs         # Pattern detection engine
├── parser.rs           # Multi-language AST parsing
├── types.rs            # Type definitions
├── compliance.rs       # NIST SC-13, OSCAL generation
├── remediation.rs      # Fix recommendations
└── bin/
    └── pqc-scanner.rs  # CLI implementation
```

### Code Style Guidelines

**Rust Best Practices:**
```rust
// ✅ GOOD: Clear error types, descriptive names
pub fn analyze(source: &str, language: &str) -> Result<AuditResult, AuditError> {
    let parsed = parse_source(source, language)?;
    let vulnerabilities = detect_vulnerabilities(&parsed)?;
    Ok(AuditResult::new(vulnerabilities))
}

// ✅ GOOD: Use proper Result propagation
fn detect_rsa_usage(node: &AstNode) -> Result<Option<Vulnerability>, DetectorError> {
    // Detection logic
}

// ❌ BAD: Unwrap in library code
let result = analyze(source, lang).unwrap(); // Never do this!

// ❌ BAD: Hardcoded values
const API_KEY = "sk-1234567890"; // Use environment variables!
```

**WASM Compatibility:**
```rust
// ✅ GOOD: Guard WASM-specific code
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn audit_code(source: &str, language: &str) -> Result<JsValue, JsValue> {
    // WASM-specific implementation
}

// ✅ GOOD: Share core logic
pub fn analyze(source: &str, language: &str) -> Result<AuditResult, AuditError> {
    // Shared logic works in both Rust and WASM
}
```

**Security Considerations:**
```rust
// ✅ GOOD: Validate inputs
pub fn scan_file(path: &Path) -> Result<AuditResult, ScanError> {
    if !path.exists() {
        return Err(ScanError::FileNotFound(path.to_owned()));
    }
    // Proceed with scan
}

// ✅ GOOD: Never log sensitive data
eprintln!("Scanning file: {}", path.display()); // OK
eprintln!("API key: {}", api_key); // ❌ NEVER!

// ✅ GOOD: Sanitize paths
let safe_path = path.canonicalize()?; // Prevent path traversal
```

## 🧪 Testing Guidelines

### Test Organization
```rust
// Unit tests in same file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_md5_usage() {
        let source = "crypto.createHash('md5')";
        let result = analyze(source, "javascript").unwrap();
        assert!(!result.vulnerabilities.is_empty());
        assert_eq!(result.vulnerabilities[0].crypto_type, CryptoType::Md5);
    }
}

// Integration tests in /tests directory
// tests/integration_tests.rs
#[test]
fn test_scan_vulnerable_app() {
    let result = scan_directory("samples/vulnerable-app-1").unwrap();
    assert!(result.stats.critical_count > 0);
}
```

### Benchmark Tests
```rust
// benches/benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_scan(c: &mut Criterion) {
    c.bench_function("scan javascript", |b| {
        b.iter(|| {
            analyze(black_box(SAMPLE_CODE), "javascript")
        });
    });
}
```

## 🚀 Development Workflow

### Adding New Language Support

1. Update `Language` enum in `src/types.rs`
2. Add parser patterns in `src/parser.rs`
3. Add detector rules in `src/detector.rs`
4. Add file extension mapping in `src/bin/pqc-scanner.rs`
5. Add tests in `tests/` directory
6. Update documentation

### Adding New Vulnerability Detection

1. Add `CryptoType` variant in `src/types.rs`
2. Add detection regex in `src/detector.rs`
3. Define severity and risk score
4. Add recommendation text
5. Add test cases
6. Update expected counts in samples

### Generating Compliance Reports

```rust
// 1. Scan code
let audit_result = analyze(source, "javascript")?;

// 2. Generate SC-13 report
let sc13_report = generate_sc13_report(&audit_result, Some("path/to/file"));

// 3. Generate OSCAL
let oscal = generate_oscal_json(&sc13_report, Some("path/to/file"));

// 4. Export as JSON
let sc13_json = export_sc13_json(&sc13_report)?;
let oscal_json = export_oscal_json(&oscal)?;

// 5. Save reports
fs::write("reports/sc13-compliance.json", sc13_json)?;
fs::write("reports/oscal-assessment.json", oscal_json)?;
```

## 🎯 Agent Task Suggestions

### Security Audit Tasks
```javascript
// Research quantum-safe migration
Task("Crypto Researcher", "Research latest NIST PQC standards and implementation recommendations", "researcher")

// Code analysis
Task("Security Analyzer", "Analyze src/detector.rs for completeness of vulnerability patterns", "code-analyzer")

// Testing
Task("Security Tester", "Create comprehensive test suite for all crypto types and severities", "tester")

// Performance
Task("Performance Analyst", "Benchmark scanning speed on large codebases, optimize hot paths", "perf-analyzer")
```

### Feature Development
```javascript
// Add new language support
Task("Language Parser Dev", "Implement PHP parser with regex patterns for crypto API detection", "coder")
Task("Integration Tester", "Test PHP scanning against WordPress and Laravel codebases", "tester")

// Enhance reporting
Task("Report Developer", "Add HTML report generation with charts and recommendations", "coder")
Task("API Documentation", "Generate OpenAPI spec for WASM/REST API endpoints", "api-docs")
```

## 📊 Performance Targets

**Scanning Speed:**
- Small project (< 100 files): < 1 second
- Medium project (< 1000 files): < 10 seconds
- Large project (> 10000 files): < 2 minutes

**Memory Usage:**
- CLI tool: < 50 MB
- WASM bundle: < 500 KB (gzipped)

**Accuracy:**
- False positives: < 5%
- False negatives: < 1% (critical vulnerabilities)
- Detection coverage: > 95% (OWASP Crypto Risks)

## 🔗 Related Resources

- **NIST PQC**: https://csrc.nist.gov/projects/post-quantum-cryptography
- **NIST 800-53**: https://csrc.nist.gov/publications/detail/sp/800-53/rev-5/final
- **OSCAL**: https://pages.nist.gov/OSCAL/
- **Rust Security**: https://rustsec.org/
- **WASM Security**: https://webassembly.org/docs/security/

## 📋 important-instruction-reminders

Do what has been asked; nothing more, nothing less.
NEVER create files unless they're absolutely necessary for achieving your goal.
ALWAYS prefer editing an existing file to creating a new one.
NEVER proactively create documentation files (*.md) or README files. Only create documentation files if explicitly requested by the User.
Never save working files, text/mds and tests to the root folder.

---

**Remember:** Focus on security, accuracy, and NIST compliance in all development work.
