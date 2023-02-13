//! Random utilities.

use rand::Rng;

use crate::{Color, Vec3};

/// Our extended Rng trait, with vector utilities.
pub trait RngExt: Rng {
    /// Returns a random float in [0, 1).
    fn float(&mut self) -> f64 {
        self.gen()
    }

    /// Returns a random float in [min, max).
    fn float_between(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.float()
    }

    /// Returns a random `Vec3` in the unit ball, i.e. with length < 1.
    fn subunit_vec(&mut self) -> Vec3 {
        loop {
            let x = self.float_between(-1.0, 1.0);
            let y = self.float_between(-1.0, 1.0);
            let z = self.float_between(-1.0, 1.0);
            let v = Vec3::new(x, y, z);

            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    /// Returns a random `Vec3` with unit length.
    fn unit_vec(&mut self) -> Vec3 {
        self.subunit_vec().unit()
    }

    /// Returns a random `Vec3` inside the hemisphere centered by `n`,
    /// i.e. their dot product >= 1.
    fn vec_in_hemisphere(&mut self, n: Vec3) -> Vec3 {
        let v = self.subunit_vec();

        if n.dot(&v) > 0.0 {
            v
        } else {
            -v
        }
    }

    /// Returns a random color.
    fn color(&mut self) -> Color {
        Color::new(self.float(), self.float(), self.float())
    }

    /// Returns a random color with each RGB component in [min, max).
    fn color_between(&mut self, min: f64, max: f64) -> Color {
        Color::new(
            self.float_between(min, max),
            self.float_between(min, max),
            self.float_between(min, max),
        )
    }
}

impl<T: Rng> RngExt for T {}
