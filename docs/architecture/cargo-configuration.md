# Cargo.toml Configuration Guide

**Project**: Quantum Crypto Audit Library
**Target**: WASM32-unknown-unknown
**Purpose**: Complete dependency and build configuration

---

## Production Cargo.toml

```toml
[package]
name = "quantum-crypto-audit"
version = "1.0.0"
edition = "2021"
authors = ["ArcQubit Team <team@arcqubit.com>"]
description = "Quantum-safe cryptography audit library compiled to WebAssembly"
documentation = "https://docs.arcqubit.com/crypto-audit"
homepage = "https://arcqubit.com"
repository = "https://github.com/arcqubit/quantum-crypto-audit"
readme = "README.md"
license = "MIT"
keywords = ["cryptography", "quantum", "security", "audit", "wasm"]
categories = ["cryptography", "wasm", "development-tools"]

[lib]
crate-type = ["cdylib", "rlib"]
# cdylib: For WASM compilation
# rlib: For native Rust testing and benchmarking

[dependencies]
# WASM bindings - Core requirement for JavaScript interop
wasm-bindgen = "0.2.92"

# Serialization - JSON support for config and reports
serde = { version = "1.0.193", features = ["derive"] }
serde-wasm-bindgen = "0.6.3"
serde_json = "1.0.108"

# Pattern matching - Regex engine for crypto detection
regex = "1.10.2"

# Error handling - Panic hooks for better WASM debugging
console_error_panic_hook = "0.1.7"

# JavaScript interop - Additional JS types and APIs
js-sys = "0.3.67"

# Lazy evaluation - Deferred computation for performance
once_cell = "1.19.0"

# Optional: Smaller allocator for reduced binary size
wee_alloc = { version = "0.4.5", optional = true }

[dev-dependencies]
# WASM testing - Browser and Node.js test runner
wasm-bindgen-test = "0.3.42"

# Benchmarking - Performance testing
criterion = "0.5.1"

# Property-based testing - Fuzz testing
proptest = "1.4.0"

# Mock testing - For testing without WASM environment
mockall = "0.12.1"

[profile.dev]
# Development builds: Fast compilation, debug info
opt-level = 0
debug = true
split-debuginfo = "unpacked"
debug-assertions = true
overflow-checks = true
incremental = true
codegen-units = 256

[profile.release]
# Production builds: Maximum optimization for size and speed
opt-level = "z"        # Optimize for size (alternatives: "s", 3)
lto = true             # Link-time optimization (slower compile, better binary)
codegen-units = 1      # Better optimization, slower compile
panic = "abort"        # Smaller binary, no unwinding
strip = true           # Remove debug symbols
debug = false
debug-assertions = false
overflow-checks = false

[profile.release-with-debug]
# Release with debug symbols for profiling
inherits = "release"
debug = true
strip = false

[profile.bench]
# Benchmarking profile
inherits = "release"
debug = true

[features]
default = []

# Small binary feature - Use wee_alloc instead of default allocator
small-binary = ["wee_alloc"]

# Extra validation - Enable additional runtime checks
extra-validation = []

# Parallel processing - Enable rayon for multi-threaded parsing
# Note: Not compatible with WASM single-threaded model
parallel = []

[package.metadata.wasm-pack.profile.release]
# wasm-opt optimization passes
wasm-opt = ["-Oz", "--enable-mutable-globals"]

[package.metadata.wasm-pack.profile.dev]
wasm-opt = false

[[bench]]
name = "crypto_detection"
harness = false
```

---

## Dependency Justification

### Core WASM Dependencies

#### 1. wasm-bindgen (0.2.92)
**Purpose**: JavaScript <-> Rust interop
**Why**: Required for WASM exports, types, and error handling
**Size Impact**: ~50KB (essential overhead)

```rust
// Enables this pattern:
#[wasm_bindgen]
pub fn audit_file(content: &str) -> Result<JsValue, JsValue>
```

#### 2. serde (1.0.193)
**Purpose**: Serialization/deserialization framework
**Why**: Type-safe JSON parsing for config and reports
**Size Impact**: ~30KB
**Features**: `derive` macro for automatic implementations

```rust
#[derive(Serialize, Deserialize)]
pub struct AuditConfig {
    pub severity_threshold: Severity,
}
```

#### 3. serde-wasm-bindgen (0.6.3)
**Purpose**: Efficient serde <-> JsValue conversion
**Why**: 2-3x faster than serde_json for WASM
**Size Impact**: ~15KB

```rust
// Efficient conversion:
let config: AuditConfig = serde_wasm_bindgen::from_value(js_value)?;
```

#### 4. regex (1.10.2)
**Purpose**: Pattern matching engine
**Why**: Core crypto detection algorithm
**Size Impact**: ~250KB (largest dependency, but essential)
**Note**: Consider lazy_static for regex compilation caching

