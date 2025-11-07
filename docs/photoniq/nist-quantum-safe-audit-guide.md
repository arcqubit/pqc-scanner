# NIST Quantum-Safe Cryptography Audit Guide

## Executive Summary

### The Quantum Threat

Quantum computers pose an existential threat to current public-key cryptography. A sufficiently powerful quantum computer running Shor's algorithm can break:
- **RSA** encryption and signatures (all key sizes)
- **Elliptic Curve Cryptography (ECC)** - ECDSA, ECDH, EdDSA
- **Diffie-Hellman (DH)** key exchange
- **DSA** digital signatures

### NIST Post-Quantum Cryptography Timeline

| Date | Milestone |
|------|-----------|
| **2016** | NIST PQC Standardization Project begins |
| **2022** | First PQC algorithms selected |
| **August 2024** | FIPS 203, 204, 205 finalized |
| **2025-2030** | Migration period begins |
| **2030** | Classical algorithms deprecated |
| **2035** | Classical algorithms prohibited in federal systems |

### Critical Action Required

**Organizations must audit and migrate cryptographic systems by 2030-2035** to maintain security and compliance.

---

## 1. NIST-Approved Quantum-Safe Algorithms

### Finalized Standards (August 2024)

#### FIPS 203: ML-KEM (Module-Lattice-Based Key Encapsulation Mechanism)
- **Previously known as**: CRYSTALS-Kyber
- **Purpose**: Key establishment for symmetric encryption
- **Security Levels**: ML-KEM-512, ML-KEM-768, ML-KEM-1024
- **Use Cases**: TLS key exchange, hybrid encryption, secure messaging
- **Implementation Libraries**:
  - `liboqs` (C/C++)
  - `pqcrypto` (Rust)
  - `Open Quantum Safe` (multiple languages)
  - `Bouncy Castle` (Java, C#)

#### FIPS 204: ML-DSA (Module-Lattice-Based Digital Signature Algorithm)
- **Previously known as**: CRYSTALS-Dilithium
- **Purpose**: Digital signatures
- **Security Levels**: ML-DSA-44, ML-DSA-65, ML-DSA-87
- **Use Cases**: Code signing, document signing, certificate authorities
- **Key Sizes**: Public keys 1-2KB, signatures 2-4KB

#### FIPS 205: SLH-DSA (Stateless Hash-Based Digital Signature Algorithm)
- **Previously known as**: SPHINCS+
- **Purpose**: Digital signatures (hash-based, conservative approach)
- **Security Levels**: Multiple parameter sets
- **Use Cases**: Long-term signatures, backup signing method
- **Note**: Larger signatures (7-50KB) but highly conservative security

### Additional NIST Candidates (Round 4)

#### FALCON
- **Status**: Under consideration for standardization
- **Type**: Lattice-based signatures
- **Advantage**: Smaller signatures than ML-DSA (~700 bytes)
- **Use Case**: Constrained environments

#### BIKE, Classic McEliece, HQC
- **Status**: Round 4 candidates for key encapsulation
- **Purpose**: Alternative KEM mechanisms
- **Timeline**: Expected standardization 2024-2025

---

## 2. Vulnerable Classical Algorithms

### Critical Vulnerabilities (Replace Immediately)

| Algorithm | Quantum Vulnerability | Timeline |
|-----------|----------------------|----------|
| **RSA (all sizes)** | Broken by Shor's algorithm | Deprecated 2030 |
| **ECDSA (all curves)** | Broken by Shor's algorithm | Deprecated 2030 |
| **ECDH** | Broken by Shor's algorithm | Deprecated 2030 |
| **DSA** | Broken by Shor's algorithm | Deprecated 2030 |
| **DH** | Broken by Shor's algorithm | Deprecated 2030 |
| **EdDSA (Ed25519)** | Broken by Shor's algorithm | Deprecated 2030 |

### Hash Functions (Quantum-Resistant)

**Still Secure with Larger Output Sizes:**
- SHA-256 → SHA-512 (double hash size for quantum resistance)
- SHA-3 family (Keccak)
- SHAKE256

**Vulnerable (Already Deprecated):**
- MD5 ❌
- SHA-1 ❌

### Symmetric Encryption (Mostly Safe with Key Size Increase)

**AES** remains secure with larger keys:
- AES-128 → Use AES-256 (quantum resistance)
- AES-192 → Use AES-256
- AES-256 → Remains secure

**ChaCha20** with 256-bit keys remains secure.

---

## 3. Multi-Layer Detection Methodology

### Overview

A comprehensive audit requires scanning at **four layers**:

1. **Static Code Analysis** - Source code scanning
2. **Dependency Analysis** - Third-party library scanning
3. **Configuration Analysis** - Config files and certificates
4. **Runtime Analysis** - Live traffic and TLS inspection

---

## 4. Layer 1: Static Code Analysis

### 4.1 Code Pattern Detection

#### Java

**Search for vulnerable algorithms:**

```bash
# Find RSA usage
grep -r "RSA" --include="*.java" .
rg "KeyPairGenerator\.getInstance\(\"RSA\"" --type java

# Find ECDSA usage
rg "EC\"|\"ECDSA\"|\"ECDH" --type java

# Find Diffie-Hellman
rg "DH\"|\"DHE\"|\"ECDHE" --type java

# Common crypto libraries
rg "java\.security\.|javax\.crypto\.|org\.bouncycastle\." --type java
```

**Vulnerable code patterns:**

```java
// ❌ VULNERABLE: RSA key generation
KeyPairGenerator keyGen = KeyPairGenerator.getInstance("RSA");
keyGen.initialize(2048);

// ❌ VULNERABLE: ECDSA
Signature sig = Signature.getInstance("SHA256withECDSA");

// ❌ VULNERABLE: ECDH key agreement
KeyAgreement ka = KeyAgreement.getInstance("ECDH");

// ✅ QUANTUM-SAFE: ML-KEM (Kyber) with Bouncy Castle
// Note: Check for post-quantum libraries
import org.bouncycastle.pqc.crypto.crystals.kyber.*;
```

#### Python

**Search commands:**

```bash
# Find cryptography library usage
rg "from cryptography\.|import cryptography" --type python

# Find RSA
rg "RSA\.|rsa\.|generate_private_key" --type python

# Find ECDSA
rg "ECDSA|ec\.|EllipticCurve" --type python

# Find specific algorithms
rg "SECP256K1|SECP384R1|SECP521R1" --type python
```

**Vulnerable patterns:**

```python
# ❌ VULNERABLE: RSA
from cryptography.hazmat.primitives.asymmetric import rsa
private_key = rsa.generate_private_key(public_exponent=65537, key_size=2048)

# ❌ VULNERABLE: ECDSA
from cryptography.hazmat.primitives.asymmetric import ec
private_key = ec.generate_private_key(ec.SECP256R1())

# ✅ QUANTUM-SAFE: Using liboqs-python
import oqs
with oqs.KeyEncapsulation("Kyber512") as kem:
    public_key = kem.generate_keypair()
```

#### JavaScript/Node.js

**Search commands:**

```bash
# Find crypto module usage
rg "require\('crypto'\)|from 'crypto'" --type js

# Find RSA/ECDSA
rg "RSA|ECDSA|ECDH|prime256v1|secp256k1" --type js

# Find popular crypto libraries
rg "node-forge|crypto-js|elliptic|jsrsasign" --type js
```

**Vulnerable patterns:**

```javascript
// ❌ VULNERABLE: RSA
const { generateKeyPairSync } = require('crypto');
const { publicKey, privateKey } = generateKeyPairSync('rsa', {
  modulusLength: 2048
});

// ❌ VULNERABLE: ECDSA
const ecdh = crypto.createECDH('secp256k1');

// ✅ QUANTUM-SAFE: Using pqc-node (example)
// const pqc = require('@stablelib/pqc');
```

#### Go

**Search commands:**

```bash
# Find crypto package usage
rg "crypto/rsa|crypto/ecdsa|crypto/elliptic" --type go

# Find RSA operations
rg "rsa\.GenerateKey|rsa\.EncryptOAEP" --type go

# Find ECDSA operations
rg "ecdsa\.GenerateKey|elliptic\.P256" --type go
```

**Vulnerable patterns:**

```go
// ❌ VULNERABLE: RSA
import "crypto/rsa"
privateKey, _ := rsa.GenerateKey(rand.Reader, 2048)

// ❌ VULNERABLE: ECDSA
import "crypto/ecdsa"
import "crypto/elliptic"
privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)

// ✅ QUANTUM-SAFE: Using liboqs-go
// import "github.com/open-quantum-safe/liboqs-go/oqs"
```

#### C/C++

**Search commands:**

```bash
# Find OpenSSL usage
rg "openssl/rsa\.h|openssl/ec\.h|EVP_PKEY" --type cpp

# Find RSA functions
rg "RSA_generate_key|RSA_public_encrypt" --type cpp

# Find EC functions
rg "EC_KEY_new|ECDH_compute_key|ECDSA_sign" --type cpp
```

**Vulnerable patterns:**

```c
// ❌ VULNERABLE: RSA with OpenSSL
#include <openssl/rsa.h>
RSA *rsa = RSA_generate_key(2048, RSA_F4, NULL, NULL);

// ❌ VULNERABLE: ECDSA
#include <openssl/ec.h>
EC_KEY *key = EC_KEY_new_by_curve_name(NID_secp256k1);

// ✅ QUANTUM-SAFE: Using liboqs
// #include <oqs/oqs.h>
// OQS_KEM *kem = OQS_KEM_new(OQS_KEM_alg_kyber_512);
```

#### C#/.NET

**Search commands:**

```bash
# Find crypto usage
rg "System\.Security\.Cryptography" --type cs

# Find RSA
rg "RSACryptoServiceProvider|RSA\.Create" --type cs

# Find ECDSA
rg "ECDsaCng|ECDiffieHellman" --type cs
```

**Vulnerable patterns:**

```csharp
// ❌ VULNERABLE: RSA
using System.Security.Cryptography;
using var rsa = RSA.Create(2048);

// ❌ VULNERABLE: ECDSA
using var ecdsa = ECDsa.Create(ECCurve.NamedCurves.nistP256);

// ✅ QUANTUM-SAFE: Using Bouncy Castle PQC
// using Org.BouncyCastle.Pqc.Crypto.Crystals.Kyber;
```

#### Rust

**Search commands:**

```bash
# Find crypto crates
rg "use rsa::|use ring::|use ed25519" --type rust

# Check Cargo.toml for dependencies
cat Cargo.toml | grep -E "rsa|ring|ed25519|secp256k1"
```

**Vulnerable patterns:**

```rust
// ❌ VULNERABLE: RSA
use rsa::{RsaPrivateKey, RsaPublicKey};
let private_key = RsaPrivateKey::new(&mut rng, 2048)?;

// ❌ VULNERABLE: Ed25519
use ed25519_dalek::Keypair;

// ✅ QUANTUM-SAFE: Using pqcrypto
// use pqcrypto_kyber::kyber512;
```

---

## 5. Layer 2: Dependency Analysis

### 5.1 Package Manager Scanning

#### NPM (Node.js)

```bash
# List all dependencies
npm list --all

# Search for crypto libraries
npm list | grep -E "crypto|rsa|ecdsa|elliptic"

# Check package.json
cat package.json | jq '.dependencies, .devDependencies' | grep -E "crypto|rsa|elliptic"

# Vulnerable packages to flag
grep -E "node-rsa|jsrsasign|node-forge|crypto-js|elliptic" package.json
```

**Common vulnerable libraries:**
- `node-rsa` - RSA implementation
- `elliptic` - Elliptic curve cryptography
- `jsrsasign` - RSA/ECDSA signing
- `node-forge` - General crypto (includes RSA/ECC)

**Quantum-safe alternatives:**
- `@stablelib/pqc` (if available)
- Custom implementations with liboqs bindings

#### pip (Python)

```bash
# List installed packages
pip list | grep -E "crypto|rsa|ecdsa"

# Check requirements.txt
cat requirements.txt | grep -E "cryptography|rsa|ecdsa|pycrypto"

# Vulnerable packages
pip list | grep -E "rsa|ecdsa|pycryptodome|cryptography"
```

**Common vulnerable libraries:**
- `cryptography` - General crypto (includes RSA/ECC)
- `rsa` - Pure Python RSA
- `ecdsa` - Pure Python ECDSA
- `pycryptodome` - PyCrypto fork with RSA/ECC

**Quantum-safe alternatives:**
- `liboqs-python` - NIST PQC algorithms
- `pqcrypto` - Post-quantum cryptography

#### Maven (Java)

```bash
# List dependencies
mvn dependency:tree | grep -E "crypto|bouncy"

# Check pom.xml
grep -E "bouncycastle|crypto" pom.xml

# Analyze with dependency-check
mvn org.owasp:dependency-check-maven:check
```

**Common libraries:**
- `org.bouncycastle:bcprov-jdk15on` - Bouncy Castle (includes RSA/ECC)
- `javax.crypto` - Java Cryptography Extension

**Quantum-safe alternatives:**
- `org.bouncycastle:bcprov-jdk18on` version 1.70+ (includes PQC)
- `liboqs-java`

#### Gradle (Java/Kotlin)

```bash
# List dependencies
./gradlew dependencies | grep -E "crypto|bouncy"

# Check build.gradle
grep -E "bouncycastle|crypto" build.gradle
```

#### Cargo (Rust)

```bash
# List dependencies
cargo tree | grep -E "crypto|rsa|ed25519"

# Check Cargo.toml
cat Cargo.toml | grep -E "rsa|ring|ed25519|secp256k1"

# Audit dependencies
cargo audit
```

**Common crates:**
- `rsa` - RSA implementation
- `ring` - Crypto library (includes ECDSA)
- `ed25519-dalek` - Ed25519 signatures
- `secp256k1` - Bitcoin's elliptic curve

**Quantum-safe alternatives:**
- `pqcrypto-kyber`
- `pqcrypto-dilithium`
- `oqs` - Rust bindings for liboqs

#### NuGet (.NET)

```bash
# List packages
dotnet list package

# Search for crypto packages
dotnet list package | grep -i crypto

# Check packages.config or .csproj
grep -E "BouncyCastle|Crypto" packages.config
```

### 5.2 Software Composition Analysis (SCA) Tools

```bash
# Snyk
snyk test --all-projects
snyk monitor

# OWASP Dependency-Check
dependency-check --scan . --format HTML

# Trivy
trivy fs --security-checks vuln,config .

# Grype
grype dir:.
```

---

## 6. Layer 3: Configuration File Analysis

### 6.1 SSL/TLS Configuration

#### Nginx

```bash
# Find nginx configs
find /etc/nginx -name "*.conf" -o -name "nginx.conf"

# Check for vulnerable ciphers
grep -r "ssl_ciphers\|ssl_protocols" /etc/nginx/

# Look for RSA/ECDSA certificates
grep -r "ssl_certificate" /etc/nginx/
```

**Vulnerable patterns:**

```nginx
# ❌ VULNERABLE: RSA/ECDSA only
ssl_certificate /etc/nginx/ssl/rsa-cert.pem;
ssl_certificate_key /etc/nginx/ssl/rsa-key.pem;

ssl_protocols TLSv1.2 TLSv1.3;
ssl_ciphers ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES256-GCM-SHA384;

# ✅ FUTURE: Hybrid post-quantum TLS (TLS 1.3 + PQC)
# This will be supported in future TLS versions
```

#### Apache

```bash
# Find Apache configs
find /etc/apache2 /etc/httpd -name "*.conf"

# Check SSL settings
grep -r "SSLCipherSuite\|SSLProtocol" /etc/apache2/

# Check certificates
grep -r "SSLCertificateFile" /etc/apache2/
```

#### HAProxy

```bash
# Check HAProxy config
grep -E "ssl-default-bind-ciphers|bind.*ssl" /etc/haproxy/haproxy.cfg
```

### 6.2 Application Configuration Files

#### application.yml / application.properties (Spring Boot)

```bash
# Search for SSL/crypto config
grep -r "ssl\|tls\|keystore\|cipher" application*.yml application*.properties

# Check for key files
find . -name "*.jks" -o -name "*.p12" -o -name "*.keystore"
```

**Example patterns:**

```yaml
# ❌ VULNERABLE: RSA keystore
server:
  ssl:
    key-store: classpath:keystore.jks
    key-store-type: JKS
    key-alias: tomcat  # Likely RSA
```

#### config.json / appsettings.json

```bash
# Search Node.js/ASP.NET configs
find . -name "config*.json" -o -name "appsettings*.json" | xargs grep -l "ssl\|crypto\|certificate"
```

### 6.3 Database Configurations

```bash
# PostgreSQL
grep -r "ssl_cert_file\|ssl_key_file" /etc/postgresql/

# MySQL
grep -r "ssl-cert\|ssl-key" /etc/mysql/

# MongoDB
grep -r "PEMKeyFile\|tlsMode" /etc/mongod.conf
```

### 6.4 API Gateway & Load Balancer Configs

#### AWS API Gateway

```bash
# Check for custom domain certificates (via AWS CLI)
aws apigateway get-domain-names --query 'items[*].[domainName,securityPolicy]'

# Cloudfront distributions
aws cloudfront list-distributions --query 'DistributionList.Items[*].[Id,ViewerCertificate]'
```

#### Kubernetes Ingress

```bash
# Find ingress TLS configs
kubectl get ingress -A -o yaml | grep -A 10 "tls:"

# Check secrets containing certificates
kubectl get secrets -A | grep tls
```

### 6.5 Certificate Inspection

```bash
# Inspect certificate algorithm
openssl x509 -in cert.pem -text -noout | grep "Public Key Algorithm\|Signature Algorithm"

# Check if RSA
openssl x509 -in cert.pem -text -noout | grep -i rsa

# Check if ECDSA
openssl x509 -in cert.pem -text -noout | grep -i ecdsa

# Example output for vulnerable cert:
# Public Key Algorithm: rsaEncryption  ❌ VULNERABLE
# Signature Algorithm: sha256WithRSAEncryption  ❌ VULNERABLE
```

**Batch certificate scanning:**

```bash
#!/bin/bash
# Scan all certificates in directory
for cert in $(find . -name "*.pem" -o -name "*.crt"); do
  echo "Checking: $cert"
  openssl x509 -in "$cert" -text -noout | grep -E "Public Key Algorithm|Signature Algorithm"
  echo "---"
done
```

---

## 7. Layer 4: Runtime Detection

### 7.1 TLS Handshake Analysis

#### testssl.sh

```bash
# Install
git clone https://github.com/drwetter/testssl.sh.git

# Test TLS configuration
./testssl.sh --protocols --ciphers https://example.com

# Check for PQC support (future)
./testssl.sh --pqc https://example.com
```

#### nmap with ssl-enum-ciphers

```bash
# Scan TLS ciphers
nmap --script ssl-enum-ciphers -p 443 example.com

# Look for RSA/ECDHE in output
nmap --script ssl-enum-ciphers -p 443 example.com | grep -E "RSA|ECDHE|ECDSA"
```

### 7.2 Network Traffic Analysis

#### Wireshark / tshark

```bash
# Capture TLS handshake
tshark -i eth0 -Y "tls.handshake.type == 1" -T fields -e tls.handshake.ciphersuite

# Filter for key exchange methods
tshark -r capture.pcap -Y "tls" -T fields -e tls.handshake.extensions.key_share

# Analyze cipher suites
tshark -r capture.pcap -Y "tls.handshake.ciphersuite" | sort | uniq -c
```

### 7.3 Certificate Chain Validation

```bash
# Verify certificate chain
openssl s_client -connect example.com:443 -showcerts

# Extract and analyze all certs in chain
openssl s_client -connect example.com:443 -showcerts 2>/dev/null | \
  awk '/BEGIN CERT/,/END CERT/ {print}' | \
  while read line; do
    if [[ "$line" == "-----BEGIN CERTIFICATE-----" ]]; then
      cert=""
    fi
    cert="$cert$line\n"
    if [[ "$line" == "-----END CERTIFICATE-----" ]]; then
      echo -e "$cert" | openssl x509 -text -noout | grep "Public Key Algorithm"
    fi
  done
```

---

## 8. Automated Detection Tools

### 8.1 Static Application Security Testing (SAST)

#### Semgrep

```bash
# Install
pip install semgrep

# Run crypto-specific rules
semgrep --config "p/security-audit" .
semgrep --config "p/crypto" .

# Custom rule for RSA detection
semgrep -e 'RSA' -lang java .
```

**Custom Semgrep rule for quantum vulnerability:**

```yaml
# semgrep-pqc-rules.yml
rules:
  - id: vulnerable-rsa-usage
    pattern: KeyPairGenerator.getInstance("RSA")
    message: RSA is quantum-vulnerable. Migrate to ML-KEM (FIPS 203)
    languages: [java]
    severity: ERROR

  - id: vulnerable-ecdsa-usage
    pattern: Signature.getInstance("SHA256withECDSA")
    message: ECDSA is quantum-vulnerable. Migrate to ML-DSA (FIPS 204)
    languages: [java]
    severity: ERROR
```

#### Bearer

```bash
# Install
curl -sfL https://raw.githubusercontent.com/Bearer/bearer/main/contrib/install.sh | sh

# Scan for crypto usage
bearer scan . --format json --output bearer-report.json
```

#### Trivy

```bash
# Scan filesystem
trivy fs --security-checks vuln,secret,config .

# Scan container images
trivy image myapp:latest

# Generate report
trivy fs --format json --output trivy-report.json .
```

### 8.2 Crypto-Specific Tools

#### cryptolyzer

```bash
# Install
pip install cryptolyzer

# Analyze TLS
cryptolyzer tls all example.com:443

# Analyze SSH
cryptolyzer ssh all example.com:22
```

#### CryptoMator

```bash
# Python tool to detect crypto usage
git clone https://github.com/TQRG/cryptomator
python cryptomator.py /path/to/code

# Outputs: Crypto API usage, algorithm detection
```

### 8.3 CI/CD Integration

#### GitHub Actions Example

```yaml
# .github/workflows/pqc-audit.yml
name: Post-Quantum Crypto Audit

on: [push, pull_request]

jobs:
  crypto-audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Run Semgrep Crypto Scan
        run: |
          pip install semgrep
          semgrep --config "p/crypto" --json --output semgrep-results.json .

      - name: Check for RSA/ECDSA
        run: |
          if grep -r "RSA\|ECDSA\|ECDH" --include="*.java" --include="*.py" .; then
            echo "::warning::Quantum-vulnerable algorithms detected"
          fi

      - name: Dependency Check
        run: |
          pip install safety
          safety check --json

      - name: Upload Results
        uses: actions/upload-artifact@v3
        with:
          name: crypto-audit-report
          path: semgrep-results.json
```

---

## 9. Risk Assessment Framework

### 9.1 Algorithm Risk Matrix

| Algorithm | Key Size | Quantum Risk | Classical Risk | Timeline | Action |
|-----------|----------|--------------|----------------|----------|--------|
| RSA | < 2048 | Critical | Critical | Replace Now | Immediate |
| RSA | 2048 | Critical | Medium | Deprecated 2030 | Plan migration |
| RSA | 3072 | Critical | Low | Deprecated 2030 | Plan migration |
| RSA | 4096 | Critical | Low | Deprecated 2030 | Plan migration |
| ECDSA P-256 | 256-bit | Critical | Low | Deprecated 2030 | Plan migration |
| ECDSA P-384 | 384-bit | Critical | Low | Deprecated 2030 | Plan migration |
| Ed25519 | 256-bit | Critical | Low | Deprecated 2030 | Plan migration |
| DH | < 2048 | Critical | Critical | Replace Now | Immediate |
| DH | 2048+ | Critical | Medium | Deprecated 2030 | Plan migration |
| AES-128 | 128-bit | Medium | Low | Use AES-256 | Upgrade |
| AES-256 | 256-bit | Low | Low | Safe | No action |
| SHA-256 | N/A | Medium | Low | Use SHA-512 | Consider upgrade |
| SHA-512 | N/A | Low | Low | Safe | No action |
| ML-KEM | N/A | Low | Low | NIST approved | ✅ Quantum-safe |
| ML-DSA | N/A | Low | Low | NIST approved | ✅ Quantum-safe |

### 9.2 Risk Scoring

**Formula:**
```
Risk Score = (Quantum Vulnerability × 0.6) + (Implementation Risk × 0.3) + (Compliance Risk × 0.1)
```

**Quantum Vulnerability:**
- 10: RSA/ECDSA/DH in critical systems
- 7: RSA/ECDSA/DH in non-critical systems
- 5: AES-128, SHA-256
- 2: AES-256, SHA-512
- 0: NIST PQC algorithms

**Implementation Risk:**
- 10: Public-facing API, high-value data
- 7: Internal systems, moderate data
- 5: Development/staging environments
- 2: Test environments

**Compliance Risk:**
- 10: Federal/government systems (NIST deadline applies)
- 7: Financial services (regulatory pressure)
- 5: Healthcare (HIPAA considerations)
- 2: General commercial

### 9.3 Prioritization Matrix

```
Priority = Risk Score × Impact × Exposure

High Priority (Score > 70):
- Public-facing authentication systems using RSA
- Payment processing with ECDSA
- VPN/TLS termination with DH

Medium Priority (Score 40-70):
- Internal APIs with RSA
- Code signing certificates
- Database encryption

Low Priority (Score < 40):
- Test environments
- Short-lived session tokens
- Non-critical internal tools
```

---

## 10. Migration Roadmap

### Phase 1: Discovery & Assessment (2024-2025)

**Objectives:**
- Complete crypto inventory
- Identify all vulnerable algorithms
- Assess migration complexity
- Establish crypto-agility framework

**Actions:**
1. Run automated scans (SAST, SCA, runtime)
2. Manual code review of critical systems
3. Document all crypto usage with data flow diagrams
4. Identify dependencies and third-party integrations
5. Calculate migration effort (time, resources, cost)

**Deliverables:**
- Comprehensive crypto inventory
- Risk assessment report
- Migration plan with timeline
- Resource allocation plan

### Phase 2: Crypto-Agility Implementation (2025-2026)

**Objectives:**
- Abstract crypto implementations
- Enable algorithm switching without code changes
- Implement hybrid encryption where possible

**Actions:**
1. Create crypto abstraction layer
2. Implement configuration-driven algorithm selection
3. Add monitoring and logging for crypto operations
4. Deploy hybrid encryption (classical + PQC)

**Example Crypto-Agility Pattern:**

```java
// Crypto abstraction interface
public interface CryptoProvider {
    KeyPair generateKeyPair();
    byte[] encrypt(byte[] data, PublicKey key);
    byte[] decrypt(byte[] data, PrivateKey key);
}

// Configuration-driven implementation
public class CryptoFactory {
    public static CryptoProvider getProvider() {
        String algorithm = Config.get("crypto.algorithm");
        switch (algorithm) {
            case "RSA":
                return new RSAProvider(); // Legacy
            case "ML-KEM":
                return new KyberProvider(); // PQC
            case "HYBRID":
                return new HybridProvider(); // Both
            default:
                throw new IllegalArgumentException("Unknown algorithm");
        }
    }
}
```

### Phase 3: Hybrid Deployment (2026-2030)

**Objectives:**
- Deploy PQC alongside classical crypto
- Maintain backward compatibility
- Test PQC implementations in production

**Hybrid Encryption Strategy:**

```
Classical + PQC Hybrid:
1. Generate both RSA and ML-KEM key pairs
2. Encrypt data with both algorithms
3. Require both decryptions to succeed
4. Provides security even if one algorithm is broken

Benefits:
- Gradual migration with safety net
- Backward compatibility
- No single point of failure
```

**Implementation Example:**

```python
# Hybrid encryption
def hybrid_encrypt(data, rsa_public_key, kyber_public_key):
    # Generate symmetric key
    aes_key = os.urandom(32)

    # Encrypt data with AES
    ciphertext = aes_encrypt(data, aes_key)

    # Encrypt AES key with both RSA and Kyber
    rsa_wrapped_key = rsa_encrypt(aes_key, rsa_public_key)
    kyber_wrapped_key = kyber_encapsulate(kyber_public_key)

    return {
        "ciphertext": ciphertext,
        "rsa_key": rsa_wrapped_key,
        "kyber_key": kyber_wrapped_key
    }
```

### Phase 4: Full PQC Migration (2030-2035)

**Objectives:**
- Replace all classical algorithms with PQC
- Deprecate hybrid mode
- Achieve full quantum resistance

**Actions:**
1. Migrate all systems to pure PQC
2. Revoke classical certificates
3. Update all third-party integrations
4. Decommission RSA/ECDSA infrastructure

**Timeline:**
- **2030**: Deprecation of classical algorithms
- **2033**: Hybrid mode mandatory for federal systems
- **2035**: Classical algorithms prohibited

---

## 11. Compliance & Standards

### 11.1 NIST Requirements

**Key Standards:**
- **NIST SP 800-208**: Recommendation for Stateful Hash-Based Signature Schemes
- **NIST SP 800-209**: (Upcoming) Post-Quantum Cryptography Recommendations
- **FIPS 203**: ML-KEM Standard
- **FIPS 204**: ML-DSA Standard
- **FIPS 205**: SLH-DSA Standard

**Federal Requirements:**
- All federal agencies must migrate to PQC by 2035
- Critical systems should begin migration by 2025
- Annual progress reports required

### 11.2 Industry-Specific Requirements

#### Financial Services

**PCI-DSS:**
- Currently requires strong cryptography (RSA 2048+, AES-256)
- Expected to mandate PQC in future versions (2026-2030)

**SOC 2:**
- Crypto-agility required for Type II compliance
- Document algorithm selection rationale

#### Healthcare

**HIPAA:**
- "Addressable" encryption requirements
- Must demonstrate quantum readiness in security risk analysis

#### Government

**FedRAMP:**
- High/Moderate systems must have PQC migration plan by 2025
- Full migration required by 2035

**CMMC:**
- Level 3+ will require PQC roadmap
- Supply chain crypto-agility expected

---

## 12. Practical Audit Checklist

### Pre-Audit Preparation

- [ ] Define audit scope (applications, systems, data flows)
- [ ] Assemble audit team (security, dev, ops)
- [ ] Identify critical systems and data
- [ ] Set up audit tools and environment
- [ ] Establish baseline crypto inventory

### Discovery Phase

**Static Analysis:**
- [ ] Run SAST tools (Semgrep, Bearer, Trivy)
- [ ] Search source code for crypto patterns (all languages)
- [ ] Identify custom crypto implementations
- [ ] Document all crypto API usage

**Dependency Analysis:**
- [ ] Scan package dependencies (npm, pip, maven, etc.)
- [ ] Identify vulnerable crypto libraries
- [ ] Check for library version vulnerabilities
- [ ] Document third-party crypto usage

**Configuration Analysis:**
- [ ] Review SSL/TLS configurations (nginx, apache, haproxy)
- [ ] Inspect application config files
- [ ] Analyze database encryption settings
- [ ] Check API gateway and load balancer configs
- [ ] Audit cloud provider crypto settings (AWS KMS, Azure Key Vault, etc.)

**Certificate Analysis:**
- [ ] Inventory all digital certificates
- [ ] Check certificate algorithms (RSA, ECDSA)
- [ ] Identify certificate expiration dates
- [ ] Validate certificate chains
- [ ] Document Certificate Authority dependencies

**Runtime Analysis:**
- [ ] Perform TLS handshake analysis (testssl.sh)
- [ ] Capture and analyze network traffic
- [ ] Test cipher suite negotiation
- [ ] Verify actual runtime crypto usage

### Risk Assessment

- [ ] Score each vulnerable algorithm (risk matrix)
- [ ] Prioritize systems by risk score
- [ ] Calculate migration effort estimates
- [ ] Identify quick wins vs. complex migrations
- [ ] Document dependencies and blockers

### Reporting

- [ ] Generate executive summary
- [ ] Create detailed findings report
- [ ] Develop remediation roadmap
- [ ] Estimate budget and timeline
- [ ] Present to stakeholders

### Ongoing Monitoring

- [ ] Set up continuous crypto scanning in CI/CD
- [ ] Implement crypto usage monitoring
- [ ] Establish quarterly re-audit schedule
- [ ] Track migration progress
- [ ] Update risk assessments regularly

---

## 13. Automated Audit Script

See `scripts/quantum-safe-audit.sh` for a complete automated audit script.

**Quick usage:**

```bash
# Run full audit
./scripts/quantum-safe-audit.sh --full

# Scan specific directory
./scripts/quantum-safe-audit.sh --scan /path/to/code

# Generate HTML report
./scripts/quantum-safe-audit.sh --full --output report.html

# CI/CD mode (exit non-zero on findings)
./scripts/quantum-safe-audit.sh --ci
```

---

## 14. Resources

### Official NIST Resources

- **NIST PQC Project**: https://csrc.nist.gov/projects/post-quantum-cryptography
- **FIPS 203 (ML-KEM)**: https://csrc.nist.gov/pubs/fips/203/final
- **FIPS 204 (ML-DSA)**: https://csrc.nist.gov/pubs/fips/204/final
- **FIPS 205 (SLH-DSA)**: https://csrc.nist.gov/pubs/fips/205/final
- **NIST IR 8413**: Status Report on the Third Round of the NIST PQC Standardization Process

### Implementation Libraries

- **liboqs**: https://github.com/open-quantum-safe/liboqs (C/C++)
- **liboqs-python**: https://github.com/open-quantum-safe/liboqs-python
- **Bouncy Castle**: https://www.bouncycastle.org/ (Java, C#)
- **pqcrypto**: https://github.com/rustpq/pqcrypto (Rust)

### Tools

- **Semgrep**: https://semgrep.dev/
- **testssl.sh**: https://testssl.sh/
- **Trivy**: https://trivy.dev/
- **OWASP Dependency-Check**: https://owasp.org/www-project-dependency-check/

### Further Reading

- **Quantum Threat Timeline**: https://globalriskinstitute.org/publications/quantum-threat-timeline/
- **NSA Quantum Computing FAQ**: https://media.defense.gov/2021/Aug/04/2002821837/-1/-1/1/Quantum_FAQs_20210804.PDF
- **CISA Post-Quantum Cryptography Initiative**: https://www.cisa.gov/quantum

---

## Appendix A: Glossary

**ML-KEM**: Module-Lattice-Based Key Encapsulation Mechanism (formerly CRYSTALS-Kyber)

**ML-DSA**: Module-Lattice-Based Digital Signature Algorithm (formerly CRYSTALS-Dilithium)

**SLH-DSA**: Stateless Hash-Based Digital Signature Algorithm (formerly SPHINCS+)

**PQC**: Post-Quantum Cryptography - Cryptographic algorithms designed to resist quantum computer attacks

**Shor's Algorithm**: Quantum algorithm that can break RSA, ECDSA, and DH in polynomial time

**Grover's Algorithm**: Quantum algorithm that reduces symmetric key security by half (e.g., AES-128 → effective 64-bit security)

**Hybrid Encryption**: Using both classical and post-quantum algorithms simultaneously for defense-in-depth

**Crypto-Agility**: Ability to rapidly change cryptographic algorithms without significant system redesign

**Harvest Now, Decrypt Later (HNDL)**: Threat where adversaries capture encrypted data today to decrypt with future quantum computers

---

*Last Updated: November 2024*
*Next Review: August 2025*
*Version: 1.0*
