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

            distance: 1,
            dir: 0,
            i: 0
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
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
    fn it_works() {
        let spiral = SpiralIterator::new(5, 5, 10);
        for (x, y) in spiral {
            println!("S: {} {}", x, y);
        }

        assert_eq!(2 + 2, 4);
    }
}
