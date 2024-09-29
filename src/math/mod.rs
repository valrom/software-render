use std::ops::{Add, Neg, Sub};

pub mod fixed_point;
pub mod matrices;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VecF2 {
    pub x: f32,
    pub y: f32,
}

impl VecF2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[allow(dead_code)]
    pub fn scalar(self, another: Self) -> f32 {
        self.x * another.x + self.y * another.y
    }

    #[allow(dead_code)]
    pub fn cross(self, another: Self) -> f32 {
        self.x * another.y - self.y * another.x
    }

    #[allow(dead_code)]
    pub fn scale(self, factor: f32) -> Self {
        (self.x * factor, self.y * factor).into()
    }
}

impl From<(f32, f32)> for VecF2 {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Add<Self> for VecF2 {
    type Output = Self;

    fn add(self, another: Self) -> Self {
        (self.x + another.x, self.y + another.y).into()
    }
}

impl Sub<Self> for VecF2 {
    type Output = Self;

    fn sub(self, another: Self) -> Self {
        (self.x - another.x, self.y - another.y).into()
    }
}

impl Neg for VecF2 {
    type Output = Self;

    fn neg(self) -> Self {
        (-self.x, -self.y).into()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VecI2 {
    pub x: i32,
    pub y: i32,
}

impl VecI2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[allow(dead_code)]
    pub fn scalar(self, another: Self) -> i32 {
        self.x * another.x + self.y * another.y
    }

    pub fn cross(self, another: Self) -> f32 {
        self.x as f32 * another.y as f32 - self.y as f32 * another.x as f32
    }

    pub fn scale(self, factor: f32) -> Self {
        (
            (self.x as f32 * factor) as i32,
            (self.y as f32 * factor) as i32,
        )
            .into()
    }
}

impl From<(i32, i32)> for VecI2 {
    fn from(item: (i32, i32)) -> Self {
        Self::new(item.0, item.1)
    }
}

impl From<VecF2> for VecI2 {
    fn from(value: VecF2) -> Self {
        Self::new(value.x as i32, value.y as i32)
    }
}

impl Add for VecI2 {
    type Output = Self;

    fn add(self, another: Self) -> Self {
        Self {
            x: self.x + another.x,
            y: self.y + another.y,
        }
    }
}

impl Neg for VecI2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Sub for VecI2 {
    type Output = Self;

    fn sub(self, another: Self) -> Self {
        self + (-another)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RectI2 {
    start: VecI2,
    end: VecI2,
}

impl RectI2 {
    pub fn new(start: VecI2, end: VecI2) -> Option<Self> {
        let diff = end - start;

        if diff.x > 0 && diff.y > 0 {
            Some(Self { start, end })
        } else {
            None
        }
    }

    pub fn start(&self) -> VecI2 {
        self.start
    }

    pub fn end(&self) -> VecI2 {
        self.end
    }

    pub fn size(&self) -> VecI2 {
        self.end - self.start
    }
}

impl TryFrom<(VecI2, VecI2)> for RectI2 {
    type Error = ();

    fn try_from(value: (VecI2, VecI2)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1).ok_or(())
    }
}

impl TryFrom<((i32, i32), (i32, i32))> for RectI2 {
    type Error = ();

    fn try_from(value: ((i32, i32), (i32, i32))) -> Result<Self, Self::Error> {
        Self::new(value.0.into(), value.1.into()).ok_or(())
    }
}

impl IntoIterator for RectI2 {
    type Item = VecI2;
    type IntoIter = RectI2Iter;

    fn into_iter(self) -> Self::IntoIter {
        RectI2Iter::new(self)
    }
}

pub struct RectI2Iter {
    rect: RectI2,
    size: VecI2,

    index: i32,
}

impl RectI2Iter {
    pub fn new(rect: RectI2) -> Self {
        Self {
            rect,
            size: rect.size(),
            index: 0,
        }
    }

    pub fn rect(&self) -> RectI2 {
        self.rect
    }
}

impl Iterator for RectI2Iter {
    type Item = VecI2;

    fn next(&mut self) -> Option<VecI2> {
        if self.index > self.size.x * self.size.y {
            return None;
        }

        let start = self.rect.start();

        let output = VecI2::new(
            start.x + self.index % self.size.x,
            start.y + self.index / self.size.x,
        );

        self.index += 1;

        Some(output)
    }
}

#[test]
fn check_area_iterator() {
    let mut iter = RectI2::try_from(((0, 0), (100, 100))).unwrap().into_iter();
    for _ in 0..100 {
        iter.next();
    }

    assert_eq!(iter.next(), Some((0, 1).into()));
}

#[test]
#[should_panic]
fn check_area_iterator_fail() {
    let _ = RectI2::try_from(((10, 10), (9, 11))).unwrap();
}

#[test]
#[should_panic]
fn empty_area_iter() {
    let _ = RectI2::try_from(((0, 0), (0, 0))).unwrap();
}
