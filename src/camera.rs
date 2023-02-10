use crate::{Point3, Ray, Vec3};

pub struct Camera {
    /// The viewpoint.
    origin: Point3,

    /// The horizontal side of the viewport.
    horizontal: Vec3,

    /// The vertical side of the viewport.
    vertical: Vec3,

    /// The lower left corner of the viewport.
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(
        // Your viewpoint.
        look_from: Point3,

        // The point you are looking at.
        // Together with the viewpoint, we can determine the direction we are
        // looking to, thus the viewport plane is fixed...
        look_at: Point3,

        // ...But the viewport can rotate in this plane, so we need a "up"
        // direction. This `vup` doesn't need to be in the plane; It will
        // handle the projection, so most of the time you can just pass in
        // (0, 1, 0).
        vup: Vec3,

        // The vertical field-of-view, in degrees.
        vfov: f64,

        // The aspect ratio of the viewport. Normally this should be the same
        // as the aspect ratio of the image, but you can change it to
        // stretch the image.
        aspect_ratio: f64,
    ) -> Self {
        // the distance from the viewpoint to the viewport plane is set to 1.0
        let viewport_height = 2.0 * (vfov.to_radians() / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let look_to = (look_at - look_from).unit();
        let unit_horizontal = look_to.cross(&vup).unit();
        let unit_vertical = unit_horizontal.cross(&look_to);

        let horizontal = viewport_width * unit_horizontal;
        let vertical = viewport_height * unit_vertical;

        Self {
            origin: look_from,
            horizontal,
            vertical,
            lower_left_corner: look_from + look_to - horizontal / 2.0 - vertical / 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
