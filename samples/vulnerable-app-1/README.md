# Vulnerable Application - Test Sample

⚠️ **WARNING: This application contains intentional security vulnerabilities for testing purposes only. DO NOT use in production or expose to the internet!**

## Purpose

This is a sample vulnerable Node.js application designed to test cryptographic security scanners, particularly the PQC (Post-Quantum Cryptography) Scanner. It contains realistic but intentionally flawed code that demonstrates common cryptographic vulnerabilities.

## Intentional Vulnerabilities

### Authentication Module (`src/auth.js`)

1. **MD5 Password Hashing**
   - Using MD5 for password hashing (broken, collision-prone)
   - Location: `hashPassword()` function
   - Severity: CRITICAL

2. **Hardcoded Secret Keys**
   - JWT secret hardcoded in source code
   - Location: `JWT_SECRET` constant
   - Severity: CRITICAL

3. **SHA-1 Usage**
   - Using SHA-1 for session tokens (deprecated, collision-prone)
   - Location: `generateSessionToken()` function
   - Severity: HIGH

4. **Weak Password Validation**
   - Only checks length, no complexity requirements
   - Location: `validatePassword()` function
   - Severity: MEDIUM

5. **Insecure HMAC**
   - Using HMAC-SHA1 instead of HMAC-SHA256
   - Location: `generateAPIKey()` function
   - Severity: HIGH

### Cryptographic Operations Module (`src/crypto.js`)

1. **DES Encryption**
   - Using DES cipher (obsolete, 56-bit keys)
   - Location: `encryptWithDES()` function
   - Severity: CRITICAL

2. **RC4 Stream Cipher**
   - Using RC4 (broken stream cipher)
   - Location: `encryptWithRC4()` function
   - Severity: CRITICAL

3. **Weak RSA Keys**
   - Generating 1024-bit RSA keys (too small, vulnerable to factorization)
   - Location: `generateWeakRSAKeys()` function
   - Severity: CRITICAL

4. **ECB Mode**
   - Using AES in ECB mode (no IV, deterministic encryption)
   - Location: `encryptWithECB()` function
   - Severity: HIGH

5. **MD5 File Hashing**
   - Using MD5 for file integrity checks
   - Location: `calculateFileHash()` function
   - Severity: HIGH

6. **SHA-1 Signatures**
   - Using RSA-SHA1 for digital signatures
   - Location: `signData()` function
   - Severity: CRITICAL

7. **Fixed IV**
   - Using static initialization vector
   - Location: `encryptWithFixedIV()` function
   - Severity: CRITICAL

8. **Deprecated createCipher**
   - Using `crypto.createCipher()` (insecure key derivation)
   - Location: `legacyEncrypt()` function
   - Severity: HIGH

9. **Triple DES**
   - Using 3DES (deprecated algorithm)
   - Location: `encryptWith3DES()` function
   - Severity: HIGH

10. **Weak Random Generation**
    - Using `Math.random()` instead of `crypto.randomBytes()`
    - Location: `generateWeakRandomToken()` function
    - Severity: CRITICAL

### Storage Module (`src/storage.js`)

1. **Hardcoded Database Credentials**
   - Database password in source code
   - Location: `DB_CONFIG` constant
   - Severity: CRITICAL

2. **MD5 Data Hashing**
   - Using MD5 for data deduplication
   - Location: `calculateDataHash()` function
   - Severity: MEDIUM

3. **DES Data Encryption**
   - Using DES for sensitive data encryption
   - Location: `encryptSensitiveData()` function
   - Severity: CRITICAL

4. **Predictable Session IDs**
   - Using SHA-1 for session ID generation
   - Location: `SessionStore.createSession()` method
   - Severity: HIGH

5. **Reversible API Key Storage**
   - Using base64 encoding instead of encryption
   - Location: `storeAPIKey()` function
   - Severity: CRITICAL

6. **ECB File Encryption**
   - Encrypting files with ECB mode
   - Location: `encryptFile()` function
   - Severity: HIGH

7. **Weak Credit Card Storage**
   - Using deprecated `createCipher()` for PCI data
   - Location: `storeCreditCard()` function
   - Severity: CRITICAL

8. **Zero IV for Backups**
   - Using null/zero initialization vector
   - Location: `createBackup()` function
   - Severity: CRITICAL

9. **Insufficient PBKDF2 Iterations**
   - Only 1000 iterations (should be 100,000+)
   - Using SHA-1 instead of SHA-256
   - Using username as salt (predictable)
   - Location: `hashPasswordForStorage()` function
   - Severity: CRITICAL

10. **Logging Sensitive Data**
    - Logging passwords and sensitive information
    - Location: `logUserActivity()` function
    - Severity: HIGH

## Expected Scanner Results

A proper cryptographic security scanner should detect:

- ✅ 10+ instances of deprecated/broken algorithms (MD5, SHA-1, DES, RC4, 3DES)
- ✅ 5+ instances of hardcoded cryptographic keys/secrets
- ✅ 3+ instances of weak key sizes (RSA-1024)
- ✅ 4+ instances of insecure cipher modes (ECB)
- ✅ 2+ instances of fixed/null initialization vectors
- ✅ 1+ instance of weak random number generation
- ✅ Multiple instances of insecure password hashing

### OSCAL Compliance Issues

This application violates multiple NIST security controls:

- **SC-13**: Cryptographic Protection - Using deprecated algorithms
- **SC-12**: Cryptographic Key Establishment - Hardcoded keys
- **IA-5**: Authenticator Management - Weak password hashing
- **AC-7**: Unsuccessful Login Attempts - No rate limiting
- **AU-9**: Protection of Audit Information - Logging sensitive data

## Installation

```bash
npm install
```

## Running the Application

```bash
npm start
```

The server will start on port 3000.

## API Endpoints

- `POST /api/register` - Register new user (vulnerable auth)
- `POST /api/login` - Login user (vulnerable auth)
- `POST /api/encrypt` - Encrypt data with weak algorithms
- `GET /api/generate-keys` - Generate weak RSA keys
- `POST /api/verify-file` - Verify file with MD5
- `POST /api/payment` - Store credit card (insecure)
- `GET /api/debug/config` - Expose configuration (CRITICAL!)
- `POST /api/backup` - Create insecure backup
- `GET /health` - Health check

## Testing with PQC Scanner

```bash
# From the pqc-scanner root directory
cargo build --release
./target/release/pqc-scanner scan samples/vulnerable-app-1
```

Expected output should include all vulnerabilities listed above with proper severity ratings and OSCAL compliance mapping.

## Security Recommendations

If this were a real application, the fixes would include:

1. Use **bcrypt** or **Argon2** for password hashing
2. Use **AES-256-GCM** for encryption
3. Use **RSA-4096** or **Ed25519** for asymmetric crypto
4. Use **SHA-256** or **SHA-3** for hashing
5. Use **crypto.randomBytes()** for random generation
6. Never hardcode secrets - use environment variables
7. Always use random IVs
8. Use proper key derivation (PBKDF2 with 100,000+ iterations)
9. Never log sensitive data
10. Implement rate limiting and proper error handling

## License

MIT - For testing purposes only
