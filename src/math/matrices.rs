use std::ops::Mul;

use super::vectors::Vector4;
use super::vectors::{Number, Vector2};

#[derive(Copy, Clone, Debug)]
pub struct Matrix4<T: Number<T>> {
    pub x: Vector4<T>,
    pub y: Vector4<T>,
    pub z: Vector4<T>,
    pub w: Vector4<T>,
}

impl<T: Number<T>> Mul for Matrix4<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            Vector4::new(
                self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x * self.x.w * rhs.w.x,
                self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
                self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
                self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w + self.x.w * rhs.w.w,
            ),
            Vector4::new(
                self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x * self.y.w * rhs.w.x,
                self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
                self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z + self.y.w * rhs.w.z,
                self.y.x * rhs.x.w + self.y.y * rhs.y.w + self.y.z * rhs.z.w + self.y.w * rhs.w.w,
            ),
            Vector4::new(
                self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x * self.z.w * rhs.w.x,
                self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y + self.z.w * rhs.w.y,
                self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z + self.z.w * rhs.w.z,
                self.z.x * rhs.x.w + self.z.y * rhs.y.w + self.z.z * rhs.z.w + self.z.w * rhs.w.w,
            ),
            Vector4::new(
                self.w.x * rhs.x.x + self.w.y * rhs.y.x + self.w.z * rhs.z.x * self.w.w * rhs.w.x,
                self.w.x * rhs.x.y + self.w.y * rhs.y.y + self.w.z * rhs.z.y + self.w.w * rhs.w.y,
                self.w.x * rhs.x.z + self.w.y * rhs.y.z + self.w.z * rhs.z.z + self.w.w * rhs.w.z,
                self.w.x * rhs.x.w + self.w.y * rhs.y.w + self.w.z * rhs.z.w + self.w.w * rhs.w.w,
            ),
        )
    }
}

impl<T: Number<T>> Mul<Vector4<T>> for Matrix4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        Vector4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl<T: Number<T>> Matrix4<T> {
    pub fn new(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
        Self { x, y, z, w }
    }

    pub fn identity() -> Self {
        Self::new(
            Vector4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vector4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }
}

impl Matrix4<f32> {
    pub fn projection(aspect: f32, fov: f32, z_near: f32, z_far: f32) -> Self {
        let a = 1.0 / aspect;
        let f = 1.0 / (fov / 2.0).tan();
        let q = z_far / (z_far - z_near);

        Self::new(
            Vector4::new(a * f, 0.0, 0.0, 0.0),
            Vector4::new(0.0, f, 0.0, 0.0),
            Vector4::new(0.0, 0.0, q, z_near * q),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
        )
    }

    pub fn rotation_x(angle: f32) -> Self {
        Self::new(
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, angle.cos(), -angle.sin(), 0.0),
            Vector4::new(0.0, angle.sin(), angle.cos(), 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn viewport(size: Vector2<i32>) -> Self {
        Self::new(
            Vector4::new(size.x as f32 / 2.0, 0.0, 0.0, size.x as f32 / 2.0),
            Vector4::new(0.0, -size.y as f32 / 2.0, 0.0, size.y as f32 / 2.0),
            Vector4::new(0.0, 0.0, 0.5, 0.5),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        )
    }
}

#[test]
fn test_polygon() {
    let projection = Matrix4::<f32>::projection(1.0, 3.14 / 2.0, 0.1, 100.0);
    let mut look = Matrix4::<f32>::identity();
    look.z.w = -3.0;

    let point = Vector4::new(1.0, 1.0, 0.0, 1.0);

    let first = (projection * look) * point;
    let second = projection * (look * point);

    assert_eq!(first, second);
}

#[test]
fn projection_test() {
    let projection = Matrix4::<f32>::projection(1.0, 3.14 / 2.0, 1.0, 100.0);

    let point = Vector4::new(1.0, 1.0, -2.0, 1.0);

    let projected = projection * point;

    assert_eq!(projected.z / projected.w, -0.5);
}
