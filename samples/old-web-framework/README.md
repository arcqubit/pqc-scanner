# Old Web Framework - Vulnerable Sample

**WARNING**: This is an intentionally vulnerable application for testing purposes. DO NOT use any of this code in production.

## Overview

A legacy Spring Boot web framework with multiple cryptographic vulnerabilities designed to test the ArcQubit PQC Scanner.

## Technology Stack

- **Language**: Java
- **Framework**: Spring Boot (old version)
- **Database**: PostgreSQL
- **Build**: Maven

## Intentional Vulnerabilities

This sample contains the following intentional vulnerabilities:

1. **3DES Encryption** (`src/main/java/com/example/security/DatabaseEncryption.java`)
   - Lines 18, 28, 42, 61, 113: Using Triple DES algorithm
   - Deprecated and vulnerable to sweet32 attack

2. **DES Encryption** (`src/main/java/com/example/security/DatabaseEncryption.java`, `src/main/java/com/example/config/CookieConfig.java`)
   - Lines 67, 20, 28, 43: Using single DES
   - Extremely weak 56-bit key size

3. **SHA-1 Hashing** (`src/main/java/com/example/security/FileIntegrity.java`)
   - Lines 20, 37, 51, 69, 83: SHA-1 for file integrity
   - Vulnerable to collision attacks

4. **MD5 Hashing** (Multiple files)
   - Password hashing, cache keys, cookie signing
   - Cryptographically broken

5. **DSA Signatures** (`src/main/java/com/example/security/SignatureService.java`)
   - Lines 13, 15, 23, 29, 46: DSA-1024 signatures
   - Quantum-vulnerable and deprecated

6. **Weak TLS Configuration** (`src/main/java/com/example/config/SecurityConfig.java`)
   - Lines 21-24, 30-35: TLS 1.0/1.1, weak cipher suites
   - Deprecated TLS versions

7. **Insecure Cookie Configuration** (`src/main/java/com/example/config/CookieConfig.java`)
   - Lines 50-51, 72-73, 100-102: Missing HttpOnly and Secure flags
   - DES/3DES encryption for cookies

8. **RSA-1024** (`src/main/java/com/example/security/SignatureService.java`)
   - Line 72: RSA-1024 key generation
   - Too small for modern security

## Expected Scanner Results

- **Vulnerabilities Found**: 18
- **Compliance Score**: 22/100
- **Risk Score**: 85/100
- **Critical Findings**: 6 (3DES, DSA-1024)
- **High Findings**: 8 (SHA-1, weak TLS)
- **Medium Findings**: 4 (cookie security)

## Installation

```bash
# Build with Maven
mvn clean install

# Run tests
mvn test
```

## Running Tests

```bash
mvn test
```

## Scanning with PQC Scanner

```bash
# From the pqc-scanner root directory
cargo run --example scan_directory -- \
  --path samples/old-web-framework/src/ \
  --output reports/old-web-framework-report.json
```

## Remediation Recommendations

1. **3DES → AES-256-GCM**: Migrate to modern authenticated encryption
2. **DES → AES-256-GCM**: Replace extremely weak DES
3. **SHA-1 → SHA-384**: Upgrade hash functions for integrity
4. **DSA-1024 → Dilithium3**: Post-quantum signature algorithm
5. **TLS 1.0/1.1 → TLS 1.3**: Use only modern TLS versions
6. **Cookie Security**: Enable HttpOnly and Secure flags
7. **MD5 → SHA-256**: Replace for all non-password uses
8. **Password Hashing → Argon2id**: Use proper password hashing

## Files Structure

```
old-web-framework/
├── src/main/java/com/example/
│   ├── security/
│   │   ├── DatabaseEncryption.java  # 3DES and DES vulnerabilities
│   │   ├── FileIntegrity.java       # SHA-1 vulnerabilities
│   │   └── SignatureService.java    # DSA-1024 vulnerabilities
│   ├── config/
│   │   ├── SecurityConfig.java      # Weak TLS configuration
│   │   └── CookieConfig.java        # Insecure cookie handling
│   └── controllers/
│       └── AuthController.java      # MD5 password hashing
├── src/test/java/
│   └── SecurityTests.java           # Test suite
├── pom.xml
└── README.md
```

## License

MIT - For testing purposes only
