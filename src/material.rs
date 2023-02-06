use crate::{object::HitRecord, random_float, Color, Ray, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let v = rec.normal() + Vec3::random_unit();
        let scatter_direction = if v.near_zero() { rec.normal() } else { v };
        let scattered = Ray::new(rec.p(), scatter_direction);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().unit().reflect(&rec.normal());
        let scattered = Ray::new(
            rec.p(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        if scattered.direction().dot(&rec.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ratio = if rec.is_front() {
            1.0 / self.ir
        } else {
            self.ir
        };

        let u = r_in.direction().unit();

        let cos = (-u).dot(&rec.normal()).min(1.0);
        let sin = (1.0 - cos * cos).sqrt();
        let cannot_refract = ratio * sin > 1.0;

        let dir = if cannot_refract || Self::reflectance(cos, ratio) > random_float() {
            u.reflect(&rec.normal())
        } else {
            u.refract(&rec.normal(), ratio)
        };

        let scattered = Ray::new(rec.p(), dir);

        Some((attenuation, scattered))
    }
}
