use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day_02::{part1, part2};

fn part1_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part1");

    let input = include_str!("../input.txt");
    group.bench_with_input(
        BenchmarkId::new("part1", "input"),
        &input,
        |b, input| b.iter(|| part1::task(input)),
    );
}

fn part2_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part2");

    let input = include_str!("../input.txt");
    group.bench_with_input(
        BenchmarkId::new("part2", "input"),
        &input,
        |b, input| b.iter(|| part2::task(input)),
    );
}

criterion_group!(benches, part1_benchmark, part2_benchmark);
criterion_main!(benches);
