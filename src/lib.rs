mod object;
mod ray;
mod vec3;

pub use object::{Sphere, World};
pub use ray::Ray;
pub use vec3::{Color, Point3, Vec3};

pub const INF: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub fn degree_to_radian(degree: f32) -> f32 {
    degree * PI / 180.0
}
