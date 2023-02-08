use std::ops;

use crate::Vec3;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Color(Vec3);

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }

    pub fn to_bytes(self, samples_per_pixel: u32) -> [u8; 3] {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (self.0.x() * scale).clamp(0.0, 0.999) * 256.0;
        let g = (self.0.y() * scale).clamp(0.0, 0.999) * 256.0;
        let b = (self.0.z() * scale).clamp(0.0, 0.999) * 256.0;

        [r as u8, g as u8, b as u8]
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self * rhs.0)
    }
}

impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color(self.0 * rhs.0)
    }
}
