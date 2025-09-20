// 62. How do you benchmark different factorial implementations (naive vs iter::fold) using criterion?
// Output a report and avoid misleading results from --release. What are best practices for benchmarking?
// benches/factorial_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};

/// Naive recursive factorial
fn factorial_recursive(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

/// Iterative factorial using fold
fn factorial_fold(n: u64) -> u64 {
    (1..=n).fold(1, |acc, x| acc * x)
}

/// Benchmark function
fn bench_factorials(c: &mut Criterion) {
    let n = 20; // input to factorial

    c.bench_function("factorial_recursive", |b| b.iter(|| factorial_recursive(n)));

    c.bench_function("factorial_fold", |b| b.iter(|| factorial_fold(n)));
}

criterion_group!(benches, bench_factorials);
criterion_main!(benches);
