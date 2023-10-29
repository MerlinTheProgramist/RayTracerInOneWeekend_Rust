use std::rc::Rc;

use crate::{interval::Interval, material::*, ray::Ray, vec3::*, Num};

pub mod hittable_list;
pub mod sphere;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: Num,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(_p: Point3, _t: Num, _m: Rc<dyn Material>) -> HitRecord {
        HitRecord {
            p: _p,
            normal: Vec3 {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            mat: _m,
            t: _t,
            front_face: true,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0f64;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
