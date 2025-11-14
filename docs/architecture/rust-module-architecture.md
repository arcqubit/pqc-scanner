# Rust Module Architecture - Quantum Crypto Audit Library

**Version**: 1.0.0
**Target**: WASM compilation for browser/Node.js
**Created**: 2025-11-06
**Architect**: ArcQubit Team

---

## 1. Architecture Overview

### 1.1 System Design Philosophy

This library follows a **modular, zero-copy, WASM-first** architecture:

- **Modular**: Each component has a single responsibility
- **Zero-copy**: Minimize allocations, use references where possible
- **WASM-first**: No platform-specific code (std::fs), all I/O handled by host
- **Error-explicit**: Result<T, E> everywhere, no panics in production
- **Test-driven**: Every module has comprehensive unit + integration tests

### 1.2 High-Level Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     JavaScript/TypeScript Host              │
│                    (Browser or Node.js)                     │
└─────────────────────────┬───────────────────────────────────┘
                          │ wasm_bindgen
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                        lib.rs (WASM Entry)                  │
│  - audit_file()      - audit_directory()                    │
│  - get_patterns()    - validate_config()                    │
└─────────────────┬───────────────────────┬───────────────────┘
                  │                       │
         ┌────────▼────────┐    ┌────────▼────────┐
         │   audit.rs      │    │   types.rs      │
         │  Orchestration  │◄───┤  Shared Types   │
         └────────┬────────┘    └─────────────────┘
                  │
      ┌───────────┼───────────┐
      │                       │
┌─────▼──────┐         ┌─────▼──────┐
│ parser.rs  │         │detector.rs │
│Multi-lang  │────────►│Pattern     │
│File Parser │         │Detection   │
└────────────┘         └────────────┘
```

---

## 2. Module Specifications

### 2.1 `lib.rs` - WASM Entry Point

**Purpose**: Export public API for JavaScript/TypeScript consumption

**Key Responsibilities**:
- WASM bindings via wasm_bindgen
- Entry point initialization
- Panic hook setup for debugging
- Version and feature flags

**Public API**:
```rust
#[wasm_bindgen]
pub fn audit_file(content: &str, file_path: &str, config_json: &str) -> Result<JsValue, JsValue>;

#[wasm_bindgen]
pub fn audit_directory(files_json: &str, config_json: &str) -> Result<JsValue, JsValue>;

#[wasm_bindgen]
pub fn get_supported_patterns() -> JsValue;

#[wasm_bindgen]
pub fn validate_config(config_json: &str) -> Result<JsValue, JsValue>;

#[wasm_bindgen]
pub fn get_version() -> String;
```

**Design Constraints**:
- No file I/O (use string parameters)
- All inputs/outputs via JSON serialization
- Error conversion to JsValue
- Minimal state (stateless where possible)

---

### 2.2 `audit.rs` - Core Audit Orchestration

**Purpose**: Coordinate the audit workflow and aggregation

**Key Responsibilities**:
- Parse configuration
- Coordinate parser → detector flow
- Aggregate findings
- Calculate risk scores
- Generate audit reports

**Core Structs**:
```rust
pub struct AuditEngine {
    config: AuditConfig,
    detector: CryptoDetector,
}

pub struct AuditReport {
    pub findings: Vec<Finding>,
    pub risk_score: RiskScore,
    pub summary: AuditSummary,
    pub metadata: AuditMetadata,
}

pub struct Finding {
    pub id: String,
    pub severity: Severity,
    pub pattern_type: PatternType,
    pub location: Location,
    pub description: String,
    pub recommendation: String,
    pub confidence: f32,
}
```

**Public Functions**:
```rust
impl AuditEngine {
    pub fn new(config: AuditConfig) -> Result<Self, AuditError>;
    pub fn audit_file(&self, content: &str, file_path: &str) -> Result<AuditReport, AuditError>;
    pub fn audit_multiple(&self, files: Vec<FileInput>) -> Result<AuditReport, AuditError>;
}
```

**Risk Scoring Algorithm**:
```rust
Risk Score = Σ(finding.severity_weight × finding.confidence) / total_lines_of_code
- Critical: 10 points
- High: 7 points
- Medium: 4 points
- Low: 1 point
```

---

### 2.3 `parser.rs` - Multi-Language File Parser

**Purpose**: Extract code structure and prepare for pattern detection

**Key Responsibilities**:
- Language detection (Python, JavaScript, TypeScript, Rust, Go, Java)
- Comment stripping (optional)
- Import/dependency extraction
- Function/method boundary detection
- Line-by-line tokenization

**Core Structs**:
```rust
pub struct Parser {
    language: Language,
    options: ParserOptions,
}

