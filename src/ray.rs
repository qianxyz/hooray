use crate::{Color, Point3, Vec3};

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

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn color(&self) -> Color {
        let center = Point3::new(0.0, 0.0, -1.0);
        let radius = 0.5;

        let t = self.hit_sphere(center, radius);
        if t > 0.0 {
            let n = (self.at(t) - center).unit();
            return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
        }

        let u = self.direction.unit();
        let t = 0.5 * (u.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    fn hit_sphere(&self, center: Point3, radius: f32) -> f32 {
        let oc = self.origin - center;

        let a = self.direction.length_squared();
        let half_b = oc.dot(&self.direction);
        let c = oc.length_squared() - radius * radius;

        let discr = half_b * half_b - a * c;

        if discr < 0.0 {
            -1.0
        } else {
            (-half_b - discr.sqrt()) / a
        }
    }
}
