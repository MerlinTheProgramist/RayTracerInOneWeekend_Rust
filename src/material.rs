use std::sync::Arc;

use rand::random;

use crate::{color::Color, hittable::HitRecord, ray::Ray, texture::TextureType, vec3::*, Num};

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Arc<TextureType> },
    Metal { albedo: Color, fuzz: Num },
    Dielectric { ir: Num },
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Self::Lambertian {
            albedo: Arc::new(TextureType::new_solid(albedo)),
        }
    }

    pub fn new_lambertian_textured(albedo: Arc<TextureType>) -> Self {
        Self::Lambertian { albedo }
    }

    pub fn new_metal(albedo: Color, fuzz: Num) -> Self {
        Self::Metal { albedo, fuzz }
    }

    pub fn new_dielectric(ir: Num) -> Self {
        Self::Dielectric { ir }
    }

    fn reflectance(cosine: Num, ref_idx: Num) -> Num {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * Num::powi(1. - cosine, 5)
    }

    // Return attenuation Color, scattered Ray
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_sphere();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                Some((
                    albedo.value(rec.u, rec.v, &rec.p),
                    Ray::new_timed(rec.p, scatter_direction, *r_in.time()),
                ))
            }
            Self::Metal { albedo, fuzz } => {
                let reflected = reflect(&normalize(&r_in.direction()), &rec.normal);
                let scattered = Ray::new_timed(
                    rec.p,
                    reflected + *fuzz * Vec3::random_unit_sphere(),
                    *r_in.time(),
                );
                match dot(scattered.direction(), &rec.normal) > 0. {
                    false => None,
                    true => Some((*albedo, scattered)),
                }
            }
            Self::Dielectric { ir } => {
                let refraction_ratio = if rec.front_face { 1. / ir } else { *ir };

                let unit_direction = normalize(r_in.direction());
                let cos_theta = Num::min(dot(&-unit_direction, &rec.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract
                    || Self::reflectance(cos_theta, refraction_ratio) > random::<Num>()
                {
                    reflect(&unit_direction, &rec.normal)
                } else {
                    refract(&unit_direction, &rec.normal, refraction_ratio)
                };

                let scattered = Ray::new_timed(rec.p, direction, *r_in.time());
                Some((Color::new(1., 1., 1.), scattered))
            }
        }
    }
}
