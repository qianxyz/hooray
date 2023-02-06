use crate::{Material, Point3, Ray, Vec3};

pub struct HitRecord<'a> {
    p: Point3,
    normal: Vec3,
    material: &'a dyn Material,
    t: f64,
    is_front: bool,
}

impl HitRecord<'_> {
    pub fn p(&self) -> Point3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn material(&self) -> &dyn Material {
        self.material
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: impl Material + 'static) -> Self {
        Self {
            center,
            radius,
            material: Box::new(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

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
        let normal = if is_front { out_normal } else { -out_normal };

        Some(HitRecord {
            p,
            t: root,
            material: &*self.material,
            normal,
            is_front,
        })
    }
}

#[derive(Default)]
pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|obj| obj.hit(ray, t_min, t_max))
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}
