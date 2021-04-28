use ark_ec::AffineCurve;            // Needed for group inclusion check
use ark_ec::PairingEngine;          // Needed for pairing
use num_traits::identities::Zero;   // Needed for zero constructions
use num_traits::identities::One;    // Needed for one constructions
use ark_ff::fields::Field;          // Needed for pow
use ark_ff::ToBytes;
use ark_std::rand::Rng as ArkRng;   // Must be in scope for rscalar
use ark_bls12_381;

use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};

use backend::{
    scalar, zero, one, rscalar, genG1, genG2, zeroG1, zeroG2,
};

fn bench_scalar(c: &mut Criterion) {
    c.bench_function(
        "scalar!",
        |b| b.iter(|| scalar!(1000u64))
    );
}

fn bench_zero(c: &mut Criterion) {
    c.bench_function(
        "zero!",
        |b| b.iter(|| zero!())
    );
}

fn bench_one(c: &mut Criterion) {
    c.bench_function(
        "one!",
        |b| b.iter(|| one!())
    );
}

fn bench_rscalar(c: &mut Criterion) {
    use rand::RngCore;
    let mut rng = ::ark_std::test_rng();    // TODO
    c.bench_function(
        "rscalar!",
        |b| b.iter(|| rscalar!(rng))
    );
}

fn bench_genG1(c: &mut Criterion) {
    c.bench_function(
        "genG1!",
        |b| b.iter(|| genG1!())
    );
}

fn bench_genG2(c: &mut Criterion) {
    c.bench_function(
        "genG2!",
        |b| b.iter(|| genG2!())
    );
}

fn bench_zeroG1(c: &mut Criterion) {
    c.bench_function(
        "zeroG1!",
        |b| b.iter(|| zeroG1!())
    );
}

fn bench_zeroG2(c: &mut Criterion) {
    c.bench_function(
        "zeroG2!",
        |b| b.iter(|| zeroG2!())
    );
}

criterion_group!(
    benches,
    bench_scalar,
    bench_zero,
    bench_one,
    bench_rscalar,
    bench_zeroG1,
    bench_zeroG2,
    bench_genG1,
    bench_genG2,
);
criterion_main!(benches);
