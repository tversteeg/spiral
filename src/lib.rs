pub struct SpiralIterator {
    max_distance: u32,
    distance: u32,

    start_x: i32,
    start_y: i32,
    x: i32,
    y: i32,
}

impl SpiralIterator {
    fn new(x: i32, y: i32, max_distance: u32) -> Self {
        SpiralIterator {
            max_distance: max_distance,
            start_x: x,
            start_y: y,

            x: x,
            y: y,
            distance: 1
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        if self.distance == self.max_distance {
            return None;
        }

        Some((self.x, self.y))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
