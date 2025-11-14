# Legacy Banking Application - Vulnerable Sample

**WARNING**: This is an intentionally vulnerable application for testing purposes. DO NOT use any of this code in production.

## Overview

A legacy banking application with multiple cryptographic vulnerabilities designed to test the ArcQubit PQC Scanner.

## Technology Stack

- **Language**: JavaScript/Node.js
- **Framework**: Express.js
- **Database**: MySQL

## Intentional Vulnerabilities

This sample contains the following intentional vulnerabilities:

1. **RSA-1024 Key Generation** (`src/crypto/key-generator.js`)
   - Lines 15, 30, 52, 70: Using 1024-bit and 512-bit RSA keys
   - Quantum-vulnerable and too small for modern security

2. **MD5 Password Hashing** (`src/auth/password-hasher.js`)
   - Lines 16, 37, 49: Using MD5 without salt
   - Cryptographically broken hash function

3. **SHA-1 Signatures** (`src/crypto/transaction-signer.js`)
   - Lines 19, 37, 55, 70, 83: Using SHA-1 for digital signatures
   - Deprecated and vulnerable to collision attacks

4. **DES Encryption** (`src/auth/session-manager.js`)
   - Lines 14, 23, 35: Using DES for session tokens
   - Weak encryption with small key size

5. **Weak Random Generation** (`src/utils/random.js`)
   - Lines 14, 24, 33, 44, 54, 66: Using Math.random()
   - Predictable and unsuitable for security

## Expected Scanner Results

- **Vulnerabilities Found**: 15
- **Compliance Score**: 28/100
- **Risk Score**: 78/100
- **Critical Findings**: 5 (RSA-1024, MD5 passwords)
- **High Findings**: 7 (SHA-1, DES)
- **Medium Findings**: 3 (weak random)

## Installation

```bash
npm install
```

## Running Tests

```bash
npm test
```

## Scanning with PQC Scanner

```bash
# From the pqc-scanner root directory
cargo run --example generate_compliance_report -- \
  --path samples/legacy-banking/src/ \
  --output reports/legacy-banking-report.json
```

## Remediation Recommendations

1. **RSA-1024 → Kyber-768**: Migrate to post-quantum key encapsulation
2. **MD5 → Argon2id**: Use proper password hashing with salt
3. **SHA-1 → SHA-256**: Update to secure hash functions
4. **DES → AES-256-GCM**: Migrate to modern authenticated encryption
5. **Math.random() → crypto.randomBytes()**: Use cryptographically secure random

## Files Structure

```
legacy-banking/
├── src/
│   ├── auth/
│   │   ├── password-hasher.js      # MD5 hashing vulnerabilities
│   │   └── session-manager.js      # DES encryption vulnerabilities
│   ├── crypto/
│   │   ├── key-generator.js        # RSA-1024 vulnerabilities
│   │   └── transaction-signer.js   # SHA-1 signature vulnerabilities
│   └── utils/
│       └── random.js                # Math.random() vulnerabilities
├── tests/
│   └── crypto.test.js               # Test suite
├── package.json
└── README.md
```

## License

MIT - For testing purposes only
