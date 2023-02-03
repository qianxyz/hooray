use crate::color::RGB;
use crate::vec3::{Point3, Vec3};

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

    pub fn color(&self) -> RGB {
        if self.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5) {
            return RGB::new(255, 0, 0);
        }

        let u = self.direction.unit();
        let t = 0.5 * (u.y() + 1.0);

        let r = (1.0 - t) * 1.0 + t * 0.5;
        let g = (1.0 - t) * 1.0 + t * 0.7;
        let b = 1.0;

        const M: f32 = 255.999;

        RGB::new((M * r) as u8, (M * g) as u8, (M * b) as u8)
    }

    fn hit_sphere(&self, center: Point3, radius: f32) -> bool {
        let oc = self.origin - center;

        let a = self.direction.dot(&self.direction);
        let b = 2.0 * oc.dot(&self.direction);
        let c = oc.dot(&oc) - radius * radius;

        let discr = b * b - 4.0 * a * c;

        discr > 0.0
    }
}