```rust
let rsa_pattern = Regex::new(r"RSA\.generate")?;
```

#### 5. console_error_panic_hook (0.1.7)
**Purpose**: Better error messages in browser console
**Why**: Essential for debugging WASM panics
**Size Impact**: ~5KB

```rust
// Setup in lib.rs initialization
console_error_panic_hook::set_once();
```

### Development Dependencies

#### 6. wasm-bindgen-test (0.3.42)
**Purpose**: Test runner for WASM environment
**Why**: Run tests in Node.js or headless browsers
**Usage**: `wasm-pack test --node`

#### 7. criterion (0.5.1)
**Purpose**: Benchmarking framework
**Why**: Performance regression detection
**Usage**: `cargo bench`

```rust
// benches/crypto_detection.rs
fn bench_rsa_detection(c: &mut Criterion) {
    c.bench_function("detect_rsa_1000_lines", |b| {
        b.iter(|| detector.detect(&large_file))
    });
}
```

---

## Build Profiles Explained

### Development Profile (dev)

**Goals**: Fast iteration, good debugging experience

```toml
opt-level = 0           # No optimization for fast builds
debug = true            # Full debug symbols
codegen-units = 256     # Parallel compilation
incremental = true      # Reuse previous compilation
```

**Build Time**: ~5-10 seconds for incremental
**Binary Size**: ~2-3MB unoptimized
**Runtime Speed**: 5-10x slower than release

### Release Profile (release)

**Goals**: Smallest binary size, best performance

```toml
opt-level = "z"         # Optimize for size (special LLVM pass)
lto = true              # Inline across crates (15-20% size reduction)
codegen-units = 1       # Single compilation unit (better optimization)
panic = "abort"         # No stack unwinding (smaller binary)
strip = true            # Remove debug symbols
```

**Build Time**: ~60-120 seconds full build
**Binary Size**: ~300-500KB gzipped
**Runtime Speed**: Near-native performance

**Alternatives**:
- `opt-level = "s"`: Optimize for size (faster compile than "z")
- `opt-level = 3`: Optimize for speed (larger binary)

### Release with Debug Profile

**Purpose**: Performance profiling and production debugging

```toml
inherits = "release"
debug = true            # Keep symbols for profilers
strip = false
```

**Usage**: `cargo build --profile release-with-debug`

---

## Feature Flags

### 1. small-binary (Optional)

**Purpose**: Use wee_alloc for smaller binary size

```toml
small-binary = ["wee_alloc"]
```

**Usage**:
```rust
// src/lib.rs
#[cfg(feature = "small-binary")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

**Trade-offs**:
- ✅ 20-30KB smaller binary
- ❌ Slightly slower allocations
- ❌ No parallel allocations (fine for WASM)

**Build**: `wasm-pack build --release -- --features small-binary`

### 2. extra-validation (Optional)

**Purpose**: Additional runtime validation for testing

```rust
#[cfg(feature = "extra-validation")]
fn validate_invariants(&self) {
    debug_assert!(self.findings.len() <= MAX_FINDINGS);
}
```

**Usage**: Testing environments only

---

## wasm-pack Configuration

### Metadata Section

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Oz", "--enable-mutable-globals"]
```

**wasm-opt Flags**:
- `-Oz`: Aggressive size optimization
- `-O3`: Aggressive speed optimization
- `-O4`: Maximum optimization (size + speed)
- `--enable-mutable-globals`: WASM feature for globals

### Build Commands

```bash
# Web target (ES modules)
wasm-pack build --target web --release

# Node.js target (CommonJS)
wasm-pack build --target nodejs --release

# Bundler target (webpack, rollup)
wasm-pack build --target bundler --release

# No-modules (legacy, script tag)
wasm-pack build --target no-modules --release
```

---

## Dependency Version Management

### Update Strategy

```bash
# Check for outdated dependencies
cargo outdated

# Update within semver constraints
cargo update

# Audit for security vulnerabilities
cargo audit

# Update to latest major versions (breaking changes)
cargo upgrade
```

### Version Pinning

**Recommendation**: Pin exact versions for production stability

```toml
# Exact version (recommended for critical dependencies)
wasm-bindgen = "=0.2.92"

# Caret version (allow patch updates)
serde = "^1.0.193"

# Tilde version (allow patch updates only)
regex = "~1.10.2"
```

---

## Binary Size Analysis

### Expected Sizes

| Configuration | Uncompressed | Gzipped | Brotli |
|--------------|-------------|---------|--------|
| Debug        | ~2.5MB      | ~800KB  | ~600KB |
| Release (z)  | ~800KB      | ~350KB  | ~250KB |
| Release (s)  | ~900KB      | ~400KB  | ~280KB |
| Release + wee_alloc | ~750KB | ~320KB | ~220KB |

