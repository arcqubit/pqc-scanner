# Testing Strategy - Quantum Crypto Audit Library

**Target**: 90%+ code coverage, zero regressions
**Approach**: Test-Driven Development (TDD) + Property-Based Testing
**Tools**: cargo test, wasm-pack test, criterion, proptest

---

## 1. Testing Pyramid

```
                 ▲
                ╱ ╲
               ╱   ╲
              ╱ E2E ╲           5% - End-to-End (full workflow)
             ╱───────╲
            ╱         ╲
           ╱Integration╲        20% - Integration (module interactions)
          ╱─────────────╲
         ╱               ╲
        ╱   Unit Tests    ╲     75% - Unit (individual functions)
       ╱___________________╲
```

### 1.1 Test Distribution Goals

| Test Type | Count | Coverage | Execution Time |
|-----------|-------|----------|----------------|
| Unit | 200+ | 90%+ lines | < 5s |
| Integration | 50+ | Critical paths | < 30s |
| E2E | 20+ | User workflows | < 60s |
| Property | 30+ | Edge cases | < 120s |

---

## 2. Unit Testing Strategy

### 2.1 Per-Module Test Organization

```
src/
  lib.rs
  audit.rs
  parser.rs
  detector.rs
  types.rs
tests/
  unit/
    audit_test.rs
    parser_test.rs
    detector_test.rs
    types_test.rs
  integration/
    full_audit_test.rs
    multi_file_test.rs
  fixtures/
    test_files/
      rsa_example.py
      ecc_example.js
      ...
```

### 2.2 Unit Test Template

```rust
// src/parser.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_python_by_extension() {
        let lang = Parser::detect_language("test.py");
        assert_eq!(lang, Language::Python);
    }

    #[test]
    fn test_detect_javascript_by_extension() {
        let lang = Parser::detect_language("app.js");
        assert_eq!(lang, Language::JavaScript);
    }

    #[test]
    fn test_parse_python_imports() {
        let content = "import rsa\nfrom cryptography import hazmat";
        let parser = Parser::new(Language::Python, ParserOptions::default());
        let result = parser.parse(content, "test.py").unwrap();

        assert_eq!(result.imports.len(), 2);
        assert_eq!(result.imports[0].module, "rsa");
        assert_eq!(result.imports[1].module, "cryptography");
    }

    #[test]
    fn test_strip_comments_python() {
        let content = "# Comment\nimport rsa  # Inline comment\nkey = 42";
        let mut options = ParserOptions::default();
        options.strip_comments = true;

        let parser = Parser::new(Language::Python, options);
        let result = parser.parse(content, "test.py").unwrap();

        // Comments should be marked
        assert!(result.lines[0].is_comment);
        assert!(!result.lines[1].is_comment);
    }

    #[test]
    fn test_detect_functions_python() {
        let content = r#"
def generate_key():
    return rsa.generate(2048)

class CryptoManager:
    def __init__(self):
        pass
        "#;

        let parser = Parser::new(Language::Python, ParserOptions::default());
        let result = parser.parse(content, "test.py").unwrap();

        assert_eq!(result.functions.len(), 2); // generate_key, __init__
        assert_eq!(result.functions[0].name, "generate_key");
    }

    #[test]
    fn test_parse_empty_file() {
        let parser = Parser::new(Language::Python, ParserOptions::default());
        let result = parser.parse("", "empty.py");

        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn test_parse_invalid_utf8() {
        let invalid_bytes = vec![0xFF, 0xFE, 0xFD];
        let invalid_str = String::from_utf8_lossy(&invalid_bytes);

        let parser = Parser::new(Language::Python, ParserOptions::default());
        let result = parser.parse(&invalid_str, "invalid.py");

        // Should handle gracefully, not panic
        assert!(result.is_ok() || result.is_err());
    }
}
```

### 2.3 Unit Test Coverage Goals

**Parser Module (parser.rs)**:
- Language detection: 100%
- Import extraction: 90%
- Function detection: 85%
- Comment stripping: 95%
- Edge cases: 80%

**Detector Module (detector.rs)**:
- Pattern matching: 95%
- Confidence scoring: 90%
- Context validation: 85%
- False positive reduction: 80%

**Audit Module (audit.rs)**:
- Report generation: 95%
- Risk scoring: 100%
- Multi-file aggregation: 90%
- Configuration handling: 95%

