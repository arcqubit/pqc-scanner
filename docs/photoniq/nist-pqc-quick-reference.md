# NIST Post-Quantum Cryptography Quick Reference

## 🎯 NIST-Approved PQC Algorithms (2024)

| Standard | Algorithm | Previous Name | Purpose | Status |
|----------|-----------|---------------|---------|--------|
| **FIPS 203** | ML-KEM | CRYSTALS-Kyber | Key Encapsulation | ✅ Finalized Aug 2024 |
| **FIPS 204** | ML-DSA | CRYSTALS-Dilithium | Digital Signatures | ✅ Finalized Aug 2024 |
| **FIPS 205** | SLH-DSA | SPHINCS+ | Hash-Based Signatures | ✅ Finalized Aug 2024 |
| *TBD* | FALCON | FALCON | Signatures (compact) | 🔄 Round 4 |

---

## ⚠️ Quantum-Vulnerable Algorithms

### Critical (Replace by 2030-2035)

```
❌ RSA (all key sizes)
❌ ECDSA / ECDH / EdDSA (all curves)
❌ DSA / Diffie-Hellman
❌ Curve25519, P-256, P-384, secp256k1
```

### Still Safe (With Upgrades)

```
✅ AES-256 (quantum-resistant)
⚠️ AES-128 → Upgrade to AES-256
✅ SHA-512 (quantum-resistant)
⚠️ SHA-256 → Consider SHA-512
✅ ChaCha20-Poly1305 with 256-bit keys
```

---

## 🔍 Quick Detection Commands

### Search All Languages

```bash
# Find RSA usage (all languages)
rg "RSA|rsa\.generate|RSA_generate|KeyPairGenerator.*RSA" .

# Find ECDSA usage
rg "ECDSA|ECDH|EdDSA|Ed25519|secp256k1|prime256v1" .

# Find Diffie-Hellman
rg "DH\"|DHE|ECDHE|DiffieHellman" .

# Find crypto libraries
rg "java\.security|javax\.crypto|cryptography\.hazmat|require\('crypto'\)|crypto/rsa" .
```

### Language-Specific

```bash
# Java
rg "KeyPairGenerator\.getInstance\(\"RSA\"|\"EC\"" --type java

# Python
rg "from cryptography\.hazmat|import rsa|import ecdsa" --type python

# JavaScript
rg "require\('crypto'\)|crypto\.createECDH|generateKeyPairSync" --type js

# Go
rg "crypto/rsa|crypto/ecdsa|crypto/elliptic" --type go

# Rust
rg "use rsa::|use ed25519" --type rust

# C/C++
rg "openssl/rsa\.h|EVP_PKEY|RSA_generate_key" --type cpp
```

### Check Dependencies

```bash
# Node.js
npm list | grep -E "rsa|ecdsa|elliptic|crypto"

# Python
pip list | grep -E "rsa|ecdsa|cryptography"

# Java
mvn dependency:tree | grep -i crypto

# Rust
cargo tree | grep -E "rsa|ring|ed25519"

# .NET
dotnet list package | grep -i crypto
```

### Check Certificates

```bash
# Inspect certificate algorithm
openssl x509 -in cert.pem -text -noout | grep "Public Key Algorithm"

# Batch scan all certificates
find . -name "*.pem" -o -name "*.crt" | while read cert; do
  echo "$cert:"
  openssl x509 -in "$cert" -text -noout | grep "Public Key Algorithm"
done
```

### Test TLS Configuration

```bash
# Quick TLS test
nmap --script ssl-enum-ciphers -p 443 example.com

# Detailed TLS analysis
testssl.sh --protocols --ciphers https://example.com

# Check specific domain
curl -vI https://example.com 2>&1 | grep -i "cipher"
```

---

## 🛠️ Essential Tools

### Static Analysis

```bash
# Semgrep (code scanning)
semgrep --config "p/crypto" .

# Trivy (filesystem & container)
trivy fs --security-checks vuln,secret,config .

# Bearer (crypto detection)
bearer scan . --format json
```

### Dependency Analysis

```bash
# OWASP Dependency-Check
dependency-check --scan . --format HTML

# Snyk
snyk test --all-projects

# Grype
grype dir:.
```

