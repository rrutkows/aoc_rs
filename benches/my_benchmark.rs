use aoc2022::d14;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = aoc2022::get_input(14);
    c.bench_function("d14p1", |b| b.iter(|| d14::solve01(&input)));
    c.bench_function("d14p2", |b| b.iter(|| d14::solve02(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
