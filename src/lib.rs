pub struct SpiralIterator {
    max_distance: u32,
    start_x: i32,
    start_y: i32,

    distance: u32,
    i: u32,
    dir: u8,
}

impl SpiralIterator {
    fn new(x: i32, y: i32, max_distance: u32) -> Self {
        SpiralIterator {
            max_distance: max_distance,
            start_x: x,
            start_y: y,

            distance: 0,
            dir: 3,
            i: 0
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        let directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

        let pos = match self.dir {
            0 => (self.start_x - self.distance as i32 + self.i as i32, self.start_y - self.i as i32),
            1 => (self.start_x + self.distance as i32 - self.i as i32, self.start_y + self.i as i32),
            2 => (self.start_x - self.i as i32, self.start_y + self.distance as i32 - self.i as i32),
            3 => (self.start_x + self.distance as i32 - self.i as i32, self.start_y - self.i as i32),
            _ => (0, 0)
        };

        self.dir += 1;
        if self.dir > 3 {
            self.dir = 0;

            self.i += 1;
            if self.i == self.distance + 1 {
                self.i = 0;

                self.distance += 1;
            }
        }

        if self.distance == self.max_distance {
            return None;
        }

        Some(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::SpiralIterator;

    #[test]
    fn output() {
        const SIZE: usize = 7;
        let mut output: [i32; SIZE * SIZE] = [0; SIZE * SIZE];

        let mut current = 0;
        let spiral = SpiralIterator::new(3, 3, 4);
        for (x, y) in spiral {
            current += 1;

            let index = x as usize + (y as usize * SIZE);
            output[index] = current;
        }

        for y in 0..SIZE {
            for x in 0..SIZE {
                let index = x as usize + (y as usize * SIZE);
                let output_val = output[index];

                if output_val > 0 {
                    print!("{:3} ", output_val);
                }else{
                    print!("    ");
                }
            }
            println!("");
        }
    }

    /*
    #[test]
    fn small() {
        let expected: [i32; 5 * 5] = [
             0,  0, 10,  0,  0,
             0,  9,  2,  6,  0,
            13,  5,  1,  3, 11,
             0,  8,  4,  7,  0,
             0,  0, 12,  0,  0];

        let mut current = 0;
        let spiral = SpiralIterator::new(2, 2, 1);
        for (x, y) in spiral {
            current += 1;

            let index = x + y * 5;

            assert_eq!(expected[index as usize], current);
        }
    }
    */
}
