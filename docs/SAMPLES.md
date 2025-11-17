# Sample Vulnerable Applications

PQC Scanner includes sample vulnerable applications for testing, demonstration, and educational purposes.

## Samples Repository

All sample applications have been moved to a dedicated repository for better organization and independent evolution.

**Repository**: https://github.com/arcqubit/pqc-scanner-samples

### Why a Separate Repository?

- ✅ **Smaller main repo**: Reduced by ~35MB
- ✅ **Easier discovery**: Dedicated showcase repository
- ✅ **Independent versioning**: Samples can evolve separately
- ✅ **Better organization**: Clear separation of scanner vs. demos
- ✅ **Easier to fork**: Users can customize samples without forking the entire scanner

---

## Quick Start

### Clone Samples Repository

```bash
# Clone the samples repository
git clone https://github.com/arcqubit/pqc-scanner-samples.git
cd pqc-scanner-samples
```

### Scan a Sample

```bash
# Scan legacy banking app
pqc-scanner scan pqc-scanner-samples/legacy-banking/src/

# Scan with JSON output
pqc-scanner scan pqc-scanner-samples/crypto-messenger/ --output report.json

# Scan all samples
for sample in crypto-messenger iot-device legacy-banking old-web-framework polyglot-app; do
  pqc-scanner scan "pqc-scanner-samples/$sample/"
done
```

---

## Available Samples

### 1. Legacy Banking Application (`legacy-banking/`)

**Language**: JavaScript/Node.js
**Framework**: Express.js
**Vulnerabilities**: 15 (RSA-1024, MD5, SHA-1, DES, weak random)
**Compliance Score**: 28/100
**Size**: ~252KB

**Use Case**: Financial application with outdated cryptography. Demonstrates typical vulnerabilities in legacy Node.js applications.

**Key Vulnerabilities**:
- RSA-1024 for key generation
- MD5 password hashing
- Weak random number generation (Math.random())
- Hardcoded cryptographic keys

