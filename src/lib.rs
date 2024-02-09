//! Iterators to iterate 2D structures in spiral patterns
//!
//! # Usage
//!
//! This crate is [on crates.io](htpps://crates.io/crates/spiral) and can be used by adding
//! `spiral` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! spiral = "0.2"
//! ```
//!
//! # Examples
//!
//! ```rust
//! use spiral::ChebyshevIterator;
//!
//! let center_x = 3;
//! let center_y = 3;
//! let radius = 4;
//! let iterator = ChebyshevIterator::new(center_x, center_y, radius);
//! for (x, y) in iterator {
//!     // Iterates over a 7x7 2D array with `x` & `y`.
//! }
//! ```
//!
//! ```rust
//! use spiral::ManhattanIterator;
//!
//! let center_x = 3;
//! let center_y = 3;
//! let radius = 4;
//! for (x, y) in ManhattanIterator::new(center_x, center_y, radius) {
//!     // Iterates over a 7x7 2D array with `x` & `y`.
//! }
//! ```

use num_traits::{FromPrimitive, PrimInt, WrappingAdd, WrappingSub};

/// Simple trait so we don't have to wrap write all the num signatures that we want every time.
pub trait Int: PrimInt + FromPrimitive + WrappingAdd + WrappingSub {}
impl<I: PrimInt + FromPrimitive + WrappingAdd + WrappingSub> Int for I {}

/// Which leg the iterator is in.
#[derive(Debug, Copy, Clone)]
enum Leg {
    /// Center of the iterator, should only be triggered once.
    Center,
    Right,
    Top,
    Left,
    Bottom,
}

/// An iterator iterating in a spiral fashion with the Chebyshev distance function.
///
/// The distance function is defined as:
///
/// `distance = max(absolute x offset from center, absolute y offset from center)`.
///
/// This creates a rectangular-shaped spiral.
#[derive(Debug, Clone)]
pub struct ChebyshevIterator<I: Int> {
    max_distance: I,
    start_x: I,
    start_y: I,

    x: I,
    y: I,
    layer: I,
    leg: Leg,
}

impl<I: Int> ChebyshevIterator<I> {
    /// Create a new iterator using the Chebyshev distance function
    ///
    /// # Arguments
    ///
    /// * `x` - The x position of the center of the spiral
    /// * `y` - The y position of the center of the spiral
    /// * `max_distance` - The radius of the spiral
    ///
    /// # Example
    /// ```
    /// use spiral::ChebyshevIterator;
    ///
    /// let spiral = ChebyshevIterator::new(3, 3, 4);
    /// for (x, y) in spiral {
    ///     // Iterates over 7x7 2D array with 'x' & 'y' following this pattern:
    ///
    ///     // 43  44  45  46  47  48  49
    ///     // 42  21  22  23  24  25  26
    ///     // 41  20   7   8   9  10  27
    ///     // 40  19   6   1   2  11  28
    ///     // 39  18   5   4   3  12  29
    ///     // 38  17  16  15  14  13  30
    ///     // 37  36  35  34  33  32  31
    /// }
    /// ```
    pub fn new(x: I, y: I, max_distance: I) -> Self {
        ChebyshevIterator {
            max_distance,
            start_x: x,
            start_y: y,

            x: I::zero(),
            y: I::zero(),
            layer: I::one(),
            leg: Leg::Center,
        }
    }
}

impl<I: Int> Iterator for ChebyshevIterator<I> {
    type Item = (I, I);

