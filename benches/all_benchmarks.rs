use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_rust_2023::*;

fn benchmark_1_a(c: &mut Criterion) {
    c.bench_function("DAY 1 :: PART A", |b| b.iter(|| solution_1::part_one()));
}
fn benchmark_1_b(c: &mut Criterion) {
    c.bench_function("DAY 1 :: PART B", |b| b.iter(|| solution_1::part_two()));
}

// DAY {day} :: PART {a|b}

criterion_group!(benches,
    benchmark_1_a,
    benchmark_1_b,
);
criterion_main!(benches);
