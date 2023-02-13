//! Rays and their colors.
//!
//! To determine the color of each pixel in the output image, the camera sends
//! out rays from the viewer to points in the viewport, and asks for the color
//! of a ray, which is in turn determined by its interaction with the world.

use crate::object::Object;
use crate::{Color, Point3, RngExt, Vec3, World, INF};

/// A ray with an origin and a direction.
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// The point where the ray travels at time `t`.
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    /// The color of the ray, given an instance of `World`
    /// and a maximum recursion depth.
    pub fn color(&self, world: &World, depth: u32, rng: &mut impl RngExt) -> Color {
        // at max depth, return black
        if depth == 0 {
            return Color::default();
        }

        // if the ray hit something
        // here t_min is set to 0.001 to prevent shadow acne
        // (i.e. the ray hitting its origin on the surface at t=0)
        if let Some(rec) = world.hit_by(self, 0.001, INF) {
            if let Some(scattered) = rec.material.scatter(self, &rec, rng) {
                // if the ray scatters into a child ray,
                // returns the attenuated color of the child ray
                return scattered.attenuation * scattered.ray.color(world, depth - 1, rng);
            } else {
                // otherwise the ray is absorbed, return black
                return Color::default();
            };
        }

        // if the ray hit nothing, lerp between whith and blue
        // according to its y component, i.e. how high it aims
        //
        // TODO: can we make them static?
        let white = Color::new(1.0, 1.0, 1.0);
        let blue = Color::new(0.5, 0.7, 1.0);

        let u = self.direction.unit();
        let t = 0.5 * (u.y() + 1.0);

        (1.0 - t) * white + t * blue
    }
}
