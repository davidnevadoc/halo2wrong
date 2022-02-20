//! Benchmarks for point operations.

use criterion::{criterion_group, criterion_main, Criterion};

use maingate::halo2::arithmetic::CurveExt;

#[cfg(not(feature = "kzg"))]
fn criterion_benchmark(c: &mut Criterion) {
    point_bench::<maingate::halo2::pallas::Point>(c, "Pallas");
    point_bench::<maingate::halo2::vesta::Point>(c, "Vesta");
}

#[cfg(not(feature = "kzg"))]
fn point_bench<C: CurveExt>(c: &mut Criterion, name: &str) {
    let mut group = c.benchmark_group(name);

    let a = C::generator();
    let b = a.double();

    group.bench_function("point doubling", |bencher| bencher.iter(|| a.double()));

    group.bench_function("point addition", |bencher| bencher.iter(|| a + b));

    group.bench_function("point subtraction", |bencher| bencher.iter(|| a - b));

    group.bench_function("point to_bytes", |bencher| bencher.iter(|| a.to_bytes()));

    let repr = a.to_bytes();
    group.bench_function("point from_bytes", |bencher| bencher.iter(|| C::from_bytes(&repr)));

    group.bench_function("point to_affine", |bencher| bencher.iter(|| a.to_affine()));

    for &n in [100, 1000, 10000].iter() {
        let input = vec![a; n];
        let mut output = vec![C::AffineRepr::default(); n];
        group.bench_function(format!("point batch_normalize/{}", n), |bencher| {
            bencher.iter(|| C::batch_normalize(input.as_slice(), output.as_mut_slice()));
        });
    }
}

#[cfg(not(feature = "kzg"))]
criterion_group!(benches, criterion_benchmark);

#[cfg(not(feature = "kzg"))]
criterion_main!(benches);
