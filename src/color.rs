//! RGB colors, with float values.
//!
//! Representing the RGB components as floats instead of bytes greatly
//! facilitates computation. Operators from `Vec3` is delegated; A particularly
//! interesting one is multiplying two colors to get another color, which is
//! useful in component-wise attenuation.

use std::ops;

use crate::Vec3;

/// A RGB color, with values as float numbers.
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Color(Vec3);

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }

    /// Converts a color to RGB bytes.
    pub fn to_bytes(self, samples_per_pixel: u32) -> [u8; 3] {
        let scale = 1.0 / samples_per_pixel as f64;

        // NOTE: sqrt is for gamma correction
        let r = (self.0.x() * scale).sqrt().clamp(0.0, 0.999) * 256.0;
        let g = (self.0.y() * scale).sqrt().clamp(0.0, 0.999) * 256.0;
        let b = (self.0.z() * scale).sqrt().clamp(0.0, 0.999) * 256.0;

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
