# testssl.sh and pqcscan Usage Guide

## Overview

This guide demonstrates how to use **testssl.sh** and **pqcscan** for detecting quantum-vulnerable TLS configurations and post-quantum cryptography support.

---

## testssl.sh - Comprehensive TLS Testing

### What testssl.sh Detects

testssl.sh analyzes live TLS/SSL endpoints for:
- ✅ Certificate algorithms (RSA, ECDSA, Ed25519)
- ✅ Key sizes and strengths
- ✅ Cipher suites (including quantum-vulnerable ones)
- ✅ Protocol versions (SSLv2, SSLv3, TLSv1.0-1.3)
- ✅ Key exchange methods (DH, ECDH, RSA)
- ✅ Vulnerabilities (Heartbleed, POODLE, etc.)

### Basic Usage

```bash
# Quick scan with minimal output
testssl.sh --quiet https://example.com

# Full comprehensive scan
testssl.sh --full https://example.com

# Specific checks
testssl.sh --protocols --ciphers https://example.com
testssl.sh --server-defaults https://example.com
testssl.sh --vulnerable https://example.com
```

### JSON Output for Automation

```bash
# Generate JSON output
testssl.sh --quiet --jsonfile example-com.json https://example.com

# Pretty print JSON
testssl.sh --quiet --jsonfile-pretty example-com-pretty.json https://example.com
```

### Quantum-Relevant Analysis

#### Check Certificate Algorithm

```bash
# Full scan with certificate details
testssl.sh --quiet --jsonfile output.json https://example.com

# Extract certificate algorithm with jq
jq -r '.scanResult[] | select(.id == "cert_signatureAlgorithm") | .finding' output.json

# Expected output examples:
# "SHA256 with RSA"  ❌ Quantum-vulnerable
# "ECDSA with SHA384" ❌ Quantum-vulnerable
# "Ed25519" ❌ Quantum-vulnerable
```

#### Check Key Exchange Methods

```bash
# Check cipher suites and key exchange
testssl.sh --quiet --ciphers https://example.com | grep -E "ECDHE|DHE|RSA"

# Example output:
# TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256  ❌ RSA authentication
# TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 ❌ ECDSA authentication
```

#### Detect Certificate Key Type and Size

```bash
# Extract key information
jq -r '.scanResult[] | select(.id == "cert_keySize") | .finding' output.json

# Example output:
# "RSA 2048 bits" ❌ Quantum-vulnerable
# "ECDSA 256 bits (P-256)" ❌ Quantum-vulnerable
```

### Batch Scanning Multiple Endpoints

```bash
#!/bin/bash
# Create endpoints file
cat > tls-endpoints.txt << EOF
example.com:443
api.example.com:443
mail.example.com:587
www.example.com:443
EOF

# Scan all endpoints
while IFS= read -r endpoint; do
    echo "Scanning: $endpoint"
    testssl.sh --quiet --jsonfile "results/${endpoint//[:\/]/_}.json" "$endpoint"
done < tls-endpoints.txt

# Analyze results
for json in results/*.json; do
    echo "=== $(basename $json) ==="
    jq -r '.scanResult[] | select(.id == "cert_signatureAlgorithm" or .id == "cert_keySize") | "\(.id): \(.finding)"' "$json"
    echo ""
done
```

### Filter for Quantum-Vulnerable Configurations

```bash
#!/bin/bash
# Find all RSA certificates
for json in results/*.json; do
    host=$(basename "$json" .json)
    sig_alg=$(jq -r '.scanResult[] | select(.id == "cert_signatureAlgorithm") | .finding' "$json")

    if echo "$sig_alg" | grep -iq "RSA"; then
        echo "❌ $host: Uses RSA - $sig_alg"
    fi
done

# Find all ECDSA certificates
for json in results/*.json; do
    host=$(basename "$json" .json)
    sig_alg=$(jq -r '.scanResult[] | select(.id == "cert_signatureAlgorithm") | .finding' "$json")

    if echo "$sig_alg" | grep -iq "ECDSA"; then
        echo "❌ $host: Uses ECDSA - $sig_alg"
    fi
done
```

### Generate Summary Report

```bash
#!/bin/bash
echo "=== TLS Quantum-Vulnerability Report ==="
echo "Generated: $(date)"
echo ""

total=0
rsa_count=0
ecdsa_count=0

for json in results/*.json; do
    total=$((total + 1))
    host=$(basename "$json" .json)
    sig_alg=$(jq -r '.scanResult[] | select(.id == "cert_signatureAlgorithm") | .finding' "$json")

    if echo "$sig_alg" | grep -iq "RSA"; then
        rsa_count=$((rsa_count + 1))
        echo "❌ $host: RSA"
    elif echo "$sig_alg" | grep -iq "ECDSA"; then
        ecdsa_count=$((ecdsa_count + 1))
        echo "❌ $host: ECDSA"
    fi
done

echo ""
echo "Summary:"
echo "  Total endpoints scanned: $total"
echo "  Endpoints with RSA: $rsa_count"
echo "  Endpoints with ECDSA: $ecdsa_count"
echo "  Quantum-vulnerable: $((rsa_count + ecdsa_count)) / $total"
```