pub struct ParsedFile {
    pub language: Language,
    pub imports: Vec<Import>,
    pub functions: Vec<FunctionInfo>,
    pub lines: Vec<Line>,
    pub raw_content: String,
}

pub struct Line {
    pub number: usize,
    pub content: String,
    pub is_comment: bool,
    pub indentation: usize,
}

pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Rust,
    Go,
    Java,
    Unknown,
}
```

**Public Functions**:
```rust
impl Parser {
    pub fn new(language: Language, options: ParserOptions) -> Self;
    pub fn parse(&self, content: &str, file_path: &str) -> Result<ParsedFile, ParseError>;
    pub fn detect_language(file_path: &str) -> Language;
}
```

**Extension Mapping**:
```rust
".py" -> Python
".js" -> JavaScript
".ts" -> TypeScript
".rs" -> Rust
".go" -> Go
".java" -> Java
```

---

### 2.4 `detector.rs` - Crypto Pattern Detection Engine

**Purpose**: Identify quantum-vulnerable cryptographic patterns

**Key Responsibilities**:
- Pattern matching via regex + semantic analysis
- Quantum vulnerability classification
- Confidence scoring
- Context-aware detection (reduce false positives)

**Core Structs**:
```rust
pub struct CryptoDetector {
    patterns: Vec<Pattern>,
    config: DetectorConfig,
}

pub struct Pattern {
    pub id: String,
    pub name: String,
    pub pattern_type: PatternType,
    pub regex: Regex,
    pub severity: Severity,
    pub quantum_vulnerable: bool,
    pub description: String,
    pub recommendation: String,
}

pub enum PatternType {
    RSA,
    ECC,
    DH,
    AES,
    SHA,
    MD5,
    DES,
    ECDSA,
    KeyGeneration,
    Certificate,
    Signature,
    HashFunction,
    SymmetricEncryption,
    AsymmetricEncryption,
}
```

**Detection Algorithm**:
```rust
impl CryptoDetector {
    pub fn detect(&self, parsed: &ParsedFile) -> Vec<Detection>;
    fn match_pattern(&self, line: &Line, pattern: &Pattern) -> Option<Detection>;
    fn calculate_confidence(&self, context: &Context) -> f32;
    fn validate_context(&self, detection: &Detection, parsed: &ParsedFile) -> bool;
}
```

**Pattern Examples**:
```rust
Pattern {
    id: "RSA-001",
    name: "RSA Key Generation",
    pattern_type: PatternType::RSA,
    regex: r"(RSA\.generate|generateKeyPair.*RSA|rsa\.GenerateKey)",
    severity: Severity::Critical,
    quantum_vulnerable: true,
    description: "RSA encryption detected - vulnerable to quantum attacks",
    recommendation: "Migrate to post-quantum algorithms like CRYSTALS-Kyber"
}
```

**Confidence Scoring**:
```rust
Confidence = base_score × context_multiplier × keyword_density
- Base: 0.7 for regex match
- Context: +0.2 if in crypto-related function name
- Keyword density: +0.1 if nearby quantum-related keywords
```

---

### 2.5 `types.rs` - Shared Types and Utilities

**Purpose**: Common types, enums, and serialization helpers

**Key Components**:

```rust
// Severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,  // Quantum-vulnerable asymmetric crypto
    High,      // Weak hash functions or key sizes
    Medium,    // Deprecated algorithms
    Low,       // Best practice violations
    Info,      // Informational findings
}

// Location tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub snippet: String,
}

// Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    pub severity_threshold: Severity,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub strip_comments: bool,
    pub max_file_size_kb: usize,
    pub enable_context_validation: bool,
}

// Risk scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub total: f32,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Catastrophic, // Score > 8.0
    High,         // Score 5.0-8.0
    Medium,       // Score 2.0-5.0
    Low,          // Score < 2.0
}
```

---

## 3. Error Handling Strategy

### 3.1 Error Type Hierarchy

```rust
// types.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditError {
    ParseError(String),
    DetectionError(String),
    ConfigError(String),
    InvalidInput(String),
    InternalError(String),
}

