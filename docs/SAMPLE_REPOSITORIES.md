# Sample Repositories Guide

## Overview

The ArcQubit PQC Scanner includes **sample vulnerable repositories** for testing, validation, and demonstration purposes. These repositories contain intentionally vulnerable cryptographic code across multiple languages and scenarios.

## Available Sample Repositories

### 1. Legacy Banking Application (`legacy-banking`)

**Technology Stack:**
- Language: JavaScript/Node.js
- Framework: Express.js
- Database: MySQL

**Vulnerabilities:**
- RSA-1024 key generation for customer data encryption
- MD5 password hashing (no salt)
- SHA-1 for transaction signatures
- DES encryption for session tokens
- Weak random number generation

**Files:**
```
legacy-banking/
├── src/
│   ├── auth/
│   │   ├── password-hasher.js      # MD5 hashing
│   │   └── session-manager.js      # DES encryption
│   ├── crypto/
│   │   ├── key-generator.js        # RSA-1024
│   │   └── transaction-signer.js   # SHA-1 signatures
│   └── utils/
│       └── random.js                # Math.random() usage
├── tests/
│   └── crypto.test.js
└── package.json
```

**Expected Results:**
- 15 vulnerabilities detected
- Compliance score: 28/100
- 5 critical findings (RSA-1024, MD5 passwords)
- 7 high findings (SHA-1, DES)
- 3 medium findings (weak random)

**Remediation Impact:**
- RSA-1024 → Kyber-768
- MD5 → Argon2id (proper password hashing)
- SHA-1 → SHA-256
- DES → AES-256-GCM
- Math.random() → crypto.randomBytes()
- **Final score: 100/100**

### 2. Crypto Messenger (`crypto-messenger`)

**Technology Stack:**
- Language: Python
- Framework: Flask
- Protocol: Custom E2E encryption

**Vulnerabilities:**
- ECDH with P-192 curve (weak)
- ECDSA with P-256 (quantum-vulnerable)
- RC4 stream cipher for messages
- Custom encryption (insecure)
- Hardcoded encryption keys

**Files:**
```
crypto-messenger/
├── app/
│   ├── crypto/
│   │   ├── key_exchange.py         # ECDH P-192
│   │   ├── message_crypto.py       # RC4 cipher
│   │   └── signatures.py           # ECDSA P-256
│   ├── models/
│   │   └── user.py                 # Hardcoded keys
│   └── routes/
│       └── messages.py
├── tests/
│   └── test_crypto.py
└── requirements.txt
```

**Expected Results:**
- 12 vulnerabilities detected
- Compliance score: 35/100
- 4 critical findings (hardcoded keys, RC4)
- 6 high findings (weak curves, ECDSA)
- 2 medium findings (custom crypto)

**Remediation Impact:**
- ECDH P-192 → Kyber-512 KEM
- ECDSA P-256 → Dilithium2 signatures
- RC4 → ChaCha20-Poly1305
- Custom crypto → Standard libraries
- Hardcoded keys → Secure key derivation
- **Final score: 98/100**

### 3. Old Web Framework (`old-web-framework`)

**Technology Stack:**
- Language: Java
- Framework: Spring Boot (old version)
- Database: PostgreSQL

**Vulnerabilities:**
- 3DES for database encryption
- SHA-1 for file integrity checks
- DSA signatures (1024-bit)
- Weak SSL/TLS configuration
- Insecure cookie encryption

**Files:**
```
old-web-framework/
├── src/main/java/com/example/
│   ├── security/
│   │   ├── DatabaseEncryption.java  # 3DES
│   │   ├── FileIntegrity.java       # SHA-1
│   │   └── SignatureService.java    # DSA-1024
│   ├── config/
│   │   ├── SecurityConfig.java      # Weak TLS
│   │   └── CookieConfig.java        # Insecure cookies
│   └── controllers/
│       └── AuthController.java
├── src/test/java/
│   └── SecurityTests.java
└── pom.xml
```

**Expected Results:**
- 18 vulnerabilities detected
- Compliance score: 22/100
- 6 critical findings (3DES, DSA-1024)
- 8 high findings (SHA-1, weak TLS)
- 4 medium findings (cookie security)

