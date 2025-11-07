# Benchmark Guide
## Running and Understanding Performance Tests

### Quick Start

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench -- file_parsing
cargo bench -- pattern_detection
cargo bench -- throughput

# Generate HTML reports
cargo bench
# View results at: target/criterion/report/index.html
```

---

## Benchmark Suite Overview

### 1. File Parsing (`benchmark_file_parsing`)
**Purpose:** Measures parsing speed for large source files

**What it tests:**
- Tokenization performance
- AST node extraction
- Import detection
- Function call identification

**Target:** Parse 1000 LOC in <10ms
**Actual:** ~0.35ms (28x faster than target)

### 2. Pattern Detection (`benchmark_pattern_detection`)
**Purpose:** Tests crypto pattern matching across different file sizes

**Test cases:**
- Small: 100 LOC
- Medium: 500 LOC
- Large: 1000 LOC

**Target:** <5ms per file
**Actual:** 0.53ms for 1000 LOC (9.5x faster)

### 3. Crypto Pattern Matching (`benchmark_crypto_pattern_matching`)
**Purpose:** Validates detection of specific vulnerabilities

**Detects:**
- Hardcoded encryption keys
- Weak hash algorithms (MD5, SHA1)
- Insecure random number generators
- Other crypto anti-patterns

**Performance:** 6.17 µs per detection cycle

### 4. Throughput (`benchmark_throughput`)
**Purpose:** Measures batch processing speed

**Test:** Process 10 files of varying sizes
**Result:** 6,296 files/second
**Production estimate:** 10,000 files in ~1.59 seconds

### 5. Memory Footprint (`benchmark_memory_footprint`)
**Purpose:** Estimates memory usage for large files

**Test:** 5000 LOC file
**Target:** <50MB
**Actual:** ~42MB

### 6. Cold Start (`benchmark_cold_start`)
**Purpose:** Measures initialization overhead

**Result:** 50.1 µs
**Impact:** Minimal (comparable to warm execution)

### 7. Scaling Analysis (`benchmark_scaling`)
**Purpose:** Validates linear performance scaling

**File sizes tested:** 100, 250, 500, 1000, 2000, 5000 LOC
**Result:** O(n) linear scaling confirmed

---

## Understanding Results

### Time Measurements

```
time: [lower_bound mean upper_bound]
```

- **Lower bound:** 95% confidence minimum
- **Mean:** Average time across all samples
- **Upper bound:** 95% confidence maximum

Example:
```
file_parsing/178_LOC    time:   [61.503 µs 62.833 µs 64.263 µs]
```
Mean parsing time: 62.833 µs (microseconds)

### Throughput Measurements

```
thrpt: [lower_bound mean upper_bound]
```

- Measured in elements/second
- Higher is better
- Shows scalability

Example:
```
thrpt: [2.7699 Melem/s 2.8329 Melem/s 2.8942 Melem/s]
```
Mean throughput: 2.83 million elements/second

### Outliers

```
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
```

**Interpretation:**
- Normal: <5% outliers
- Acceptable: 5-10% outliers
- Investigate: >10% outliers

**Common causes:**
- OS scheduling
- Garbage collection
- Background processes
- Thermal throttling

---

## Customizing Benchmarks

### Add New Test Case

```rust
// In benches/benchmarks.rs
fn benchmark_my_test(c: &mut Criterion) {
    let test_code = "...";

    c.bench_function("my_test_name", |b| {
        b.iter(|| {
            audit::analyze(black_box(test_code), black_box("rust"))
        });
    });
}

// Add to criterion_group!
criterion_group!(
    benches,
    benchmark_file_parsing,
    benchmark_my_test  // Add here
);
```

### Adjust Benchmark Settings

```rust
let mut group = c.benchmark_group("my_group");

// Change measurement time
group.measurement_time(Duration::from_secs(15));

// Change sample count
group.sample_size(50);

// Set throughput for better reporting
group.throughput(Throughput::Elements(1000));

group.bench_function(...);
group.finish();
```

---

## Continuous Performance Testing

### In CI/CD

```yaml
# .github/workflows/benchmark.yml
- name: Run benchmarks
  run: cargo bench --no-fail-fast

- name: Compare with baseline
  run: |
    cargo bench -- --save-baseline main
    cargo bench -- --baseline main
```

### Regression Detection

```bash
# Establish baseline
cargo bench -- --save-baseline v1.0.0

# After changes, compare
cargo bench -- --baseline v1.0.0

# Criterion will show % change
```

---

## Performance Profiling

### Flamegraph Generation

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --bench benchmarks

# Opens flamegraph.svg in browser
```

### Memory Profiling

```bash
# Install valgrind and cargo-valgrind
cargo install cargo-valgrind

# Profile memory usage
cargo valgrind --bench benchmarks
```

---

## Troubleshooting

### Benchmarks Run Too Slowly

**Problem:** Criterion extends measurement time beyond 5 seconds

**Solutions:**
1. Reduce sample size: `group.sample_size(50)`
2. Reduce warm-up time: `group.warm_up_time(Duration::from_secs(1))`
3. Use faster test fixtures

### High Variance in Results

**Problem:** Large confidence intervals, many outliers

**Solutions:**
1. Close background applications
2. Run on dedicated hardware
3. Disable CPU frequency scaling
4. Increase sample size

```bash
# Linux: Disable CPU scaling
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

### Comparing Across Machines

**Problem:** Absolute times differ between machines

**Solution:** Use relative comparisons (throughput, scaling factors)

---

## Interpreting Criterion HTML Reports

After running `cargo bench`, open:
```
target/criterion/report/index.html
```

### Key Sections

1. **Summary:** Overview of all benchmarks
2. **Violin Plots:** Distribution visualization
3. **PDF Plots:** Probability density function
4. **Mean/Std Dev:** Statistical analysis
5. **History:** Compare runs over time

---

## Performance Targets Reference

| Metric | Target | Achieved | Margin |
|--------|--------|----------|--------|
| Parse 1000 LOC | <10ms | 0.35ms | 28x faster |
| Pattern Detection | <5ms | 0.53ms | 9.5x faster |
| Crypto Match | N/A | 6.17µs | Ultra-fast |
| Memory (5000 LOC) | <50MB | 42MB | 16% under |
| Throughput | High | 6296 files/s | Excellent |

---

## Next Steps

1. ✅ **Baseline Established:** Current performance documented
2. 🎯 **Monitor:** Track performance across releases
3. 🎯 **Optimize:** Address any regressions
4. 🎯 **Scale:** Test with real-world codebases

---

**For questions or issues, see:**
- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Performance Report](./PERFORMANCE_REPORT.md)
- Project benchmarks: `benches/benchmarks.rs`
