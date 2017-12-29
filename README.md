# spiral

A Rust Library for iterating over 2D structures in spiral patterns

[![Build Status](https://travis-ci.org/tversteeg/spiral.svg?branch=master)](https://travis-ci.org/tversteeg/spiral) [![Cargo](https://img.shields.io/crates/v/spiral.svg)](https://crates.io/crates/spiral) [![License: GPL-3.0](https://img.shields.io/crates/l/spiral.svg)](#license) [![Downloads]( 	https://img.shields.io/crates/d/spiral.svg)](#downloads)

### [Documentation](https://docs.rs/spiral/)

## Usage

Add this to your `Cargo.tml`:

```toml
[dependencies]
spiral = "0.1"
```

And this to your crate root:

```rust
extern crate spiral;
```

## Examples

```rust
let center_x = 3;
let center_y = 3;
let radius = 4;
for (x, y) in ManhattanIterator::new(center_x, center_y, radius) {
	// Iterates over 7x7 2D array with 'x' & 'y' following this pattern:
	//
	//  0   0   0  23   0   0   0 
	//  0   0  22  12  24   0   0 
	//  0  21  11   5  13  25   0 
	// 20  10   4   1   2   6  14 
	//  0  19   9   3   7  15   0 
	//  0   0  18   8  16   0   0 
	//  0   0   0  17   0   0   0 
}
```
