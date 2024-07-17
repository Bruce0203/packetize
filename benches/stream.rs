use criterion::Criterion;

fn benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| {
        b.iter(|| {});
    });
}

criterion::criterion_group!(benches, benchmark);
criterion::criterion_main!(benches);
