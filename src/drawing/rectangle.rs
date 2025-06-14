use crate::math::vectors::Vector2;

pub struct Rect {
    start: Vector2<i32>,
    end: Vector2<i32>,
    position: i32,
    position_end: i32,
    delta: Vector2<i32>,
}

impl Rect {
    pub fn new(start: Vector2<i32>, end: Vector2<i32>) -> Option<Self> {
        let delta = end - start;

        if delta.x < 1 || delta.y < 1 {
            return None;
        }

        Some(Self {
            start,
            end,
            position: 0,
            position_end: delta.x * delta.y,
            delta,
        })
    }
}

impl Iterator for Rect {
    type Item = Vector2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.position_end {
            return None;
        }

        let coords = Vector2::new(self.position % self.delta.y, self.position / self.delta.y);

        self.position += 1;

        Some(self.start + coords)
    }
}
