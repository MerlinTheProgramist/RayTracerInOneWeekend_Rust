use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material {
    // Return attenuation Color, scattered Ray
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub mod materials {

    use crate::{color::Color, ray::Ray, vec3::*, Num};
    use rand::Rng;

    pub struct Lambertian {
        albedo: Color,
    }

    pub struct Metal {
        albedo: Color,
        fuzz: Num,
    }

    pub struct Dielectric {
        ir: Num,
    }

    impl Lambertian {
        pub fn new(a: &Color) -> Lambertian {
            Lambertian { albedo: *a }
        }
    }
    impl Metal {
        pub fn new(a: &Color, f: Num) -> Metal {
            Metal {
                albedo: *a,
                fuzz: Num::min(f, 1.),
            }
        }
    }
    impl Dielectric {
        pub fn new(_ir: &Num) -> Dielectric {
            Dielectric { ir: *_ir }
        }
    }

    impl super::Material for Lambertian {
        fn scatter(
            &self,
            _r_in: &crate::ray::Ray,
            rec: &crate::hittable::HitRecord,
        ) -> Option<(Color, Ray)> {
            let mut scatter_direction = rec.normal + Vec3::random_unit_sphere();

            // Catch degenerate scatter direction
            if scatter_direction.near_zero() {
                scatter_direction = rec.normal;
            }

            Some((self.albedo, Ray::new(rec.p, scatter_direction)))
        }
    }

    impl super::Material for Metal {
        fn scatter(&self, r_in: &Ray, rec: &crate::hittable::HitRecord) -> Option<(Color, Ray)> {
            let reflected = reflect(&normalize(&r_in.direction()), &rec.normal);
            let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_unit_sphere());
            match dot(scattered.direction(), &rec.normal) > 0. {
                false => None,
                true => Some((self.albedo, scattered)),
            }
        }
    }
    impl super::Material for Dielectric {
        fn scatter(&self, r_in: &Ray, rec: &crate::hittable::HitRecord) -> Option<(Color, Ray)> {
            let mut rand = rand::thread_rng();

            let refraction_ratio = if rec.front_face {
                1. / self.ir
            } else {
                self.ir
            };

            let unit_direction = normalize(r_in.direction());
            let cos_theta = Num::min(dot(&-unit_direction, &rec.normal), 1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

            let cannot_refract = refraction_ratio * sin_theta > 1.0;
            let direction =
                if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand.gen::<Num>() {
                    reflect(&unit_direction, &rec.normal)
                } else {
                    refract(&unit_direction, &rec.normal, refraction_ratio)
                };

            let scattered = Ray::new(rec.p, direction);
            Some((Color::new(1., 1., 1.), scattered))
        }
    }

    fn reflectance(cosine: Num, ref_idx: Num) -> Num {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * Num::powi(1. - cosine, 5)
    }
}
