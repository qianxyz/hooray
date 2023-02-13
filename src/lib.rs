mod camera;
mod color;
mod material;
mod object;
mod point3;
mod random;
mod ray;
mod vec3;

// re-exports
pub use camera::Camera;
pub use color::Color;
pub use material::Material;
pub use object::{Sphere, World};
pub use point3::Point3;
pub use random::RngExt;
pub use ray::Ray;
pub use vec3::Vec3;

pub const INF: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