[View Sample](https://github.com/arcqubit/pqc-scanner-samples/tree/main/legacy-banking)

---

### 2. Crypto Messenger (`crypto-messenger/`)

**Language**: Python
**Framework**: Flask
**Vulnerabilities**: 12 (ECDH P-192, ECDSA, RC4, hardcoded keys)
**Compliance Score**: 35/100
**Size**: ~60KB

**Use Case**: End-to-end encrypted messaging application with weak cryptography.

**Key Vulnerabilities**:
- ECDH with P-192 curve
- ECDSA signatures (quantum-vulnerable)
- RC4 stream cipher
- Hardcoded encryption keys

[View Sample](https://github.com/arcqubit/pqc-scanner-samples/tree/main/crypto-messenger)

---

### 3. Old Web Framework (`old-web-framework/`)

**Language**: Java
**Framework**: Spring Boot
**Vulnerabilities**: 18 (3DES, SHA-1, DSA-1024, weak TLS)
**Compliance Score**: 22/100
**Size**: ~88KB

**Use Case**: Legacy enterprise web framework with deprecated security configurations.

**Key Vulnerabilities**:
- 3DES encryption
- SHA-1 digital signatures
- DSA-1024 key agreement
- TLS 1.0/1.1 support

[View Sample](https://github.com/arcqubit/pqc-scanner-samples/tree/main/old-web-framework)

---

### 4. IoT Device Firmware (`iot-device/`)

**Language**: C++
**Platform**: Embedded Linux
**Vulnerabilities**: 14 (RSA-512, MD5, XOR cipher, no forward secrecy)
**Compliance Score**: 18/100
**Size**: ~60KB

**Use Case**: Resource-constrained IoT device with compromised security.

**Key Vulnerabilities**:
- RSA-512 (weak key size)
- MD5 for firmware integrity
- Custom XOR cipher
- No forward secrecy in key exchange

[View Sample](https://github.com/arcqubit/pqc-scanner-samples/tree/main/iot-device)

---

### 5. Polyglot Application (`polyglot-app/`)

**Languages**: JavaScript, Python, Go, Rust, Java
**Architecture**: Microservices monorepo
**Vulnerabilities**: 35+ across all languages
**Compliance Score**: 31/100
**Size**: ~60KB

**Use Case**: Multi-language application with inconsistent cryptographic practices across services.

**Key Vulnerabilities**:
- Mixed cryptographic implementations
- Inconsistent key sizes
- Multiple deprecated algorithms
- Cross-service crypto mismatches

[View Sample](https://github.com/arcqubit/pqc-scanner-samples/tree/main/polyglot-app)

---

## Vulnerability Summary

| Sample | Language | LOC | Vulns | Critical | High | Medium | Score |
|--------|----------|-----|-------|----------|------|--------|-------|
| legacy-banking | JavaScript | 4,523 | 15 | 5 | 7 | 3 | 28/100 |
| crypto-messenger | Python | 3,201 | 12 | 4 | 6 | 2 | 35/100 |
| old-web-framework | Java | 8,745 | 18 | 6 | 8 | 4 | 22/100 |
| iot-device | C++ | 2,156 | 14 | 7 | 5 | 2 | 18/100 |
| polyglot-app | Multi | 15,234 | 35+ | 15 | 15 | 8 | 31/100 |
| **TOTAL** | **Mixed** | **33,859** | **94+** | **37** | **41** | **19** | **27/100** |

---

## Common Vulnerability Types

### Quantum-Vulnerable Algorithms
- **RSA**: Factoring vulnerable to Shor's algorithm
- **ECDSA/ECDH**: Discrete log vulnerable to Shor's algorithm
- **DSA**: Quantum-vulnerable signatures

### Deprecated Hash Functions
- **MD5**: Collision attacks, not cryptographically secure
- **SHA-1**: Collision vulnerabilities, deprecated by NIST

### Weak Ciphers
- **DES/3DES**: Small key space, Sweet32 attack
- **RC4**: Biased keystream, multiple attacks
- **Custom XOR**: Easily broken

### Configuration Issues
- **Hardcoded Keys**: Keys in source code
- **Weak TLS**: TLS 1.0/1.1
- **No Forward Secrecy**: Static key exchange

### Random Number Generation
- **Math.random()**: Predictable PRNG
- **Weak Seeds**: Timestamp-based seeding

---

## Using Samples for Testing

### Integration Testing

```bash
# Clone both repositories
git clone https://github.com/arcqubit/pqc-scanner.git
git clone https://github.com/arcqubit/pqc-scanner-samples.git

# Build scanner
cd pqc-scanner
cargo build --release

# Test against samples
./target/release/pqc-scanner scan ../pqc-scanner-samples/legacy-banking/src/

# Verify all 15 vulnerabilities detected
```

### Benchmarking

Each sample includes expected scan times and detection accuracy for performance validation.

Expected Performance (Intel i7-12700K, 32GB RAM):

| Sample | Scan Time | Remediation Time |
|--------|-----------|------------------|
| legacy-banking | 0.8s | 12.3s |
| crypto-messenger | 0.6s | 9.1s |
| old-web-framework | 1.4s | 18.7s |
| iot-device | 0.4s | 8.2s |
| polyglot-app | 2.8s | 32.4s |

### CI/CD Integration

```yaml
# .github/workflows/validate-scanner.yml
- name: Clone samples
  run: git clone https://github.com/arcqubit/pqc-scanner-samples.git

- name: Test scanner against samples
  run: |
    ./pqc-scanner scan samples/legacy-banking/src/ --output test-results.json
    # Verify 15 vulnerabilities detected
    test $(jq '.vulnerabilities | length' test-results.json) -eq 15
```

---

## Documentation

Each sample includes:
- **README.md**: Sample overview and usage instructions
- **VULNERABILITIES.md**: Detailed vulnerability documentation
- **Test Suite**: Functional tests to verify the vulnerable code works
- **Build Files**: Language-specific build configuration

---

## Contributing Samples

We welcome contributions of new vulnerable samples! See the [samples repository CONTRIBUTING.md](https://github.com/arcqubit/pqc-scanner-samples/blob/main/CONTRIBUTING.md) for guidelines.

### Sample Requirements

- Realistic, real-world vulnerable code patterns
- Multiple vulnerability types
- Comprehensive documentation
- Test suite included
- No dependencies with known security issues (only intentional vulnerabilities)

---

## Disclaimer

⚠️ **WARNING**: All code in the samples repository contains intentional security vulnerabilities.

**DO NOT** use any sample code in production systems.

Samples exist solely for:
- Testing PQC Scanner functionality
- Educational demonstrations
- Security training
- Vulnerability research

---

## Additional Resources

- **Samples Repository**: https://github.com/arcqubit/pqc-scanner-samples
- **Main Scanner**: https://github.com/arcqubit/pqc-scanner
- **Documentation**: [Sample Repository Guidelines](SAMPLE_REPOSITORIES.md)

---

**Last Updated**: 2025-11-17
**Samples Version**: 1.0
