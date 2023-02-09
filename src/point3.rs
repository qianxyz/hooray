//! Points in 3D and affine properties.
//!
//! Note that a point differs from a vector in its arithmetic capability:
//! For example, adding two vectors result in another vector, but adding
//! two points doesn't make sense mathematically.
//!
//! More specifically, a point supports the following operations:
//! - Adding a vector to a point yields a point (commutatively)
//! - Subtracting a vector from a point yields a point
//! - Subtracting a point from a point yields a vector
//!
//! As an implementation detail, we use the NewType pattern and delegates its
//! operator overloading methods to ensure algebraic safety.

use std::ops;

use crate::Vec3;

/// A point in 3D space.
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Point3(Vec3);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl ops::Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point3(self.0 + rhs)
    }
}

impl ops::Add<Point3> for Vec3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self::Output {
        rhs + self
    }
}

impl ops::Sub for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl ops::Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self + (-rhs)
    }
}
