//! Materials and their behavior when hit.

use crate::object::HitRecord;
use crate::{Color, Ray, RngExt, Vec3};

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

/// For dielectric materials, even when refraction is possible,
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

/// A collection of information when a child ray is scattered.
pub(crate) struct Scattered {
    /// The color attenuation due to the surface color.
    pub(crate) attenuation: Color,

    /// The child ray itself.
    pub(crate) ray: Ray,
}

/// A material that can scatter incoming rays.
pub enum Material {
    /// Surface with Lambertian diffuse, like matte.
    Lambertian {
        /// The base color of the surface.
        albedo: Color,
    },

    /// Surface like metal that mostly reflects.
    Metal {
        /// The base color of the surface.
        albedo: Color,

        /// The fuzziness of the metal. The closer it is to 0,
        /// the more it appears like a perfect mirror.
        fuzz: f64,
    },

    /// Material that are transparent and refracts, like glass.
    Dielectric { refractive_index: f64 },
}

impl Material {
    pub fn lambertian(albedo: Color) -> Self {
        Self::Lambertian { albedo }
    }

    pub fn metal(albedo: Color, fuzz: f64) -> Self {
        Self::Metal {
            albedo,
            fuzz: fuzz.min(1.0), // not too much fuzziness
        }
    }

    pub fn dielectric(refractive_index: f64) -> Self {
        Self::Dielectric { refractive_index }
    }

    /// Given a record of hit and the incoming ray itself,
    /// returns the scatter information (or `None` when it's absorbed).
    pub(crate) fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut impl RngExt,
    ) -> Option<Scattered> {
        match *self {
            Self::Lambertian { albedo } => {
                // NOTE: there are different ways to sample a scattered ray.
                // Here we choose the true Lambertian diffusion.
                let v = rec.normal + rng.unit_vec();
                // caution: numeric stability
                let direction = if v.near_zero() { rec.normal } else { v };
                let ray = Ray::new(rec.p, direction);

                Some(Scattered {
                    attenuation: albedo,
                    ray,
                })
            }

            Self::Metal { albedo, fuzz } => {
                let ref_v = reflect(r_in.direction().unit(), rec.normal);
                // add fuzziness to the reflected direction
                let direction = ref_v + fuzz * rng.subunit_vec();

                // it is possible that after adding fuzziness, the direction went down
                // under the surface, which means the surface absorbs the ray
                if direction.dot(&rec.normal) > 0.0 {
                    Some(Scattered {
                        attenuation: albedo,
                        ray: Ray::new(rec.p, direction),
                    })
                } else {
                    None
                }
            }

            Self::Dielectric { refractive_index } => {
                // transparent, no reduction of color intensity
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let ratio = if rec.is_front {
                    1.0 / refractive_index
                } else {
                    refractive_index
                };

                let v_in = r_in.direction().unit();

                let dir = match refract(v_in, rec.normal, ratio) {
                    Some(v) => {
                        // can refract theoretically, reflection coefficient test
                        let cos = -v_in.dot(&rec.normal);
                        if rng.float() > reflectance(cos, ratio) {
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
    }
}
