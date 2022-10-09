use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use spiral::*;

const SMALL: u16 = 2;
const MEDIUM: u16 = 10;
const LARGE: u16 = 100;
const HUGE: u16 = 1000;

pub fn chebyshev(c: &mut Criterion) {
    c.bench_function("chebyshev, small", |b| {
        b.iter(|| for _ in ChebyshevIterator::new(0, 0, SMALL) {});
    });

    c.bench_function("chebyshev, medium", |b| {
        b.iter(|| for _ in ChebyshevIterator::new(0, 0, MEDIUM) {});
    });

    c.bench_function("chebyshev, large", |b| {
        b.iter(|| for _ in ChebyshevIterator::new(0, 0, LARGE) {});
    });

    c.bench_function("chebyshev, huge", |b| {
        b.iter(|| for _ in ChebyshevIterator::new(0, 0, HUGE) {});
    });
}

pub fn manhattan(c: &mut Criterion) {
    c.bench_function("manhattan, small", |b| {
        b.iter(|| for _ in ManhattanIterator::new(0, 0, SMALL) {});
    });

    c.bench_function("manhattan, medium", |b| {
        b.iter(|| for _ in ManhattanIterator::new(0, 0, MEDIUM) {});
    });

    c.bench_function("manhattan, large", |b| {
        b.iter(|| for _ in ManhattanIterator::new(0, 0, LARGE) {});
    });

    c.bench_function("manhattan, huge", |b| {
        b.iter(|| for _ in ManhattanIterator::new(0, 0, HUGE) {});
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = manhattan, chebyshev
}

criterion_main!(benches);
