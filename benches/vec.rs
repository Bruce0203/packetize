use std::f128::consts::PI;

use arrayvec::ArrayVec;
use criterion::{criterion_group, criterion_main, Criterion};
use smallvec::SmallVec;

fn benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| {
        b.iter(|| {
            let vec = ArrayVec::new();
            vec.push(100);
        })
    })
}

criterion_main!(benches);
criterion_group!(benches, benchmark);
