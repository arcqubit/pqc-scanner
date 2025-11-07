# Quantum-Safe Audit Tools Installation Guide

## Overview

This guide provides detailed installation instructions for all tools used in the NIST Post-Quantum Cryptography audit workflow.

## Tool Categories

### 🔧 Required Tools (Core Functionality)
- **ripgrep** - Fast code pattern matching
- **jq** - JSON processing
- **OpenSSL** - Certificate analysis

### 🔍 Recommended Tools (Enhanced Detection)
- **testssl.sh** - Comprehensive TLS/SSL testing
- **pqcscan** - Post-quantum crypto detection

### 📦 Optional Tools (Advanced Analysis)
- **Semgrep** - Static application security testing
- **Trivy** - Vulnerability and secret scanning

---

## Quick Install (Automated)

```bash
# Run the audit script's installation helper
./scripts/quantum-safe-audit.sh --tools

# This installs: ripgrep, jq, testssl.sh automatically
# And provides instructions for optional tools
```

---

## 1. Required Tools

### ripgrep (rg)

Fast regex search tool for code analysis.

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y ripgrep
```

**CentOS/RHEL:**
```bash
sudo yum install -y ripgrep
```

**macOS:**
```bash
brew install ripgrep
```

**From source:**
```bash
cargo install ripgrep
```

**Verify:**
```bash
rg --version
# Expected: ripgrep 14.0.0 or higher
```

---

### jq

JSON processor for parsing tool outputs.

**Ubuntu/Debian:**
```bash
sudo apt-get install -y jq
```

**CentOS/RHEL:**
```bash
sudo yum install -y jq
```

**macOS:**
```bash
brew install jq
```

**Verify:**
```bash
jq --version
# Expected: jq-1.6 or higher
```

---

### OpenSSL

Certificate analysis tool (usually pre-installed).

**Ubuntu/Debian:**
```bash
sudo apt-get install -y openssl
```

**macOS:** (pre-installed)
```bash
brew install openssl
```

**Verify:**
```bash
openssl version
# Expected: OpenSSL 1.1.1 or higher
```

---

## 2. Recommended Tools

### testssl.sh

Comprehensive TLS/SSL testing tool that detects quantum-vulnerable algorithms in live endpoints.

**Features:**
- ✅ Detects RSA, ECDSA, DH usage in TLS
- ✅ Certificate algorithm analysis
- ✅ Cipher suite enumeration
- ✅ Protocol version detection (SSLv2-TLSv1.3)
- ✅ JSON output for automation
- ✅ Vulnerability checks (Heartbleed, POODLE, etc.)

**Installation (Method 1: Git Clone):**
```bash
# Clone to home directory
git clone --depth 1 https://github.com/drwetter/testssl.sh.git ~/.local/testssl.sh

# Create symlink (optional, for easy access)
mkdir -p ~/.local/bin
ln -s ~/.local/testssl.sh/testssl.sh ~/.local/bin/testssl.sh

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"
```

**Installation (Method 2: Direct Download):**
```bash
# Download latest release
curl -L https://github.com/drwetter/testssl.sh/archive/refs/heads/3.2.tar.gz -o testssl.tar.gz
tar -xzf testssl.tar.gz
cd testssl.sh-3.2
./testssl.sh --version
```

**Usage:**
```bash
# Test single endpoint
testssl.sh --quiet --jsonfile output.json https://example.com

# Full scan with all tests
testssl.sh --full https://example.com

# Check for PQC support (when available)
testssl.sh --protocols --ciphers https://example.com
```

**Integration with Audit Script:**

The audit script automatically detects testssl.sh in these locations:
1. `testssl.sh` in PATH
2. `./testssl.sh/testssl.sh` (local clone)
3. `./testssl.sh` (direct file)

**Create TLS Endpoints File:**
```bash
# Create file with endpoints to scan
cat > tls-endpoints.txt << EOF
example.com:443
api.example.com:443
mail.example.com:587
EOF

# Or use environment variable
export TLS_ENDPOINTS="example.com:443,api.example.com:443"

