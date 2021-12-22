use ditonic_sort::ditonic_sort;

use rand::Rng;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut vector: Vec<i32> = (0..20000).map(|_| rng.gen_range(0..1000)).collect();
    c.bench_function("ditonic_sort", |b| b.iter(|| ditonic_sort(&mut vector)));
    c.bench_function("rust_sort", |b| b.iter(|| vector.sort()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);