---

## 3. Integration Testing

### 3.1 Full Audit Workflow Tests

```rust
// tests/integration/full_audit_test.rs

use quantum_crypto_audit::{AuditEngine, AuditConfig};

#[test]
fn test_full_audit_python_rsa() {
    let content = include_str!("../fixtures/test_files/rsa_example.py");
    let config = AuditConfig::default();
    let engine = AuditEngine::new(config).unwrap();

    let report = engine.audit_file(content, "rsa_example.py").unwrap();

    // Assert findings
    assert!(report.findings.len() > 0);

    // Assert RSA detection
    let rsa_findings: Vec<_> = report.findings.iter()
        .filter(|f| f.pattern_type == "RSA")
        .collect();
    assert_eq!(rsa_findings.len(), 1);
    assert_eq!(rsa_findings[0].severity, Severity::Critical);

    // Assert risk score
    assert!(report.risk_score.critical_count >= 1);
    assert!(matches!(report.risk_score.risk_level, RiskLevel::High | RiskLevel::Catastrophic));
}

#[test]
fn test_multi_file_audit() {
    let files = vec![
        FileInput {
            path: "crypto.py".to_string(),
            content: "import rsa\nkey = rsa.generate(2048)".to_string(),
        },
        FileInput {
            path: "auth.js".to_string(),
            content: "const crypto = require('crypto'); crypto.generateKeyPairSync('rsa', {})".to_string(),
        },
    ];

    let config = AuditConfig::default();
    let engine = AuditEngine::new(config).unwrap();

    let report = engine.audit_multiple(files).unwrap();

    // Should aggregate findings from both files
    assert!(report.findings.len() >= 2);

    // Check summary
    assert_eq!(report.summary.total_files, 2);
    assert!(report.summary.languages_detected.contains(&"Python".to_string()));
    assert!(report.summary.languages_detected.contains(&"JavaScript".to_string()));
}

#[test]
fn test_severity_filtering() {
    let content = r#"
import rsa  # Critical
import hashlib
md5 = hashlib.md5()  # High
sha256 = hashlib.sha256()  # Medium
    "#;

    let mut config = AuditConfig::default();
    config.severity_threshold = Severity::High;

    let engine = AuditEngine::new(config).unwrap();
    let report = engine.audit_file(content, "test.py").unwrap();

    // Should only include Critical and High findings
    for finding in &report.findings {
        assert!(matches!(finding.severity, Severity::Critical | Severity::High));
    }
}
```

### 3.2 Cross-Language Tests

```rust
#[test]
fn test_detect_rsa_across_languages() {
    let test_cases = vec![
        ("Python", "test.py", "rsa.generate_private_key(key_size=2048)"),
        ("JavaScript", "test.js", "RSA.generateKey(2048)"),
        ("TypeScript", "test.ts", "crypto.generateKeyPairSync('rsa', { modulusLength: 2048 })"),
        ("Go", "test.go", "rsa.GenerateKey(rand.Reader, 2048)"),
        ("Java", "test.java", "KeyPairGenerator.getInstance(\"RSA\")"),
        ("Rust", "test.rs", "RsaPrivateKey::new(&mut rng, 2048)"),
    ];

    let config = AuditConfig::default();
    let engine = AuditEngine::new(config).unwrap();

    for (lang, path, code) in test_cases {
        let report = engine.audit_file(code, path).unwrap();

        assert!(
            report.findings.len() > 0,
            "Failed to detect RSA in {}: {}",
            lang,
            code
        );

        let has_rsa = report.findings.iter().any(|f| f.pattern_type == "RSA");
        assert!(has_rsa, "No RSA pattern detected for {}", lang);
    }
}
```

---

## 4. WASM Integration Tests

### 4.1 WASM Test Setup

