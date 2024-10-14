use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Number<T>: Copy + Clone + Debug + num::Num + Neg<Output = T> {}

impl Number<f32> for f32 {}
impl Number<f64> for f64 {}

impl Number<i32> for i32 {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2<T: Number<T>> {
    pub x: T,
    pub y: T,
}

impl<T: Number<T>> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Number<T>> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Number<T>> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl<T: Number<T>> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T: Number<T>> Mul for Vector2<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Number<T>> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Number<T>> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3<T: Number<T>> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Number<T>> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl<T: Number<T>> Add for Vector3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Number<T>> Neg for Vector3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Number<T>> Sub for Vector3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T: Number<T>> Mul for Vector3<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: Number<T>> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: Number<T>> Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector4<T: Number<T>> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Number<T>> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: Number<T>> Mul for Vector4<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl<T: Number<T>> From<Vector4<T>> for Vector3<T> {
    fn from(value: Vector4<T>) -> Self {
        let div = if value.w.is_zero() { T::one() } else { value.w };

        Self::new(value.x / div, value.y / div, value.z / div)
    }
}

impl<T: Number<T>> From<Vector3<T>> for Vector4<T> {
    fn from(value: Vector3<T>) -> Self {
        Self::new(value.x, value.y, value.z, T::one())
    }
}

impl<T: Number<T>> From<Vector3<T>> for Vector2<T> {
    fn from(value: Vector3<T>) -> Self {
        Self::new(value.x, value.y)
    }
}

impl<T: Number<T>> From<Vector2<T>> for Vector3<T> {
    fn from(value: Vector2<T>) -> Self {
        Self::new(value.x, value.y, T::zero())
    }
}
