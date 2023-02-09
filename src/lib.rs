mod camera;
mod color;
mod material;
mod object;
mod point3;
mod ray;
mod vec3;

// random utilities need public
pub mod random;

// re-exports
pub use camera::Camera;
pub use color::Color;
pub use material::{Dielectric, Lambertian, Metal};
pub use object::{Sphere, World};
pub use point3::Point3;
pub use ray::Ray;
pub use vec3::Vec3;

pub const INF: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degree_to_radian(degree: f64) -> f64 {
    degree * PI / 180.0
}