**Remediation Impact:**
- 3DES → AES-256-GCM
- SHA-1 → SHA-384
- DSA-1024 → Dilithium3 signatures
- TLS 1.0/1.1 → TLS 1.3 only
- Cookie encryption → Secure configuration
- **Final score: 100/100**

### 4. IoT Device Firmware (`iot-device`)

**Technology Stack:**
- Language: C++
- Platform: Embedded Linux
- Protocol: MQTT

**Vulnerabilities:**
- RSA-512 for device authentication
- MD5 for firmware updates
- Custom XOR cipher
- No forward secrecy
- Weak key storage

**Files:**
```
iot-device/
├── src/
│   ├── crypto/
│   │   ├── device_auth.cpp         # RSA-512
│   │   ├── firmware_verify.cpp     # MD5
│   │   └── message_crypto.cpp      # XOR cipher
│   ├── network/
│   │   └── mqtt_client.cpp         # No forward secrecy
│   └── storage/
│       └── key_manager.cpp         # Plaintext keys
├── tests/
│   └── crypto_tests.cpp
└── CMakeLists.txt
```

**Expected Results:**
- 14 vulnerabilities detected
- Compliance score: 18/100
- 7 critical findings (RSA-512, MD5, XOR)
- 5 high findings (no forward secrecy)
- 2 medium findings (key storage)

**Remediation Impact:**
- RSA-512 → Falcon-512 signatures
- MD5 → SHA-256
- XOR cipher → AES-128-GCM (constrained devices)
- Add ECDHE for forward secrecy
- Secure key storage with HSM
- **Final score: 95/100** (some constraints remain)

### 5. Multi-Language Monorepo (`polyglot-app`)

**Technology Stack:**
- Languages: JavaScript, Python, Go, Rust, Java
- Purpose: Demonstrate cross-language detection

**Vulnerabilities:**
- Mixed crypto implementations
- Inconsistent security levels
- Language-specific weak patterns
- Legacy code sections

**Files:**
```
polyglot-app/
├── frontend/          # JavaScript - RSA-1024, MD5
├── backend/           # Python - ECDSA, SHA-1
├── microservices/     # Go - DES, weak random
├── lib-crypto/        # Rust - Some secure, some weak
└── legacy-service/    # Java - Multiple issues
```

**Expected Results:**
- 35+ vulnerabilities detected across languages
- Compliance score: 31/100
- Demonstrates cross-language consistency issues

**Remediation Impact:**
- Unified crypto policy across all languages
- **Final score: 100/100**

## Usage

### Clone Sample Repositories

```bash
# Clone all samples
git clone https://github.com/arcqubit/pqc-scanner-samples.git samples/

# Or clone individual repos
git clone https://github.com/arcqubit/pqc-scanner-samples.git \
  --branch legacy-banking \
  --single-branch samples/legacy-banking
```

### Run Scans

```bash
# Scan single repository
cd samples/legacy-banking
cargo run --example generate_compliance_report -- \
  --path src/ \
  --output report.json

# Scan all samples
make scan-samples

# Or use script
./scripts/scan-all-samples.sh
```

### Run with Auto-Remediation

```bash
# Remediate legacy banking app
cd samples/legacy-banking
cargo run --example generate_compliance_report -- \
  --path src/ \
  --remediate \
  --output remediation-report.json

# Compare before/after
diff -u src/auth/password-hasher.js.backup \
        src/auth/password-hasher.js
```

### Batch Processing

```bash
# Process all sample repos
for repo in samples/*/; do
  echo "Scanning $repo..."
  cd "$repo"
  pqc-scanner --remediate --output "../reports/$(basename $repo).json"
  cd ..
done
```

## Sample Report Output

### Legacy Banking - Before Remediation

