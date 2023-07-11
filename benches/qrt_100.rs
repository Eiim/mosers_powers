use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mosers_powers::qrt2;

pub fn bench(c: &mut Criterion) {
	c.bench_function("qrt 100", |b| b.iter(|| qrt2(black_box(100))));
}

criterion_group!(benches, bench);
criterion_main!(benches);
