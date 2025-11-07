# Rust Crypto Libraries & WASM Compatibility Research Report
**Phase 1: Core Infrastructure Research**
**Scout Explorer Mission Report**
**Date: 2025-11-06**

---

## Executive Summary

This report provides comprehensive research on Rust cryptography libraries, WASM compatibility requirements, and quantum-safe cryptography patterns for building a secure crypto audit tool. Key findings include recommended libraries, WASM compatibility checklists, and detection patterns for quantum-vulnerable algorithms.

---

## 1. Rust Crypto Libraries Analysis

### 1.1 Ring Library

**Overview:**
- Safe, fast cryptography using BoringSSL primitives
- Version: 0.16.15 (stable)
- Wide adoption in Rust ecosystem

**WASM Compatibility:**
- ❌ **Limited native WASM support**
- ⚠️ Compilation fails for embedded targets (thumbv7em-none-eabi)
- ✅ **WASI variant available** (`ring-wasi` crate)
  - Supports: `wasm32-wasi` and `wasm32-wasix`
  - Precompiled with Zig 0.11 (no C compiler needed)
- ❌ **Browser WASM issues**: Import errors with `ring::signature` prevent browser loading

**no_std Support:**
- ❌ **Not natively no_std**
- Requires "alloc" feature (heap allocation for RSA)
- Requires "std" feature for libstd functionality

**Verdict:** ⚠️ Not recommended for browser WASM. Consider for WASI environments only.

---

### 1.2 Rustls (TLS Library)

**Overview:**
- Modern TLS library in pure Rust
- Traditionally based on Ring library

**WASM Compatibility:**
- ❌ **Cannot compile to standard WASM** (Ring dependency)
- ✅ **WasmEdge-specific solutions exist**
  - Requires patching with WasmEdge socket API
  - Use `reqwest` with `rustls-tls` features
  - Import `hyper-rustls`, `rustls`, `webpki-roots` with patches

**Browser Support:**
- ❌ **No native browser WASM support** with wasm-bindgen
- Alternative projects exist using `rust-crypto` and `wasi-crypto`

**Verdict:** ❌ Not recommended for browser-based WASM crypto audit tool.

---

### 1.3 RustCrypto Suite ⭐ **RECOMMENDED**

**Overview:**
- Pure Rust implementations of cryptographic algorithms
- Modular crate ecosystem
- Designed for embedded and constrained environments

**Key Crates:**

#### AES (Advanced Encryption Standard)
- ✅ Pure Rust implementation
- ✅ Optional hardware acceleration (AES-NI, ARMv8)
- ✅ no_std compatible
- ✅ WASM compatible

#### SHA-2 (Secure Hash Algorithm)
- ✅ Pure Rust SHA-224, SHA-256, SHA-384, SHA-512
- ✅ no_std compatible
- ✅ WASM compatible

#### ChaCha20 (Stream Cipher)
- ✅ Pure Rust using RustCrypto `cipher` traits
- ✅ Optional hardware acceleration (AVX2, SSE2)
- ✅ Portable implementation works everywhere including WASM
- ✅ no_std compatible
- ✅ Mobile support (Android, iOS)

#### ChaCha20Poly1305 (AEAD)
- ✅ Pure Rust RFC 8439 implementation
- ✅ Optional hardware acceleration
- ✅ no_std compatible
- ✅ WASM compatible

**Verdict:** ✅ **HIGHLY RECOMMENDED** - Pure Rust, full WASM support, no_std compatible

---

### 1.4 Post-Quantum Cryptography Libraries ⭐ **RECOMMENDED**

#### pqc_kyber (ML-KEM / Kyber)
**Overview:**
- Rust implementation of NIST ML-KEM (Kyber) post-quantum KEM
- Developed by Avarok Cybersecurity

**WASM Compatibility:**
- ✅ **Compiles to WASM** using wasm-bindgen
- ✅ **Pre-built NPM package available**
- ✅ Enable `wasm` feature for compilation
- ✅ no_std compatible
- ✅ No allocator required (embedded-friendly)
- ✅ Pure Rust, no unsafe code in reference implementation

