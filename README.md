<h1 align="center">spiral</h1>
<p align="center">
   A Rust Library for iterating over 2D structures in spiral patterns.
</p>

<p align="center">
	<a href="https://github.com/tversteeg/spiral/actions"><img src="https://github.com/tversteeg/spiral/workflows/rust/badge.svg" alt="CI"/></a>
	<a href="https://crates.io/crates/spiral"><img src="https://img.shields.io/crates/v/spiral.svg" alt="Version"/></a>
	<a href="https://docs.rs/spiral"><img src="https://img.shields.io/badge/api-rustdoc-blue.svg" alt="Rust Documentation"/></a>
	<img src="https://img.shields.io/crates/l/spiral.svg" alt="License"/>
</p>

## Usage

Add this to your `Cargo.toml`:

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
