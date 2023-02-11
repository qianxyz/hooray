//! Random utilities.

// TODO: seeding

// use std::sync::Mutex;
//
// use once_cell::sync::Lazy;
// use rand::{self, rngs::StdRng, Rng, SeedableRng};

use crate::{Color, Vec3};

// TODO: StdRng or SmallRng
// static SEED: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(StdRng::seed_from_u64(42)));

/// Returns a random float in [0, 1).
pub fn float() -> f64 {
    rand::random()
    // SEED.lock().unwrap().gen()
}

/// Returns a random float in [min, max).
pub fn float_between(min: f64, max: f64) -> f64 {
    min + (max - min) * float()
}

/// Returns a random `Vec3` in the unit ball, i.e. with length < 1.
pub fn subunit_vec() -> Vec3 {
    loop {
        let x = float_between(-1.0, 1.0);
        let y = float_between(-1.0, 1.0);
        let z = float_between(-1.0, 1.0);
        let v = Vec3::new(x, y, z);

        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

/// Returns a random `Vec3` with unit length.
pub fn unit_vec() -> Vec3 {
    subunit_vec().unit()
}

/// Returns a random `Vec3` inside the hemisphere centered by `n`,
/// i.e. their dot product >= 1.
pub fn vec_in_hemisphere(n: Vec3) -> Vec3 {
    let v = subunit_vec();

    if n.dot(&v) > 0.0 {
        v
    } else {
        -v
    }
}

/// Returns a random color.
pub fn color() -> Color {
    Color::new(float(), float(), float())
}

/// Returns a random color with each RGB component in [min, max).
pub fn color_between(min: f64, max: f64) -> Color {
    Color::new(
        float_between(min, max),
        float_between(min, max),
        float_between(min, max),
    )
}
