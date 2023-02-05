mod camera;
mod object;
mod ray;
mod vec3;

pub use camera::Camera;
pub use object::{Sphere, World};
pub use ray::Ray;
pub use vec3::{Color, Point3, Vec3};

pub const INF: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degree_to_radian(degree: f64) -> f64 {
    degree * PI / 180.0
}

pub fn random_float() -> f64 {
    rand::random()
}

pub fn random_between(min: f64, max: f64) -> f64 {
    min + (max - min) * random_float()
}