### Size Optimization Tools

```bash
# Analyze binary size by section
wasm-opt --print-binary-size quantum_crypto_audit_bg.wasm

# Further optimize (already done by wasm-pack)
wasm-opt -Oz -o output.wasm input.wasm

# Inspect what functions take space
twiggy top quantum_crypto_audit_bg.wasm

# Analyze monomorphization (large templates)
cargo bloat --release --crate-type cdylib
```

---

## Testing Configuration

### Unit Tests

```bash
# Run native Rust tests (rlib)
cargo test

# Run with coverage
cargo tarpaulin --out Html
```

### WASM Tests

```bash
# Test in Node.js environment
wasm-pack test --node

# Test in headless Chrome
wasm-pack test --headless --chrome

# Test in headless Firefox
wasm-pack test --headless --firefox

# Watch mode for TDD
cargo watch -x "test"
```

### Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench crypto_detection

# Save baseline for comparison
cargo bench -- --save-baseline before-optimization

# Compare to baseline
cargo bench -- --baseline before-optimization
```

---

## Continuous Integration Example

```yaml
# .github/workflows/rust-wasm-ci.yml
name: Rust WASM CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          override: true
          components: rustfmt, clippy

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache target directory
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-target-${{ hashFiles('**/Cargo.lock') }}

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Clippy linting
        run: cargo clippy -- -D warnings

      - name: Run tests
        run: cargo test --verbose

      - name: Build WASM (release)
        run: wasm-pack build --release --target web

      - name: Test WASM
        run: wasm-pack test --node --release

      - name: Check binary size
        run: |
          ls -lh pkg/*.wasm
          gzip -k pkg/*.wasm
          ls -lh pkg/*.wasm.gz

      - name: Security audit
        run: |
          cargo install cargo-audit
          cargo audit

  benchmark:
    runs-on: ubuntu-latest
    if: github.event_name == 'push'
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run benchmarks
        run: cargo bench -- --save-baseline ${{ github.sha }}
```

---

## Dependency Alternatives Considered

### 1. Regex Alternatives

**Current**: `regex` (1.10.2) - 250KB
**Alternatives**:
- `fancy-regex` - Full regex features, larger
- `regex-lite` - Subset, 50% smaller, limited features
- Custom parsing - Smallest, most work

**Decision**: Use `regex` for full feature set and maintenance

### 2. JSON Alternatives

**Current**: `serde_json` + `serde-wasm-bindgen`
**Alternatives**:
- `simd-json` - Faster, but larger and uses unsafe
- `sonic-rs` - Faster parsing, less mature
- Manual parsing - Smallest, error-prone

**Decision**: Stick with serde ecosystem for type safety

### 3. Error Handling Alternatives

**Current**: Custom `AuditError` enum
**Alternatives**:
- `thiserror` - Derive macro, nice but adds dependency
- `anyhow` - Dynamic errors, not ideal for library
- `miette` - Beautiful errors, too heavy for WASM

**Decision**: Hand-rolled errors for WASM size

---

## Troubleshooting Common Issues

### Issue 1: Binary Size Too Large

**Symptoms**: WASM binary > 1MB compressed

**Solutions**:
1. Enable LTO: `lto = true`
2. Use opt-level "z": `opt-level = "z"`
3. Strip symbols: `strip = true`
4. Enable wee_alloc: `--features small-binary`
5. Check for duplicate dependencies: `cargo tree -d`

### Issue 2: Slow Compilation

**Symptoms**: `cargo build` takes > 5 minutes

**Solutions**:
1. Use dev profile for iteration
2. Increase codegen-units: `codegen-units = 16`
3. Disable LTO for dev: `lto = false`
4. Use `cargo check` instead of `cargo build`
5. Enable incremental compilation

### Issue 3: Runtime Panics in WASM

**Symptoms**: "unreachable executed" in browser console

**Solutions**:
1. Add `console_error_panic_hook::set_once()`
2. Use `RUST_BACKTRACE=1` for native tests
3. Enable source maps in wasm-pack
4. Add explicit error handling, avoid `unwrap()`

---

## Maintenance Schedule

### Weekly
- Update dependencies: `cargo update`
- Run security audit: `cargo audit`

### Monthly
- Check for breaking changes: `cargo outdated`
- Review new Rust/WASM features

### Quarterly
- Major version updates: `cargo upgrade`
- Performance benchmarking against baseline
- Binary size analysis and optimization

---

**End of Cargo Configuration Guide**

This configuration provides:
- ✅ Minimal dependencies (no bloat)
- ✅ Optimized for size and speed
- ✅ Comprehensive testing setup
- ✅ CI/CD ready
- ✅ Production-grade profiles