**Usage:**
```bash
# Compile WASM yourself
wasm-pack build -- --features "wasm"

# Or use pre-built NPM package
npm install pqc_kyber
```

#### pqc_dilithium (ML-DSA / Dilithium)
**Overview:**
- Rust implementation of NIST ML-DSA (Dilithium) post-quantum signatures

**WASM Compatibility:**
- ✅ **Compiles to WASM** using wasm-bindgen
- ✅ Enable `wasm` feature for compilation
- ✅ Outputs to `./pkg/` directory
- ✅ Supports different modes (mode2, mode3, mode5)

**Usage:**
```bash
# Default compilation
wasm-pack build -- --features "wasm"

# Specific mode
wasm-pack build --out-dir pkg_mode5/ -- --features "wasm mode5"
```

**Verdict:** ✅ **HIGHLY RECOMMENDED** - Full WASM support, NIST-standardized, no_std compatible

---

## 2. WASM Compatibility Requirements

### 2.1 Core Requirements Checklist

#### Must-Have Features:
- ✅ **Pure Rust implementation** (no C dependencies)
- ✅ **no_std support** (optional but highly recommended)
- ✅ **wasm-bindgen compatibility**
- ✅ **No OS syscalls** (no std::fs, std::net)
- ✅ **No file I/O operations**
- ✅ **Portable across WASM runtimes**

#### Nice-to-Have Features:
- ⭐ Pre-built WASM binaries (NPM packages)
- ⭐ Hardware acceleration (when available)
- ⭐ Mobile platform support
- ⭐ SIMD support

---

### 2.2 Memory Management Best Practices

#### Key Considerations:

**1. Ownership and Allocation:**
- Memory passed from Rust to JavaScript remains Rust-owned
- Must explicitly free Rust-allocated memory from JS
- Use wasm-bindgen's generated free functions
- Libraries like wasm-bindgen handle most allocation/deallocation

**2. Performance Trade-offs:**
- wasm-bindgen: Convenience for JS-heavy projects
- Raw WASM: Higher performance for compute-intensive operations
- Data copying: Receiving JS arrays as `&[f32]` copies data into Rust memory

**3. Allocator Options:**
- **dlmalloc**: Traditional WASM allocator
- **Talc** (2025): Smaller and faster alternative
- Use `alloc` crate for Vec/String in no_std

**4. Crypto-Specific Guidance:**
- Start with performance-critical modules (crypto, hashing, math, Merkle proofs)
- Minimize data copying across WASM boundary
- Use zero-copy patterns when possible
- Consider memory-safe alternatives to avoid allocations

---

### 2.3 Avoiding OS Syscalls

#### WASM Constraints:

**What's Available:**
- ✅ `std` crate (partial implementation)
- ✅ Vec, String, HashSet
- ✅ Basic collections and memory allocation
- ✅ Math operations and algorithms

**What's NOT Available:**
- ❌ File I/O (`std::fs`)
- ❌ Network operations (`std::net`)
- ❌ OS-specific features
- ❌ Threading (single-threaded model)

#### Best Practices:

**1. Feature Flags for Conditional Compilation:**
```toml
[features]
default = []
std = []
```

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::fs::File;

#[cfg(not(feature = "std"))]
// Alternative implementation without File I/O
```

**2. no_std Pattern:**
- Use `#![no_std]` attribute
- Import `alloc` crate for heap allocations
- Replace std types with core/alloc equivalents
- Test with `wasm32-unknown-unknown` target

**3. Modern Optimization:**
- Modern tools can strip unused code
- no_std may be optional for WASM-only targets
- Binary size concerns less critical with tree-shaking

---

### 2.4 serde-wasm-bindgen Best Practices

**Efficient JS Interop:**
- More efficient than JSON serialization
- Direct serialization between Rust and JavaScript
- Use for complex data structures

