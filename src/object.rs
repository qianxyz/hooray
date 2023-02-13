//! Objects and world.

use crate::{Material, Point3, Ray, Vec3};

/// A collection of information when a ray hits an object.
///
/// TODO: This struct is forced to be public due to the following pattern:
/// - `World::add` is public (so that we can add objects!);
/// - So its parameter, with type `impl Object`, must be public;
/// - So the trait `Object` must be public;
/// - So the return type of `Object::hit_by` must be public.
///
/// However this type is just a hodge-podge of information intended to be
/// kept private, and a construction method is deliberately left out
/// to emphasize such intention.
///
/// In the future, there may be 2 ways to solve this problem:
/// - Keep this struct public, stablize its fields, provide a constructor.
///   This allows user extension of custom objects outside the crate.
/// - Change `Object` into a tagged enum, provide constructors for each
///   variant, but keep `hit_by` private. This sacrifices extensibility
///   while making the API extremely clean and easy.
///
/// A parallel situation exists for the `Scattered` type in `Material`.
pub struct HitRecord<'a> {
    /// The travel time of the incoming ray at the hit moment.
    t: f64,

    /// The hit point.
    pub(crate) p: Point3,

    /// The normal vector of the hit surface at the hit point.
    ///
    /// When there are two normals (inward and outward), we always store
    /// the one that points against the incoming ray.
    pub(crate) normal: Vec3,

    /// Whether the ray hit the object at its front face.
    pub(crate) is_front: bool,

    /// The material of the hit object.
    pub(crate) material: &'a Material,
}

/// An object that can be hit by a ray.
pub trait Object: Sync + Send {
    /// Given an incoming ray and a time interval, returns if there is a hit.
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Object for Sphere {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

        // quadratic equation coefficients
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discr = half_b * half_b - a * c;
        if discr < 0.0 {
            return None;
        }
        let sqrtd = discr.sqrt();

        // find the smaller root in the range
        let root = match ((-half_b - sqrtd) / a, (-half_b + sqrtd) / a) {
            (t, _) if t > t_min && t < t_max => t,
            (_, t) if t > t_min && t < t_max => t,
            _ => return None,
        };

        let p = ray.at(root);
        let out_normal = (p - self.center) / self.radius;
        let is_front = ray.direction().dot(&out_normal) < 0.0;
        // the normal points against the incoming ray
        let normal = if is_front { out_normal } else { -out_normal };

        Some(HitRecord {
            t: root,
            p,
            normal,
            is_front,
            material: &self.material,
        })
    }
}

/// A world that contains many objects.
#[derive(Default)]
pub struct World {
    objects: Vec<Box<dyn Object>>,
}

impl World {
    /// Creates an empty world.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an object to the world.
    pub fn add(&mut self, object: impl Object + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Object for World {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|obj| obj.hit_by(ray, t_min, t_max))
            // get the first hit with smallest `t`
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}