---

## pqcscan - Post-Quantum Crypto Detection

### What pqcscan Detects

pqcscan specifically looks for post-quantum cryptography in TLS:
- ✅ ML-KEM (Kyber) key encapsulation
- ✅ ML-DSA (Dilithium) signatures
- ✅ Hybrid cipher suites (classical + PQC)
- ✅ TLS 1.3 PQC extensions
- ✅ X25519Kyber768 hybrid key exchange

### Basic Usage

```bash
# Scan single endpoint
pqcscan --host example.com --port 443

# With JSON output
pqcscan --host example.com --port 443 --json

# Verbose mode
pqcscan --host example.com --port 443 --verbose
```

### JSON Output Analysis

```bash
# Scan and save JSON
pqcscan --host example.com --port 443 --json > pqc-scan.json

# Check for PQC support
jq -r '.pqc_support' pqc-scan.json
# true = PQC detected ✅
# false = No PQC ❌

# List detected PQC algorithms
jq -r '.algorithms[]?' pqc-scan.json

# Example output:
# "X25519Kyber768"
# "Kyber512"
# "Dilithium2"
```

### Batch Scanning for PQC Support

```bash
#!/bin/bash
# Scan multiple endpoints for PQC
cat > tls-endpoints.txt << EOF
example.com:443
pqc-enabled.example.com:443
api.example.com:443
EOF

mkdir -p pqcscan-results

while IFS= read -r endpoint; do
    host="${endpoint%:*}"
    port="${endpoint##*:}"

    echo "Scanning $endpoint for PQC support..."
    pqcscan --host "$host" --port "$port" --json > "pqcscan-results/${host}_${port}.json"

    # Check result
    if jq -e '.pqc_support == true' "pqcscan-results/${host}_${port}.json" > /dev/null; then
        echo "✅ $endpoint: PQC DETECTED"
    else
        echo "❌ $endpoint: No PQC support"
    fi
done < tls-endpoints.txt
```

### Generate PQC Readiness Report

```bash
#!/bin/bash
echo "=== Post-Quantum Cryptography Readiness Report ==="
echo "Generated: $(date)"
echo ""

total=0
pqc_count=0

for json in pqcscan-results/*.json; do
    total=$((total + 1))
    host=$(basename "$json" .json)

    if jq -e '.pqc_support == true' "$json" > /dev/null 2>&1; then
        pqc_count=$((pqc_count + 1))
        algorithms=$(jq -r '.algorithms[]?' "$json" | tr '\n' ', ' | sed 's/,$//')
        echo "✅ $host: $algorithms"
    else
        echo "❌ $host: No PQC support"
    fi
done

echo ""
echo "Summary:"
echo "  Total endpoints: $total"
echo "  PQC-enabled: $pqc_count"
echo "  PQC-ready: $(awk "BEGIN {printf \"%.1f\", ($pqc_count/$total)*100}")%"
```

---

## Combined Usage: testssl.sh + pqcscan

### Complete TLS Quantum Analysis

