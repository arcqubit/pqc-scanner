# IoT Device Firmware - Vulnerable Sample

**WARNING**: This is an intentionally vulnerable application for testing purposes. DO NOT use any of this code in production.

## Overview

An IoT device firmware implementation with multiple cryptographic vulnerabilities designed to test the ArcQubit PQC Scanner.

## Technology Stack

- **Language**: C++
- **Platform**: Embedded Linux
- **Protocol**: MQTT
- **Build**: CMake

## Intentional Vulnerabilities

This sample contains the following intentional vulnerabilities:

1. **RSA-512 Device Authentication** (`src/crypto/device_auth.cpp`)
   - Lines 20-25: RSA-512 key generation (extremely weak)
   - Lines 36-43: RSA-1024 key generation
   - Easily factored with modern computers

2. **MD5 Firmware Verification** (`src/crypto/firmware_verify.cpp`)
   - Lines 20-23: MD5 hash for firmware integrity
   - Lines 36-40, 48-55, 74-78, 88-93, 108-115, 126-129: Multiple MD5 uses
   - Vulnerable to collision attacks

3. **Custom XOR Cipher** (`src/crypto/message_crypto.cpp`)
   - Lines 14-22, 28-32: XOR encryption
   - Lines 38-43: Simple substitution cipher
   - Custom weak cryptography

4. **No Forward Secrecy** (`src/network/mqtt_client.cpp`)
   - Lines 30-35: TLS 1.0 configuration
   - Lines 44-46: No ECDHE/DHE cipher suites
   - Static RSA key exchange only

5. **Weak Key Storage** (`src/storage/key_manager.cpp`)
   - Lines 18-19: Hardcoded keys in source
   - Lines 27-38, 46-57: Plaintext key storage
   - Lines 82-100: Weak XOR obfuscation

6. **Weak TLS Configuration** (`src/network/mqtt_client.cpp`)
   - Lines 72-78: Disabling TLS 1.2/1.3
   - Lines 84-91: SSL 3.0/TLS 1.0 enabled
   - Weak cipher suites

## Expected Scanner Results

- **Vulnerabilities Found**: 14
- **Compliance Score**: 18/100
- **Risk Score**: 88/100
- **Critical Findings**: 7 (RSA-512, MD5, XOR cipher)
- **High Findings**: 5 (no forward secrecy, weak TLS)
- **Medium Findings**: 2 (key storage)

## Building

```bash
# Create build directory
mkdir build && cd build

# Configure with CMake
cmake ..

# Build
make
```

## Running Tests

```bash
# From build directory
make test
```

## Scanning with PQC Scanner

```bash
# From the pqc-scanner root directory
cargo run --example generate_compliance_report -- \
  --path samples/iot-device/src/ \
  --output reports/iot-device-report.json
```

## Remediation Recommendations

1. **RSA-512 → Falcon-512**: Post-quantum signature algorithm for constrained devices
2. **MD5 → SHA-256**: Upgrade firmware verification to secure hash
3. **XOR Cipher → AES-128-GCM**: Use standard authenticated encryption
4. **Add Forward Secrecy**: Enable ECDHE/DHE cipher suites
5. **Secure Key Storage**: Use Hardware Security Module (HSM) or secure element
6. **TLS 1.2+ Only**: Disable old TLS versions
7. **Remove Hardcoded Keys**: Use secure key derivation and storage

## Files Structure

```
iot-device/
├── src/
│   ├── crypto/
│   │   ├── device_auth.cpp         # RSA-512 vulnerabilities
│   │   ├── firmware_verify.cpp     # MD5 vulnerabilities
│   │   └── message_crypto.cpp      # XOR cipher vulnerabilities
│   ├── network/
│   │   └── mqtt_client.cpp         # No forward secrecy
│   └── storage/
│       └── key_manager.cpp         # Weak key storage
├── tests/
│   └── crypto_tests.cpp
├── CMakeLists.txt
└── README.md
```

## Special Considerations for IoT

This sample demonstrates vulnerabilities common in resource-constrained IoT devices:
- Use of weak crypto due to performance constraints
- Plaintext key storage without HSM
- Legacy TLS for compatibility
- Custom crypto implementations

Even with constraints, modern IoT devices should use:
- Post-quantum algorithms optimized for embedded systems
- Hardware-backed key storage
- TLS 1.2+ with forward secrecy
- Standard cryptographic libraries

## License

MIT - For testing purposes only