impl std::fmt::Display for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AuditError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AuditError::DetectionError(msg) => write!(f, "Detection error: {}", msg),
            AuditError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            AuditError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AuditError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AuditError {}
```

### 3.2 Error Conversion for WASM

```rust
// lib.rs
impl From<AuditError> for JsValue {
    fn from(err: AuditError) -> JsValue {
        let error_obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &error_obj,
            &JsValue::from_str("error"),
            &JsValue::from_str(&err.to_string()),
        ).unwrap();
        error_obj.into()
    }
}
```

### 3.3 Error Handling Guidelines

1. **Never panic in production code** - use Result<T, E>
2. **Propagate errors with `?` operator** - let caller handle
3. **Provide context** - include file paths, line numbers
4. **Log errors at WASM boundary** - use console_error_panic_hook
5. **Validate inputs early** - fail fast with clear messages

---

## 4. Performance Optimization Strategies

### 4.1 Zero-Copy Principles

```rust
// ✅ Good: Use references
fn process_line(line: &str) -> Result<Finding, AuditError>

// ❌ Bad: Unnecessary cloning
fn process_line(line: String) -> Result<Finding, AuditError>
```

### 4.2 Regex Compilation

```rust
// Compile patterns once at initialization
pub struct CryptoDetector {
    compiled_patterns: Vec<CompiledPattern>,
}

struct CompiledPattern {
    id: String,
    regex: Regex, // Pre-compiled
    metadata: PatternMetadata,
}

impl CryptoDetector {
    pub fn new(patterns: Vec<Pattern>) -> Result<Self, AuditError> {
        let compiled = patterns.into_iter()
            .map(|p| CompiledPattern {
                id: p.id,
                regex: Regex::new(&p.regex)?,
                metadata: p.into(),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { compiled_patterns: compiled })
    }
}
```

### 4.3 Lazy Evaluation

```rust
// Parse on-demand, not eagerly
pub struct ParsedFile {
    raw_content: String,
    functions: OnceCell<Vec<FunctionInfo>>, // Lazy
}
```

### 4.4 WASM Memory Management

- Use `wee_alloc` for smaller binary size (optional)
- Minimize JsValue allocations
- Return borrowed references where possible
- Use `serde-wasm-bindgen` for efficient serialization

---

## 5. Testing Architecture

### 5.1 Unit Tests (Per Module)

```rust
// parser.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_python() {
        let parser = Parser::new(Language::Python, ParserOptions::default());
        let result = parser.parse("import rsa\nkey = rsa.generate(2048)", "test.py");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().imports.len(), 1);
    }

    #[test]
    fn test_strip_comments() {
        // Test comment removal
    }
}
```

### 5.2 Integration Tests

```rust
// tests/integration_test.rs
use quantum_crypto_audit::{audit_file, AuditConfig};

#[test]
fn test_full_audit_rsa() {
    let content = r#"
        from cryptography.hazmat.primitives.asymmetric import rsa
        private_key = rsa.generate_private_key(public_exponent=65537, key_size=2048)
    "#;

    let config = AuditConfig::default();
    let report = audit_file(content, "test.py", &config).unwrap();

    assert!(report.findings.len() > 0);
    assert_eq!(report.findings[0].pattern_type, PatternType::RSA);
    assert_eq!(report.findings[0].severity, Severity::Critical);
}
```

### 5.3 WASM Tests

```rust
// tests/wasm_test.rs
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_wasm_audit_file() {
    let content = "RSA.generate(2048)";
    let config = r#"{"severity_threshold":"Low"}"#;
    let result = audit_file(content, "test.js", config);
    assert!(result.is_ok());
}
```

### 5.4 Test Coverage Goals

- **Unit tests**: 90%+ coverage per module
- **Integration tests**: All public API paths
- **WASM tests**: All exported functions
- **Edge cases**: Empty files, malformed input, large files

---

## 6. Cargo.toml Configuration

```toml
[package]
name = "quantum-crypto-audit"
version = "1.0.0"
edition = "2021"
authors = ["ArcQubit Team"]
description = "Quantum-safe cryptography audit library compiled to WASM"
license = "MIT"
repository = "https://github.com/arcqubit/quantum-crypto-audit"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2.92"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
serde_json = "1.0"
regex = "1.10"
console_error_panic_hook = "0.1.7"
js-sys = "0.3"
once_cell = "1.19"

