use criterion::{Criterion, criterion_group, criterion_main};
use year2025::{d01, d02, d03, d04, d05, d06, d07};

fn bench_day<F1, F2, O1, O2>(c: &mut Criterion, day: u8, p1: F1, p2: F2)
where
    F1: Fn(&str) -> O1,
    F2: Fn(&str) -> O2,
{
    let input = year2025::get_input(day);
    c.bench_function(&format!("d{day:02}p1"), |b| b.iter(|| p1(&input)));
    c.bench_function(&format!("d{day:02}p2"), |b| b.iter(|| p2(&input)));
}

fn bench_day_both<F, O>(c: &mut Criterion, day: u8, p: F)
where
    F: Fn(&str) -> O,
{
    let input = year2025::get_input(day);
    c.bench_function(&format!("d{day:02}single"), |b| b.iter(|| p(&input)));
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_day(c, 1, d01::solve01, d01::solve02);
    bench_day(c, 2, d02::solve01, d02::solve02);
    bench_day(
        c,
        3,
        |input| d03::solve(input, 2),
        |input| d03::solve(input, 12),
    );
    bench_day(c, 4, d04::solve01, d04::solve02);
    bench_day(c, 5, d05::solve01, d05::solve02);
    bench_day(c, 6, d06::solve01, d06::solve02);
    bench_day_both(c, 7, d07::solve);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