### Certificate/TLS Testing

```bash
# testssl.sh (comprehensive TLS analysis)
testssl.sh --quiet --jsonfile output.json https://example.com
testssl.sh --full https://example.com

# pqcscan (PQC detection)  ⭐ NEW!
pqcscan --host example.com --port 443 --json

# nmap SSL scan
nmap --script ssl-cert,ssl-enum-ciphers -p 443 example.com

# OpenSSL client test
openssl s_client -connect example.com:443 -showcerts
```

### Network TLS Scanning (New! ⭐)

```bash
# Create endpoints file
cat > tls-endpoints.txt << EOF
example.com:443
api.example.com:443
EOF

# Scan with quantum-safe audit script (uses testssl.sh + pqcscan)
./scripts/quantum-safe-audit.sh --full

# Or use environment variable
export TLS_ENDPOINTS="example.com:443,api.example.com:443"
./scripts/quantum-safe-audit.sh --full
```

---

## 📊 Risk Priority Matrix

| Algorithm | Timeline | Action | Priority |
|-----------|----------|--------|----------|
| RSA < 2048 | **Replace NOW** | Critical vulnerability | 🔴 CRITICAL |
| MD5, SHA-1 | **Replace NOW** | Already broken | 🔴 CRITICAL |
| RSA 2048-4096 | Deprecated 2030 | Plan migration | 🟡 HIGH |
| ECDSA/ECDH | Deprecated 2030 | Plan migration | 🟡 HIGH |
| Ed25519 | Deprecated 2030 | Plan migration | 🟡 HIGH |
| AES-128 | Consider upgrade | Use AES-256 | 🟢 MEDIUM |
| SHA-256 | Consider upgrade | Use SHA-512 | 🟢 LOW |

---

## 🚀 Migration Timeline

```
2024-2025: DISCOVERY & ASSESSMENT
├─ Run automated scans
├─ Complete crypto inventory
├─ Risk assessment
└─ Migration planning

2025-2026: CRYPTO-AGILITY
├─ Abstract crypto implementations
├─ Configuration-driven algorithms
├─ Monitoring & logging
└─ Hybrid encryption prep

2026-2030: HYBRID DEPLOYMENT
├─ Deploy PQC + classical together
├─ Test in production
├─ Gradual rollout
└─ Maintain compatibility

2030-2035: FULL PQC MIGRATION
├─ Replace all classical algorithms
├─ Revoke RSA/ECDSA certificates
├─ Pure PQC deployment
└─ Compliance verification
```

---

## 📦 PQC Implementation Libraries

### C/C++
```bash
# liboqs - Open Quantum Safe
git clone https://github.com/open-quantum-safe/liboqs.git
cd liboqs && mkdir build && cd build
cmake -GNinja .. && ninja && ninja install
```

### Python
```bash
pip install liboqs-python
```

### Java
```xml
<!-- Bouncy Castle with PQC support -->
<dependency>
    <groupId>org.bouncycastle</groupId>
    <artifactId>bcprov-jdk18on</artifactId>
    <version>1.78</version>
</dependency>
```

### Rust
```toml
# Cargo.toml
[dependencies]
pqcrypto-kyber = "0.8"
pqcrypto-dilithium = "0.5"
```

### JavaScript/Node.js
```bash
# Limited support, check for updated libraries
npm search pqc kyber dilithium
```

---

## 💡 Quick Code Examples

### Vulnerable Pattern (Java)
```java
// ❌ QUANTUM-VULNERABLE
KeyPairGenerator keyGen = KeyPairGenerator.getInstance("RSA");
keyGen.initialize(2048);
KeyPair keyPair = keyGen.generateKeyPair();
```

### Quantum-Safe Pattern (Java)
```java
// ✅ QUANTUM-SAFE (with Bouncy Castle)
import org.bouncycastle.pqc.crypto.crystals.kyber.*;

KyberKeyPairGenerator keyGen = new KyberKeyPairGenerator();
keyGen.init(new KyberKeyGenerationParameters(new SecureRandom(), KyberParameters.kyber512));
AsymmetricCipherKeyPair keyPair = keyGen.generateKeyPair();
```