```rust
// tests/wasm_test.rs

use wasm_bindgen_test::*;
use quantum_crypto_audit::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_wasm_audit_file() {
    let content = "import rsa\nkey = rsa.generate(2048)";
    let config = "{}";

    let result = audit_file(content, "test.py", config);

    assert!(result.is_ok());
    let report: AuditReport = serde_wasm_bindgen::from_value(result.unwrap()).unwrap();
    assert!(report.findings.len() > 0);
}

#[wasm_bindgen_test]
fn test_wasm_get_version() {
    let version = get_version();
    assert!(version.starts_with("1."));
}

#[wasm_bindgen_test]
fn test_wasm_get_supported_patterns() {
    let patterns = get_supported_patterns();
    assert!(patterns.is_array());

    let patterns: Vec<PatternInfo> = serde_wasm_bindgen::from_value(patterns).unwrap();
    assert!(patterns.len() > 0);
}

#[wasm_bindgen_test]
fn test_wasm_validate_config() {
    let valid_config = r#"{"severity_threshold":"High"}"#;
    let result = validate_config(valid_config);
    assert!(result.is_ok());

    let invalid_config = r#"{"severity_threshold":"Invalid"}"#;
    let result = validate_config(invalid_config);
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_wasm_error_handling() {
    // Test with oversized file (exceeds max_file_size_kb)
    let huge_content = "x".repeat(20 * 1024 * 1024); // 20MB
    let config = r#"{"max_file_size_kb":10240}"#; // 10MB limit

    let result = audit_file(&huge_content, "huge.py", config);
    assert!(result.is_err());
}
```

### 4.2 Browser Testing

```bash
# Run tests in headless Chrome
wasm-pack test --headless --chrome

# Run tests in headless Firefox
wasm-pack test --headless --firefox

# Run tests in Node.js
wasm-pack test --node
```

---

## 5. Property-Based Testing

### 5.1 Using Proptest

```rust
// tests/property_test.rs

use proptest::prelude::*;
use quantum_crypto_audit::*;

proptest! {
    #[test]
    fn test_parser_never_panics(content in "\\PC*", path in "[a-z]+\\.(py|js|rs)") {
        let lang = Parser::detect_language(&path);
        let parser = Parser::new(lang, ParserOptions::default());

        // Should never panic, even with random input
        let _ = parser.parse(&content, &path);
    }

    #[test]
    fn test_risk_score_is_non_negative(
        critical in 0usize..100,
        high in 0usize..100,
        medium in 0usize..100,
        low in 0usize..100
    ) {
        let score = calculate_risk_score(critical, high, medium, low, 1000);
        assert!(score >= 0.0);
    }

    #[test]
    fn test_confidence_score_in_range(
        has_keyword in any::<bool>(),
        in_crypto_function in any::<bool>(),
        has_comment in any::<bool>()
    ) {
        let confidence = calculate_confidence(has_keyword, in_crypto_function, has_comment);
        assert!(confidence >= 0.0 && confidence <= 1.0);
    }
}

// Property: Parsing a file twice should yield identical results
proptest! {
    #[test]
    fn test_parser_deterministic(content in "\\PC*") {
        let parser = Parser::new(Language::Python, ParserOptions::default());

        let result1 = parser.parse(&content, "test.py");
        let result2 = parser.parse(&content, "test.py");

        match (result1, result2) {
            (Ok(r1), Ok(r2)) => {
                assert_eq!(r1.lines.len(), r2.lines.len());
                assert_eq!(r1.imports.len(), r2.imports.len());
            },
            (Err(_), Err(_)) => {}, // Both failed consistently
            _ => panic!("Non-deterministic behavior detected"),
        }
    }
}
```

### 5.2 Fuzzing

```rust
// fuzz/fuzz_targets/audit_file.rs (requires cargo-fuzz)

#![no_main]
use libfuzzer_sys::fuzz_target;
use quantum_crypto_audit::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(content) = std::str::from_utf8(data) {
        let config = AuditConfig::default();
        let engine = AuditEngine::new(config).unwrap();

        // Should never panic or crash
        let _ = engine.audit_file(content, "fuzz.py");
    }
});
```

```bash
# Run fuzzing
cargo install cargo-fuzz
cargo fuzz run audit_file -- -max_total_time=300 # 5 minutes
```

---

## 6. Performance Testing (Benchmarks)

### 6.1 Criterion Benchmarks

