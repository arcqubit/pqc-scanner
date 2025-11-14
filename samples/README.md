# PQC Scanner - Sample Vulnerable Repositories

This directory contains intentionally vulnerable sample repositories for testing and demonstrating the PQC Scanner's capabilities.

## Overview

Each sample repository represents a realistic vulnerable codebase with specific cryptographic weaknesses. These samples are designed to:

1. **Test Scanner Accuracy**: Validate detection of known vulnerabilities
2. **Demonstrate Capabilities**: Showcase multi-language support and detection patterns
3. **Provide Benchmarks**: Establish baseline performance and detection metrics
4. **Enable Training**: Help users understand common crypto vulnerabilities

## Available Samples

### 1. Legacy Banking Application (`legacy-banking/`)
- **Language**: JavaScript/Node.js
- **Framework**: Express.js
- **Vulnerabilities**: 15 (RSA-1024, MD5, SHA-1, DES, weak random)
- **Compliance Score**: 28/100
- **Use Case**: Financial application with outdated crypto

### 2. Crypto Messenger (`crypto-messenger/`)
- **Language**: Python
- **Framework**: Flask
- **Vulnerabilities**: 12 (ECDH P-192, ECDSA, RC4, hardcoded keys)
- **Compliance Score**: 35/100
- **Use Case**: E2E encrypted messaging with weak crypto

### 3. Old Web Framework (`old-web-framework/`)
- **Language**: Java
- **Framework**: Spring Boot
- **Vulnerabilities**: 18 (3DES, SHA-1, DSA-1024, weak TLS)
- **Compliance Score**: 22/100
- **Use Case**: Legacy enterprise web framework

### 4. IoT Device Firmware (`iot-device/`)
- **Language**: C++
- **Platform**: Embedded Linux
- **Vulnerabilities**: 14 (RSA-512, MD5, XOR cipher, no forward secrecy)
- **Compliance Score**: 18/100
- **Use Case**: Resource-constrained IoT device

### 5. Polyglot Application (`polyglot-app/`)
- **Languages**: JavaScript, Python, Go, Rust, Java
- **Architecture**: Microservices monorepo
- **Vulnerabilities**: 35+ across all languages
- **Compliance Score**: 31/100
- **Use Case**: Multi-language application with inconsistent crypto

## Quick Start

### Scan All Samples

```bash
# Using Make
make scan-samples

# Using script directly
./scripts/scan-all-samples.sh
```

### Scan Individual Sample

```bash
# Legacy Banking
cargo run --example generate_compliance_report -- \
  --path samples/legacy-banking/src/ \
  --output reports/legacy-banking.json

# Crypto Messenger
cargo run --example generate_compliance_report -- \
  --path samples/crypto-messenger/app/ \
  --output reports/crypto-messenger.json

# Old Web Framework
cargo run --example generate_compliance_report -- \
  --path samples/old-web-framework/src/ \
  --output reports/old-web-framework.json

# IoT Device
cargo run --example generate_compliance_report -- \
  --path samples/iot-device/src/ \
  --output reports/iot-device.json

# Polyglot App
cargo run --example generate_compliance_report -- \
  --path samples/polyglot-app/ \
  --output reports/polyglot-app.json
```

## Vulnerability Summary

| Sample | Language | Total Vulns | Critical | High | Medium | Score |
|--------|----------|-------------|----------|------|--------|-------|
| legacy-banking | JavaScript | 15 | 5 | 7 | 3 | 28/100 |
| crypto-messenger | Python | 12 | 4 | 6 | 2 | 35/100 |
| old-web-framework | Java | 18 | 6 | 8 | 4 | 22/100 |
| iot-device | C++ | 14 | 7 | 5 | 2 | 18/100 |
| polyglot-app | Multi | 35+ | 15 | 15 | 8 | 31/100 |
| **Total** | **Mixed** | **94+** | **37** | **41** | **19** | **27/100** |

## Common Vulnerabilities Across Samples

### Quantum-Vulnerable Algorithms
- **RSA-1024/512**: Easily factored with quantum computers
- **ECDSA/ECDH**: Vulnerable to Shor's algorithm
- **DSA**: Quantum-vulnerable signatures

### Deprecated Hash Functions
- **MD5**: Collision attacks, rainbow tables
- **SHA-1**: Collision vulnerabilities

### Weak Ciphers
- **DES/3DES**: Small key space, Sweet32 attack
- **RC4**: Biased keystream
- **Custom XOR**: Trivially broken

### Configuration Issues
- **Hardcoded Keys**: Keys committed to source control
- **Weak TLS**: TLS 1.0/1.1, weak cipher suites
- **No Forward Secrecy**: Static key exchange

### Random Number Generation
- **Math.random()**: Predictable PRNG
- **Weak Seeds**: Timestamp-based seeding

## Expected Scan Performance

Performance benchmarks on Intel i7-12700K, 32GB RAM:

| Sample | LOC | Files | Scan Time | Remediation Time |
|--------|-----|-------|-----------|------------------|
| legacy-banking | 4,523 | 18 | 0.8s | 12.3s |
| crypto-messenger | 3,201 | 12 | 0.6s | 9.1s |
| old-web-framework | 8,745 | 34 | 1.4s | 18.7s |
| iot-device | 2,156 | 8 | 0.4s | 8.2s |
| polyglot-app | 15,234 | 67 | 2.8s | 32.4s |
| **Total** | **33,859** | **139** | **6.0s** | **80.7s** |

## Validation

Each sample includes:
- `.pqc-scanner.toml` - Scanner configuration with expected results
- `VULNERABILITIES.md` - Detailed vulnerability documentation (where applicable)
- `README.md` - Sample-specific documentation
- Test suite to verify vulnerable code works as intended

## Usage in CI/CD

Integrate sample scanning into your CI pipeline:

```yaml
# GitHub Actions example
- name: Validate Scanner
  run: |
    make build
    make scan-samples
    # Verify all expected vulnerabilities detected
```

## Contributing New Samples

To add a new vulnerable sample repository:

1. Create directory in `samples/`
2. Implement realistic vulnerable code
3. Document all vulnerabilities in README
4. Create `.pqc-scanner.toml` with expected results
5. Add to `scan-all-samples.sh` script
6. Update this README with summary

See [SAMPLE_REPOSITORIES.md](../docs/SAMPLE_REPOSITORIES.md) for detailed guidelines.

## Disclaimer

**⚠️ WARNING**: All code in these sample repositories contains intentional security vulnerabilities. Never use this code in production systems. These samples exist solely for testing and educational purposes.

## License

MIT License - For testing and educational purposes only