# Optional: Smaller binary size
# wee_alloc = { version = "0.4.5", optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3"
criterion = "0.5" # For benchmarks

[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization, slower compile

[features]
default = []
# small-binary = ["wee_alloc"]
```

---

## 7. Build and Deployment Pipeline

### 7.1 Build Commands

```bash
# Development build
wasm-pack build --target web --dev

# Production build
wasm-pack build --target web --release

# Node.js target
wasm-pack build --target nodejs --release

# Test
wasm-pack test --node
wasm-pack test --headless --chrome
```

### 7.2 CI/CD Integration

```yaml
# .github/workflows/rust-wasm.yml
name: Rust WASM Build
on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      - run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      - run: wasm-pack build --release
      - run: wasm-pack test --node
```

---

## 8. API Design for WASM Exports

### 8.1 JavaScript/TypeScript Interface

```typescript
// Generated by wasm-bindgen
export function audit_file(
  content: string,
  file_path: string,
  config_json: string
): Promise<AuditReport>;

export function audit_directory(
  files_json: string,
  config_json: string
): Promise<AuditReport>;

export function get_supported_patterns(): PatternInfo[];

export function validate_config(config_json: string): ConfigValidation;

export function get_version(): string;

// Types
interface AuditReport {
  findings: Finding[];
  risk_score: RiskScore;
  summary: AuditSummary;
  metadata: AuditMetadata;
}

interface Finding {
  id: string;
  severity: "Critical" | "High" | "Medium" | "Low" | "Info";
  pattern_type: string;
  location: Location;
  description: string;
  recommendation: string;
  confidence: number;
}
```

### 8.2 Usage Example

```typescript
import init, { audit_file } from './pkg/quantum_crypto_audit.js';

await init();

const config = {
  severity_threshold: "Medium",
  strip_comments: true
};

const report = await audit_file(
  fileContent,
  "crypto.py",
  JSON.stringify(config)
);

console.log(`Found ${report.findings.length} issues`);
console.log(`Risk level: ${report.risk_score.risk_level}`);
```

---

## 9. Security Considerations

### 9.1 Input Validation

```rust
impl AuditEngine {
    fn validate_input(&self, content: &str, file_path: &str) -> Result<(), AuditError> {
        if content.len() > self.config.max_file_size_kb * 1024 {
            return Err(AuditError::InvalidInput("File too large".into()));
        }

        if file_path.contains("..") {
            return Err(AuditError::InvalidInput("Invalid file path".into()));
        }

        Ok(())
    }
}
```

### 9.2 Resource Limits

- Max file size: 10MB default
- Timeout for regex: 5 seconds per pattern
- Max findings per file: 1000
- Memory limit: Configurable in WASM host

### 9.3 Sandboxing

- No file system access (WASM constraint)
- No network access
- No external process execution
- Pure computation only

---

## 10. Architecture Decision Records (ADRs)

### ADR-001: WASM-First Architecture

**Context**: Need to run crypto audit in browser and Node.js
**Decision**: Build as WASM library compiled from Rust
**Consequences**:
- ✅ Cross-platform compatibility
- ✅ Near-native performance
- ❌ Larger binary size (mitigated with opt-level="z")
- ❌ No direct file I/O (host must provide content)

### ADR-002: Zero-Copy Design

**Context**: Large files can cause memory pressure
**Decision**: Use references (&str) instead of owned Strings where possible
**Consequences**:
- ✅ Reduced memory allocations
- ✅ Better performance
- ❌ More complex lifetime management

### ADR-003: Regex-Based Pattern Matching

**Context**: Need flexible pattern detection across languages
**Decision**: Use compiled regex patterns with semantic context validation
**Consequences**:
- ✅ Flexible and maintainable patterns
- ✅ Language-agnostic
- ❌ Potential false positives (mitigated with confidence scoring)
- ❌ Performance overhead (mitigated with pre-compilation)

### ADR-004: JSON Configuration

**Context**: Need runtime configuration from JavaScript
**Decision**: Accept JSON strings for config, serialize/deserialize with serde
**Consequences**:
- ✅ Familiar format for JS developers
- ✅ Type-safe with serde
- ❌ Serialization overhead (acceptable for config)

---

## 11. Future Extensibility

### 11.1 Planned Enhancements

1. **Machine Learning Integration**
   - Train model on known vulnerabilities
   - Improve confidence scoring
   - Reduce false positives

2. **Incremental Audits**
   - Cache previous results
   - Only re-audit changed files
   - Persistent state management

3. **Custom Pattern DSL**
   - User-defined patterns without recompiling
   - Pattern marketplace/sharing

4. **Performance Profiling**
   - Export performance metrics
   - Identify slow patterns
   - Auto-optimize pattern order

### 11.2 Extension Points

```rust
// Plugin trait for custom detectors
pub trait Detector {
    fn detect(&self, parsed: &ParsedFile) -> Vec<Detection>;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
}

// Plugin registration
impl AuditEngine {
    pub fn register_detector(&mut self, detector: Box<dyn Detector>) {
        self.detectors.push(detector);
    }
}
```

---

## 12. Documentation Requirements

### 12.1 Code Documentation

- **Module-level docs**: Purpose, responsibilities, examples
- **Function docs**: Parameters, return values, errors, examples
- **Type docs**: Field meanings, invariants, usage notes
- **Example code**: In doctests for verification

### 12.2 API Documentation

```rust
/// Audits a single file for quantum-vulnerable cryptography
///
/// # Arguments
/// * `content` - The file content as a UTF-8 string
/// * `file_path` - The file path (used for language detection)
/// * `config_json` - Configuration as JSON string
///
/// # Returns
/// * `Ok(AuditReport)` - Audit results with findings and risk score
/// * `Err(JsValue)` - Error details as JavaScript object
///
/// # Examples
/// ```javascript
/// const report = await audit_file(
///   "import rsa\nkey = rsa.generate(2048)",
///   "crypto.py",
///   '{"severity_threshold":"Low"}'
/// );
/// ```
#[wasm_bindgen]
pub fn audit_file(content: &str, file_path: &str, config_json: &str) -> Result<JsValue, JsValue>
```

---

## 13. Success Metrics

### 13.1 Performance Targets

- **Parse speed**: < 50ms for 1000 LOC
- **Detection speed**: < 100ms for 1000 LOC
- **Binary size**: < 500KB gzipped
- **Memory usage**: < 10MB for typical audit

### 13.2 Quality Metrics

- **False positive rate**: < 10%
- **False negative rate**: < 5%
- **Test coverage**: > 90%
- **Documentation coverage**: 100% public API

### 13.3 Usability Metrics

- **Setup time**: < 5 minutes
- **API complexity**: < 5 functions for basic use
- **Error clarity**: 100% errors have actionable messages

---

## Appendix A: Pattern Library Example

```rust
// Built-in patterns (detector.rs)
const PATTERNS: &[(&str, &str, PatternType, Severity, bool)] = &[
    // (ID, Regex, Type, Severity, QuantumVulnerable)
    ("RSA-001", r"RSA\.generate|generateKeyPair.*RSA", PatternType::RSA, Severity::Critical, true),
    ("ECC-001", r"ECDSA|elliptic.*curve|secp256k1", PatternType::ECC, Severity::Critical, true),
    ("DH-001", r"Diffie.*Hellman|DHE|ECDHE", PatternType::DH, Severity::High, true),
    ("SHA2-001", r"SHA256|SHA384|SHA512", PatternType::SHA, Severity::Medium, false),
    ("MD5-001", r"MD5|md5", PatternType::MD5, Severity::High, false),
    ("AES-001", r"AES|aes", PatternType::AES, Severity::Low, false),
];
```

---

## Appendix B: Risk Level Thresholds

| Risk Level   | Score Range | Description | Recommended Action |
|--------------|-------------|-------------|-------------------|
| Catastrophic | 8.0+        | Multiple critical quantum vulnerabilities | Immediate migration required |
| High         | 5.0 - 8.0   | Several high-severity issues | Migration planning within 1-3 months |
| Medium       | 2.0 - 5.0   | Moderate vulnerabilities | Migration within 6-12 months |
| Low          | 0 - 2.0     | Minor issues or best practice violations | Monitor and improve over time |

---

**End of Architecture Document**

This architecture provides a solid foundation for the 3 Rust coders to implement:
1. **Coder 1**: lib.rs, types.rs
2. **Coder 2**: audit.rs, parser.rs
3. **Coder 3**: detector.rs

All modules are designed to work together with clear interfaces and minimal coupling.
