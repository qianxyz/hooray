mod camera;
mod object;
mod ray;
mod vec3;

pub use camera::Camera;
pub use object::{Sphere, World};
pub use ray::Ray;
pub use vec3::{Color, Point3, Vec3};

pub const INF: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub fn degree_to_radian(degree: f32) -> f32 {
    degree * PI / 180.0
}

pub fn random_float() -> f32 {
    rand::random()
}

pub fn random_between(min: f32, max: f32) -> f32 {
    min + (max - min) * random_float()
}
