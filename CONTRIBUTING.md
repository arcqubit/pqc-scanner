# Contributing to PQC Scanner

Thank you for your interest in contributing to PQC Scanner! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Community Guidelines](#community-guidelines)

---

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to conduct@arcqubit.io.

---

## Getting Started

### Prerequisites

- **Rust**: 1.70.0 or later
- **Node.js**: 18.x or later
- **wasm-pack**: Latest version
- **Git**: For version control
- **Make**: For build automation (optional but recommended)

### Setting Up Your Development Environment

1. **Fork the Repository**
   ```bash
   # Fork via GitHub UI, then clone your fork
   git clone https://github.com/YOUR_USERNAME/pqc-scanner.git
   cd pqc-scanner
   ```

2. **Add Upstream Remote**
   ```bash
   git remote add upstream https://github.com/arcqubit/pqc-scanner.git
   git fetch upstream
   ```

3. **Install Dependencies**
   ```bash
   # Install Rust dependencies
   cargo build

   # Install Node.js dependencies
   npm install

   # Install wasm-pack
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

4. **Verify Installation**
   ```bash
   # Run tests to ensure everything is working
   make test
   # or
   cargo test && npm test
   ```

### Understanding the Project Structure

```
pqc-scanner/
├── src/              # Rust source code
│   ├── lib.rs        # WASM bindings
│   ├── scanner.rs    # Core scanning logic
│   ├── detector.rs   # Pattern detection
│   ├── analyzer.rs   # Code analysis
│   └── remediation.rs # Remediation suggestions
├── tests/            # Integration tests
├── benches/          # Performance benchmarks
├── docs/             # Documentation
├── samples/          # Sample applications
├── mcp/              # MCP server implementation
├── .github/          # GitHub workflows and templates
│   └── workflows/    # CI/CD pipelines
├── scripts/          # Build and automation scripts
└── pkg/              # WASM build output (generated)
```

---

## Development Workflow

### Branching Strategy

We follow **GitHub Flow** for development:

1. `main` - Production-ready code, protected branch
2. `develop` - Integration branch for ongoing development (optional)
3. `feature/*` - Feature branches
4. `bugfix/*` - Bug fix branches
5. `hotfix/*` - Critical production fixes

### Creating a Branch

```bash
# Update your local main branch
git checkout main
git pull upstream main

# Create a feature branch
git checkout -b feature/your-feature-name

# Or for a bug fix
git checkout -b bugfix/issue-number-description
```

### Making Changes

1. **Write Code**: Follow the [Coding Standards](#coding-standards)
2. **Write Tests**: See [Testing Guidelines](#testing-guidelines)
3. **Update Documentation**: Keep docs in sync with code changes
4. **Run Checks Locally**:
   ```bash
   # Format code
   cargo fmt

   # Run clippy
   cargo clippy -- -D warnings

   # Run tests
   cargo test

   # Build WASM
   npm run build

   # Run all checks
   make ci
   ```

### Committing Your Changes

We follow [Conventional Commits](https://www.conventionalcommits.org/) for commit messages:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style changes (formatting, no logic change)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Build process, tooling, dependencies
- `ci`: CI/CD changes

**Examples:**
```bash
git commit -m "feat(scanner): add support for Kyber detection"
git commit -m "fix(wasm): resolve memory leak in analysis loop"
git commit -m "docs(readme): update installation instructions"
git commit -m "test(detector): add edge cases for RSA detection"
```

**Commit Best Practices:**
- Use imperative mood ("add" not "added")
- Keep subject line under 50 characters
- Provide detailed body for complex changes
- Reference issues: `Fixes #123` or `Relates to #456`
- Sign commits with GPG (recommended)

---

## Coding Standards

### Rust Code Style

Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/) and project-specific conventions:

1. **Formatting**: Use `rustfmt` (runs automatically via `cargo fmt`)
   ```bash
   cargo fmt --check  # Verify formatting
   cargo fmt          # Auto-format
   ```

2. **Linting**: Use `clippy` for catching common mistakes
   ```bash
   cargo clippy -- -D warnings  # Fail on warnings
   ```

3. **Naming Conventions**:
   - Types: `PascalCase`
   - Functions/variables: `snake_case`
   - Constants: `SCREAMING_SNAKE_CASE`
   - Modules: `snake_case`

4. **Error Handling**:
   - Use `Result<T, E>` for recoverable errors
   - Use custom error types with `thiserror`
   - Document error conditions
   - Avoid `unwrap()` in library code (use `expect()` with message)

5. **Documentation**:
   - All public items must have doc comments (`///`)
   - Include examples in doc comments where appropriate
   - Document panics, errors, and safety invariants
   ```rust
   /// Analyzes source code for quantum-vulnerable cryptographic patterns.
   ///
   /// # Arguments
   ///
   /// * `source` - The source code to analyze
   /// * `language` - The programming language of the source
   ///
   /// # Returns
   ///
   /// Returns `Ok(AnalysisResult)` on success, or `Err(AnalyzerError)` on failure.
   ///
   /// # Examples
   ///
   /// ```
   /// use pqc_scanner::analyze;
   /// let result = analyze("const rsa = ...", "javascript")?;
   /// ```
   pub fn analyze(source: &str, language: &str) -> Result<AnalysisResult, AnalyzerError> {
       // ...
   }
   ```

6. **Safety**:
   - Minimize `unsafe` code
   - Document all `unsafe` blocks with safety invariants
   - Use `cargo-geiger` to audit unsafe usage
   ```rust
   // SAFETY: The pointer is guaranteed to be valid because...
   unsafe {
       // unsafe operation
   }
   ```

### JavaScript/TypeScript Code Style

1. **Formatting**: Use Prettier (auto-format via pre-commit hook)
2. **Linting**: Use ESLint
3. **TypeScript**: Prefer TypeScript over JavaScript
4. **Type Safety**: Enable strict mode in `tsconfig.json`

### General Best Practices

- **DRY**: Don't Repeat Yourself
- **SOLID**: Follow SOLID principles
- **KISS**: Keep It Simple, Stupid
- **Performance**: Optimize after profiling, not before
- **Security**: Follow [OWASP guidelines](https://owasp.org/)
- **Accessibility**: Ensure web interfaces are accessible

---

## Testing Guidelines

### Test Requirements

- **All new features** must include tests
- **Bug fixes** must include regression tests
- **Aim for >90% code coverage**
- **Performance-critical code** should include benchmarks

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in release mode (faster)
cargo test --release

# Run benchmarks
cargo bench

# Run WASM tests
wasm-pack test --node
```

### Writing Tests

1. **Unit Tests**: Test individual functions/modules
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_rsa_detection() {
           let source = "RSA.generateKey(2048)";
           let result = detect_crypto(source);
           assert!(result.contains_rsa());
       }

       #[test]
       #[should_panic(expected = "invalid input")]
       fn test_invalid_input() {
           analyze(null, "rust").unwrap();
       }
   }
   ```

2. **Integration Tests**: Test component interactions
   ```rust
   // tests/integration_test.rs
   use pqc_scanner::*;

   #[test]
   fn test_full_analysis_workflow() {
       let source = std::fs::read_to_string("samples/test.js").unwrap();
       let result = analyze(&source, "javascript").unwrap();
       assert_eq!(result.vulnerabilities.len(), 3);
   }
   ```

3. **Benchmark Tests**: Measure performance
   ```rust
   // benches/scanner_bench.rs
   use criterion::{black_box, criterion_group, criterion_main, Criterion};

   fn bench_analysis(c: &mut Criterion) {
       c.bench_function("analyze 1000 LOC", |b| {
           b.iter(|| analyze(black_box(SOURCE), black_box("rust")))
       });
   }

   criterion_group!(benches, bench_analysis);
   criterion_main!(benches);
   ```

### Test Organization

- Unit tests: In same file as code (`#[cfg(test)]` module)
- Integration tests: In `tests/` directory
- Benchmarks: In `benches/` directory
- Test fixtures: In `tests/fixtures/` directory

---

## Documentation

### Documentation Requirements

1. **Code Documentation**:
   - All public APIs must have doc comments
   - Include examples in doc comments
   - Document error conditions and panics

2. **README Updates**:
   - Update feature list for new capabilities
   - Update usage examples for API changes
   - Keep installation instructions current

3. **Changelog**:
   - Document all user-facing changes in `CHANGELOG.md`
   - Follow [Keep a Changelog](https://keepachangelog.com/) format

4. **Architecture Documentation**:
   - Update `docs/architecture.md` for design changes
   - Document architectural decisions in ADRs (docs/adr/)

### Building Documentation

```bash
# Build Rust documentation
cargo doc --open

# Build with private items
cargo doc --no-deps --document-private-items --open
```

---

## Pull Request Process

### Before Submitting a PR

- [ ] Code passes all tests (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Code passes clippy (`cargo clippy -- -D warnings`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (for user-facing changes)
- [ ] Commit messages follow Conventional Commits
- [ ] Branch is up-to-date with `main`

### Submitting a PR

1. **Push Your Branch**:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create Pull Request**:
   - Use the GitHub UI to create a PR
   - Fill out the PR template completely
   - Link related issues (e.g., "Fixes #123")
   - Add appropriate labels

3. **PR Title Format**:
   ```
   <type>(<scope>): <description>
   ```
   Example: `feat(scanner): add Kyber detection support`

4. **PR Description**:
   - Explain what and why (not how)
   - Include screenshots for UI changes
   - List breaking changes
   - Provide testing instructions

### Code Review Process

1. **Automated Checks**:
   - CI must pass (tests, lints, security scans)
   - Code coverage must not decrease significantly

2. **Peer Review**:
   - At least 1 approval required from maintainers
   - Address all review comments
   - Mark conversations as resolved when fixed

3. **Review Criteria**:
   - Code quality and style
   - Test coverage
   - Documentation completeness
   - Performance impact
   - Security implications
   - Breaking changes

### After Approval

1. **Squash and Merge** (default):
   - Maintainers will merge using squash merge
   - Ensures clean git history

2. **Release**:
   - Changes merged to `main` will be included in next release
   - CalVer versioning: `YYYY.MM.PATCH`

---

## Community Guidelines

### Asking Questions

- **General Questions**: Use [GitHub Discussions](https://github.com/arcqubit/pqc-scanner/discussions)
- **Bug Reports**: Use [GitHub Issues](https://github.com/arcqubit/pqc-scanner/issues) with bug template
- **Security Issues**: Email security@arcqubit.io (see [SECURITY.md](SECURITY.md))

### Getting Help

- Read existing documentation in `docs/`
- Check [GitHub Discussions](https://github.com/arcqubit/pqc-scanner/discussions)
- Review closed issues for similar problems
- Ask in discussions (don't open issues for questions)

### Issue Guidelines

1. **Search First**: Check if issue already exists
2. **Use Templates**: Fill out the appropriate issue template
3. **Be Specific**: Provide detailed reproduction steps
4. **One Issue Per Report**: Don't combine multiple issues
5. **Include Context**: Environment, versions, error messages

### Communication

- **Be respectful**: Follow the Code of Conduct
- **Be patient**: Maintainers are volunteers
- **Be constructive**: Provide actionable feedback
- **Be clear**: Use precise language
- **Be inclusive**: Welcome newcomers

---

## Recognition

Contributors will be:
- Listed in `CHANGELOG.md` for their contributions
- Mentioned in release notes
- Added to `CONTRIBUTORS.md` (if they wish)

Significant contributors may be invited to become project maintainers.

---

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see [LICENSE](LICENSE) file).

---

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [GitHub Flow](https://guides.github.com/introduction/flow/)
- [Conventional Commits](https://www.conventionalcommits.org/)

---

## Questions?

- Open a [GitHub Discussion](https://github.com/arcqubit/pqc-scanner/discussions)
- Email: support@arcqubit.io
- Review [SUPPORT.md](SUPPORT.md) for additional resources

**Thank you for contributing to PQC Scanner!**