**Patterns:**
```rust
#[wasm_bindgen]
pub fn analyze_crypto(data: JsValue) -> Result<JsValue, JsValue> {
    let input: InputData = serde_wasm_bindgen::from_value(data)?;
    let result = perform_analysis(input);
    Ok(serde_wasm_bindgen::to_value(&result)?)
}
```

**Performance Tips:**
- Minimize boundary crossings
- Batch operations when possible
- Use `js-sys` and `web-sys` for browser APIs
- Handle errors with Result<T, JsValue>

---

## 3. NIST Post-Quantum Cryptography Standards

### 3.1 Published Standards (August 2024)

**Three Finalized Standards:**
- **FIPS 203**: ML-KEM (Kyber) - Key Encapsulation
- **FIPS 204**: ML-DSA (Dilithium) - Digital Signatures
- **FIPS 205**: SLH-DSA (SPHINCS+) - Hash-Based Signatures

**Additional Standard (March 2025):**
- **HQC**: Hamming Quasi-Cyclic - Key Encapsulation/Exchange

---

### 3.2 ML-KEM (Module-Lattice-Based Key Encapsulation Mechanism)

**Former Name:** CRYSTALS-Kyber

**Purpose:** Key encapsulation for secure key exchange

**Status:** Primary standard for post-quantum key establishment

**Implementation:**
- Lattice-based mathematics
- NIST standardized August 2024
- Rust crate: `pqc_kyber`
- WASM compatible

**Detection Pattern:**
```rust
// Look for Kyber/ML-KEM usage
"kyber" | "ml-kem" | "crystals-kyber"
```

---

### 3.3 ML-DSA (Module-Lattice-Based Digital Signature Algorithm)

**Former Name:** CRYSTALS-Dilithium

**Purpose:** Digital signatures for authentication and integrity

**Status:** **Primary standard** for post-quantum signatures (NIST recommendation)

**Implementation:**
- Lattice-based mathematics
- NIST FIPS 204 standardized August 2024
- Rust crate: `pqc_dilithium`
- WASM compatible

**NIST Guidance:** ML-DSA should be the default choice for post-quantum signatures in most situations.

**Detection Pattern:**
```rust
// Look for Dilithium/ML-DSA usage
"dilithium" | "ml-dsa" | "crystals-dilithium"
```

---

### 3.4 SLH-DSA (Stateless Hash-Based Digital Signature Algorithm)

**Former Name:** SPHINCS+

**Purpose:** Digital signatures (cryptographic backup)

**Status:** Backup method in case ML-DSA is compromised

**Implementation:**
- Hash-based mathematics (different from lattice-based)
- NIST FIPS 205 standardized August 2024
- Selected as cryptographic fallback

**Use Case:** Alternative if breakthrough compromises lattice-based schemes

**Detection Pattern:**
```rust
// Look for SPHINCS+/SLH-DSA usage
"sphincs" | "slh-dsa" | "sphincs+"
```

---

### 3.5 Quantum-Vulnerable Algorithms ⚠️

**Algorithms Broken by Shor's Algorithm:**

#### RSA (Rivest-Shamir-Adleman)
- **Vulnerability:** Quantum computers can factor large integers in polynomial time
- **Impact:** Complete break of RSA encryption and signatures
- **Timeline:** Vulnerable once large-scale quantum computers exist

**Detection Pattern:**
```rust
// RSA usage patterns
use rsa::*;
RSA::new()
RsaPrivateKey::
RsaPublicKey::
"-----BEGIN RSA"
PKCS1::
```

#### DSA (Digital Signature Algorithm)
- **Vulnerability:** Discrete logarithm problem solved by Shor's algorithm
- **Impact:** Signature forgery possible
- **Timeline:** Vulnerable to quantum attacks

**Detection Pattern:**
```rust
// DSA usage patterns
use dsa::*;
DSA::
DsaPrivateKey::
DsaPublicKey::
SigningKey::
VerifyingKey::
```

#### ECDSA (Elliptic Curve Digital Signature Algorithm)
- **Vulnerability:** Elliptic curve discrete logarithm solved by Shor's algorithm
- **Impact:** Complete break of ECDSA signatures
- **Timeline:** Vulnerable to quantum attacks