```rust
// benches/crypto_detection.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use quantum_crypto_audit::*;

fn bench_parse_file(c: &mut Criterion) {
    let content = include_str!("../tests/fixtures/large_file.py");

    c.bench_function("parse_large_python_file", |b| {
        let parser = Parser::new(Language::Python, ParserOptions::default());
        b.iter(|| {
            parser.parse(black_box(content), "large.py").unwrap()
        });
    });
}

fn bench_detect_patterns(c: &mut Criterion) {
    let content = include_str!("../tests/fixtures/large_file.py");
    let parser = Parser::new(Language::Python, ParserOptions::default());
    let parsed = parser.parse(content, "large.py").unwrap();

    c.bench_function("detect_patterns_large_file", |b| {
        let detector = CryptoDetector::new(get_default_patterns()).unwrap();
        b.iter(|| {
            detector.detect(black_box(&parsed))
        });
    });
}

fn bench_full_audit(c: &mut Criterion) {
    let sizes = vec![100, 500, 1000, 5000];

    let mut group = c.benchmark_group("full_audit_by_size");

    for size in sizes {
        let content = generate_test_file(size);

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &content,
            |b, content| {
                let config = AuditConfig::default();
                let engine = AuditEngine::new(config).unwrap();

                b.iter(|| {
                    engine.audit_file(black_box(content), "test.py").unwrap()
                });
            }
        );
    }

    group.finish();
}

criterion_group!(benches, bench_parse_file, bench_detect_patterns, bench_full_audit);
criterion_main!(benches);

fn generate_test_file(lines: usize) -> String {
    let mut content = String::new();
    for i in 0..lines {
        if i % 10 == 0 {
            content.push_str("import rsa\n");
        } else {
            content.push_str(&format!("x{} = {}\n", i, i));
        }
    }
    content
}
```

### 6.2 Performance Regression Tests

```bash
# Baseline before optimization
cargo bench -- --save-baseline before

# After optimization
cargo bench -- --baseline before

# Example output:
# parse_large_python_file
#   time:   [45.231 ms 45.789 ms 46.421 ms]
#   change: [-8.3421% -6.2341% -4.1234%] (improvement)
```

---

## 7. Test Fixtures and Data

### 7.1 Fixture Organization

```
tests/fixtures/
  test_files/
    python/
      rsa_vulnerable.py        # RSA usage
      ecc_vulnerable.py        # ECC usage
      hash_weak.py             # MD5/SHA1
      crypto_secure.py         # Post-quantum safe
      mixed_severity.py        # Multiple issues
    javascript/
      crypto_node.js           # Node.js crypto
      webcrypto.js             # Web Crypto API
      subtle_crypto.ts         # TypeScript
    go/
      rsa_keygen.go
      tls_config.go
    rust/
      ring_crypto.rs
      openssl_bindings.rs
  patterns/
    test_patterns.json         # Custom patterns for testing
  configs/
    default.json
    strict.json
    permissive.json
```

### 7.2 Test Data Generators

```rust
// tests/common/fixtures.rs

pub fn create_rsa_example(language: Language) -> String {
    match language {
        Language::Python => r#"
from cryptography.hazmat.primitives.asymmetric import rsa
private_key = rsa.generate_private_key(
    public_exponent=65537,
    key_size=2048
)
"#.to_string(),
        Language::JavaScript => r#"
const crypto = require('crypto');
const { publicKey, privateKey } = crypto.generateKeyPairSync('rsa', {
  modulusLength: 2048,
});
"#.to_string(),
        _ => unimplemented!("Language not supported in test"),
    }
}

pub fn create_mixed_severity_file() -> String {
    r#"
import rsa  # Critical
from cryptography.hazmat.primitives import hashes

# High severity
md5_hash = hashes.MD5()

# Medium severity
sha256_hash = hashes.SHA256()

# Low severity (best practice violation)
key_size = 1024  # Too small
"#.to_string()
}
```

---

## 8. Test Execution Strategy

### 8.1 Local Development Workflow

```bash
# Quick check (fast tests only)
cargo test --lib

# Full test suite
cargo test

# Watch mode for TDD
cargo watch -x test -x "test --lib"

# With coverage
cargo tarpaulin --out Html --output-dir coverage/

# Specific module
cargo test parser::tests

# WASM tests
wasm-pack test --node
```

### 8.2 CI/CD Pipeline

```yaml
# .github/workflows/test.yml

name: Test Suite

on: [push, pull_request]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --lib --all-features

  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: cargo test --test '*'

  wasm-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      - run: wasm-pack test --node --release

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/tarpaulin@v0.1
        with:
          args: '--out Xml --output-dir coverage/'
      - uses: codecov/codecov-action@v3
        with:
          files: ./coverage/cobertura.xml

  benchmarks:
    runs-on: ubuntu-latest
    if: github.event_name == 'push'
    steps:
      - uses: actions/checkout@v3
      - run: cargo bench -- --save-baseline ${{ github.sha }}
```

