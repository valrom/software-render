use crate::math::vectors::Vector2;

pub struct Buffer<T> {
    data: Vec<T>,
    size: Vector2<i32>,
}

impl<T: Copy + Clone> Buffer<T> {
    fn new(size: Vector2<i32>, init: T) -> Option<Self> {
        if size.x <= 0 || size.y <= 0 {
            return None;
        }

        Some(Self {
            data: vec![init; (size.x * size.y) as usize],
            size,
        })
    }

    fn set_pixel(&mut self, position: Vector2<i32>, value: T) {
        self.data[(position.x + position.y * self.size.x) as usize] = value;
    }

    fn get_pixel(&mut self, position: Vector2<i32>) -> T {
        self.data[(position.x + position.y * self.size.x) as usize]
    }

    fn clear(&mut self, value: T) {
        self.data.fill(value);
    }
}

#[test]
fn test_buffer_init() {
    let mut buffer = Buffer::new(Vector2::new(100, 100), 0u32).unwrap();

    buffer.set_pixel(Vector2::new(50, 50), 100);
}

#[test]
#[should_panic]
fn test_out_of_buffer() {
    let mut buffer = Buffer::new(Vector2::new(100, 100), 0u32).unwrap();

    buffer.set_pixel(Vector2::new(100, 100), 0);
}
