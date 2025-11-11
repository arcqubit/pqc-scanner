# Contributing to PQC Scanner

Thank you for your interest in contributing to PQC Scanner! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Security Vulnerabilities](#security-vulnerabilities)

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to conduct@arcqubit.io.

## Getting Started

### Prerequisites

- **Rust**: 1.75 or later ([Install Rust](https://rustup.rs/))
- **Node.js**: 18+ (for WASM tooling and testing)
- **wasm-pack**: Latest version (`curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`)
- **Git**: For version control

### First-Time Setup

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR-USERNAME/pqc-scanner.git
   cd pqc-scanner
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/arcqubit/pqc-scanner.git
   ```
4. **Install dependencies**:
   ```bash
   cargo build
   ```
5. **Run tests** to verify setup:
   ```bash
   cargo test
   ```

## Development Setup

### Build Commands

```bash
# Build native Rust binary
cargo build

# Build release version
cargo build --release

# Build WASM targets
wasm-pack build --target bundler --out-dir pkg --release
wasm-pack build --target nodejs --out-dir pkg-nodejs --release
wasm-pack build --target web --out-dir pkg-web --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code formatting
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings

# Security audit
cargo audit
```

### Development Workflow

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our [coding standards](#coding-standards)

3. **Run tests and checks**:
   ```bash
   cargo test
   cargo fmt
   cargo clippy
   cargo audit
   ```

4. **Commit your changes** using [conventional commits](#commit-guidelines)

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Open a Pull Request** against the `main` branch

## How to Contribute

### Types of Contributions

We welcome various types of contributions:

- **Bug Reports**: Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md)
- **Feature Requests**: Use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.md)
- **Code Contributions**: Bug fixes, new features, performance improvements
- **Documentation**: Improve README, API docs, examples
- **Testing**: Add test cases, improve test coverage
- **Security**: Report vulnerabilities via [SECURITY.md](SECURITY.md)

### Finding Issues to Work On

- Look for issues labeled [`good first issue`](https://github.com/arcqubit/pqc-scanner/labels/good%20first%20issue) for beginner-friendly tasks
- Issues labeled [`help wanted`](https://github.com/arcqubit/pqc-scanner/labels/help%20wanted) are ready for contribution
- Comment on an issue to let others know you're working on it

## Coding Standards

### Rust Style Guide

We follow the official [Rust Style Guide](https://rust-lang.github.io/api-guidelines/):

- Use `cargo fmt` for automatic formatting
- Use `cargo clippy` to catch common mistakes
- Follow Rust naming conventions:
  - `snake_case` for functions, variables, modules
  - `CamelCase` for types and traits
  - `SCREAMING_SNAKE_CASE` for constants

### Code Organization

- Keep functions under 50 lines when possible
- Keep files under 500 lines
- Separate concerns into modules
- Use meaningful variable and function names
- Add doc comments for public APIs

### Documentation

All public APIs must have documentation:

```rust
/// Analyzes source code for quantum-vulnerable cryptographic algorithms.
///
/// # Arguments
///
/// * `source` - The source code to analyze
/// * `language` - The programming language (e.g., "rust", "python")
///
/// # Returns
///
/// Returns `Ok(AuditResult)` on success, or `Err` if analysis fails
///
/// # Example
///
/// ```
/// use pqc_scanner::analyze;
///
/// let result = analyze("use md5::Md5;", "rust")?;
/// assert!(!result.vulnerabilities.is_empty());
/// ```
pub fn analyze(source: &str, language: &str) -> Result<AuditResult> {
    // Implementation
}
```

### Error Handling

- Use `Result<T, E>` for recoverable errors
- Use custom error types when appropriate
- Provide context with error messages
- Avoid unwrap() in production code

## Testing Requirements

### Test Coverage

- **Minimum coverage**: 80% for new code
- **Critical paths**: 100% coverage required
- **Edge cases**: Test boundary conditions and error paths

### Test Types

1. **Unit Tests**: Test individual functions
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_md5_detection() {
           let result = analyze("hashlib.md5()", "python").unwrap();
           assert_eq!(result.vulnerabilities.len(), 1);
       }
   }
   ```

2. **Integration Tests**: Place in `tests/` directory
3. **Benchmarks**: Place in `benches/` directory

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_md5_detection

# With output
cargo test -- --nocapture

# Coverage report
cargo tarpaulin --out Html
```

## Commit Guidelines

We use [Conventional Commits](https://www.conventionalcommits.org/):

### Commit Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, no logic change)
- **refactor**: Code refactoring
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Maintenance tasks
- **ci**: CI/CD changes

### Examples

```bash
feat(scanner): Add support for ECDSA detection

Implement ECDSA curve detection for Python and JavaScript.
Detects weak curves (NIST P-192, P-224) and recommends
quantum-safe alternatives.

Closes #42
```

```bash
fix(wasm): Resolve memory leak in scanner initialization

The scanner was not properly freeing memory allocated during
initialization, causing memory usage to grow over time.

Fixes #123
```

## Pull Request Process

### Before Submitting

- [ ] Tests pass: `cargo test`
- [ ] Linting passes: `cargo clippy`
- [ ] Formatting applied: `cargo fmt`
- [ ] Security audit clean: `cargo audit`
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (for notable changes)

### PR Template

When opening a PR, include:

1. **Description**: What does this PR do?
2. **Motivation**: Why is this change needed?
3. **Testing**: How was this tested?
4. **Breaking Changes**: List any breaking changes
5. **Related Issues**: Link to related issues

### Review Process

1. **Automated Checks**: CI must pass (tests, linting, security)
2. **Code Review**: At least one maintainer approval required
3. **Discussion**: Address reviewer feedback
4. **Approval**: Maintainer merges after approval

### Merge Requirements

- All CI checks pass
- At least 1 approving review from a maintainer
- No unresolved conversations
- Branch is up to date with `main`
- Commits follow conventional commit format

## Security Vulnerabilities

**DO NOT** create public issues for security vulnerabilities.

Instead, follow the process in [SECURITY.md](SECURITY.md):

1. Report via GitHub Security Advisories (preferred)
2. Or email: security@arcqubit.io

## Getting Help

- **Questions**: Open a [Discussion](https://github.com/arcqubit/pqc-scanner/discussions)
- **Bug Reports**: Open an [Issue](https://github.com/arcqubit/pqc-scanner/issues)
- **Email**: support@arcqubit.io
- **Chat**: [Join our Discord](https://discord.gg/arcqubit) (Coming soon)

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).

## Recognition

Contributors are recognized in:

- README.md Contributors section
- Release notes for notable contributions
- Hall of Fame for significant contributions

Thank you for contributing to PQC Scanner! 🎉