---

## 9. Test Quality Metrics

### 9.1 Coverage Targets

| Module | Line Coverage | Branch Coverage | Function Coverage |
|--------|--------------|-----------------|-------------------|
| lib.rs | 95% | 90% | 100% |
| audit.rs | 92% | 88% | 100% |
| parser.rs | 90% | 85% | 95% |
| detector.rs | 90% | 85% | 95% |
| types.rs | 95% | N/A | 100% |

### 9.2 Quality Gates

**PR Merge Requirements**:
- ✅ All tests pass (unit, integration, WASM)
- ✅ Code coverage ≥ 90%
- ✅ No clippy warnings
- ✅ Formatted with rustfmt
- ✅ Benchmarks show no regression > 10%

### 9.3 Monitoring Test Health

```bash
# Test execution time tracking
cargo test -- --report-time

# Flaky test detection (run multiple times)
for i in {1..10}; do cargo test || echo "Failed on run $i"; done

# Code coverage trends
cargo tarpaulin --out Json | jq '.coverage_percent'
```

---

## 10. Test Maintenance Guidelines

### 10.1 Test Naming Convention

```rust
// Pattern: test_<function>_<scenario>_<expected_result>

#[test]
fn test_parse_python_file_with_imports_succeeds() { }

#[test]
fn test_detect_rsa_in_python_returns_critical_finding() { }

#[test]
fn test_audit_empty_file_returns_no_findings() { }

#[test]
fn test_validate_config_with_invalid_severity_fails() { }
```

### 10.2 Test Documentation

```rust
/// Tests that the parser correctly identifies RSA key generation in Python code.
///
/// This test verifies:
/// 1. Pattern matching for Python RSA imports
/// 2. Detection of rsa.generate_private_key() calls
/// 3. Correct severity assignment (Critical)
/// 4. Accurate line number reporting
#[test]
fn test_detect_rsa_python() {
    // ... test implementation
}
```

### 10.3 Handling Flaky Tests

```rust
// Use retry logic for potentially flaky tests (e.g., timing-dependent)
#[test]
#[retry(3)]
fn test_performance_threshold() {
    let start = Instant::now();
    run_audit();
    let duration = start.elapsed();

    assert!(duration < Duration::from_millis(100));
}
```

---

## 11. Test Prioritization

### 11.1 Critical Path Tests (Must Pass)

1. RSA detection across all supported languages
2. Risk score calculation accuracy
3. WASM API compatibility
4. Configuration validation
5. Multi-file aggregation

### 11.2 Regression Tests

Maintain a regression test suite for all previously reported bugs:

```rust
// tests/regression_tests.rs

/// Regression test for issue #42: False positive on RSA variable name
#[test]
fn test_issue_42_rsa_variable_name_false_positive() {
    let content = "rsa_algorithm = 'described in paper'\nmy_var = 42";
    let report = audit_file(content, "test.py", "{}").unwrap();

    // Should not flag 'rsa_algorithm' as crypto usage
    assert_eq!(report.findings.len(), 0);
}
```

---

## 12. Testing Checklist for Developers

**Before committing**:
- [ ] Run `cargo test` (all tests pass)
- [ ] Run `cargo clippy` (no warnings)
- [ ] Run `cargo fmt` (code formatted)
- [ ] Add tests for new functionality
- [ ] Update integration tests if API changed
- [ ] Check coverage with `cargo tarpaulin`

**Before creating PR**:
- [ ] Run `wasm-pack test --node` (WASM tests pass)
- [ ] Run `cargo bench` (no significant regression)
- [ ] Update test documentation
- [ ] Add regression test if fixing a bug
- [ ] Verify all CI checks pass

**Code review checklist**:
- [ ] Tests cover edge cases
- [ ] Test names are descriptive
- [ ] No unnecessary test duplication
- [ ] Assertions have clear failure messages
- [ ] Property-based tests for complex logic

---

**End of Testing Strategy**

This comprehensive testing approach ensures:
- ✅ High confidence in correctness (90%+ coverage)
- ✅ Fast feedback loop (< 5s for unit tests)
- ✅ Protection against regressions
- ✅ Performance monitoring
- ✅ WASM compatibility verification
