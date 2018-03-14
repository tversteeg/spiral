#![crate_name = "spiral"]

//! Iterators to iterate 2D structures in spiral patterns
//!
//! # Usage
//!
//! This crate is [on crates.io](htpps://crates.io/crates/spiral) and can be used by adding
//! `spiral` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! spiral = "0.1"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate spiral;
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

/// An iterator iterating in a spiral fashion with the Chebyshev distance function.
///
/// The distance function is defined as:
///
/// `distance = max(absolute x offset from center, absolute y offset from center)`.
///
/// This creates a rectangular-shaped spiral.
pub struct ChebyshevIterator {
    max_distance: i32,
    start_x: i32,
    start_y: i32,

    x: i32,
    y: i32,
    layer: i32,
    leg: i32
}

impl ChebyshevIterator {
    #[allow(dead_code)]
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
    pub fn new(x: i32, y: i32, max_distance: u16) -> Self {
        ChebyshevIterator {
            max_distance: max_distance as i32,
            start_x: x,
            start_y: y,

            x: 0,
            y: 0,
            layer: 1,
            leg: -1
        }
    }
}

impl Iterator for ChebyshevIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        match self.leg {
            // Use -1 as the center
            -1 => {
                self.leg = 0;
            }
            0 => {
                self.x += 1;
                if self.x == self.layer {
                    self.leg = 1;

                    if self.layer == self.max_distance {
                        return None
                    }
                }
            },
            1 => {
                self.y += 1;
                if self.y == self.layer {
                    self.leg = 2;
                }
            },
            2 => {
                self.x -= 1;
                if -self.x == self.layer {
                    self.leg = 3;
                }
            },
            3 => {
                self.y -= 1;
                if -self.y == self.layer {
                    self.leg = 0;
                    self.layer += 1;
                }
            },
            _ => unreachable!()
        }

        Some((self.start_x + self.x, self.start_y + self.y))
    }
}

/// An iterator iterating in a spiral fashion with the Manhattan distance function.
///
/// The distance function is defined as:
///
/// `distance = absolute x offset from center + absolute y offset from center`.
///
/// This creates a diamond-shaped spiral.
pub struct ManhattanIterator {
    max_distance: i32,
    start_x: i32,
    start_y: i32,

    x: i32,
    y: i32,
    layer: i32,
    leg: i32
}

impl ManhattanIterator {
    #[allow(dead_code)]
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
    /// use spiral::ChebyshevIterator;
    ///
    /// let spiral = ChebyshevIterator::new(3, 3, 4);
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
    pub fn new(x: i32, y: i32, max_distance: u16) -> Self {
        ManhattanIterator {
            max_distance: max_distance as i32,
            start_x: x,
            start_y: y,

            x: 2,
            y: -1,
            layer: 1,
            leg: -1
        }
    }
}

impl Iterator for ManhattanIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        match self.leg {
            // Use -1 as the center
            -1 => {
                self.leg = 0;
                
                return Some((self.start_x, self.start_y));
            }
            0 => {
                if self.max_distance == 1 {
                    return None;
                }

                self.x -= 1;
                self.y += 1;
                if self.x == 0 {
                    self.leg = 1;
                }
            },
            1 => {
                self.x -= 1;
                self.y -= 1;
                if self.y == 0 {
                    self.leg = 2;
                }
            },
            2 => {
                self.x += 1;
                self.y -= 1;
                if self.x == 0 {
                    self.leg = 3;
                }
            },
            3 => {
                self.x += 1;
                self.y += 1;
                if self.y == 0 {
                    self.x += 1;
                    self.leg = 0;
                    self.layer += 1;

                    if self.layer == self.max_distance {
                        return None
                    }
                }
            },
            _ => unreachable!()
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
            println!("");
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
            println!("");
        }
    }

    #[test]
    fn manhattan_bounds() {
        for size in 1..100 {
            let max_distance = (size + 1) as i32;
            for (x, y) in ManhattanIterator::new(0, 0, size) {
                let distance = x.abs() + y.abs();
                assert!(distance <= max_distance, "spiral was out of bounds: distance {}, size: {}, x: {}, y: {}", distance, size, x, y);
            }
        }
    }

    #[test]
    fn chebyshev_bounds() {
        for size in 1..100 {
            let max_distance = (size + 1) as i32;
            for (x, y) in ChebyshevIterator::new(0, 0, size) {
                let distance = std::cmp::max(x.abs(), y.abs());
                assert!(distance <= max_distance, "spiral was out of bounds: distance {}, size: {}, x: {}, y: {}", distance, size, x, y);
            }
        }
    }

    #[test]
    fn manhattan() {
        let expected: [i32; 3 * 3] = [
            0, 5, 0,
            4, 1, 2,
            0, 3, 0];

        let mut current = 0;
        for (x, y) in ManhattanIterator::new(1, 1, 2) {
            current += 1;

            let index = x + y * 3;

            assert_eq!(expected[index as usize], current);
        }

        let expected: [i32; 5 * 5] = [
             0,  0, 12,  0,  0,
             0, 11,  5, 13,  0,
            10,  4,  1,  2,  6,
             0,  9,  3,  7,  0,
             0,  0,  8,  0,  0];

        current = 0;
        for (x, y) in ManhattanIterator::new(2, 2, 3) {
            current += 1;

            let index = x + y * 5;

            assert_eq!(expected[index as usize], current);
        }
    }

    #[test]
    fn chebyshev() {
        let expected: [i32; 5 * 5] = [
            21, 22, 23, 24, 25,
            20,  7,  8,  9, 10,
            19,  6,  1,  2, 11,
            18,  5,  4,  3, 12,
            17, 16, 15, 14, 13];

        let mut current = 0;
        for (x, y) in ChebyshevIterator::new(2, 2, 3) {
            current += 1;

            let index = x + y * 5;

            assert_eq!(expected[index as usize], current);
        }
    }

}
