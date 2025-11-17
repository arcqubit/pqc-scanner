use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use pqc_scanner::analyze;
use std::hint::black_box;

fn benchmark_file_parsing(c: &mut Criterion) {
    let source = generate_code_sample(1000);
    let lines = source.lines().count();

    let mut group = c.benchmark_group("file_parsing");
    group.throughput(Throughput::Elements(lines as u64));

    group.bench_with_input(
        BenchmarkId::from_parameter(format!("{}_LOC", lines)),
        &source,
        |b, source| {
            b.iter(|| analyze(black_box(source), black_box("rust")));
        },
    );

    group.finish();
}

fn benchmark_pattern_detection(c: &mut Criterion) {
    let test_cases = vec![
        ("small", generate_code_sample(100)),
        ("medium", generate_code_sample(500)),
        ("large", generate_code_sample(1000)),
    ];

    let mut group = c.benchmark_group("pattern_detection");

    for (name, code) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &code, |b, code| {
            b.iter(|| analyze(black_box(code), black_box("rust")));
        });
    }

    group.finish();
}

fn benchmark_throughput(c: &mut Criterion) {
    let samples: Vec<String> = (0..10)
        .map(|i| generate_code_sample(100 + i * 50))
        .collect();

    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Elements(samples.len() as u64));

    group.bench_function("process_multiple_files", |b| {
        b.iter(|| {
            for sample in &samples {
                let _ = analyze(black_box(sample), black_box("rust"));
            }
        });
    });

    group.finish();
}

fn benchmark_scaling(c: &mut Criterion) {
    let sizes = vec![100, 250, 500, 1000, 2000];

    let mut group = c.benchmark_group("scaling");

    for size in sizes {
        let code = generate_code_sample(size);
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_lines", size)),
            &code,
            |b, code| {
                b.iter(|| analyze(black_box(code), black_box("rust")));
            },
        );
    }

    group.finish();
}

fn generate_code_sample(lines: usize) -> String {
    let mut code = String::new();
    code.push_str("use std::collections::HashMap;\n\n");
    code.push_str("pub fn process_data() {\n");

    for i in 0..lines {
        if i % 10 == 0 {
            code.push_str(&format!("    let var_{} = {};\n", i, i));
        } else if i % 7 == 0 {
            code.push_str("    if condition {\n        do_something();\n    }\n");
        } else if i % 5 == 0 {
            code.push_str("    for item in items {\n        process(item);\n    }\n");
        } else {
            code.push_str(&format!("    // Comment line {}\n", i));
        }
    }

    code.push_str("}\n");
    code
}

criterion_group!(
    benches,
    benchmark_file_parsing,
    benchmark_pattern_detection,
    benchmark_throughput,
    benchmark_scaling
);

criterion_main!(benches);
