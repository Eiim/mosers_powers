use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_bigint::BigUint;
use mosers_powers::calc_to_from;

pub fn bench(c: &mut Criterion) {
	c.bench_function("calc 280", |b| b.iter(|| calc_to_from(black_box(2), black_box(280), black_box(BigUint::from(3u32)))));
}

criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(10);
    targets = bench
}
criterion_main!(benches);