    fn next(&mut self) -> Option<(I, I)> {
        match self.leg {
            Leg::Center => {
                self.leg = Leg::Right;
            }
            Leg::Right => {
                self.x = self.x.wrapping_add(&I::one());

                if self.x == self.layer {
                    self.leg = Leg::Top;

                    if self.layer == self.max_distance {
                        return None;
                    }
                }
            }
            Leg::Top => {
                self.y = self.y.wrapping_add(&I::one());

                if self.y == self.layer {
                    self.leg = Leg::Left;
                }
            }
            Leg::Left => {
                self.x = self.x.wrapping_sub(&I::one());

                // -self.x == self.layer
                if self.x.wrapping_add(&self.layer).is_zero() {
                    self.leg = Leg::Bottom;
                }
            }
            Leg::Bottom => {
                self.y = self.y.wrapping_sub(&I::one());

                // -self.y == self.layer
                if self.y.wrapping_add(&self.layer).is_zero() {
                    self.leg = Leg::Right;

                    self.layer = self.layer.add(I::one());
                }
            }
        }

        Some((
            self.start_x.wrapping_add(&self.x),
            self.start_y.wrapping_add(&self.y),
        ))
    }
}

/// An iterator iterating in a spiral fashion with the Manhattan distance function.
///
/// The distance function is defined as:
///
/// `distance = absolute x offset from center + absolute y offset from center`.
///
/// This creates a diamond-shaped spiral.
#[derive(Debug, Clone)]
pub struct ManhattanIterator<I: Int> {
    max_distance: I,
    start_x: I,
    start_y: I,

    x: I,
    y: I,
    layer: I,
    leg: Leg,
}

impl<I: Int> ManhattanIterator<I> {
    /// Create a new iterator using the Manhattan distance function
    ///
    /// # Arguments
    ///
    /// * `x` - The x position of the center of the spiral
    /// * `y` - The y position of the center of the spiral
    /// * `max_distance` - The radius of the spiral
    ///
    /// # Example
    /// ```
    /// use spiral::ManhattanIterator;
    ///
    /// let spiral = ManhattanIterator::new(3, 3, 4);
    /// for (x, y) in spiral {
    ///     // Iterates over 7x7 2D array with 'x' & 'y' following this pattern:
    ///
    ///     //  0   0   0  23   0   0   0
    ///     //  0   0  22  12  24   0   0
    ///     //  0  21  11   5  13  25   0
    ///     // 20  10   4   1   2   6  14
    ///     //  0  19   9   3   7  15   0
    ///     //  0   0  18   8  16   0   0
    ///     //  0   0   0  17   0   0   0
    /// }
    /// ```
    pub fn new(x: I, y: I, max_distance: I) -> Self {
        ManhattanIterator {
            max_distance,
            start_x: x,
            start_y: y,

            x: I::from_u8(2).expect("Could not instantiate number"),
            // -1
            y: I::zero().wrapping_sub(&I::one()),

            layer: I::one(),
            leg: Leg::Center,
        }
    }
}

impl<I: Int> Iterator for ManhattanIterator<I> {
    type Item = (I, I);

