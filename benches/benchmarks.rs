#![feature(test)]

extern crate spiral;
extern crate test;

use test::Bencher;

use spiral::*;

const SMALL: u16 = 2;
const MEDIUM: u16 = 10;
const LARGE: u16 = 100;
const HUGE: u16 = 1000;

// Small
#[bench]
fn chebyshev_small(b: &mut Bencher) {
    b.iter(|| for _ in ChebyshevIterator::new(0, 0, SMALL) {});
}

#[bench]
fn manhattan_small(b: &mut Bencher) {
    b.iter(|| for _ in ManhattanIterator::new(0, 0, SMALL) {});
}

// Medium
#[bench]
fn chebyshev_medium(b: &mut Bencher) {
    b.iter(|| for _ in ChebyshevIterator::new(0, 0, MEDIUM) {});
}

#[bench]
fn manhattan_medium(b: &mut Bencher) {
    b.iter(|| for _ in ManhattanIterator::new(0, 0, MEDIUM) {});
}

// Large
#[bench]
fn chebyshev_large(b: &mut Bencher) {
    b.iter(|| for _ in ChebyshevIterator::new(0, 0, LARGE) {});
}

#[bench]
fn manhattan_large(b: &mut Bencher) {
    b.iter(|| for _ in ManhattanIterator::new(0, 0, LARGE) {});
}

// Huge
#[bench]
fn chebyshev_huge(b: &mut Bencher) {
    b.iter(|| for _ in ChebyshevIterator::new(0, 0, HUGE) {});
}

#[bench]
fn manhattan_huge(b: &mut Bencher) {
    b.iter(|| for _ in ManhattanIterator::new(0, 0, HUGE) {});
}
