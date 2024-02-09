# spiral

[![Build Status](https://github.com/tversteeg/spiral/workflows/CI/badge.svg)](https://github.com/tversteeg/spiral/actions?workflow=CI)
[![Crates.io](https://img.shields.io/crates/v/spiral.svg)](https://crates.io/crates/spiral)
[![Documentation](https://docs.rs/spiral/badge.svg)](https://docs.rs/spiral)
[![License: GPL-3.0](https://img.shields.io/crates/l/spiral.svg)](#license)
[![Downloads](https://img.shields.io/crates/d/spiral.svg)](#downloads)

### [Documentation](https://docs.rs/spiral/)

<!-- cargo-rdme start -->

Iterators to iterate 2D structures in spiral patterns

#### Usage

This crate is [on crates.io](htpps://crates.io/crates/spiral) and can be used by adding
`spiral` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
spiral = "0.2"
```

#### Examples

```rust
use spiral::ChebyshevIterator;

let center_x = 3;
let center_y = 3;
let radius = 4;
let iterator = ChebyshevIterator::new(center_x, center_y, radius);
for (x, y) in iterator {
    // Iterates over a 7x7 2D array with `x` & `y`.
}
```

```rust
use spiral::ManhattanIterator;

let center_x = 3;
let center_y = 3;
let radius = 4;
for (x, y) in ManhattanIterator::new(center_x, center_y, radius) {
    // Iterates over a 7x7 2D array with `x` & `y`.
}
```

<!-- cargo-rdme end -->