```json
{
  "metadata": {
    "report_id": "scan-001",
    "repository": "legacy-banking",
    "timestamp": "2025-11-06T10:00:00Z"
  },
  "summary": {
    "lines_scanned": 4523,
    "files_scanned": 18,
    "vulnerabilities_found": 15,
    "compliance_score": 28,
    "risk_score": 78
  },
  "findings": [
    {
      "id": "RSA-001",
      "file": "src/crypto/key-generator.js",
      "line": 15,
      "severity": "critical",
      "algorithm": "RSA-1024",
      "message": "Quantum-vulnerable key size",
      "recommendation": "Use Kyber-768 or higher"
    }
  ]
}
```

### Legacy Banking - After Remediation

```json
{
  "metadata": {
    "report_id": "remediation-001",
    "repository": "legacy-banking",
    "timestamp": "2025-11-06T10:15:00Z"
  },
  "summary": {
    "lines_scanned": 4598,
    "files_scanned": 18,
    "vulnerabilities_found": 0,
    "compliance_score": 100,
    "risk_score": 5
  },
  "remediation": {
    "fixes_applied": 15,
    "time_taken": "12.3s",
    "success_rate": "100%"
  }
}
```

## Expected Benchmarks

| Repository | LOC | Files | Vulns | Scan Time | Remediation Time |
|------------|-----|-------|-------|-----------|------------------|
| legacy-banking | 4,523 | 18 | 15 | 0.8s | 12.3s |
| crypto-messenger | 3,201 | 12 | 12 | 0.6s | 9.1s |
| old-web-framework | 8,745 | 34 | 18 | 1.4s | 18.7s |
| iot-device | 2,156 | 8 | 14 | 0.4s | 8.2s |
| polyglot-app | 15,234 | 67 | 35+ | 2.8s | 32.4s |

*Tested on Intel i7-12700K, 32GB RAM*

## Testing Scenarios

### Scenario 1: Compliance Audit

```bash
# Generate compliance report
cd samples/legacy-banking
pqc-scanner --path src/ \
  --format oscal \
  --output compliance-report.json

# Submit to compliance tool
oscal-cli validate compliance-report.json
```

### Scenario 2: Migration Planning

```bash
# Generate migration plan
pqc-scanner --path src/ \
  --remediate \
  --dry-run \
  --output migration-plan.json

# Estimate effort
cat migration-plan.json | jq '.remediation.estimated_effort'
```

### Scenario 3: CI/CD Integration

```bash
# Fail build if score < 80
SCORE=$(pqc-scanner --path src/ --format json | jq '.summary.compliance_score')
if [ $SCORE -lt 80 ]; then
  echo "Compliance score too low: $SCORE"
  exit 1
fi
```

### Scenario 4: Performance Benchmarking

```bash
# Benchmark scanning performance
time pqc-scanner --path samples/ --benchmark

# Expected output:
# Files scanned: 139
# Time: 3.2s
# Throughput: 43.4 files/sec
```

## Contributing Sample Repositories

Want to add more samples? Follow these guidelines:

1. **Create Realistic Scenario**: Real-world use case
2. **Document Vulnerabilities**: List all intentional issues
3. **Provide Context**: Explain why code is vulnerable
4. **Include Tests**: Test suite for validation
5. **Add README**: Setup and usage instructions

### Template Structure

```
sample-repo-name/
├── README.md                    # Description and usage
├── VULNERABILITIES.md           # List of intentional issues
├── src/                         # Source code
├── tests/                       # Test suite
├── .pqc-scanner.toml            # Scanner configuration
└── expected-results.json        # Expected scan output
```

## Troubleshooting

### Issue: "Sample repo not found"

```bash
# Ensure samples are cloned
git clone https://github.com/photoniq/pqc-scanner-samples.git samples/
```

### Issue: "Scan results don't match expected"

```bash
# Update scanner to latest version
cargo install pqc-scanner --force

# Clear cache
rm -rf .pqc-scanner-cache/
```

### Issue: "Remediation fails"

```bash
# Check dependencies
npm install  # For JS samples
pip install -r requirements.txt  # For Python samples
```

## References

- [Sample Repository Source](https://github.com/arcqubit/pqc-scanner-samples)
- [Contributing Guidelines](../CONTRIBUTING.md)
- [Auto-Remediation Guide](AUTO_REMEDIATION.md)
- [Scanner Documentation](../README.md)
