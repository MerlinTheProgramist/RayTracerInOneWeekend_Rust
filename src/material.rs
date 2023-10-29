use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material {
    // Return attenuation Color, scattered Ray
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub mod materials {

    use crate::{color::Color, ray::Ray, vec3::*, Num};

    pub struct Lambertian {
        albedo: Color,
    }

    pub struct Metal {
        albedo: Color,
        fuzz: Num,
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
}
