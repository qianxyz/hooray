use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Self {
        let vp_h = 2.0;
        let vp_w = aspect_ratio * vp_h;
        let foc = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(vp_w, 0.0, 0.0);
        let vertical = Vec3::new(0.0, vp_h, 0.0);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, foc),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
