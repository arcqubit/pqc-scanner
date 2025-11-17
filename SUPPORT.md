# Support

Thank you for using PQC Scanner! This document provides resources for getting help and support.

## Table of Contents

- [Getting Help](#getting-help)
- [Documentation](#documentation)
- [Community Resources](#community-resources)
- [Reporting Issues](#reporting-issues)
- [Commercial Support](#commercial-support)
- [Frequently Asked Questions](#frequently-asked-questions)

---

## Getting Help

### Before Asking for Help

1. **Search Documentation**: Check the [docs/](docs/) directory for guides
2. **Check README**: Review the [README.md](README.md) for quick start info
3. **Search Issues**: Look through [existing issues](https://github.com/arcqubit/pqc-scanner/issues)
4. **Search Discussions**: Browse [GitHub Discussions](https://github.com/arcqubit/pqc-scanner/discussions)

### Where to Ask Questions

Choose the appropriate channel based on your question:

#### GitHub Discussions (Recommended)
**Best for**: General questions, usage help, ideas, and community discussion

[Start a Discussion](https://github.com/arcqubit/pqc-scanner/discussions)

**Categories**:
- **Q&A**: Ask questions about usage, installation, configuration
- **Ideas**: Share feature ideas and discuss enhancements
- **Show and Tell**: Share your projects using PQC Scanner
- **General**: Everything else

#### GitHub Issues
**Best for**: Bug reports, feature requests, documentation issues

[Create an Issue](https://github.com/arcqubit/pqc-scanner/issues/new/choose)

**Use issue templates for**:
- Bug reports
- Feature requests
- Documentation issues
- Performance problems

**Do NOT use issues for**:
- General questions (use Discussions)
- Support requests (use Discussions)
- Security vulnerabilities (see [SECURITY.md](SECURITY.md))

#### Email Support
**Email**: support@arcqubit.io

**Best for**:
- Private or sensitive questions
- Enterprise inquiries
- Partnership discussions

**Response time**: 3-5 business days

---

## Documentation

### Official Documentation

- **[README.md](README.md)**: Quick start and overview
- **[CONTRIBUTING.md](CONTRIBUTING.md)**: Contribution guidelines
- **[SECURITY.md](SECURITY.md)**: Security policy and vulnerability reporting
- **[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)**: Community guidelines
- **[docs/](docs/)**: Comprehensive documentation

### Key Documentation Files

| Document | Description |
|----------|-------------|
| [docs/architecture.md](docs/architecture.md) | System architecture and design |
| [docs/CALVER.md](docs/CALVER.md) | Versioning scheme explanation |
| [docs/security-scanning.md](docs/security-scanning.md) | TruffleHog and Trivy scanning guide |
| [docs/openssf-scorecard-improvements.md](docs/openssf-scorecard-improvements.md) | Security hardening documentation |

### API Documentation

```bash
# Build and open Rust documentation
cargo doc --open

# Build with examples and private items
cargo doc --no-deps --document-private-items --open
```

### Examples

Check the `examples/` directory for usage examples:

```bash
# Run compliance report example
cargo run --example generate_compliance_report

# List all examples
ls examples/
```

---

## Community Resources

### GitHub Discussions
- **Announcements**: Release notes and project updates
- **Q&A**: Ask and answer questions
- **Ideas**: Feature discussions and roadmap
- **Show and Tell**: Community projects and use cases

[Join the Discussion](https://github.com/arcqubit/pqc-scanner/discussions)

### Social Media
- **Twitter/X**: [@arcqubit](https://twitter.com/arcqubit) (when available)
- **Blog**: https://arcqubit.io/blog (when available)

### Related Projects
- [NIST Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [Open Quantum Safe](https://openquantumsafe.org/)
- [Rust Crypto](https://github.com/RustCrypto)

---

## Reporting Issues

### Bug Reports
Use the [Bug Report template](https://github.com/arcqubit/pqc-scanner/issues/new?template=bug_report.yml)

**Include**:
- PQC Scanner version
- Operating system and version
- Rust/Node.js version (if applicable)
- Minimal reproduction steps
- Error messages and logs
- Expected vs. actual behavior

### Feature Requests
Use the [Feature Request template](https://github.com/arcqubit/pqc-scanner/issues/new?template=feature_request.yml)

**Include**:
- Clear problem statement
- Proposed solution
- Use cases
- Alternatives considered

### Security Vulnerabilities
**DO NOT** create public issues for security vulnerabilities.

Instead:
1. Email security@arcqubit.io
2. Or use [GitHub Security Advisories](https://github.com/arcqubit/pqc-scanner/security/advisories/new)

See [SECURITY.md](SECURITY.md) for full details.

---

## Commercial Support

### Enterprise Support
For enterprise users requiring:
- Dedicated support
- SLA guarantees
- Custom feature development
- Training and consulting
- Priority bug fixes

**Contact**: enterprise@arcqubit.io

### Consulting Services
ArcQubit offers consulting services for:
- Post-quantum cryptography migration
- Security audits and compliance
- Custom integration development
- Training and workshops

**Contact**: consulting@arcqubit.io

---

## Frequently Asked Questions

### Installation & Setup

**Q: What are the minimum requirements?**
A: Rust 1.70.0+, Node.js 18+ (for WASM), wasm-pack (for WASM builds)

**Q: How do I install PQC Scanner?**
A: See the [README.md](README.md#quick-start) for installation instructions

**Q: The WASM build fails, what should I do?**
A: Ensure you have wasm-pack installed: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`

### Usage

**Q: What programming languages are supported?**
A: Rust, JavaScript, TypeScript, Python, Java, Go, C++, C#

**Q: What cryptographic algorithms are detected?**
A: RSA, ECDSA, ECDH, DSA, DH, MD5, SHA-1, DES, 3DES, RC4 (10 patterns total)

**Q: Can PQC Scanner scan compiled binaries?**
A: No, PQC Scanner analyzes source code only, not compiled binaries

**Q: How accurate is the detection?**
A: The scanner has >90% accuracy on tested codebases. Some false positives may occur with string literals containing crypto keywords.

### Performance

**Q: How fast is PQC Scanner?**
A: ~0.35ms for 1000 lines of code (28x faster than target). See benchmarks: `cargo bench`

**Q: Can I scan large repositories?**
A: Yes, PQC Scanner is optimized for large codebases. Use `--parallel` flag for multi-threaded scanning.

**Q: What is the WASM bundle size?**
A: <500KB gzipped, optimized for web deployment

### Compliance & Reporting

**Q: What compliance frameworks are supported?**
A: NIST 800-53 SC-13, Canadian CCCS/CSE (ITSG-33, ITSP.40.111, ITSP.40.062)

**Q: What output formats are available?**
A: JSON, OSCAL 1.1.2, CSV, HTML (via examples)

**Q: Can I customize compliance reports?**
A: Yes, see `examples/generate_compliance_report.rs` for customization options

### Contributing

**Q: How can I contribute?**
A: See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines

**Q: I found a bug, what should I do?**
A: Create a [bug report](https://github.com/arcqubit/pqc-scanner/issues/new?template=bug_report.yml)

**Q: Can I request a feature?**
A: Yes! Create a [feature request](https://github.com/arcqubit/pqc-scanner/issues/new?template=feature_request.yml)

### Licensing

**Q: What license is PQC Scanner released under?**
A: See the [LICENSE](LICENSE) file for license information

**Q: Can I use PQC Scanner in commercial projects?**
A: Check the LICENSE file for usage terms. For commercial licensing inquiries: licensing@arcqubit.io

### Troubleshooting

**Q: Tests are failing, what should I check?**
A:
1. Ensure you have the latest Rust version: `rustup update`
2. Clean build: `cargo clean && cargo build`
3. Check test output: `cargo test -- --nocapture`

**Q: I'm getting compilation errors**
A:
1. Update dependencies: `cargo update`
2. Check Rust version: `rustc --version` (should be 1.70.0+)
3. Clear cache: `rm -rf target/`

**Q: WASM build produces large files**
A:
1. Use release mode: `wasm-pack build --release`
2. Enable wasm-opt: Included automatically in release builds
3. Check output: `ls -lh pkg/*.wasm`

---

## Additional Resources

### Learning Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [WASM Book](https://rustwasm.github.io/docs/book/)
- [Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)

### Tools
- [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit): Audit Rust dependencies
- [wasm-pack](https://rustwasm.github.io/wasm-pack/): WASM build tool
- [criterion](https://github.com/bheisler/criterion.rs): Benchmarking framework

### Standards
- [NIST SP 800-53](https://csrc.nist.gov/publications/detail/sp/800-53/rev-5/final)
- [OSCAL](https://pages.nist.gov/OSCAL/)
- [Canadian CCCS](https://www.cyber.gc.ca/)

---

## Contact Information

- **General Support**: support@arcqubit.io
- **Security Issues**: security@arcqubit.io (see [SECURITY.md](SECURITY.md))
- **Code of Conduct Violations**: conduct@arcqubit.io
- **Enterprise Inquiries**: enterprise@arcqubit.io
- **Consulting**: consulting@arcqubit.io
- **Licensing**: licensing@arcqubit.io

---

## Response Times

| Channel | Expected Response Time |
|---------|------------------------|
| GitHub Discussions | 1-3 days (community-driven) |
| GitHub Issues | 3-5 days (maintainers) |
| Email (support@) | 3-5 business days |
| Email (security@) | 24-48 hours (see SECURITY.md) |
| Enterprise Support | Per SLA agreement |

---

**Thank you for using PQC Scanner!**

We appreciate your support and welcome your feedback to make this project better.
