use criterion::*;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("works", |b| {
    b.iter(|| {
      //
      unimplemented!()
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