### Vulnerable Pattern (Python)
```python
# ❌ QUANTUM-VULNERABLE
from cryptography.hazmat.primitives.asymmetric import rsa
private_key = rsa.generate_private_key(public_exponent=65537, key_size=2048)
```

### Quantum-Safe Pattern (Python)
```python
# ✅ QUANTUM-SAFE (with liboqs)
import oqs

with oqs.KeyEncapsulation("Kyber512") as kem:
    public_key = kem.generate_keypair()
    ciphertext, shared_secret = kem.encap_secret(public_key)
```

---

## 🎯 One-Liner Audit Commands

### Quick Vulnerability Scan
```bash
# Check for any quantum-vulnerable crypto in source code
rg -i "RSA|ECDSA|ECDH|EdDSA|secp256k1|prime256v1|Diffie-Hellman" \
  --type-add 'code:*.{java,py,js,go,rs,cpp,cs}' -tcode .
```

### Count Vulnerable Instances
```bash
# Count RSA usage by language
echo "Java: $(rg 'RSA' --type java -c | awk '{s+=$1} END {print s}')"
echo "Python: $(rg 'rsa|RSA' --type python -c | awk '{s+=$1} END {print s}')"
echo "JavaScript: $(rg 'RSA|rsa' --type js -c | awk '{s+=$1} END {print s}')"
```

### Generate Quick Report
```bash
# Simple text report
{
  echo "=== Quantum Crypto Audit Report ==="
  echo "Date: $(date)"
  echo ""
  echo "=== RSA Usage ==="
  rg -l "RSA" --type java --type python --type js
  echo ""
  echo "=== ECDSA Usage ==="
  rg -l "ECDSA|ECDH" --type java --type python --type js
  echo ""
  echo "=== Certificate Analysis ==="
  find . -name "*.pem" -o -name "*.crt" | wc -l
  echo " certificates found"
} > quick-audit-report.txt
```

---

## 🔗 Essential Links

| Resource | URL |
|----------|-----|
| **NIST PQC Project** | https://csrc.nist.gov/projects/post-quantum-cryptography |
| **FIPS 203 (ML-KEM)** | https://csrc.nist.gov/pubs/fips/203/final |
| **FIPS 204 (ML-DSA)** | https://csrc.nist.gov/pubs/fips/204/final |
| **FIPS 205 (SLH-DSA)** | https://csrc.nist.gov/pubs/fips/205/final |
| **Open Quantum Safe** | https://openquantumsafe.org/ |
| **liboqs GitHub** | https://github.com/open-quantum-safe/liboqs |
| **Bouncy Castle** | https://www.bouncycastle.org/ |
| **CISA PQC Initiative** | https://www.cisa.gov/quantum |

---

## 📋 Quick Checklist

- [ ] Run `rg "RSA|ECDSA"` to find vulnerable crypto
- [ ] Check dependencies with `npm list` / `pip list` / `mvn dependency:tree`
- [ ] Inspect certificates with `openssl x509 -in cert.pem -text`
- [ ] Test TLS with `testssl.sh https://example.com`
- [ ] Review SSL/TLS configs in nginx/apache
- [ ] Scan with Semgrep: `semgrep --config "p/crypto" .`
- [ ] Run Trivy: `trivy fs .`
- [ ] Document all findings
- [ ] Prioritize by risk score
- [ ] Create migration roadmap

---

## 🆘 Emergency Response

### Found Vulnerable Crypto in Production?

1. **Assess Impact**
   ```bash
   # Identify what's affected
   rg "RSA|ECDSA" /path/to/critical/system
   ```

2. **Immediate Actions**
   - Do NOT panic - quantum computers can't break RSA yet
   - Document all vulnerable components
   - Assess "Harvest Now, Decrypt Later" risk

3. **Short-Term (< 6 months)**
   - Implement crypto-agility
   - Plan hybrid encryption
   - Begin PQC testing

4. **Long-Term (2025-2030)**
   - Full PQC migration
   - Certificate replacement
   - Third-party coordination

---

*Keep this reference handy during your audit!*
*For detailed information, see: [docs/nist-quantum-safe-audit-guide.md](./nist-quantum-safe-audit-guide.md)*
