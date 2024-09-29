use std::ops::{Div, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(value: (f32, f32, f32)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl From<Vector4> for Vector3 {
    fn from(value: Vector4) -> Vector3 {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<Vector3> for Vector4 {
    fn from(value: Vector3) -> Vector4 {
        Self::from_xyz(value.x, value.y, value.z)
    }
}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from(value: (f32, f32, f32, f32)) -> Vector4 {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Mul<Vector4> for Vector4 {
    type Output = f32;

    fn mul(self, rhs: Vector4) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

impl Matrix4 {
    pub fn new(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Self {
        Self { x, y, z, w }
    }

    pub fn identity() -> Self {
        (
            (1.0, 0.0, 0.0, 0.0),
            (0.0, 1.0, 0.0, 0.0),
            (0.0, 0.0, 1.0, 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
            .into()
    }

    pub fn projection(aspect: f32, fov: f32, z_near: f32, z_far: f32) -> Self {
        let a = 1.0 / aspect;
        let f = 1.0 / (fov / 2.0).tan();
        let q = z_far / (z_far - z_near);

        (
            (a * f, 0.0, 0.0, 0.0),
            (0.0, f, 0.0, 0.0),
            (0.0, 0.0, q, -z_near * q),
            (0.0, 0.0, 1.0f32, 0.0),
        )
            .into()
    }

    pub fn rotation_x(angle: f32) -> Self {
        (
            (1.0, 0.0, 0.0, 0.0),
            (0.0, angle.cos(), angle.sin(), 0.0),
            (0.0, -angle.sin(), angle.cos(), 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
            .into()
    }
}

impl From<(Vector4, Vector4, Vector4, Vector4)> for Matrix4 {
    fn from(value: (Vector4, Vector4, Vector4, Vector4)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl
    From<(
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
    )> for Matrix4
{
    fn from(
        value: (
            (f32, f32, f32, f32),
            (f32, f32, f32, f32),
            (f32, f32, f32, f32),
            (f32, f32, f32, f32),
        ),
    ) -> Self {
        let matrix: (Vector4, Vector4, Vector4, Vector4) = (
            value.0.into(),
            value.1.into(),
            value.2.into(),
            value.3.into(),
        );

        matrix.into()
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        (
            (
                self.x * rhs.x,
                self.x * rhs.y,
                self.x * rhs.z,
                self.x * rhs.w,
            ),
            (
                self.y * rhs.x,
                self.y * rhs.y,
                self.y * rhs.z,
                self.y * rhs.w,
            ),
            (
                self.z * rhs.x,
                self.z * rhs.y,
                self.z * rhs.z,
                self.z * rhs.w,
            ),
            (
                self.w * rhs.x,
                self.w * rhs.y,
                self.w * rhs.z,
                self.w * rhs.w,
            ),
        )
            .into()
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        (self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs).into()
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector3) -> Self::Output {
        self * Vector4::from(rhs)
    }
}

#[test]
fn check_vector34_matrix() {
    let matrix = Matrix4::identity();
    let vector = Vector3::new(1.0, 2.0, 3.0);

    assert_eq!(Vector4::new(1.0, 2.0, 3.0, 1.0), matrix * vector);
}

#[test]
fn check_matrix_transform_identity() {
    let matrix = Matrix4::identity();
    let vector = Vector4::from_xyz(1.0, 2.0, 3.0);

    assert_eq!(vector, matrix * vector);
}

#[test]
fn check_matrix_transform() {
    let matrix: Matrix4 = (
        (1.0f32, 0.0, 0.0, 0.0),
        (0.0, 2.0f32, 0.0, 0.0),
        (0.0, 0.0, 3.0f32, 0.0),
        (0.0, 0.0, 0.0, 4.0f32),
    )
        .into();

    let vector = Vector4::new(1.0, 2.0, 3.0, 4.0);

    assert_eq!(Vector4::new(1.0, 4.0, 9.0, 16.0), matrix * vector);
}
