use criterion::*;

fn criterion_benchmark(c: &mut Criterion) {
  use wifi_visualizer::ieee802_11::*;
  use wifi_visualizer::test_packets::*;

  use std::io::Cursor;
  c.bench_function("BasicFrame::parse BEACON", |b| {
    b.iter(|| BasicFrame::parse(&mut Cursor::new(BEACON.to_vec())).unwrap())
  });
  c.bench_function("BasicFrame::parse DATA_FROM_DS", |b| {
    b.iter(|| BasicFrame::parse(&mut Cursor::new(DATA_FROM_DS.to_vec())).unwrap())
  });
  c.bench_function("BasicFrame::parse PROBE_RESPONSE_RETRY", |b| {
    b.iter(|| BasicFrame::parse(&mut Cursor::new(PROBE_RESPONSE_RETRY.to_vec())).unwrap())
  });
  c.bench_function("BasicFrame::parse RADIOTAP_FRAME_WITH_FCS", |b| {
    b.iter(|| BasicFrame::parse(&mut Cursor::new(RADIOTAP_FRAME_WITH_FCS.to_vec())).unwrap())
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
