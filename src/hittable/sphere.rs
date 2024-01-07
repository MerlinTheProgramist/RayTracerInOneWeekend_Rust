use log::info;

use super::{HitRecord, Hittable, HittableObject};
use crate::{aabb::AABB, interval::Interval, material::Material, ray::Ray, vec3::*, Num};

pub struct Sphere {
    center0: Point3,
    radius: Num,
    mat: Box<Material>,
    is_moving: bool,
    center_vec: Vec3,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center0: Point3, r: Num, m: Box<Material>) -> HittableObject {
        let rvec = Vec3::new(r, r, r);
        HittableObject::Sphere(Sphere {
            center0,
            radius: r,
            mat: m,
            is_moving: false,
            center_vec: Vec3::ZERO,
            bbox: AABB::from_corners(center0 - rvec, center0 + rvec),
        })
    }
    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        r: Num,
        m: Box<Material>,
    ) -> HittableObject {
        let rvec = Vec3::new(r, r, r);
        let box0 = AABB::from_corners(center0 - rvec, center0 + rvec);
        let box1 = AABB::from_corners(center1 - rvec, center1 + rvec);

        HittableObject::Sphere(Sphere {
            center0,
            radius: r,
            mat: m,
            is_moving: true,
            center_vec: center1 - center0,
            bbox: AABB::union(&box0, &box1),
        })
    }

    fn center(&self, time: Num) -> Point3 {
        if self.is_moving {
            self.center0 + time * self.center_vec
        } else {
            self.center0
        }
    }

    fn get_uv(&self, p: &Point3) -> (Num, Num) {
        use std::f64::consts::PI;
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        let theta = (-p.y).acos();
        let phi = -p.z.atan2(p.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        info!("SPHERE HIT!!!");
        let oc = *r.origin() - self.center(*r.time());
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

        let p = r.at(root);
        let outward_normal = (p - self.center0) / self.radius; // normalized by dividing by radius
        let (u, v) = self.get_uv(&outward_normal);

        // save hit record
        let mut rec = HitRecord::new(p, root, self.mat.clone(), u, v);
        rec.set_face_normal(r, &outward_normal);

        return Some(rec);
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
