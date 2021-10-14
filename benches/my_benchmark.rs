use criterion::{criterion_group, criterion_main, Criterion};
use foundation::{leak, no_leak_vec, no_leak_autoreleasepool};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("to string");
    let str = &"aaa 🍺".repeat(100);
    group.bench_with_input("old", str, |b, str| b.iter(|| leak(str)));
    group.bench_with_input("autorelease", str, |b, str| b.iter(|| no_leak_autoreleasepool(str)));
    group.bench_with_input("vec", str, |b, str| b.iter(|| no_leak_vec(str)));
    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
