# Crypto Messenger - Vulnerable Sample

**WARNING**: This is an intentionally vulnerable application for testing purposes. DO NOT use any of this code in production.

## Overview

An end-to-end encrypted messaging application with multiple cryptographic vulnerabilities designed to test the ArcQubit PQC Scanner.

## Technology Stack

- **Language**: Python
- **Framework**: Flask
- **Protocol**: Custom E2E encryption

## Intentional Vulnerabilities

This sample contains the following intentional vulnerabilities:

1. **ECDH with P-192 Curve** (`app/crypto/key_exchange.py`)
   - Lines 16, 24, 36: Using SECP192R1 (P-192) curve
   - Provides only ~80 bits of security (insufficient)

2. **ECDSA Signatures** (`app/crypto/signatures.py`)
   - Lines 17, 30, 51, 77, 91: ECDSA with P-256
   - Quantum-vulnerable signature algorithm

3. **RC4 Stream Cipher** (`app/crypto/message_crypto.py`)
   - Lines 22, 32, 44: Using RC4 (ARC4) cipher
   - Broken stream cipher with known biases

4. **Custom Encryption** (`app/crypto/message_crypto.py`)
   - Lines 55, 69, 81: Custom XOR cipher and hash
   - Never implement custom cryptography

5. **Hardcoded Keys** (Multiple files)
   - `app/crypto/message_crypto.py`: Lines 94-99
   - `app/models/user.py`: Lines 15-16, 47, 63-67
   - `app/routes/messages.py`: Line 13
   - Encryption keys committed to source code

6. **MD5 Usage** (Multiple files)
   - Password hashing, token generation, signatures
   - Cryptographically broken hash function

7. **SHA-1 Usage** (`app/crypto/signatures.py`, `app/routes/messages.py`)
   - Lines 50, 76, 47: SHA-1 for signatures and hashing
   - Vulnerable to collision attacks

## Expected Scanner Results

- **Vulnerabilities Found**: 12
- **Compliance Score**: 35/100
- **Risk Score**: 72/100
- **Critical Findings**: 4 (hardcoded keys, RC4)
- **High Findings**: 6 (weak curves, ECDSA)
- **Medium Findings**: 2 (custom crypto)

## Installation

```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
```

## Running Tests

```bash
pytest tests/test_crypto.py -v
```

## Scanning with PQC Scanner

```bash
# From the pqc-scanner root directory
cargo run --example generate_compliance_report -- \
  --path samples/crypto-messenger/app/ \
  --output reports/crypto-messenger-report.json
```

## Remediation Recommendations

1. **ECDH P-192 → Kyber-512 KEM**: Post-quantum key encapsulation
2. **ECDSA P-256 → Dilithium2**: Post-quantum signatures
3. **RC4 → ChaCha20-Poly1305**: Modern authenticated encryption
4. **Custom Crypto → Standard Libraries**: Use vetted implementations
5. **Hardcoded Keys → Secure Key Management**: Use key derivation and HSM
6. **MD5 → SHA-256/SHA-3**: Upgrade hash functions
7. **SHA-1 → SHA-256**: Replace deprecated hashes

## Files Structure

```
crypto-messenger/
├── app/
│   ├── crypto/
│   │   ├── key_exchange.py         # ECDH P-192 vulnerabilities
│   │   ├── message_crypto.py       # RC4 and custom crypto
│   │   └── signatures.py           # ECDSA vulnerabilities
│   ├── models/
│   │   └── user.py                 # Hardcoded keys
│   └── routes/
│       └── messages.py              # API endpoints with vulnerabilities
├── tests/
│   └── test_crypto.py               # Test suite
├── requirements.txt
└── README.md
```

## License

MIT - For testing purposes only
