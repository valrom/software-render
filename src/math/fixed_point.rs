// 8 bit for part after point

use std::ops::{Add, Neg, Sub};

const AFTER_POINT_BITS: i32 = 8;
pub struct FixedPoint(i32);

impl From<f32> for FixedPoint {
    fn from(value: f32) -> Self {
        let trunc = value.trunc() as i32;
        let fract = value.fract();

        let trunc = trunc << AFTER_POINT_BITS;
        let fract = (fract * (1 << AFTER_POINT_BITS) as f32) as i32;

        FixedPoint(trunc + fract)
    }
}

impl Add<FixedPoint> for FixedPoint {
    type Output = Self;

    fn add(self, another: Self) -> Self {
        FixedPoint(self.0 + another.0)
    }
}

impl Neg for FixedPoint {
    type Output = Self;

    fn neg(self) -> Self {
        FixedPoint(-self.0)
    }
}

impl Sub<Self> for FixedPoint {
    type Output = Self;

    fn sub(self, another: Self) -> Self {
        self + -another
    }
}
