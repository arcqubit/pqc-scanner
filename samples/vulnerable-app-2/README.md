# Vulnerable Python Application - Sample 2

This is a **deliberately vulnerable** Python application created for testing the PQC Scanner. It contains multiple quantum-vulnerable cryptographic implementations.

⚠️ **WARNING**: This code contains intentional security vulnerabilities. **DO NOT USE IN PRODUCTION**.

## Overview

This sample application demonstrates common cryptographic vulnerabilities in Python applications, specifically focusing on algorithms that are vulnerable to quantum computing attacks.

## Vulnerabilities Included

### 1. MD5 Hash Function (`src/api_auth.py`)

**Severity**: High
**Quantum Vulnerability**: Yes (Grover's algorithm)

- **Lines 28-30**: API token generation using MD5
- **Lines 50-52**: Token validation with MD5
- **Lines 58-61**: Password hashing with MD5
- **Lines 89-92**: Session ID generation with MD5
- **Lines 103-106**: CSRF token generation with MD5
- **Lines 116-118**: API signature using MD5

**Impact**:
- MD5 collisions can be generated in seconds
- Quantum computers can break MD5 in O(2^64) operations with Grover's algorithm
- Completely unsuitable for any security purpose

**Should use**: SHA-256, SHA-3, or BLAKE3

### 2. Triple DES (3DES) Encryption (`src/encryption.py`)

**Severity**: High
**Quantum Vulnerability**: Critical

- **Lines 38-52**: 3DES encryption in CBC mode
- **Lines 54-68**: 3DES decryption
- **Lines 169-176**: Database field encryption with 3DES

**Impact**:
- 3DES has effective 112-bit classical security
- Quantum computers reduce this to 56-bit security (Grover's algorithm)
- 64-bit block size vulnerable to birthday attacks
- Deprecated by NIST SP 800-131A

**Should use**: AES-256-GCM or post-quantum alternatives (Kyber)

### 3. RC4 Stream Cipher (`src/encryption.py`)

**Severity**: Critical
**Quantum Vulnerability**: Yes

- **Lines 95-105**: RC4 encryption
- **Lines 107-117**: RC4 decryption

**Impact**:
- RC4 has known statistical biases in keystream
- Completely broken by multiple attacks (BEAST, CRIME, etc.)
- Prohibited in TLS since 2015 (RFC 7465)
- Can leak plaintext within first bytes of keystream

**Should use**: ChaCha20-Poly1305 or AES-GCM

### 4. ECDSA Signatures (`src/tokens.py`)

**Severity**: Critical
**Quantum Vulnerability**: Complete break

- **Lines 31-44**: ECDSA signing with secp256k1
- **Lines 46-58**: ECDSA signature verification
- **Lines 68-80**: Public key verification

**Impact**:
- Shor's algorithm on quantum computer can:
  - Recover private key from public key in polynomial time
  - Forge signatures
  - Break all ECDSA-based authentication
- Provides ZERO security against quantum attackers

**Should use**: Dilithium, Falcon, or SPHINCS+ (NIST PQC standards)

### 5. Weak Random Number Generation (`src/tokens.py`)

**Severity**: High
**Quantum Vulnerability**: N/A (already broken classically)

- **Lines 108-110**: Mersenne Twister seeded with time
- **Lines 117-124**: Token generation with `random.choice()`
- **Lines 126-133**: Session key with `random.randint()`
- **Lines 135-142**: Nonce generation with weak RNG
- **Lines 212-218**: Cryptographic key with `random.getrandbits()`

**Impact**:
- Mersenne Twister state can be recovered from 624 outputs
- Time-based seeding is highly predictable
- Not cryptographically secure even classically
- Makes all derived keys, tokens, and nonces predictable

**Should use**: `secrets` module or `os.urandom()`

## File Structure

```
vulnerable-app-2/
├── requirements.txt          # Python dependencies
├── README.md                 # This file
└── src/
    ├── api_auth.py          # MD5-based authentication
    ├── encryption.py        # 3DES and RC4 encryption
    └── tokens.py            # ECDSA signatures and weak RNG
```

## Installation

```bash
cd samples/vulnerable-app-2
pip install -r requirements.txt
```

## Running Examples

Each module can be run independently to demonstrate vulnerabilities:

```bash
# Test MD5-based authentication
python src/api_auth.py

# Test 3DES and RC4 encryption
python src/encryption.py

# Test ECDSA signatures and weak RNG
python src/tokens.py
```

## Scanning with PQC Scanner

Run the scanner against this application:

```bash
# From the project root
cargo run -- scan samples/vulnerable-app-2

# Or with detailed output
cargo run -- scan samples/vulnerable-app-2 --format json --output vulnerable-app-2-report.json
```

Expected findings:
- 6+ MD5 usage instances
- 3+ 3DES encryption instances
- 2+ RC4 cipher instances
- 3+ ECDSA signature instances
- 5+ weak RNG instances

## Remediation Guide

### Replace MD5
```python
# ❌ Vulnerable
import hashlib
token_hash = hashlib.md5(data.encode()).hexdigest()

# ✅ Secure
import hashlib
token_hash = hashlib.sha256(data.encode()).hexdigest()
# Or use HMAC for message authentication
import hmac
token_hash = hmac.new(key, data.encode(), hashlib.sha256).hexdigest()
```

### Replace 3DES
```python
# ❌ Vulnerable
from Crypto.Cipher import DES3
cipher = DES3.new(key, DES3.MODE_CBC)

# ✅ Secure (classical)
from Crypto.Cipher import AES
cipher = AES.new(key, AES.MODE_GCM)

# ✅ Post-quantum secure
# Use Kyber for key encapsulation
```

### Replace RC4
```python
# ❌ Vulnerable
from Crypto.Cipher import ARC4
cipher = ARC4.new(key)

# ✅ Secure
from Crypto.Cipher import ChaCha20_Poly1305
cipher = ChaCha20_Poly1305.new(key=key)
```

### Replace ECDSA
```python
# ❌ Vulnerable
from ecdsa import SigningKey, SECP256k1
sk = SigningKey.generate(curve=SECP256k1)

# ✅ Post-quantum secure
# Use Dilithium, Falcon, or SPHINCS+
# (Python implementations available via pqcrypto or liboqs-python)
```

### Replace Weak RNG
```python
# ❌ Vulnerable
import random
token = ''.join(random.choice(chars) for _ in range(32))

# ✅ Secure
import secrets
token = secrets.token_urlsafe(32)
```

## Quantum Threat Timeline

| Algorithm | Classical Security | Quantum Security | Timeline |
|-----------|-------------------|------------------|----------|
| MD5 | Broken (2004) | N/A | Replace immediately |
| 3DES | 112-bit | 56-bit | Deprecated (NIST) |
| RC4 | Broken (2013) | N/A | Prohibited (RFC 7465) |
| ECDSA-256 | 128-bit | 0-bit | Replace by 2030 |
| Weak RNG | Varies | N/A | Replace immediately |

## References

- [NIST Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [NIST SP 800-131A Rev. 2](https://csrc.nist.gov/publications/detail/sp/800-131a/rev-2/final) - Deprecated Algorithms
- [RFC 7465](https://tools.ietf.org/html/rfc7465) - Prohibiting RC4 Cipher Suites
- [OWASP Cryptographic Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html)

## Testing Notes

This application is designed to trigger multiple scanner detections:

1. **Hash functions**: MD5 usage should be detected and flagged as high severity
2. **Block ciphers**: 3DES should be flagged as quantum-vulnerable
3. **Stream ciphers**: RC4 should be flagged as critically insecure
4. **Signatures**: ECDSA should be flagged for quantum vulnerability
5. **RNG**: Weak random generation should be detected

## License

This vulnerable code is provided for educational and testing purposes only.
DO NOT use in production systems.
