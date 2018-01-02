#![crate_name = "spiral"]

//! Iterators to iterate 2D structures in a spiral pattern

pub enum Spiral {
    Euclidean(EuclideanIterator),
    Manhattan(ManhattanIterator),
    Chebyshev(ChebyshevIterator)
}

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
            _ => return None
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
            _ => return None
        }

        Some((self.start_x + self.x, self.start_y + self.y))
    }
}

/// An iterator iterating in a spiral fashion with the Euclidean distance function.
///
/// The distance function is defined as:
///
/// `distance = sqrt((absolute x offset from center ^ 2) + (absolute y offset from center ^ 2))`.
///
/// This creates a diamond-shaped spiral.
pub struct EuclideanIterator {
    start_x: i32,
    start_y: i32,

    i: usize,
    dir: u8,

    lut: Vec<(i32, i32)>,
}

impl EuclideanIterator {
    #[allow(dead_code)]
    /// Create a new iterator using the Euclidean distance function.
    ///
    /// # Arguments
    ///
    /// * `x` - The x position of the center of the spiral
    /// * `y` - The y position of the center of the spiral
    /// * `max_distance` - The radius of the spiral
    ///
    /// # Example
    /// ```
    /// use spiral::EuclideanIterator;
    ///
    /// let spiral = EuclideanIterator::new(3, 3, 4);
    /// for (x, y) in spiral {
    ///     // Iterate over 2D array with 'x' & 'y'
    /// }
    /// ```
    pub fn new(x: i32, y: i32, max_distance: u16) -> Self {
        EuclideanIterator {
            start_x: x,
            start_y: y,

            i: 0,
            dir: 7,
            lut: EuclideanIterator::build_lut_table(max_distance as i32)
        }
    }

    fn build_lut_table(max_distance: i32) -> Vec<(i32, i32)> {
        let mut lut = Vec::new();

        lut.push((0, 0));

        for y in 0 .. max_distance {
            for x in 0 .. y + 1 {
                lut.push((x, y));
            }
        }

        lut
    }
}

impl Iterator for EuclideanIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        let x = self.lut[self.i].0;
        let y = self.lut[self.i].1;

        let pos = match self.dir {
            0 => (x, y),
            1 => (x, -y),
            2 => (-x, y),
            3 => (-x, -y),

            4 => (y, x),
            5 => (y, -x),
            6 => (-y, x),
            7 => (-y, -x),
            _ => return None
        };

        self.dir += 1;
        if self.dir >= 8 {
            self.dir = 0;

            self.i += 1;
            if self.i >= self.lut.len() {
                return None;
            }
        }

        Some((self.start_x + pos.0, self.start_y + pos.1))
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

        println!("Euclidean");
        output = [0; SIZE * SIZE];

        current = 0;
        let spiral = EuclideanIterator::new(3, 3, 4);
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
    fn manhattan() {
        let expected: [i32; 5 * 5] = [
             0,  0, 12,  0,  0,
             0, 11,  5, 13,  0,
            10,  4,  1,  2,  6,
             0,  9,  3,  7,  0,
             0,  0,  8,  0,  0];

        let mut current = 0;
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
