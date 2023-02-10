//! A portable camera with aperture simulation.

use crate::{random, Point3, Ray, Vec3};

pub struct Camera {
    /// The viewpoint.
    origin: Point3,

    /// The horizontal side of the viewport.
    horizontal: Vec3,

    /// The vertical side of the viewport.
    vertical: Vec3,

    /// The lower left corner of the viewport.
    lower_left_corner: Point3,

    /// The radius of the lens.
    /// The larger it is, the shallower DoF will be.
    /// With 0 radius it is an ideal pinhole camera.
    lens_radius: f64,
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
        // direction. This `vup` doesn't need to be in the plane; The code
        // handle the projection, so most of the time you can just pass in
        // (0, 1, 0).
        vup: Vec3,

        // The vertical field-of-view, in degrees.
        vfov: f64,

        // The aspect ratio of the viewport. Normally this should be the same
        // as the aspect ratio of the image, but you can change it to
        // stretch the image.
        aspect_ratio: f64,

        // The aperture of the camera. The larger it is, the shallower
        // the DoF will be. Set it to 0 for an ideal camera with no blurring.
        aperture: f64,

        // The focus distance of the camera, i.e. if you travel such distance
        // from the viewpoint to the direction you are looking to, the object
        // at that point will be perfectly clear no matter the aperture.
        // Normally this should be set to the distance between `look_from`
        // and `look_at` for a clear view in the middle.
        focus_dist: f64,
    ) -> Self {
        // create virtual viewport at the focus distance
        let viewport_height = 2.0 * (vfov.to_radians() / 2.0).tan() * focus_dist;
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
            lower_left_corner: look_from + focus_dist * look_to - horizontal / 2.0 - vertical / 2.0,
            lens_radius: aperture / 2.0,
        }
    }

    /// Gets the ray passing through a point in the viewport,
    /// specified by its relative width and height.
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        // sample offset vector parallel to the viewport plane
        let offset = loop {
            // sample from the unit disc with rejection method
            let x = random::float_between(-1.0, 1.0);
            let y = random::float_between(-1.0, 1.0);
            if x * x + y * y < 1.0 {
                break (x * self.horizontal.unit() + y * self.vertical.unit()) * self.lens_radius;
            }
        };

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