# Run audit
./scripts/quantum-safe-audit.sh --full
```

**Verify:**
```bash
testssl.sh --version
# Expected: testssl.sh 3.2 or higher
```

---

### pqcscan

Purpose-built tool for detecting post-quantum cryptography algorithms in TLS connections.

**Features:**
- ✅ Detects ML-KEM (Kyber), ML-DSA (Dilithium)
- ✅ Identifies PQC hybrid cipher suites
- ✅ TLS 1.3 PQC extension detection
- ✅ JSON output for automation
- ✅ Fast, written in Rust

**Installation (Method 1: Cargo):**
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install pqcscan
cargo install pqcscan

# Verify
pqcscan --version
```

**Installation (Method 2: Pre-built Binaries):**
```bash
# Download latest release for your platform
# Linux x86_64
curl -L https://github.com/anvilsecure/pqcscan/releases/latest/download/pqcscan-linux-x86_64 -o pqcscan
chmod +x pqcscan
sudo mv pqcscan /usr/local/bin/

# macOS (ARM)
curl -L https://github.com/anvilsecure/pqcscan/releases/latest/download/pqcscan-macos-arm64 -o pqcscan
chmod +x pqcscan
sudo mv pqcscan /usr/local/bin/
```

**Installation (Method 3: From Source):**
```bash
# Clone repository
git clone https://github.com/anvilsecure/pqcscan.git
cd pqcscan

# Build
cargo build --release

# Install
sudo cp target/release/pqcscan /usr/local/bin/
```

**Usage:**
```bash
# Scan single endpoint
pqcscan --host example.com --port 443 --json

# Scan multiple endpoints
cat tls-endpoints.txt | while read endpoint; do
  host="${endpoint%:*}"
  port="${endpoint##*:}"
  pqcscan --host "$host" --port "$port" --json > "pqcscan-$host.json"
done
```

**Integration with Audit Script:**

The audit script automatically uses pqcscan if it's in PATH. Results are saved to `pqcscan-results/` directory.

**Verify:**
```bash
pqcscan --version
# Expected: pqcscan 0.x.x or higher
```

---

## 3. Optional Tools

### Semgrep

Static application security testing (SAST) tool with crypto-specific rules.

**Installation:**
```bash
# Using pip
pip install semgrep

# Using Homebrew (macOS)
brew install semgrep

# Using Docker
docker pull returntocorp/semgrep
```

**Usage:**
```bash
# Scan with crypto rules
semgrep --config "p/crypto" .

# Output JSON
semgrep --config "p/crypto" --json --output semgrep-results.json .
```

**Verify:**
```bash
semgrep --version
# Expected: 1.x.x or higher
```

---

### Trivy

Comprehensive vulnerability and secret scanner.

**Installation:**

**Ubuntu/Debian:**
```bash
sudo apt-get install wget apt-transport-https gnupg lsb-release
wget -qO - https://aquasecurity.github.io/trivy-repo/deb/public.key | sudo apt-key add -
echo "deb https://aquasecurity.github.io/trivy-repo/deb $(lsb_release -sc) main" | sudo tee -a /etc/apt/sources.list.d/trivy.list
sudo apt-get update
sudo apt-get install trivy
```

**macOS:**
```bash
brew install trivy
```

**Binary:**
```bash
# Download latest release
VERSION=$(curl -sI https://github.com/aquasecurity/trivy/releases/latest | grep -i location | awk -F/ '{print $NF}' | tr -d '\r')
curl -L https://github.com/aquasecurity/trivy/releases/download/${VERSION}/trivy_${VERSION#v}_Linux-64bit.tar.gz | tar xzv
sudo mv trivy /usr/local/bin/
```

**Usage:**
```bash
# Scan filesystem
trivy fs --security-checks vuln,secret,config .

# Output JSON
trivy fs --format json --output trivy-results.json .
```

**Verify:**
```bash
trivy --version
# Expected: Version: 0.x.x or higher
```

---

## Tool Comparison Matrix

