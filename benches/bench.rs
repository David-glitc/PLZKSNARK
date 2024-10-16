extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn zk_benchmark(c: &mut Criterion) {
    c.bench_function("zkSNARK proof generation", |b| b.iter(|| generate_proof()));
}

criterion_group!(benches, zk_benchmark);
criterion_main!(benches);
