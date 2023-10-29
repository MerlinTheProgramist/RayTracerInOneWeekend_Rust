use std::rc::Rc;

use super::{HitRecord, Hittable};
use crate::{interval::Interval, material::Material, ray::Ray, vec3::*, Num};

pub struct Sphere {
    center: Point3,
    radius: Num,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3, r: Num, m: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: c,
            radius: r,
            mat: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = r.direction().lenght_sqr();
        let half_b = dot(&oc, r.direction());
        let c = oc.lenght_sqr() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0 as Num {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root <= ray_t.min || root >= ray_t.max {
            root = (-half_b + sqrtd) / a;
            if root <= ray_t.min || root >= ray_t.max {
                return None;
            }
        }

        // save hit record
        let mut rec = HitRecord::new(r.at(root), root, self.mat.clone());
        let outward_normal = (rec.p - self.center) / self.radius; // normalized by dividing by radius
        rec.set_face_normal(r, &outward_normal);

        return Some(rec);
    }
}