**Detection Pattern:**
```rust
// ECDSA usage patterns
use ecdsa::*;
ECDSA::
EcdsaPrivateKey::
EcdsaPublicKey::
Signature::
p256::
secp256k1::
```

#### ECDH (Elliptic Curve Diffie-Hellman)
- **Vulnerability:** Key exchange broken by quantum computers
- **Impact:** Past communications can be decrypted (store-now-decrypt-later)
- **Timeline:** Immediate threat for long-term secret data

**Detection Pattern:**
```rust
// ECDH/ECC key exchange patterns
use ecdh::*;
EphemeralSecret::
PublicKey::
SharedSecret::
diffie_hellman
```

#### DH/DHE (Diffie-Hellman Key Exchange)
- **Vulnerability:** Discrete logarithm problem
- **Impact:** Key exchange security broken
- **Timeline:** Vulnerable to quantum attacks

**Detection Pattern:**
```rust
// Diffie-Hellman patterns
use dh::*;
DiffieHellman::
DhPrivateKey::
DhPublicKey::
```

---

### 3.6 Quantum-Safe Algorithms ✅

**Still Secure Against Quantum Computers:**

#### Symmetric Ciphers
- AES-128, AES-192, AES-256
- ChaCha20, ChaCha20-Poly1305
- **Recommendation:** Double key sizes (AES-256 instead of AES-128)

#### Hash Functions
- SHA-256, SHA-384, SHA-512, SHA-3
- BLAKE2, BLAKE3
- **Recommendation:** Use SHA-384 or SHA-512 for long-term security

#### MAC Algorithms
- HMAC-SHA256, HMAC-SHA512
- Poly1305
- **Safe against quantum attacks**

**Detection Pattern:**
```rust
// Quantum-safe symmetric crypto
"aes" | "chacha20" | "sha256" | "sha512" | "hmac" | "blake"
```

---

## 4. Detection Patterns Summary

### 4.1 Quantum-Vulnerable Algorithm Detection

```rust
// High-priority quantum-vulnerable patterns to flag
pub const QUANTUM_VULNERABLE_PATTERNS: &[&str] = &[
    // RSA
    "use rsa::",
    "RSA::",
    "RsaPrivateKey",
    "RsaPublicKey",
    "PKCS1",
    "-----BEGIN RSA",

    // DSA
    "use dsa::",
    "DSA::",
    "DsaPrivateKey",
    "DsaPublicKey",

    // ECDSA
    "use ecdsa::",
    "ECDSA::",
    "p256::",
    "secp256k1::",
    "EcdsaPrivateKey",
    "EcdsaPublicKey",

    // ECDH / ECC
    "use ecdh::",
    "EphemeralSecret",
    "diffie_hellman",

    // DH
    "use dh::",
    "DiffieHellman",
    "DhPrivateKey",
];
```

### 4.2 Post-Quantum Algorithm Detection

```rust
// Post-quantum crypto patterns to identify (good!)
pub const POST_QUANTUM_PATTERNS: &[&str] = &[
    // NIST ML-KEM (Kyber)
    "pqc_kyber",
    "ml-kem",
    "kyber",
    "crystals-kyber",

    // NIST ML-DSA (Dilithium)
    "pqc_dilithium",
    "ml-dsa",
    "dilithium",
    "crystals-dilithium",

    // NIST SLH-DSA (SPHINCS+)
    "sphincs",
    "slh-dsa",
    "sphincs+",

    // Other PQC
    "pqcrypto",
    "post-quantum",
    "lattice-based",
];
```

### 4.3 WASM Compatibility Detection

