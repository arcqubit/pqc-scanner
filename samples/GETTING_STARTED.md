# Getting Started with Sample Repositories

This guide will help you quickly start using the PQC Scanner sample repositories.

## Quick Start (5 minutes)

### 1. Build the Scanner

```bash
# From project root
cargo build --release
```

### 2. Scan All Samples

```bash
# Using Make (recommended)
make scan-samples

# Or run script directly
./scripts/scan-all-samples.sh
```

### 3. View Results

```bash
# Results are saved in reports/ directory
ls -lh reports/

# View a specific report
cat reports/legacy-banking-*.json | jq
```

## Scanning Individual Samples

### Legacy Banking (JavaScript)

```bash
# Scan
cargo run --example generate_compliance_report -- \
  --path samples/legacy-banking/src/ \
  --output reports/legacy-banking.json

# Expected: 15 vulnerabilities, compliance score 28/100
```

### Crypto Messenger (Python)

```bash
# Scan
cargo run --example generate_compliance_report -- \
  --path samples/crypto-messenger/app/ \
  --output reports/crypto-messenger.json

# Expected: 12 vulnerabilities, compliance score 35/100
```

### Old Web Framework (Java)

```bash
# Scan
cargo run --example generate_compliance_report -- \
  --path samples/old-web-framework/src/ \
  --output reports/old-web-framework.json

# Expected: 18 vulnerabilities, compliance score 22/100
```

### IoT Device (C++)

```bash
# Scan
cargo run --example generate_compliance_report -- \
  --path samples/iot-device/src/ \
  --output reports/iot-device.json

# Expected: 14 vulnerabilities, compliance score 18/100
```

### Polyglot App (Multi-language)

```bash
# Scan entire polyglot repository
cargo run --example generate_compliance_report -- \
  --path samples/polyglot-app/ \
  --output reports/polyglot-app.json

# Expected: 35+ vulnerabilities, compliance score 31/100
```

## Understanding the Results

### Compliance Score

The compliance score (0-100) indicates overall cryptographic security:
- **90-100**: Excellent - Modern post-quantum crypto
- **70-89**: Good - Secure classical crypto
- **50-69**: Fair - Some weak algorithms present
- **30-49**: Poor - Multiple vulnerabilities
- **0-29**: Critical - Severe security issues

### Vulnerability Severity

- **Critical**: Immediate security risk (RSA-512, MD5 passwords, hardcoded keys)
- **High**: Significant risk (RSA-1024, SHA-1, DES, weak TLS)
- **Medium**: Moderate risk (weak random, missing security flags)
- **Low**: Best practice violations

## What to Look For

### 1. Quantum-Vulnerable Algorithms
- RSA-1024/2048
- ECDSA/ECDH (all curves)
- DSA
- → **Remediation**: Migrate to Kyber, Dilithium, Falcon

### 2. Broken Hash Functions
- MD5
- SHA-1
- → **Remediation**: Use SHA-256, SHA-3, or BLAKE2

### 3. Weak Ciphers
- DES/3DES
- RC4
- Custom crypto
- → **Remediation**: AES-256-GCM, ChaCha20-Poly1305

### 4. Configuration Issues
- Hardcoded keys
- Weak TLS (1.0/1.1)
- No forward secrecy
- → **Remediation**: Key management, TLS 1.3, ECDHE

## Sample Use Cases

### 1. Validate Scanner Installation

```bash
# Quick test - scan one sample
make scan-samples

# Check that vulnerabilities are detected
cat reports/legacy-banking-*.json | jq '.summary.vulnerabilities_found'
# Should output: 15
```

### 2. Performance Benchmarking

```bash
# Time the scan
time ./scripts/scan-all-samples.sh

# Expected total time: ~6 seconds for all samples
```

### 3. Training and Demos

```bash
# Show specific vulnerability
cd samples/legacy-banking
cat src/crypto/key-generator.js

# Point out line 15: RSA-1024 generation
# Then show scanner detecting it
```

### 4. CI/CD Integration Testing

```bash
# Test scanner in CI environment
make build
make scan-samples

# Verify exit code
echo $?  # Should be 0 if all scans succeed
```

## Troubleshooting

### Scanner Not Found

```bash
# Build the scanner first
cargo build --release
```

### No Vulnerabilities Detected

Check that you're scanning the source directory:
```bash
# Correct
--path samples/legacy-banking/src/

# Wrong
--path samples/legacy-banking/
```

### Reports Not Generated

Ensure reports directory exists:
```bash
mkdir -p reports
```

### Permission Denied on Script

Make script executable:
```bash
chmod +x scripts/scan-all-samples.sh
```

## Next Steps

1. **Read Documentation**: See [SAMPLE_REPOSITORIES.md](../docs/SAMPLE_REPOSITORIES.md)
2. **Explore Samples**: Review vulnerable code in each repository
3. **Customize Scans**: Modify `.pqc-scanner.toml` files
4. **Compare Results**: Check reports against expected values
5. **Test Remediation**: Try the auto-remediation features (if available)

## Advanced Usage

### Scan with Auto-Remediation

```bash
cargo run --example generate_compliance_report -- \
  --path samples/legacy-banking/src/ \
  --remediate \
  --output reports/legacy-banking-remediated.json
```

### Generate OSCAL Report

```bash
cargo run --example generate_compliance_report -- \
  --path samples/legacy-banking/src/ \
  --format oscal \
  --output reports/legacy-banking-oscal.json
```

### Batch Scan with Custom Config

```bash
for sample in samples/*/; do
    echo "Scanning $(basename $sample)..."
    cargo run --example generate_compliance_report -- \
      --path "$sample" \
      --config "$sample/.pqc-scanner.toml" \
      --output "reports/$(basename $sample).json"
done
```

## Support

- **Documentation**: [../docs/](../docs/)
- **Issues**: https://github.com/arcqubit/pqc-scanner/issues
- **Sample Guidelines**: [SAMPLE_REPOSITORIES.md](../docs/SAMPLE_REPOSITORIES.md)
