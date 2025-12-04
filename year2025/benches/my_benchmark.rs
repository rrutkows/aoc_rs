use criterion::{Criterion, criterion_group, criterion_main};
use year2025::{d01, d02, d03, d04};

fn criterion_benchmark(c: &mut Criterion) {
    let input = year2025::get_input(1);
    c.bench_function("d01p1", |b| b.iter(|| d01::solve01(&input)));
    c.bench_function("d01p2", |b| b.iter(|| d01::solve02(&input)));

    let input = year2025::get_input(2);
    c.bench_function("d02p1", |b| b.iter(|| d02::solve01(&input)));
    c.bench_function("d02p2", |b| b.iter(|| d02::solve02(&input)));

    let input = year2025::get_input(3);
    c.bench_function("d03p1", |b| b.iter(|| d03::solve(&input, 2)));
    c.bench_function("d03p2", |b| b.iter(|| d03::solve(&input, 12)));

    let input = year2025::get_input(4);
    c.bench_function("d04p1", |b| b.iter(|| d04::solve01(&input)));
    c.bench_function("d04p2", |b| b.iter(|| d04::solve02(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