```rust
// WASM-incompatible patterns
pub const WASM_INCOMPATIBLE_PATTERNS: &[&str] = &[
    // File I/O
    "std::fs::",
    "File::open",
    "File::create",
    "std::io::Write",
    "std::io::Read",

    // Network I/O
    "std::net::",
    "TcpStream",
    "UdpSocket",

    // Threading
    "std::thread",
    "std::sync::Mutex",

    // Process operations
    "std::process",
    "Command::",
];

// WASM-compatible patterns
pub const WASM_COMPATIBLE_PATTERNS: &[&str] = &[
    "#![no_std]",
    "wasm-bindgen",
    "wasm_bindgen::",
    "#[wasm_bindgen]",
    "pure rust",
    "no unsafe",
];
```

---

## 5. Recommended Libraries for WASM Crypto Audit

### 5.1 Core Crypto Libraries ⭐

**Tier 1 - Highly Recommended:**
1. **RustCrypto Suite**
   - AES, SHA-2, ChaCha20, ChaCha20Poly1305
   - Pure Rust, no_std, WASM compatible
   - Use: Analyzing symmetric crypto usage

2. **pqc_kyber**
   - NIST ML-KEM implementation
   - WASM compatible, no_std
   - Use: Detecting/recommending post-quantum key exchange

3. **pqc_dilithium**
   - NIST ML-DSA implementation
   - WASM compatible, no_std
   - Use: Detecting/recommending post-quantum signatures

**Tier 2 - Conditional Use:**
4. **ring-wasi** (NOT ring)
   - Only for WASI environments
   - Not for browser WASM
   - Use: WASI-specific crypto analysis

### 5.2 Supporting Libraries

**Parser and Analysis:**
- `syn` - Rust source code parsing
- `proc-macro2` - Token processing
- `quote` - Code generation

**WASM Infrastructure:**
- `wasm-bindgen` - JS interop
- `serde-wasm-bindgen` - Data serialization
- `js-sys` - JavaScript standard library bindings
- `web-sys` - Web API bindings

**Utilities:**
- `regex` - Pattern matching
- `serde` - Serialization framework
- `serde_json` - JSON support

---

## 6. Example Detection Code Patterns

### 6.1 Detecting RSA Usage

```rust
use syn::{visit::Visit, Item, ItemUse};

pub struct CryptoDetector {
    pub rsa_found: bool,
    pub locations: Vec<String>,
}

impl<'ast> Visit<'ast> for CryptoDetector {
    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        let use_path = quote::quote!(#node).to_string();

        if use_path.contains("rsa::") || use_path.contains("RSA") {
            self.rsa_found = true;
            self.locations.push(format!("RSA usage: {}", use_path));
        }

        syn::visit::visit_item_use(self, node);
    }
}
```

### 6.2 Detecting ECDSA Usage

```rust
pub fn detect_ecdsa(source: &str) -> Vec<String> {
    let patterns = vec![
        regex::Regex::new(r"use\s+ecdsa::").unwrap(),
        regex::Regex::new(r"p256::").unwrap(),
        regex::Regex::new(r"secp256k1::").unwrap(),
    ];

    let mut findings = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        for pattern in &patterns {
            if pattern.is_match(line) {
                findings.push(format!(
                    "Line {}: ECDSA detected - {}",
                    line_num + 1,
                    line.trim()
                ));
            }
        }
    }

    findings
}
```

### 6.3 Detecting WASM-Incompatible Patterns

```rust
pub fn check_wasm_compatibility(source: &str) -> Vec<String> {
    let incompatible = vec![
        (r"std::fs::", "File I/O not supported in WASM"),
        (r"std::net::", "Network I/O not supported in WASM"),
        (r"std::thread", "Threading not supported in browser WASM"),
    ];

    let mut issues = Vec::new();

    for (pattern_str, message) in incompatible {
        let pattern = regex::Regex::new(pattern_str).unwrap();

        for (line_num, line) in source.lines().enumerate() {
            if pattern.is_match(line) {
                issues.push(format!(
                    "Line {}: {} - {}",
                    line_num + 1,
                    message,
                    line.trim()
                ));
            }
        }
    }

    issues
}
```

---

## 7. Key Recommendations

### 7.1 For Building WASM Crypto Audit Tool

