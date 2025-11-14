# Vulnerability Documentation - Legacy Banking

This document lists all intentional vulnerabilities in the legacy-banking sample repository.

## Summary

- **Total Vulnerabilities**: 15
- **Critical**: 5
- **High**: 7
- **Medium**: 3

## Detailed Vulnerability List

### 1. RSA-1024 Key Generation (Critical)

**File**: `src/crypto/key-generator.js`

**Locations**:
- Line 15: `modulusLength: 1024` in `generateCustomerEncryptionKeys()`
- Line 30: `modulusLength: 1024` in `generateTransactionKeys()`
- Line 52: `modulusLength: 1024` in `generateDatabaseKey()`

**Issue**: RSA-1024 is quantum-vulnerable and too small for modern security standards. Can be broken by Shor's algorithm on a quantum computer.

**Remediation**: Migrate to Kyber-768 or higher for post-quantum security.

### 2. RSA-512 Key Generation (Critical)

**File**: `src/crypto/key-generator.js`

**Location**: Line 70: `modulusLength: 512` in `generateLegacyKeys()`

**Issue**: RSA-512 can be broken with classical computers. Completely insecure.

**Remediation**: Immediately upgrade to post-quantum algorithms.

### 3. MD5 Password Hashing (Critical)

**File**: `src/auth/password-hasher.js`

**Locations**:
- Line 16: `crypto.createHash('md5')` in `hashPassword()`
- Line 37: `crypto.createHash('md5')` in `legacyHash()`
- Line 49: `crypto.createHash('md5')` in `generateResetToken()`

**Issue**: MD5 is cryptographically broken and vulnerable to collision attacks. No salt is used, making it vulnerable to rainbow table attacks.

**Remediation**: Use Argon2id with proper salt and iterations.

### 4. DES Encryption (High)

**File**: `src/auth/session-manager.js`

**Locations**:
- Line 14: `this.algorithm = 'des'`
- Line 23: `crypto.createCipher(this.algorithm, this.key)` in `encryptSession()`
- Line 35: `crypto.createDecipher(this.algorithm, this.key)` in `decryptSession()`
- Line 47: DES encryption in `generateSessionId()`
- Line 59: DES encryption in `createSecureCookie()`

**Issue**: DES has a 56-bit key size and is vulnerable to brute-force attacks. Can be broken in hours with modern hardware.

**Remediation**: Migrate to AES-256-GCM for authenticated encryption.

### 5. SHA-1 Digital Signatures (High)

**File**: `src/crypto/transaction-signer.js`

**Locations**:
- Line 19: `crypto.createSign('SHA1')` in `signTransaction()`
- Line 37: `crypto.createVerify('SHA1')` in `verifyTransaction()`
- Line 55: `crypto.createHash('sha1')` in `hashTransaction()`
- Line 70: `crypto.createHash('sha1')` in `createFingerprint()`
- Line 83: SHA-1 signature in `signBatch()`

**Issue**: SHA-1 is vulnerable to collision attacks. Not suitable for digital signatures or integrity checking.

**Remediation**: Upgrade to SHA-256 or SHA-3 for hash functions. Use Dilithium for post-quantum signatures.

### 6. Weak Random Number Generation (Medium)

**File**: `src/utils/random.js`

**Locations**:
- Line 14: `Math.random()` in `generateAccountNumber()`
- Line 24: `Math.random()` in `generatePIN()`
- Line 33: `Math.random()` in `generateTransactionId()`
- Line 44: `Math.random()` in `generateReferenceNumber()`
- Line 54: `Math.random()` in `generateVerificationCode()`
- Line 66: `Math.random()` in `shuffleArray()`

**Issue**: Math.random() is not cryptographically secure. Predictable and can be exploited to guess account numbers, PINs, and other sensitive values.

**Remediation**: Use `crypto.randomBytes()` or `crypto.randomInt()` for all security-sensitive random generation.

## Expected Scan Results

```json
{
  "summary": {
    "vulnerabilities_found": 15,
    "compliance_score": 28,
    "risk_score": 78
  },
  "findings": {
    "critical": 5,
    "high": 7,
    "medium": 3,
    "low": 0
  }
}
```

## Remediation Priority

1. **Immediate** (Critical):
   - Replace RSA-512 (completely broken)
   - Replace MD5 password hashing (active threat)
   - Remove RSA-1024 (quantum threat)

2. **High Priority**:
   - Upgrade DES to AES-256-GCM
   - Replace SHA-1 signatures with SHA-256/Dilithium

3. **Medium Priority**:
   - Replace Math.random() with crypto.randomBytes()

## Testing Commands

```bash
# Install dependencies
npm install

# Run tests
npm test

# Scan with PQC Scanner
cd ../..
cargo run --example generate_compliance_report -- \
  --path samples/legacy-banking/src/ \
  --output reports/legacy-banking.json
```
