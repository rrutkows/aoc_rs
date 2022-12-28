use aoc2022::{d14, d15, d16, d17, d18, d19};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = aoc2022::get_input(14);
    c.bench_function("d14p1", |b| b.iter(|| d14::solve01(&input)));
    c.bench_function("d14p2", |b| b.iter(|| d14::solve02(&input)));

    let input = aoc2022::get_input(15);
    c.bench_function("d15p1", |b| b.iter(|| d15::solve01(&input)));
    c.bench_function("d15p2", |b| b.iter(|| d15::solve02(&input)));

    let input = aoc2022::get_input(16);
    c.bench_function("d16p1", |b| b.iter(|| d16::solve(&input, 1)));
    c.bench_function("d16p2", |b| b.iter(|| d16::solve(&input, 2)));

    let input = aoc2022::get_input(17);
    c.bench_function("d17p1", |b| b.iter(|| d17::solve(&input, 2022)));
    c.bench_function("d17p2", |b| b.iter(|| d17::solve(&input, 1000000000000)));

    let input = aoc2022::get_input(18);
    c.bench_function("d18p1", |b| b.iter(|| d18::solve01(&input)));
    c.bench_function("d18p2", |b| b.iter(|| d18::solve02(&input)));

    let input = aoc2022::get_input(19);
    c.bench_function("d19p1", |b| b.iter(|| d19::solve01(&input)));
    c.bench_function("d19p2", |b| b.iter(|| d19::solve02(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
