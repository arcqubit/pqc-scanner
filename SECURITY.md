# Security Policy

## Reporting Security Vulnerabilities

The ArcQubit team takes security seriously. We appreciate your efforts to responsibly disclose any security vulnerabilities you find.

### How to Report a Vulnerability

**Please DO NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via one of the following methods:

1. **GitHub Security Advisories (Preferred)**
   - Navigate to the [Security tab](https://github.com/arcqubit/pqc-scanner/security/advisories)
   - Click "Report a vulnerability"
   - Fill out the advisory form with details

2. **Email**
   - Send an email to: **security@arcqubit.io**
   - Include detailed information about the vulnerability
   - PGP key available upon request

### What to Include in Your Report

To help us better understand and address the issue, please include:

- Type of vulnerability (e.g., buffer overflow, SQL injection, XSS)
- Full paths of source file(s) related to the vulnerability
- Location of the affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the vulnerability and potential attack scenarios

### Response Timeline

We are committed to responding to security reports promptly:

| Severity | Initial Response | Status Update | Fix Timeline |
|----------|-----------------|---------------|--------------|
| **Critical** | 24 hours | Every 48 hours | 7-14 days |
| **High** | 48 hours | Weekly | 14-30 days |
| **Medium** | 5 business days | Bi-weekly | 30-60 days |
| **Low** | 10 business days | Monthly | 60-90 days |

### What to Expect

1. **Acknowledgment**: We'll confirm receipt within the timeframes above
2. **Validation**: We'll validate and triage the vulnerability
3. **Updates**: Regular status updates on our progress
4. **Disclosure**: We'll work with you on responsible disclosure timing
5. **Credit**: We'll publicly acknowledge your contribution (if desired)

## Security Update Policy

### Supported Versions

We release security patches for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.2.x   | :white_check_mark: |
| 1.1.x   | :white_check_mark: |
| 1.0.x   | :x: |
| < 1.0   | :x: |

### Security Advisories

Security advisories are published at:
- [GitHub Security Advisories](https://github.com/arcqubit/pqc-scanner/security/advisories)
- Project documentation: [docs/security/](docs/security/)

### Automated Security Scanning

This project uses multiple security tools:

- **Dependabot**: Automated dependency vulnerability scanning
- **CodeQL**: Static analysis security testing (SAST)
- **Cargo Audit**: Rust security advisory database checks
- **Secret Scanning**: Detects accidentally committed secrets

## Security Best Practices for Contributors

When contributing to this project, please:

1. **Keep Dependencies Updated**: Run `cargo update` regularly
2. **Run Security Checks**: Use `cargo audit` before submitting PRs
3. **Follow Secure Coding Guidelines**: See [CONTRIBUTING.md](CONTRIBUTING.md)
4. **Avoid Hardcoding Secrets**: Use environment variables
5. **Sign Commits**: Use GPG-signed commits when possible

## Security Considerations for PQC Scanner

### Cryptographic Scanning

This tool analyzes code for quantum-vulnerable cryptography. Key security considerations:

1. **False Positives**: The scanner may flag legitimate uses of classical crypto
2. **Scope**: Focuses on cryptographic library detection, not runtime behavior
3. **Privacy**: All scanning is performed locally; no code is transmitted externally

### WASM Security

When using the WASM version:

- WASM modules run in a sandboxed environment
- No network access or file system access from WASM
- Input validation is performed before WASM execution

### Supply Chain Security

We maintain supply chain security through:

- **Cargo.lock committed**: Ensures reproducible builds
- **Dependency review**: All dependency updates reviewed by maintainers
- **SBOM generation**: Software Bill of Materials included with releases
- **Signed releases**: All release artifacts are checksummed

## Vulnerability Disclosure Policy

We follow **coordinated disclosure** principles:

1. **90-Day Disclosure Window**: We aim to fix and disclose within 90 days
2. **Early Disclosure**: Critical vulnerabilities may be disclosed earlier with a patch
3. **Credit**: Security researchers are credited in advisories (with permission)
4. **CVE Assignment**: We work with MITRE for CVE assignment when applicable

## Security Hall of Fame

We recognize security researchers who help improve our security:

<!-- Security researchers will be listed here -->

*No vulnerabilities reported yet. Be the first to help secure PQC Scanner!*

## Contact

- **Security Team**: security@arcqubit.io
- **General Questions**: support@arcqubit.io
- **Project Lead**: [@arcqubit](https://github.com/arcqubit)

## Additional Resources

- [OpenSSF Best Practices Badge](https://bestpractices.coreinfrastructure.org/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)

---

**Last Updated**: 2025-11-11
**Policy Version**: 1.0.0