```bash
#!/bin/bash
# Complete quantum-safe TLS analysis script

ENDPOINT="$1"
if [ -z "$ENDPOINT" ]; then
    echo "Usage: $0 <host:port>"
    exit 1
fi

HOST="${ENDPOINT%:*}"
PORT="${ENDPOINT##*:}"
[ "$HOST" = "$ENDPOINT" ] && PORT="443"

OUTPUT_DIR="tls-analysis-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$OUTPUT_DIR"

echo "╔════════════════════════════════════════════════╗"
echo "║  Complete Quantum-Safe TLS Analysis           ║"
echo "╚════════════════════════════════════════════════╝"
echo ""
echo "Target: $HOST:$PORT"
echo "Output: $OUTPUT_DIR"
echo ""

# Step 1: testssl.sh analysis
echo "[1/2] Running testssl.sh..."
testssl.sh --quiet --jsonfile "$OUTPUT_DIR/testssl.json" "$ENDPOINT"

# Step 2: pqcscan analysis
echo "[2/2] Running pqcscan..."
pqcscan --host "$HOST" --port "$PORT" --json > "$OUTPUT_DIR/pqcscan.json"

# Analyze results
echo ""
echo "╔════════════════════════════════════════════════╗"
echo "║  Analysis Results                              ║"
echo "╚════════════════════════════════════════════════╝"
echo ""

# Certificate algorithm
echo "📜 Certificate:"
cert_alg=$(jq -r '.scanResult[] | select(.id == "cert_signatureAlgorithm") | .finding' "$OUTPUT_DIR/testssl.json")
cert_key=$(jq -r '.scanResult[] | select(.id == "cert_keySize") | .finding' "$OUTPUT_DIR/testssl.json")
echo "   Algorithm: $cert_alg"
echo "   Key: $cert_key"

if echo "$cert_alg" | grep -iq "RSA\|ECDSA"; then
    echo "   Status: ❌ Quantum-vulnerable"
else
    echo "   Status: ✅ May be quantum-safe"
fi

echo ""

# PQC support
echo "🔒 Post-Quantum Crypto:"
pqc_support=$(jq -r '.pqc_support' "$OUTPUT_DIR/pqcscan.json" 2>/dev/null || echo "false")
if [ "$pqc_support" = "true" ]; then
    echo "   Status: ✅ PQC DETECTED"
    pqc_algos=$(jq -r '.algorithms[]?' "$OUTPUT_DIR/pqcscan.json" | tr '\n' ', ' | sed 's/,$//')
    echo "   Algorithms: $pqc_algos"
else
    echo "   Status: ❌ No PQC support"
fi

echo ""

# Recommendation
echo "📋 Recommendation:"
if [ "$pqc_support" = "true" ]; then
    echo "   ✅ This endpoint has PQC support - on track for 2030"
elif echo "$cert_alg" | grep -iq "RSA\|ECDSA"; then
    echo "   ⚠️  Migrate to PQC by 2030 (NIST deadline)"
    echo "   →  Consider hybrid approach: classical + PQC"
fi

echo ""
echo "Full reports saved to: $OUTPUT_DIR/"
```

### Usage Example

```bash
# Make script executable
chmod +x complete-tls-analysis.sh

# Scan endpoint
./complete-tls-analysis.sh example.com:443

# Expected output:
# ╔════════════════════════════════════════════════╗
# ║  Complete Quantum-Safe TLS Analysis           ║
# ╚════════════════════════════════════════════════╝
#
# Target: example.com:443
#
# [1/2] Running testssl.sh...
# [2/2] Running pqcscan...
#
# ╔════════════════════════════════════════════════╗
# ║  Analysis Results                              ║
# ╚════════════════════════════════════════════════╝
#
# 📜 Certificate:
#    Algorithm: SHA256 with RSA
#    Key: RSA 2048 bits
#    Status: ❌ Quantum-vulnerable
#
# 🔒 Post-Quantum Crypto:
#    Status: ❌ No PQC support
#
# 📋 Recommendation:
#    ⚠️  Migrate to PQC by 2030 (NIST deadline)
#    →  Consider hybrid approach: classical + PQC
```

---

## Integration with Audit Script

The quantum-safe audit script automatically uses these tools when available:

```bash
# Create TLS endpoints file
cat > tls-endpoints.txt << EOF
example.com:443
api.example.com:443
EOF

# Run full audit (includes testssl.sh and pqcscan)
./scripts/quantum-safe-audit.sh --full

# Results saved to:
# - testssl-results/*.json
# - pqcscan-results/*.json
```

**Or use environment variable:**

```bash
export TLS_ENDPOINTS="example.com:443,api.example.com:443"
./scripts/quantum-safe-audit.sh --full
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: TLS Quantum-Safe Audit

on: [push, pull_request]

jobs:
  tls-audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install tools
        run: |
          # Install testssl.sh
          git clone --depth 1 https://github.com/drwetter/testssl.sh.git

          # Install pqcscan
          cargo install pqcscan

      - name: Scan production endpoints
        run: |
          # Create endpoints file
          echo "${{ secrets.PRODUCTION_ENDPOINTS }}" > tls-endpoints.txt

          # Run testssl.sh
          while read endpoint; do
            ./testssl.sh/testssl.sh --quiet --jsonfile "testssl-$endpoint.json" "$endpoint"
          done < tls-endpoints.txt

          # Run pqcscan
          while read endpoint; do
            host="${endpoint%:*}"
            port="${endpoint##*:}"
            pqcscan --host "$host" --port "$port" --json > "pqcscan-$host.json"
          done < tls-endpoints.txt

      - name: Analyze results
        run: |
          # Check for quantum-vulnerable configs
          for json in testssl-*.json; do
            if jq -e '.scanResult[] | select(.id == "cert_signatureAlgorithm") | .finding' "$json" | grep -iq "RSA\|ECDSA"; then
              echo "::warning::Quantum-vulnerable TLS detected in $json"
            fi
          done
```

---

## Resources

- **testssl.sh GitHub**: https://github.com/drwetter/testssl.sh
- **testssl.sh Documentation**: https://testssl.sh/
- **pqcscan GitHub**: https://github.com/anvilsecure/pqcscan
- **NIST PQC Project**: https://csrc.nist.gov/projects/post-quantum-cryptography

---

*Last Updated: January 2025*