**DO:**
- ✅ Use RustCrypto suite (AES, SHA, ChaCha20)
- ✅ Use pqc_kyber and pqc_dilithium for PQC analysis
- ✅ Target `wasm32-unknown-unknown`
- ✅ Use wasm-bindgen for JS interop
- ✅ Implement pure Rust, no_std compatible code
- ✅ Use serde-wasm-bindgen for efficient data exchange
- ✅ Minimize WASM boundary crossings

**DON'T:**
- ❌ Don't use Ring for browser WASM
- ❌ Don't use Rustls (Ring dependency)
- ❌ Don't use std::fs or std::net
- ❌ Don't expect threading support
- ❌ Don't use C dependencies
- ❌ Don't copy large data across WASM boundary

### 7.2 Detection Priorities

**High Priority:**
1. RSA, DSA, ECDSA, ECDH (quantum-vulnerable)
2. WASM-incompatible patterns (std::fs, std::net)
3. Unsafe code in crypto contexts

**Medium Priority:**
4. Weak key sizes (RSA < 2048, AES-128)
5. Deprecated algorithms (MD5, SHA-1)
6. Missing post-quantum alternatives

**Low Priority:**
7. Code style issues
8. Performance optimizations
9. Documentation quality

### 7.3 Migration Path Recommendations

**For Projects Using Quantum-Vulnerable Crypto:**

1. **Immediate Actions:**
   - Audit all RSA/ECDSA usage
   - Identify long-term secret data
   - Plan migration timeline

2. **Short-term (2025):**
   - Implement hybrid crypto (classical + PQC)
   - Add ML-KEM for key exchange
   - Add ML-DSA for signatures

3. **Long-term (2026+):**
   - Complete migration to PQC
   - Remove classical vulnerable algorithms
   - Update security policies

---

## 8. Implementation Roadmap

### Phase 1: Parser Development ✅ (This Research)
- Research complete
- Libraries identified
- Detection patterns defined

### Phase 2: Core Detection Engine
- Build AST parser with `syn`
- Implement pattern matching
- Create quantum-vulnerable algorithm detector
- Implement WASM compatibility checker

### Phase 3: WASM Compilation
- Set up wasm-bindgen infrastructure
- Implement JS interop layer
- Build web interface
- Test in browser environment

### Phase 4: Reporting & Recommendations
- Generate audit reports
- Provide migration guidance
- Suggest PQC alternatives
- Export findings (JSON, HTML, MD)

---

## 9. Conclusion

This research establishes a solid foundation for building a WASM-compatible crypto audit tool. Key findings:

1. **RustCrypto suite** is the best choice for WASM crypto operations
2. **pqc_kyber** and **pqc_dilithium** provide NIST-standardized PQC with full WASM support
3. **Ring** and **Rustls** should be avoided for browser WASM
4. **RSA, DSA, ECDSA, ECDH** are quantum-vulnerable and must be flagged
5. **WASM requires careful attention** to avoid OS syscalls and manage memory efficiently

The recommended approach uses pure Rust, no_std compatible libraries that compile to WASM seamlessly, enabling comprehensive crypto auditing directly in the browser.

---

## 10. References

### Libraries
- RustCrypto: https://github.com/RustCrypto
- pqc_kyber: https://crates.io/crates/pqc_kyber
- pqc_dilithium: https://crates.io/crates/pqc_dilithium
- ring: https://github.com/briansmith/ring
- wasm-bindgen: https://rustwasm.github.io/wasm-bindgen/

### Standards
- NIST PQC: https://csrc.nist.gov/projects/post-quantum-cryptography
- FIPS 203 (ML-KEM): https://csrc.nist.gov/publications/fips/203
- FIPS 204 (ML-DSA): https://csrc.nist.gov/publications/fips/204
- FIPS 205 (SLH-DSA): https://csrc.nist.gov/publications/fips/205

### Documentation
- Rust WASM Book: https://rustwasm.github.io/docs/book/
- Rust no_std Playbook: https://hackmd.io/@alxiong/rust-no-std
- Awesome Rust Cryptography: https://cryptography.rs/

---

**End of Report**
