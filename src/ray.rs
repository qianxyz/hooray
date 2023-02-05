use crate::object::Hittable;
use crate::{Color, Point3, Vec3, World, INF};

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

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &World, depth: u32) -> Color {
        if depth == 0 {
            return Color::default();
        }

        if let Some(rec) = world.hit(self, 0.0, INF) {
            let target = rec.p() + rec.normal() + Vec3::random_in_unit_sphere();
            let child_ray = Ray::new(rec.p(), target - rec.p());
            return 0.5 * child_ray.color(&world, depth - 1);
        }

        let u = self.direction.unit();
        let t = 0.5 * (u.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
