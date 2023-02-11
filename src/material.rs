//! Materials and their behavior when hit.

use crate::object::HitRecord;
use crate::random;
use crate::{Color, Ray, Vec3};

/// Returns the reflect vector from the surface with given normal.
///
/// WARN: The normal here must be a unit vector.
/// The incoming vector don't need to be of unit length,
/// and the returning vector will have the same length as it.
fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * normal.dot(&v) * normal
}

/// Given an incoming vector `v`, the surface normal,
/// and the ratio `n_in / n_out`, returns the refracted vector,
/// or `None` if total internal reflection occurs.
///
/// WARN: Both `v` and the normal must be unit vectors.
/// Also it may not work when `v` does not face against the normal.
fn refract(v: Vec3, normal: Vec3, ratio: f64) -> Option<Vec3> {
    let cos = (-v.dot(&normal)).min(1.0); // min for numeric stability
    let sin = (1.0 - cos * cos).sqrt();

    // cannot refract (total internal reflection)
    if ratio * sin > 1.0 {
        return None;
    }

    // component perpendicular to the normal
    let out_prep = ratio * (v + cos * normal);
    // parallel (abs for numeric stability)
    let out_para = -(1.0 - out_prep.length_squared()).abs().sqrt() * normal;

    Some(out_prep + out_para)
}

/// A collection of information when a child ray is scattered.
///
/// TODO: This struct is forced public; See doc for `HitRecord`.
pub struct Scattered {
    /// The color attenuation due to the surface color.
    pub(crate) attenuation: Color,

    /// The child ray itself.
    pub(crate) ray: Ray,
}

/// A material that can scatter incoming rays.
pub trait Material: Sync {
    /// Given a record of hit and the incoming ray itself,
    /// returns the scatter information (or `None` when it's absorbed).
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered>;
}

/// Surface with Lambertian diffuse, like matte.
pub struct Lambertian {
    /// The base color of the surface.
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        // NOTE: there are different ways to sample a scattered ray.
        // Here we choose the true Lambertian diffusion.
        let v = rec.normal + random::unit_vec();
        // caution: numeric stability
        let direction = if v.near_zero() { rec.normal } else { v };
        let ray = Ray::new(rec.p, direction);

        Some(Scattered {
            attenuation: self.albedo,
            ray,
        })
    }
}

/// Surface like metal that mostly reflects.
pub struct Metal {
    /// The base color of the surface.
    albedo: Color,

    /// The fuzziness of the metal. The closer it is to 0,
    /// the more it appears like a perfect mirror.
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0), // not too much fuzziness
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let ref_v = reflect(r_in.direction().unit(), rec.normal);
        // add fuzziness to the reflected direction
        let direction = ref_v + self.fuzz * random::subunit_vec();

        // it is possible that after adding fuzziness, the direction went down
        // under the surface, which means the surface absorbs the ray
        if direction.dot(&rec.normal) > 0.0 {
            Some(Scattered {
                attenuation: self.albedo,
                ray: Ray::new(rec.p, direction),
            })
        } else {
            None
        }
    }
}

/// Material that are transparent and refracts, like glass.
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    /// Even when refraction is possible,
    /// there is always a part of light that is reflected.
    ///
    /// Using Schlick's approximation, given the cosine of the incoming angle
    /// and the ratio of two RI (it doesn't matter which over which),
    /// returns the probability of reflection.
    fn reflectance(cosine: f64, ref_ratio: f64) -> f64 {
        let r0 = (1.0 - ref_ratio) / (1.0 + ref_ratio);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        // transparent, no reduction of color intensity
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ratio = if rec.is_front {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let v_in = r_in.direction().unit();

        let dir = match refract(v_in, rec.normal, ratio) {
            Some(v) => {
                // can refract theoretically, reflection coefficient test
                let cos = -v_in.dot(&rec.normal);
                if random::float() > Self::reflectance(cos, ratio) {
                    v // refracts
                } else {
                    reflect(v_in, rec.normal) // reflects instead
                }
            }
            None => reflect(v_in, rec.normal),
        };

        let ray = Ray::new(rec.p, dir);

        Some(Scattered { attenuation, ray })
    }
}