| Tool | Purpose | Detection Capability | Output Format | Installation |
|------|---------|---------------------|---------------|--------------|
| **ripgrep** | Code search | RSA/ECDSA patterns in code | Text | Easy (package manager) |
| **jq** | JSON processing | N/A (utility) | N/A | Easy (package manager) |
| **OpenSSL** | Certificate analysis | Local cert files (RSA/ECDSA) | Text | Pre-installed |
| **testssl.sh** | TLS testing | Live TLS (RSA/ECDSA/DH) | JSON/Text | Easy (git clone) |
| **pqcscan** | PQC detection | PQC algorithms in TLS | JSON | Medium (Rust/binary) |
| **Semgrep** | SAST | Crypto API misuse | JSON/Text | Easy (pip) |
| **Trivy** | Vulnerability scanning | Dependencies, secrets | JSON/Text | Easy (package/binary) |

---

## Verification Script

Use this script to verify all tools are installed:

```bash
#!/bin/bash

echo "=== Quantum-Safe Audit Tools Verification ==="
echo ""

# Required tools
echo "Required Tools:"
command -v rg >/dev/null 2>&1 && echo "  ✅ ripgrep" || echo "  ❌ ripgrep (REQUIRED)"
command -v jq >/dev/null 2>&1 && echo "  ✅ jq" || echo "  ❌ jq (REQUIRED)"
command -v openssl >/dev/null 2>&1 && echo "  ✅ openssl" || echo "  ❌ openssl (REQUIRED)"

echo ""
echo "Recommended Tools:"
command -v testssl.sh >/dev/null 2>&1 && echo "  ✅ testssl.sh" || echo "  ⚠️  testssl.sh (recommended for TLS analysis)"
command -v pqcscan >/dev/null 2>&1 && echo "  ✅ pqcscan" || echo "  ⚠️  pqcscan (recommended for PQC detection)"

echo ""
echo "Optional Tools:"
command -v semgrep >/dev/null 2>&1 && echo "  ✅ semgrep" || echo "  ℹ️  semgrep (optional for enhanced code analysis)"
command -v trivy >/dev/null 2>&1 && echo "  ✅ trivy" || echo "  ℹ️  trivy (optional for vulnerability scanning)"

echo ""
echo "=== Verification Complete ==="
```

Save as `scripts/verify-tools.sh` and run:
```bash
chmod +x scripts/verify-tools.sh
./scripts/verify-tools.sh
```

---

## Troubleshooting

### testssl.sh: Command not found

**Solution 1:** Add to PATH
```bash
export PATH="$HOME/.local/bin:$PATH"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

**Solution 2:** Use direct path in audit script
```bash
# Edit TESTSSL_PATH in quantum-safe-audit.sh
TESTSSL_PATH="$HOME/.local/testssl.sh/testssl.sh"
```

### pqcscan: Rust not installed

**Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

### Permission denied errors

**Fix permissions:**
```bash
chmod +x testssl.sh
chmod +x pqcscan
chmod +x scripts/quantum-safe-audit.sh
```

### SSL certificate verification failed (behind corporate proxy)

**Disable SSL verification temporarily:**
```bash
export GIT_SSL_NO_VERIFY=true
git clone https://github.com/drwetter/testssl.sh.git
unset GIT_SSL_NO_VERIFY
```

---

## Next Steps

After installing tools:

1. **Verify installation:**
   ```bash
   ./scripts/verify-tools.sh
   ```

2. **Run test audit:**
   ```bash
   ./scripts/quantum-safe-audit.sh --quick
   ```

3. **Add TLS endpoints (optional):**
   ```bash
   echo "example.com:443" > tls-endpoints.txt
   ```

4. **Run full audit:**
   ```bash
   ./scripts/quantum-safe-audit.sh --full --output report.txt
   ```

---

## Resources

- **testssl.sh**: https://github.com/drwetter/testssl.sh
- **pqcscan**: https://github.com/anvilsecure/pqcscan
- **Semgrep**: https://semgrep.dev/
- **Trivy**: https://trivy.dev/
- **ripgrep**: https://github.com/BurntSushi/ripgrep
- **OpenSSL**: https://www.openssl.org/

---

*Last Updated: January 2025*