    fn next(&mut self) -> Option<(I, I)> {
        match self.leg {
            Leg::Center => {
                self.leg = Leg::Right;

                return Some((self.start_x, self.start_y));
            }
            Leg::Right => {
                if self.max_distance == I::one() {
                    return None;
                }

                self.x = self.x.wrapping_sub(&I::one());
                self.y = self.y.wrapping_add(&I::one());

                if self.x.is_zero() {
                    self.leg = Leg::Top;
                }
            }
            Leg::Top => {
                self.x = self.x.wrapping_sub(&I::one());
                self.y = self.y.wrapping_sub(&I::one());

                if self.y.is_zero() {
                    self.leg = Leg::Left;
                }
            }
            Leg::Left => {
                self.x = self.x.wrapping_add(&I::one());
                self.y = self.y.wrapping_sub(&I::one());

                if self.x.is_zero() {
                    self.leg = Leg::Bottom;
                }
            }
            Leg::Bottom => {
                self.x = self.x.wrapping_add(&I::one());
                self.y = self.y.wrapping_add(&I::one());

                if self.y.is_zero() {
                    self.x = self.x.wrapping_add(&I::one());
                    self.leg = Leg::Right;

                    self.layer = self.layer.wrapping_add(&I::one());

                    if self.layer == self.max_distance {
                        return None;
                    }
                }
            }
        }

        Some((self.start_x + self.x, self.start_y + self.y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output() {
        const SIZE: usize = 7;

        println!("Manhattan");
        let mut output: [i32; SIZE * SIZE] = [0; SIZE * SIZE];

        let mut current = 0;
        let spiral = ManhattanIterator::new(3, 3, 4);
        for (x, y) in spiral {
            current += 1;

            let index = x as usize + (y as usize * SIZE);
            output[index] = current;
        }

        for y in 0..SIZE {
            for x in 0..SIZE {
                let index = x as usize + (y as usize * SIZE);
                let output_val = output[index];

                print!("{:3} ", output_val);
            }
            println!();
        }

        println!("Chebyshev");
        output = [0; SIZE * SIZE];

        current = 0;
        let spiral = ChebyshevIterator::new(3, 3, 4);
        for (x, y) in spiral {
            current += 1;

            let index = x as usize + (y as usize * SIZE);
            output[index] = current;
        }

        for y in 0..SIZE {
            for x in 0..SIZE {
                let index = x as usize + (y as usize * SIZE);
                let output_val = output[index];

                print!("{:3} ", output_val);
            }
            println!();
        }
    }

    #[test]
    fn manhattan_bounds() {
        for size in 1..100 {
            let max_distance = size + 1;
            for (x, y) in ManhattanIterator::new(0i16, 0i16, size) {
                let distance = x.abs() + y.abs();
                assert!(
                    distance <= max_distance,
                    "spiral was out of bounds: distance {}, size: {}, x: {}, y: {}",
                    distance,
                    size,
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn chebyshev_bounds() {
        for size in 1..100 {
            let max_distance = (size + 1) as i32;
            for (x, y) in ChebyshevIterator::new(0i32, 0i32, size) {
                let distance = std::cmp::max(x.abs(), y.abs());
                assert!(
                    distance <= max_distance,
                    "spiral was out of bounds: distance {}, size: {}, x: {}, y: {}",
                    distance,
                    size,
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn manhattan() {
        let expected: [i32; 3 * 3] = [0, 5, 0, 4, 1, 2, 0, 3, 0];

        let mut current = 0;
        for (x, y) in ManhattanIterator::new(1, 1, 2) {
            current += 1;

            let index = x + y * 3;

            assert_eq!(expected[index as usize], current);
        }

        let expected: [i32; 5 * 5] = [
            0, 0, 12, 0, 0, 0, 11, 5, 13, 0, 10, 4, 1, 2, 6, 0, 9, 3, 7, 0, 0, 0, 8, 0, 0,
        ];

        current = 0;
        for (x, y) in ManhattanIterator::new(2, 2, 3) {
            current += 1;

            let index = x + y * 5;

            assert_eq!(expected[index as usize], current);
        }
    }

    #[test]
    fn manhattan_unsigned() {
        ManhattanIterator::new(0u32, 0u32, 4u32).for_each(drop);
    }

    #[test]
    fn chebyshev() {
        let expected: [i32; 5 * 5] = [
            21, 22, 23, 24, 25, 20, 7, 8, 9, 10, 19, 6, 1, 2, 11, 18, 5, 4, 3, 12, 17, 16, 15, 14,
            13,
        ];

        let mut current = 0;
        for (x, y) in ChebyshevIterator::new(2i32, 2i32, 3i32) {
            current += 1;

            let index = x + y * 5;

            assert_eq!(expected[index as usize], current);
        }
    }

    #[test]
    fn chebyshev_unsigned() {
        let expected: [u32; 5 * 5] = [
            21, 22, 23, 24, 25, 20, 7, 8, 9, 10, 19, 6, 1, 2, 11, 18, 5, 4, 3, 12, 17, 16, 15, 14,
            13,
        ];

        let mut current = 0;
        for (x, y) in ChebyshevIterator::new(2u32, 2u32, 3u32) {
            current += 1;

            let index = x + y * 5;

            assert_eq!(expected[index as usize], current);
        }
    }
}
