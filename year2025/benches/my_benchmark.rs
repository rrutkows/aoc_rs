use criterion::{Criterion, criterion_group, criterion_main};
use year2025::d01;

fn criterion_benchmark(c: &mut Criterion) {
    let input = year2025::get_input(1);
    c.bench_function("d01p1", |b| b.iter(|| d01::solve01(&input)));
    c.bench_function("d01p2", |b| b.iter(|| d01::solve02(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
