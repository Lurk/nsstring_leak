use criterion::{criterion_group, criterion_main, Criterion};
use foundation::{leak, no_leak};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("to string");
    let str = &"aaa 🍺".repeat(100);
    group.bench_with_input("old", str, |b, str| b.iter(|| leak(str)));
    group.bench_with_input("new", str, |b, str| b.iter(|| no_leak(str)));
    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
