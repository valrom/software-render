use crate::math::vectors::Vector2;

pub struct Line {
    start: Vector2<i32>,
    end: Vector2<i32>,
    delta: Vector2<i32>,
    position: Vector2<i32>,
    increment: Vector2<i32>,
    is_x_main: bool,
    module: i32,
}

impl Line {
    pub fn new(start: Vector2<i32>, end: Vector2<i32>) -> Option<Self> {
        let (start, end) = if start.x < end.x {
            (start, end)
        } else {
            (end, start)
        };

        let delta = end - start;

        let is_x_main = delta.x.abs() >= delta.y.abs();

        Some(Self {
            start,
            end,
            delta,
            position: Vector2::new(0, 0),
            increment: Vector2::new(delta.x.signum(), delta.y.signum()),
            module: 0,
            is_x_main,
        })
    }
}

impl Iterator for Line {
    type Item = Vector2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.x.abs() >= self.delta.x.abs()
            || self.position.y.abs() >= self.delta.y.abs()
        {
            return None;
        }

        let output = self.position + self.start;

        if self.is_x_main {
            self.position.x += self.increment.x;

            self.module += self.delta.y.abs();

            if self.module >= self.delta.x.abs() {
                self.position.y += self.increment.y;
                self.module -= self.delta.x.abs();
            }
        } else {
            self.position.y += self.increment.y;

            self.module += self.delta.x.abs();

            if self.module > self.delta.y.abs() {
                self.position.x += self.increment.x;
                self.module %= self.delta.y.abs();
            }
        }

        Some(output)
    }
}
