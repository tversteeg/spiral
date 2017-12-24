#![crate_name = "spiral"]

//! Iterators to iterate 2D structures in a spiral pattern

pub enum Spiral {
    Euclidean(EuclideanIterator),
    Manhattan(ManhattanIterator),
    Chebyshev(ChebyshevIterator)
}

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
    ///     // Iterates over 9x9 2D array with 'x' & 'y' following this pattern:
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

pub struct ManhattanIterator {
    max_distance: i32,
    start_x: i32,
    start_y: i32,

    distance: i32,
    i: i32,
    dir: u8,
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
    ///     // Iterate over 2D array with 'x' & 'y'
    /// }
    /// ```
    pub fn new(x: i32, y: i32, max_distance: u16) -> Self {
        ManhattanIterator {
            max_distance: max_distance as i32,
            start_x: x,
            start_y: y,

            distance: 0,
            dir: 0,
            i: 0
        }
    }
}

impl Iterator for ManhattanIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        if self.distance == 0 {
            self.distance += 1;

            return Some((self.start_x, self.start_y));
        }

        let pos = match self.dir {
            0 => (self.start_x + self.distance - self.i, self.start_y + self.i),
            2 => (self.start_x - self.distance + self.i, self.start_y - self.i),
            1 => (self.start_x, self.start_y + self.distance),
            3 => (self.start_x, self.start_y - self.distance),
            _ => return None
        };

        self.dir += 1;
        if self.dir > 3 {
            self.dir = 0;

            self.i += 1;
            if self.i == self.distance {
                self.i = 0;

                self.distance += 1;
            }
        }

        if self.distance >= self.max_distance {
            return None;
        }

        Some(pos)
    }
}

pub struct EuclideanIterator {
    max_distance: i32,
    start_x: i32,
    start_y: i32,

    x: i32,
    y: i32,
    length: i32,
    dir: u8,

    y2_new: i32,
    y2: i32,
    ty: i32
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
        // Approximate cos pi/4 by 185363 / 2 ^ 18
        let length = (max_distance as i32 * 185363) >> 18;

        let diameter = (max_distance * max_distance) as i32;

        EuclideanIterator {
            max_distance: max_distance as i32,
            start_x: x,
            start_y: y,

            x: 0,
            y: max_distance as i32 - 1,
            length: length,
            dir: 0,

            y2: diameter,
            y2_new: 0,
            ty: (max_distance as i32 * 2) - 1
        }
    }
}

impl Iterator for EuclideanIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        let pos = match self.dir {
            0 => (self.x, self.y),
            1 => (self.x, -self.y),
            2 => (-self.x, self.y),
            3 => (-self.x, -self.y),

            4 => (self.y, self.x),
            5 => (self.y, -self.x),
            6 => (-self.y, self.x),
            7 => (-self.y, -self.x),
            _ => return None
        };

        self.dir += 1;
        if self.dir >= 8 {
            self.dir = 0;

            self.x += 1;
            if self.x > self.length {
                return None;
            }

            self.y2_new -= (self.x * 2) - 3;

            if self.y2 - self.y2_new >= self.ty {
                self.y2 -= self.ty;
                self.y -= 1;
                self.ty -= 2;
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
    fn small() {
        let expected: [i32; 5 * 5] = [
             0,  0, 10,  0,  0,
             0,  9,  2,  6,  0,
            13,  5,  1,  3, 11,
             0,  8,  4,  7,  0,
             0,  0, 12,  0,  0];

        let mut current = 0;
        let spiral = ManhattanIterator::new(2, 2, 1);
        for (x, y) in spiral {
            current += 1;

            let index = x + y * 5;

            assert_eq!(expected[index as usize], current);
        }
    }
}
