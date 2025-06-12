use image::Rgb;

use crate::math::vectors::Vector3;

#[derive(Copy, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn to_u32(self) -> u32 {
        0x01000000 * ((self.a * 255.0).trunc() as u32 % 256)
            + 0x00010000 * ((self.r * 255.0).trunc() as u32 % 256)
            + 0x00000100 * ((self.g * 255.0).trunc() as u32 % 256)
            + 0x00000001 * ((self.b * 255.0).trunc() as u32 % 256)
    }

    pub fn scale(self, factor: f32) -> Self {
        Self {
            a: self.a * factor,
            b: self.b * factor,
            r: self.r * factor,
            g: self.g * factor,
        }
    }
}

impl From<Rgb<u8>> for Color {
    fn from(rgb: Rgb<u8>) -> Self {
        Self::from_rgb(
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
        )
    }
}

impl From<Color> for Vector3<f32> {
    fn from(color: Color) -> Self {
        Self::new(color.r, color.g, color.b)
    }
}
