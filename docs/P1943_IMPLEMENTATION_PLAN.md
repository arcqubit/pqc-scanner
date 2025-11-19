# IEEE P1943 Implementation Plan for PQC-Scanner

## Executive Summary

This document outlines the comprehensive implementation plan for integrating IEEE P1943 standard (Post-Quantum Cryptographic Inventory and Assessment) into the ArcQubit pqc-scanner project. The implementation includes:

- **P1943 Detection Matrix**: 12 control areas for cryptographic inventory
- **ACDI JSON Schema**: OMB M-23-02 automated crypto discovery alignment
- **Q-CMM Maturity Model**: Quantum Cryptography Maturity Model (Levels 0-5)
- **Network Protocol Detection**: TLS/QUIC cipher suite analysis
- **Package Attribution**: Cryptographic library identification
- **Hybrid Mode Detection**: Classical/Hybrid/PQC-only classification

**Timeline**: 10-12 weeks
**Estimated LOC**: ~3,000 new code, ~1,500 test code
**Backward Compatibility**: 100% (all existing features preserved)

---

## Table of Contents

1. [Current Codebase Analysis](#1-current-codebase-analysis)
2. [P1943 Integration Architecture](#2-p1943-integration-architecture)
3. [Type Definitions](#3-type-definitions)
4. [P1943 Control Implementation](#4-p1943-control-implementation)
5. [ACDI Schema Generator](#5-acdi-schema-generator)
6. [Q-CMM Mapper](#6-q-cmm-mapper)
7. [Detection Algorithm Enhancements](#7-detection-algorithm-enhancements)
8. [GitHub Actions Integration](#8-github-actions-integration)
9. [Testing Strategy](#9-testing-strategy)
10. [Implementation Phases](#10-implementation-phases)
11. [CLI Enhancements](#11-cli-enhancements)
12. [Documentation Requirements](#12-documentation-requirements)
13. [Performance Considerations](#13-performance-considerations)
14. [Success Metrics](#14-success-metrics)
15. [Dependencies](#15-dependencies)

---

## 1. Current Codebase Analysis

### 1.1 Module Breakdown (4,541 total lines)

**Core Modules:**
- `/src/lib.rs` (165 lines) - Public API, WASM exports
- `/src/types.rs` (976 lines) - Comprehensive type definitions (NIST SC-13, ITSG-33, OSCAL)
- `/src/audit.rs` (530 lines) - Core scanning engine with regex-based detection
- `/src/detector.rs` (242 lines) - Pattern detection with severity classification
- `/src/compliance.rs` (575 lines) - NIST 800-53 SC-13 compliance reporting
- `/src/canadian_compliance.rs` (810 lines) - ITSG-33 compliance with CCCS integration
- `/src/algorithm_database.rs` (444 lines) - CCCS algorithm validation
- `/src/parser.rs` - Language-specific parsing
- `/src/remediation.rs` - Automated fix suggestions

**Data Assets:**
- `/data/cccs_algorithms.json` - CCCS algorithm approval database
- `/data/cmvp_certificates.json` - CMVP validation certificates

**Infrastructure:**
- WASM compilation support (bundler, nodejs, web targets)
- GitHub Actions CI/CD with security scanning
- OSCAL JSON output capability

### 1.2 Current Detection Capabilities

**10 Crypto Patterns Detected:**
1. RSA (with key size extraction: 512, 1024, 2048, 3072, 4096, 8192)
2. ECDSA (P-256, P-384, P-521, secp256k1, secp384r1)
3. ECDH (including Curve25519)
4. DSA
5. Diffie-Hellman (DH, DHE)
6. MD5
7. SHA-1
8. DES
9. 3DES
10. RC4

**Current Severity Model:**
- Critical: MD5, SHA-1, DES, RC4, RSA <2048 bits (100-95 risk score)
- High: RSA ≥2048, ECDSA, ECDH, DSA, DH, 3DES (85-90 risk score)

### 1.3 Existing Compliance Framework Support

**NIST 800-53 SC-13:**
- Full OSCAL JSON output
- Implementation status tracking
- Evidence collection
- Assessment results generation

**Canadian ITSG-33:**
- CCCS algorithm approval status
- Security classification levels (Unclassified, Protected A/B/C)
- CMVP validation tracking
- ITSP.40.111 and ITSP.40.062 references
- Unified NIST+Canadian reporting

---

## 2. P1943 Integration Architecture

### 2.1 New Module Structure

```
src/
├── p1943/
│   ├── mod.rs                    # P1943 module root
│   ├── types.rs                  # P1943-specific types (ACDI, Q-CMM)
│   ├── controls.rs               # 12 P1943 control area implementations
│   ├── acdi_schema.rs            # ACDI JSON schema generation
│   ├── qcmm_mapper.rs            # Q-CMM maturity mapping
│   ├── network_detector.rs       # TLS/QUIC protocol detection
│   ├── package_detector.rs       # Library/package attribution
│   ├── platform_detector.rs      # OS/container/VM detection
│   ├── hybrid_analyzer.rs        # Hybrid mode detection
│   ├── downgrade_detector.rs     # Cipher suite downgrade detection
│   ├── key_lifecycle.rs          # Key age/rotation/reuse tracking
│   └── policy_engine.rs          # Policy compliance checker
├── compliance.rs (EXISTING)
├── canadian_compliance.rs (EXISTING)
└── p1943_compliance.rs (NEW)     # P1943 compliance orchestrator
```

### 2.2 P1943 Detection Matrix (12 Control Areas)

| Control ID | Control Area | Detection Signal(s) | Data Source(s) | Output Field(s) |
|------------|--------------|---------------------|----------------|-----------------|
| **P1943-NET-ALG-01** | Algorithm Identification | TLS/QUIC handshake metadata, cipher suites, PQC KEMs (Kyber, FrodoKEM), signatures (Dilithium, Falcon) | pcap/traffic captures, TLS logs, service banners, config files | `algorithms[]`, `cipher_suites[]`, `pqc_algorithms[]`, `hybrid_mode` |
| **P1943-NET-SVC-02** | Service Role Classification | Classify flows/endpoints by role: KEX-only (KEM), channel (TLS/IPsec), signature-only (code-signing, PKI) | Protocol fingerprints, certificate usage, application logs | `service_roles[]`: `key_exchange`, `encrypted_connection`, `signature`, `storage_encryption` |
| **P1943-KEY-LEN-03** | Key Strength & Parameters | Extract key sizes (RSA, ECC, PQC), parameter sets (Kyber-512/768/1024, Dilithium-II/III/V) | Certificates, SSH host keys, KMS APIs, HSM metadata, config files | `key_length_bits`, `parameter_set`, `security_level_estimate` (NIST category) |
| **P1943-INV-04** | Crypto Inventory Completeness | Count discovered crypto objects vs expected, track coverage %, flag dark/unknown crypto | Host agents, SW inventory, container scans, SBOM/CBOM | `inventory_items[]`, `coverage_percent`, `undiscovered_estimate` |
| **P1943-PKG-05** | Package Attribution | Correlate libraries/binaries (OpenSSL, BoringSSL, wolfSSL, liboqs) to products and vendors | Package managers, SBOMs, file hashes, vendor catalogs | `package_name`, `package_version`, `vendor`, `package_type` (COTS/GOTS/custom) |
| **P1943-OS-06** | Platform & OS Context | Map crypto libraries to OS image, kernel, environment (container vs bare metal vs function) | OS queries, container manifests, cloud metadata APIs | `os_name`, `os_version`, `platform_type` (vm/container/func/appliance) |
| **P1943-HYB-07** | Hybrid Mode Detection | Identify dual-use cipher suites (ECDHE+Kyber), dual-signature objects (ECDSA+Dilithium) | TLS analytics, cert chains, protocol traces | `mode`: `classical`, `hybrid`, `pqc_only`; `hybrid_components[]` |
| **P1943-DOWN-08** | Downgrade Control | Compare client-offered vs negotiated suites; detect deprecated/ban-listed algorithms | Traffic logs, gateway logs, reverse proxy telemetry | `downgrade_detected` (bool), `downgrade_reason`, `legacy_fallback_used` |
| **P1943-KEYLIFE-09** | Key Lifecycle Observation | Track cert/key issuance & expiry, rotation intervals, reuse across services | KMS/HSM APIs, cert-manager logs, Vault, PKI telemetry | `first_seen`, `last_seen`, `age_days`, `rotation_interval_days`, `reused_across_services` |
| **P1943-ARCH-10** | Network Security Zone Classification | Map endpoints to systems, zones, and FIPS-199 categorizations via CMDB or tags | CMDB, tagging systems, cloud labels, asset inventory | `system_id`, `fisma_level`, `is_hva` (bool), `zone_id` |
| **P1943-LOG-11** | Audit & Evidence Generation | Emit structured events: algorithm selection, negotiation path, failures, downgrade attempts | Syslog, SIEM feeds, API logs, scanner logs | `events[]` with `event_type`, `timestamp`, `system_id`, `algorithm`, `result` |
| **P1943-POLICY-12** | Policy Compliance Check | Compare discovered crypto against P1943-aligned crypto policy | `crypto-policy.yaml`, scanner output, policy engine | `policy_profile`, `violations[]`, `recommended_actions[]` |

---

## 3. Type Definitions

### 3.1 P1943 Core Types (`src/p1943/types.rs`)

```rust
// IEEE P1943 Control Area Identifiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum P1943ControlArea {
    NetAlg01,      // Algorithm Identification
    NetSvc02,      // Service Role Classification
    KeyLen03,      // Key Strength & Parameters
    Inv04,         // Crypto Inventory Completeness
    Pkg05,         // Package Attribution
    Os06,          // Platform & OS Context
    Hyb07,         // Hybrid Mode Detection
    Down08,        // Downgrade Control
    KeyLife09,     // Key Lifecycle Observation
    Arch10,        // Network Security Zone
    Log11,         // Audit & Evidence
    Policy12,      // Policy Compliance
}

// ACDI Schema Root (OMB M-23-02)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACDIReport {
    pub schema_version: String,           // "1.0.0"
    pub generated_at: String,             // ISO 8601
    pub generator: GeneratorInfo,
    pub systems: Vec<SystemInventory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorInfo {
    pub name: String,                     // "pqc-scanner"
    pub version: String,                  // "2025.11.18"
    pub vendor: String,                   // "ArcQubit"
    pub standards: Vec<String>,           // ["IEEE-P1943", "OMB-M-23-02", "NIST-SP-800-53"]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInventory {
    pub system_id: String,
    pub system_name: String,
    pub fisma_level: FISMAImpactLevel,
    pub is_hva: bool,                     // High Value Asset
    pub environment: DeploymentEnvironment,
    pub data_lifecycle: DataLifecycle,
    pub crypto_systems: Vec<CryptoSystemEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FISMAImpactLevel {
    Low,
    Moderate,
    High,
}

// Crypto System Entry (Core P1943 data structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoSystemEntry {
    pub algorithm: AlgorithmInfo,
    pub package: PackageInfo,
    pub platform: PlatformInfo,
    pub key_lifecycle: KeyLifecycleInfo,
    pub deployment_context: DeploymentContext,
    pub pqc_status: PQCStatus,
    pub evidence: Vec<P1943Evidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmInfo {
    pub name: String,                     // "RSA-2048", "ECDSA-P256", "ML-KEM-768"
    pub family: AlgorithmFamily,
    pub service_role: ServiceRole,
    pub key_size: Option<u32>,
    pub parameter_set: Option<String>,    // NIST PQC security category
    pub nist_category: Option<NISTPQCCategory>,
    pub cipher_suites: Vec<String>,       // TLS cipher suites
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlgorithmFamily {
    RSA,
    ECC,
    DSA,
    DH,
    AES,
    ChaCha20,
    SHA2,
    SHA3,
    MLKEM,                                // CRYSTALS-Kyber (ML-KEM)
    MLDSA,                                // CRYSTALS-Dilithium (ML-DSA)
    SLHDSA,                               // SPHINCS+ (SLH-DSA)
    FrodoKEM,
    NTRU,
    BIKE,
    HQC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceRole {
    KeyExchange,                          // KEX
    ChannelEncryption,                    // Channel
    DigitalSignature,                     // Signature
    DataAtRestEncryption,                 // Storage
    Authentication,
    Hashing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NISTPQCCategory {
    Category1,                            // AES-128 equivalent
    Category2,                            // SHA-256 collision
    Category3,                            // AES-192 equivalent
    Category4,                            // SHA-384 collision
    Category5,                            // AES-256 equivalent
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub library_name: String,             // "OpenSSL", "BoringSSL", "wolfSSL", "liboqs"
    pub version: String,
    pub vendor: String,
    pub cmvp_cert: Option<String>,
    pub fips_validated: bool,
    pub pqc_capable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub os_family: OSFamily,
    pub os_version: String,
    pub architecture: String,             // "x86_64", "aarch64", "wasm32"
    pub deployment_type: DeploymentType,
    pub container_runtime: Option<String>,
    pub cloud_provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentType {
    BareMetal,
    VirtualMachine,
    Container,
    Serverless,
    IoT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PQCStatus {
    pub mode: CryptoMode,
    pub quantum_safe: bool,
    pub migration_priority: MigrationPriority,
    pub sunset_date: Option<String>,
    pub hybrid_components: Option<HybridComponents>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoMode {
    ClassicalOnly,
    HybridClassicalPQC,
    PQCOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationPriority {
    Immediate,                            // Broken algorithms
    High,                                 // Quantum-vulnerable, short timeline
    Medium,                               // Quantum-vulnerable, standard timeline
    Low,                                  // Already quantum-safe
    None,                                 // No migration needed
}

// Q-CMM Maturity Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCMMAssessment {
    pub overall_level: QCMMLevel,
    pub domain_scores: QCMMDomainScores,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QCMMLevel {
    Level0,                               // Pre-Quantum Ready (Unaware)
    Level1,                               // Quantum Cryptography Compliant
    Level2,                               // Quantum Aware
    Level3,                               // Quantum Enabled
    Level4,                               // Quantum Integrated
    Level5,                               // Quantum Optimized
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCMMDomainScores {
    pub governance: DomainScore,
    pub architecture: DomainScore,
    pub quantum_crypto: DomainScore,
    pub workforce: DomainScore,
    pub performance: DomainScore,
}
```

### 3.2 Extended CryptoType Enum (`src/types.rs`)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CryptoType {
    // Existing
    Rsa,
    Ecdsa,
    Ecdh,
    Dsa,
    DiffieHellman,
    Sha1,
    Md5,
    Des,
    TripleDes,
    Rc4,

    // NEW: Post-Quantum Algorithms
    MlKem,                                // CRYSTALS-Kyber (ML-KEM)
    MlDsa,                                // CRYSTALS-Dilithium (ML-DSA)
    SlhDsa,                               // SPHINCS+ (SLH-DSA)
    FrodoKem,
    Ntru,
    Bike,
    Hqc,

    // NEW: Modern Symmetric
    Aes128Gcm,
    Aes256Gcm,
    ChaCha20Poly1305,

    // NEW: Modern Hash
    Sha256,
    Sha384,
    Sha512,
    Sha3_256,
    Sha3_512,
    Blake2,
    Blake3,
}
```

---

## 4. P1943 Control Implementation

### 4.1 Control Functions (`src/p1943/controls.rs`)

```rust
// P1943-NET-ALG-01: Algorithm Identification
pub fn detect_algorithms_p1943(
    source: &str,
    language: &Language,
) -> Result<Vec<AlgorithmDetection>, P1943Error> {
    // Enhanced detection beyond current patterns
    // - TLS cipher suite extraction
    // - QUIC crypto config detection
    // - PQC algorithm detection (ML-KEM, ML-DSA, SLH-DSA)
    // - Hybrid mode identification
}

// P1943-NET-SVC-02: Service Role Classification
pub fn classify_service_role(
    algorithm: &AlgorithmInfo,
    context: &CodeContext,
) -> ServiceRole {
    // Classify based on usage patterns:
    // - Key exchange: DH, ECDH, RSA-KEM, ML-KEM
    // - Signatures: RSA-PSS, ECDSA, ML-DSA, SLH-DSA
    // - Encryption: AES-GCM, ChaCha20-Poly1305
}

// P1943-PKG-05: Package Attribution
pub fn detect_crypto_packages(
    source: &str,
    language: &Language,
) -> Vec<PackageInfo> {
    // Detect:
    // - Import statements
    // - OpenSSL, BoringSSL, wolfSSL, libsodium, liboqs
    // - Version extraction from lock files
    // - CMVP certificate lookup
}

// P1943-HYB-07: Hybrid Mode Detection
pub fn detect_hybrid_crypto(
    detections: &[AlgorithmDetection],
) -> Vec<HybridModeDetection> {
    // Identify:
    // - Classical + PQC algorithm pairs
    // - Hybrid TLS configurations
    // - Key combiner functions
}

// P1943-POLICY-12: Policy Compliance Check
pub fn check_policy_compliance(
    entry: &CryptoSystemEntry,
    policy: &CryptoPolicy,
) -> PolicyComplianceResult {
    // Check against:
    // - Approved algorithm lists
    // - Banned/deprecated algorithms
    // - Minimum key sizes
    // - Sunset date enforcement
}
```

---

## 5. ACDI Schema Generator

### 5.1 Implementation (`src/p1943/acdi_schema.rs`)

```rust
pub fn generate_acdi_report(
    audit_results: &AuditResult,
    system_config: &SystemConfig,
) -> Result<ACDIReport, ACDIError> {
    // OMB M-23-02 alignment:
    // - Top-level metadata
    // - System inventory with FISMA levels
    // - HVA designation
    // - Environment classification
    // - Crypto system entries with full P1943 controls
    // - Evidence traceability
}

pub fn export_acdi_json(report: &ACDIReport) -> Result<String, serde_json::Error> {
    // Export with:
    // - Schema validation
    // - Pretty-printing
    // - Digital signature (optional)
}
```

### 5.2 Sample ACDI Output

```json
{
  "schema_version": "1.0.0",
  "generated_at": "2025-11-18T14:00:00Z",
  "generator": {
    "name": "pqc-scanner",
    "version": "2025.11.18",
    "vendor": "ArcQubit",
    "standards": ["IEEE-P1943", "OMB-M-23-02", "NIST-SP-800-53"]
  },
  "systems": [
    {
      "system_id": "arcqubit-platform-prod",
      "system_name": "ArcQubit Production Platform",
      "fisma_level": "Moderate",
      "is_hva": true,
      "environment": "Production",
      "crypto_systems": [
        {
          "algorithm": {
            "name": "ML-KEM-768",
            "family": "MLKEM",
            "service_role": "KeyExchange",
            "parameter_set": "Kyber-768",
            "nist_category": "Category3"
          },
          "package": {
            "library_name": "liboqs",
            "version": "0.10.1",
            "vendor": "Open Quantum Safe",
            "pqc_capable": true
          },
          "pqc_status": {
            "mode": "HybridClassicalPQC",
            "quantum_safe": true,
            "migration_priority": "Low"
          }
        }
      ]
    }
  ]
}
```

---

## 6. Q-CMM Mapper

### 6.1 Implementation (`src/p1943/qcmm_mapper.rs`)

```rust
pub fn assess_qcmm_maturity(
    acdi_report: &ACDIReport,
    p1943_controls: &P1943ControlStatus,
) -> QCMMAssessment {
    // Map to Q-CMM levels:
    // Level 0: No PQC awareness
    // Level 1: Basic inventory exists
    // Level 2: Policy-driven approach
    // Level 3: Standardized processes
    // Level 4: Quantitative management
    // Level 5: Continuous optimization
}

pub fn calculate_domain_scores(
    controls: &P1943ControlStatus,
) -> QCMMDomainScores {
    // Score 5 domains:
    // - Governance: Policy, oversight, risk management
    // - Architecture: Crypto-agility, hybrid design
    // - Quantum Crypto: PQC adoption, migration progress
    // - Workforce: Training, expertise
    // - Performance: Testing, benchmarking, monitoring
}
```

### 6.2 Q-CMM Level Criteria

| Level | Name | Criteria | P1943 Controls Required |
|-------|------|----------|------------------------|
| **Level 0** | Pre-Quantum Ready | No PQC awareness, no crypto inventory | None |
| **Level 1** | Quantum Cryptography Compliant | P1943 referenced in policy, basic ACDI inventory, 80-90% coverage | P1943-INV-04, P1943-NET-ALG-01 |
| **Level 2** | Quantum Aware | Q-CMM roadmap exists, hybrid planning, downgrade controls | P1943-HYB-07, P1943-DOWN-08, P1943-POLICY-12 |
| **Level 3** | Quantum Enabled | Hybrid KEX/signatures in production, automated ACDI via CI/CD | All 12 controls, automated workflows |
| **Level 4** | Quantum Integrated | PQC standard in critical paths, continuous monitoring, dual cert chains | All controls + runtime monitoring |
| **Level 5** | Quantum Optimized | Crypto-agility automation, external benchmarking, continuous optimization | All controls + P7131 metrics integration |

---

## 7. Detection Algorithm Enhancements

### 7.1 Enhanced Regex Patterns (`src/p1943/network_detector.rs`)

```rust
lazy_static! {
    // PQC Algorithm Detection
    static ref ML_KEM_PATTERN: Regex = Regex::new(
        r"(?i)(ML[-_]?KEM|CRYSTALS[-_]?KYBER|kyber[-_]?(512|768|1024))"
    ).unwrap();

    static ref ML_DSA_PATTERN: Regex = Regex::new(
        r"(?i)(ML[-_]?DSA|CRYSTALS[-_]?DILITHIUM|dilithium[2357])"
    ).unwrap();

    static ref SLH_DSA_PATTERN: Regex = Regex::new(
        r"(?i)(SLH[-_]?DSA|SPHINCS\+|sphincs[-_]?(128|192|256)[sf])"
    ).unwrap();

    // TLS 1.3 with PQC
    static ref TLS_PQC_HYBRID: Regex = Regex::new(
        r"(?i)(TLS_|SSL_)(KYBER|MLKEM|HYBRID)_([A-Z0-9_]+)"
    ).unwrap();

    // Package/Library Detection
    static ref OPENSSL_PATTERN: Regex = Regex::new(
        r#"(?i)(openssl|libssl|libcrypto)["\s]*([0-9]+\.[0-9]+\.[0-9]+)?"#
    ).unwrap();

    static ref LIBOQS_PATTERN: Regex = Regex::new(
        r#"(?i)(liboqs|oqs)["\s]*([0-9]+\.[0-9]+\.[0-9]+)?"#
    ).unwrap();
}
```

### 7.2 TLS Cipher Suite Detection

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSConfiguration {
    pub protocol: ProtocolType,           // TLS 1.2, TLS 1.3, QUIC
    pub version: String,
    pub cipher_suites: Vec<CipherSuite>,
    pub key_exchange: Vec<String>,
    pub signatures: Vec<String>,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CipherSuite {
    pub name: String,                     // "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256"
    pub kex: String,                      // "ECDHE"
    pub auth: String,                     // "RSA"
    pub cipher: String,                   // "AES-128-GCM"
    pub mac: String,                      // "SHA256"
    pub quantum_safe: bool,
    pub pqc_hybrid: bool,
}

pub fn detect_tls_config(source: &str, language: &Language) -> Vec<TLSConfiguration> {
    // Detect TLS configurations from:
    // - SSLContext, TLSContext initialization
    // - Cipher suite strings
    // - TLS version constraints
    // - QUIC crypto configuration
}
```

---

## 8. GitHub Actions Integration

### 8.1 New Workflow: P1943 Compliance Scan

**File**: `.github/workflows/p1943-scan.yml`

```yaml
name: IEEE P1943 Compliance Scan

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]
  schedule:
    - cron: '0 0 * * 0'  # Weekly on Sunday

permissions:
  contents: read
  security-events: write

jobs:
  p1943-scan:
    name: P1943 ACDI Generation
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build pqc-scanner
        run: cargo build --release

      - name: Run P1943 Scan
        run: |
          ./target/release/pqc-scanner p1943-scan \
            --source-dir . \
            --output reports/p1943-acdi.json \
            --format acdi \
            --fisma-level moderate \
            --is-hva false \
            --qcmm-assessment

      - name: Generate Q-CMM Report
        run: |
          ./target/release/pqc-scanner qcmm-report \
            --input reports/p1943-acdi.json \
            --output reports/qcmm-maturity.json

      - name: Upload ACDI Artifact
        uses: actions/upload-artifact@v4
        with:
          name: p1943-acdi-report
          path: reports/p1943-acdi.json
          retention-days: 90

      - name: Upload Q-CMM Artifact
        uses: actions/upload-artifact@v4
        with:
          name: qcmm-maturity-report
          path: reports/qcmm-maturity.json
          retention-days: 90

      - name: Check Policy Compliance
        run: |
          ./target/release/pqc-scanner policy-check \
            --input reports/p1943-acdi.json \
            --policy .pqc-policy.yml \
            --fail-on-violations
```

### 8.2 Policy Configuration File

**File**: `.pqc-policy.yml`

```yaml
version: "1.0.0"
standard: "IEEE-P1943"

approved_algorithms:
  - name: "AES"
    min_key_size: 128
    approved_modes: ["GCM", "CCM"]

  - name: "ML-KEM"
    parameter_sets: ["ML-KEM-512", "ML-KEM-768", "ML-KEM-1024"]

  - name: "ML-DSA"
    parameter_sets: ["ML-DSA-44", "ML-DSA-65", "ML-DSA-87"]

deprecated_algorithms:
  - name: "3DES"
    sunset_date: "2025-12-31"
    migration_required: true

  - name: "RSA"
    conditions:
      - "minimum 3072-bit keys"
      - "legacy systems only"
    sunset_date: "2030-12-31"

prohibited_algorithms:
  - "MD5"
  - "SHA-1"
  - "DES"
  - "RC4"

hybrid_mode:
  required: true
  approved_combiners:
    - "concatenation"
    - "KDF-based"

minimum_key_sizes:
  RSA: 3072
  ECC: 256
  AES: 128

fisma_requirements:
  high:
    cmvp_required: true
    min_aes_key_size: 256
    pqc_required_by: "2026-12-31"
  moderate:
    cmvp_required: true
    min_aes_key_size: 128
    pqc_required_by: "2028-12-31"
```

---

## 9. Testing Strategy

### 9.1 Unit Tests

**File**: `tests/p1943_tests.rs`

```rust
#[cfg(test)]
mod p1943_tests {
    use super::*;

    #[test]
    fn test_ml_kem_detection() {
        let source = r#"
            import { Kyber768 } from 'pqc-kyber';
            const kem = new Kyber768();
        "#;

        let detections = detect_algorithms_p1943(source, &Language::JavaScript).unwrap();
        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].algorithm.family, AlgorithmFamily::MLKEM);
        assert_eq!(detections[0].algorithm.parameter_set, Some("Kyber-768".to_string()));
    }

    #[test]
    fn test_hybrid_mode_detection() {
        let source = r#"
            const hybrid = {
                classical: generateECDHKey(),
                pqc: generateKyberKey(),
                combiner: kdfCombine
            };
        "#;

        let hybrids = detect_hybrid_crypto(&analyze(source, "javascript").unwrap());
        assert_eq!(hybrids.len(), 1);
        assert_eq!(hybrids[0].mode, CryptoMode::HybridClassicalPQC);
    }

    #[test]
    fn test_tls_cipher_suite_extraction() {
        let source = r#"
            ctx.set_ciphersuites([
                "TLS_AES_256_GCM_SHA384",
                "TLS_CHACHA20_POLY1305_SHA256"
            ]);
        "#;

        let tls_configs = detect_tls_config(source, &Language::Rust).unwrap();
        assert_eq!(tls_configs[0].cipher_suites.len(), 2);
    }

    #[test]
    fn test_acdi_generation() {
        let audit = create_test_audit_result();
        let config = SystemConfig {
            system_id: "test-system".to_string(),
            fisma_level: FISMAImpactLevel::Moderate,
            is_hva: false,
        };

        let acdi = generate_acdi_report(&audit, &config).unwrap();
        assert_eq!(acdi.schema_version, "1.0.0");
        assert!(!acdi.systems.is_empty());
    }

    #[test]
    fn test_qcmm_scoring() {
        let acdi = load_test_acdi_report();
        let qcmm = assess_qcmm_maturity(&acdi, &P1943ControlStatus::default());

        // Level 1 if basic inventory exists
        assert!(qcmm.overall_level >= QCMMLevel::Level1);
    }
}
```

### 9.2 Test Fixtures

Create comprehensive test files in `tests/fixtures/p1943/`:

```
tests/fixtures/p1943/
├── pqc_algorithms.rs          # ML-KEM, ML-DSA, SLH-DSA samples
├── hybrid_tls.rs              # Hybrid TLS 1.3 configurations
├── legacy_crypto.py           # Classical + deprecated algorithms
├── network_protocols.js       # TLS/QUIC cipher suites
└── package_imports.ts         # Various crypto library imports
```

**Test Coverage Goal**: >90% for all new P1943 modules

---

## 10. Implementation Phases

### Phase 1: Type System Extension (Week 1-2)

**Deliverables:**
- Create `src/p1943/types.rs` with ACDI/Q-CMM types
- Extend `CryptoType` enum with PQC algorithms
- Add P1943-specific structures to `src/types.rs`
- Ensure backward compatibility with existing types

**Validation:**
- All existing tests pass
- New types compile without errors
- JSON serialization/deserialization works

### Phase 2: Detection Engine (Week 3-4)

**Deliverables:**
- Implement `network_detector.rs` for TLS/QUIC
- Implement `package_detector.rs` for library attribution
- Add PQC algorithm patterns to `detector.rs`
- Implement `hybrid_analyzer.rs`
- Implement `downgrade_detector.rs`

**Validation:**
- Unit tests for each detector (>90% coverage)
- Integration with existing audit flow
- Performance benchmarks (ensure <30% overhead)

### Phase 3: Control Implementation (Week 5-6)

**Deliverables:**
- Implement all 12 P1943 control area functions in `controls.rs`
- Create control assessment logic
- Implement evidence collection framework
- Add structured logging for audit trail

**Validation:**
- Each control has dedicated tests
- Evidence properly linked to controls
- Structured logs validate against schema

### Phase 4: ACDI Schema (Week 7)

**Deliverables:**
- Implement ACDI report generator in `acdi_schema.rs`
- Add JSON schema validation
- Implement export functions
- Test OMB M-23-02 compliance

**Validation:**
- ACDI JSON validates against schema
- All ACDI fields populated correctly
- Export/import roundtrip works

### Phase 5: Q-CMM Mapping (Week 8)

**Deliverables:**
- Implement Q-CMM maturity assessment in `qcmm_mapper.rs`
- Create domain scoring logic (5 domains)
- Map P1943 controls to Q-CMM levels
- Generate actionable recommendations

**Validation:**
- Q-CMM levels correctly assigned
- Domain scores accurate
- Recommendations align with gaps

### Phase 6: Integration & Testing (Week 9-10)

**Deliverables:**
- Integrate with existing NIST/ITSG33 modules
- Add CLI commands (`p1943-scan`, `qcmm-report`, `policy-check`)
- Write comprehensive test suite (50+ unit, 20+ integration)
- Update GitHub Actions workflows
- Create documentation

**Validation:**
- All integration tests pass
- CLI commands functional
- GitHub Actions workflows run successfully
- Documentation complete and accurate

### Phase 7: Buffer & Refinement (Week 11-12)

**Deliverables:**
- Address feedback and bug fixes
- Performance optimization
- Documentation polish
- Release preparation

---

## 11. CLI Enhancements

### 11.1 New Commands

```bash
# P1943 ACDI compliance scan
pqc-scanner p1943-scan \
  --source-dir ./src \
  --output acdi-report.json \
  --fisma-level moderate \
  --is-hva false \
  --include-qcmm

# Q-CMM maturity assessment
pqc-scanner qcmm-report \
  --input acdi-report.json \
  --output qcmm-maturity.json \
  --format json

# Policy compliance check
pqc-scanner policy-check \
  --input acdi-report.json \
  --policy .pqc-policy.yml \
  --fail-on-violations

# Unified scan (NIST + ITSG33 + P1943)
pqc-scanner unified-scan \
  --source-dir ./src \
  --standards nist,itsg33,p1943 \
  --output unified-report.json \
  --classification protected-a \
  --fisma-level moderate
```

### 11.2 CLI Structure (`src/bin/pqc-scanner.rs`)

```rust
#[derive(Parser)]
enum Command {
    // Existing
    Scan(ScanArgs),

    // NEW
    P1943Scan(P1943ScanArgs),
    QcmmReport(QcmmArgs),
    PolicyCheck(PolicyCheckArgs),
    UnifiedScan(UnifiedScanArgs),
}

#[derive(Args)]
struct P1943ScanArgs {
    #[arg(short, long)]
    source_dir: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(long, value_enum)]
    fisma_level: FISMAImpactLevel,

    #[arg(long)]
    is_hva: bool,

    #[arg(long)]
    include_qcmm: bool,

    #[arg(long)]
    policy: Option<PathBuf>,
}
```

---

## 12. Documentation Requirements

### 12.1 New Documentation Files

```
docs/p1943/
├── README.md                      # P1943 integration overview
├── control-matrix.md              # 12 control area details
├── acdi-schema.md                 # ACDI JSON schema specification
├── qcmm-mapping.md                # Q-CMM level criteria
├── migration-guide.md             # NIST/ITSG33 to P1943 migration
├── policy-configuration.md        # Policy YAML format guide
└── examples/
    ├── acdi-report-sample.json
    ├── qcmm-assessment-sample.json
    └── policy-config-sample.yml
```

### 12.2 API Documentation

- Rustdoc comments for all public P1943 APIs
- WASM export documentation
- JSON schema documentation (OpenAPI/JSON Schema spec)
- Example code snippets for each control area

### 12.3 User Guides

- **Quick Start**: P1943 scanning in 5 minutes
- **Control Reference**: Detailed explanation of each control
- **Q-CMM Maturity Guide**: How to advance from Level 1 to Level 5
- **Policy Management**: Creating and maintaining crypto policies

---

## 13. Performance Considerations

### 13.1 Expected Performance Impact

- **Single file scan**: +15-25% processing time (multi-pass analysis)
- **Directory scan**: +20-30% (additional file I/O for package detection)
- **WASM bundle size**: +40-60KB (new types and regex patterns)
- **Memory overhead**: +10-20% (additional data structures)

### 13.2 Optimization Strategies

1. **Lazy regex compilation**: Use `once_cell` for all new patterns
2. **Parallel file processing**: Leverage `rayon` for directory scans
3. **Incremental analysis**: Cache previous scan results
4. **WASM optimization**: Tree-shake unused P1943 features for browser builds
5. **Smart detection**: Skip P1943 controls if not needed (opt-in basis)

### 13.3 Performance Benchmarks

```bash
# Before P1943 integration
pqc-scanner scan ./pqc-scanner-samples/polyglot-app/
Time: 1.2s, Memory: 42MB

# After P1943 integration (all controls enabled)
pqc-scanner p1943-scan ./pqc-scanner-samples/polyglot-app/
Target Time: <1.6s, Memory: <50MB
```

---

## 14. Success Metrics

### 14.1 Functional Metrics

- ✅ All 12 P1943 controls implemented: 100%
- ✅ ACDI schema validation pass rate: 100%
- ✅ Q-CMM accuracy (vs. manual assessment): >90%
- ✅ Policy engine coverage: >95%
- ✅ Test coverage: >90% for all new modules

### 14.2 Performance Metrics

- ✅ Scan time increase: <30%
- ✅ WASM bundle size increase: <100KB
- ✅ Memory overhead: <20%
- ✅ False positive rate: <5%

### 14.3 Adoption Metrics

- GitHub Action usage (weekly runs)
- ACDI report downloads
- Community contributions to policy templates
- Integration with CI/CD pipelines
- User satisfaction surveys

### 14.4 Quality Metrics

- Zero critical bugs in production
- Documentation completeness: 100%
- API stability (no breaking changes)
- Backward compatibility: 100%

---

## 15. Dependencies

### 15.1 External Dependencies (New)

**Add to `Cargo.toml`:**

```toml
[dependencies]
# Existing dependencies remain unchanged

# NEW for P1943
yaml-rust = "0.4"              # Policy file parsing
jsonschema = "0.17"            # ACDI schema validation
git2 = "0.18"                  # Key lifecycle analysis (optional)
sysinfo = "0.30"               # Platform detection
once_cell = "1.19"             # Lazy regex compilation
```

### 15.2 Data Requirements

- **IEEE P1943 standard** (when published): Control definitions and requirements
- **OMB M-23-02 ACDI schema**: JSON schema specification
- **Q-CMM criteria document**: Maturity level definitions
- **NIST PQC parameter sets**: ML-KEM, ML-DSA, SLH-DSA specifications
- **TLS cipher suite database**: IANA registry for validation

### 15.3 Tool Requirements

- Rust 1.70+ (stable toolchain)
- wasm-pack 0.12+ (WASM compilation)
- Node.js 18+ (for WASM testing)
- GitHub Actions (CI/CD)

---

## 16. Backward Compatibility Strategy

### 16.1 Unified Compliance Report

```rust
// Combines all three compliance standards
pub fn generate_unified_compliance_report(
    audit_result: &AuditResult,
    config: &UnifiedConfig,
) -> UnifiedComplianceReport {
    UnifiedComplianceReport {
        // Existing standards
        nist_sc13: generate_sc13_report(audit_result, config.file_path),
        itsg33: generate_itsg33_report(audit_result, config.classification, config.file_path),

        // NEW standard
        p1943: generate_p1943_report(audit_result, &config.p1943_config),

        // Cross-mapping
        control_mapping: generate_control_cross_reference(),
    }
}
```

### 16.2 Opt-In P1943 Features

- P1943 scanning is **opt-in** via CLI flag or config
- Existing workflows continue to work without changes
- Unified reports combine all standards seamlessly
- No breaking changes to existing APIs

### 16.3 Migration Path

1. **Week 1-2**: Existing users see no changes (P1943 not enabled by default)
2. **Week 3-4**: Early adopters test P1943 via `--enable-p1943` flag
3. **Week 5-6**: P1943 becomes recommended for new projects
4. **Week 7+**: P1943 included in default unified scans (optional)

---

## 17. Risk Assessment & Mitigation

### 17.1 Technical Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Performance degradation >30% | Medium | High | Aggressive optimization, lazy evaluation, opt-in controls |
| WASM bundle size bloat | Medium | Medium | Tree-shaking, feature flags, split bundles |
| Regex pattern false positives | Medium | Medium | Extensive testing, confidence scoring, user feedback |
| Schema validation failures | Low | High | Comprehensive schema tests, validation library |
| Backward compatibility breaks | Low | Critical | Strict versioning, deprecation warnings, extensive testing |

### 17.2 Schedule Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| P1943 standard not finalized | High | Medium | Use draft spec, plan for changes |
| Dependency vulnerabilities | Medium | Medium | Regular audits, `cargo audit` in CI |
| Testing bottleneck | Medium | High | Parallel test execution, early test writing |
| Documentation lag | Medium | Medium | Documentation-driven development |

### 17.3 Adoption Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Complex configuration | Medium | Medium | Sensible defaults, wizard tool, templates |
| Unclear value proposition | Low | High | Clear documentation, case studies, benchmarks |
| Migration friction | Medium | Medium | Gradual rollout, migration guide, support |

---

## 18. Deliverables Summary

### 18.1 Code Deliverables (~3,000 LOC)

1. **`src/p1943/` module** (10 files, ~2,000 LOC)
   - `mod.rs`, `types.rs`, `controls.rs`, `acdi_schema.rs`, `qcmm_mapper.rs`
   - `network_detector.rs`, `package_detector.rs`, `platform_detector.rs`
   - `hybrid_analyzer.rs`, `downgrade_detector.rs`, `key_lifecycle.rs`, `policy_engine.rs`

2. **Enhanced existing modules** (~500 LOC)
   - `src/types.rs`: Extended `CryptoType` enum
   - `src/lib.rs`: WASM exports for P1943
   - `src/bin/pqc-scanner.rs`: New CLI commands

3. **Configuration and data** (~500 LOC)
   - `.pqc-policy.yml` template
   - P1943 control definitions JSON
   - Q-CMM criteria database

### 18.2 Test Deliverables (~1,500 LOC)

1. **Unit tests** (`tests/p1943_tests.rs`, ~800 LOC)
   - 50+ tests covering all 12 controls
   - Algorithm detection tests
   - ACDI generation tests
   - Q-CMM scoring tests

2. **Integration tests** (`tests/integration_tests.rs`, ~400 LOC)
   - End-to-end P1943 scanning
   - Unified compliance reporting
   - GitHub Actions workflow simulation

3. **Test fixtures** (~300 LOC)
   - PQC algorithm samples
   - Hybrid TLS configurations
   - Network protocol examples

### 18.3 CI/CD Deliverables

1. **`.github/workflows/p1943-scan.yml`**
   - Automated P1943 scanning
   - ACDI report generation
   - Q-CMM assessment
   - Policy violation checking

2. **`.github/workflows/p1943-test.yml`**
   - P1943 module test suite
   - Performance benchmarking
   - Schema validation

### 18.4 Documentation Deliverables

1. **Technical documentation** (`docs/p1943/`)
   - `README.md`: Overview and quick start
   - `control-matrix.md`: Detailed control specifications
   - `acdi-schema.md`: ACDI JSON schema reference
   - `qcmm-mapping.md`: Q-CMM maturity criteria
   - `migration-guide.md`: Upgrade path from NIST/ITSG33
   - `policy-configuration.md`: Policy YAML format guide

2. **API documentation**
   - Rustdoc for all public APIs
   - WASM export reference
   - JSON schema specifications

3. **Examples**
   - Sample ACDI reports
   - Sample Q-CMM assessments
   - Policy configuration templates

---

## 19. Timeline & Milestones

### Overall Timeline: 10-12 weeks

| Week | Phase | Deliverables | Validation |
|------|-------|--------------|------------|
| **1-2** | Type System Extension | P1943 types, extended `CryptoType` | All existing tests pass, new types compile |
| **3-4** | Detection Engine | Network/package/PQC detectors | Unit tests >90% coverage, performance <30% overhead |
| **5-6** | Control Implementation | All 12 P1943 controls | Each control tested, evidence collection works |
| **7** | ACDI Schema | ACDI generator, JSON schema validation | Schema validates, OMB M-23-02 compliant |
| **8** | Q-CMM Mapping | Q-CMM assessment, domain scoring | Q-CMM levels accurate, recommendations actionable |
| **9-10** | Integration & Testing | CLI, GitHub Actions, comprehensive tests | All integration tests pass, workflows functional |
| **11-12** | Buffer & Refinement | Bug fixes, optimization, documentation | Production-ready, documentation complete |

### Key Milestones

- ✅ **Week 2**: Type system complete, backward compatible
- ✅ **Week 4**: All detectors functional, performance acceptable
- ✅ **Week 6**: All 12 controls implemented and tested
- ✅ **Week 7**: ACDI JSON export working
- ✅ **Week 8**: Q-CMM maturity assessment complete
- ✅ **Week 10**: Integration complete, ready for beta
- ✅ **Week 12**: Production release

---

## 20. Conclusion

This implementation plan provides a comprehensive roadmap for integrating IEEE P1943 standard support into the pqc-scanner project. The plan ensures:

1. **Full P1943 Compliance**: All 12 control areas implemented
2. **ACDI JSON Schema**: OMB M-23-02 alignment for government reporting
3. **Q-CMM Maturity Model**: Quantum readiness assessment (Levels 0-5)
4. **Backward Compatibility**: Existing NIST/ITSG33 features preserved
5. **Performance**: <30% overhead with optimization strategies
6. **Quality**: >90% test coverage, comprehensive documentation
7. **Adoption**: Clear migration path, sensible defaults, excellent docs

The modular architecture allows for incremental development, testing, and deployment. Each phase delivers tangible value and can be validated independently, reducing risk and enabling iterative improvement.

**Next Steps:**
1. Review and approve this implementation plan
2. Set up project tracking (GitHub Projects or similar)
3. Begin Phase 1: Type System Extension
4. Establish weekly progress reviews
5. Engage with IEEE P1943 working group for standard updates

---

**Document Version**: 1.0
**Last Updated**: 2025-11-18
**Author**: ArcQubit Engineering Team
**Reviewers**: [To be assigned]
**Approval**: [Pending]
