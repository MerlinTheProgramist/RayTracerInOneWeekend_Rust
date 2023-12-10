use crate::{aabb::AABB, interval::Interval, material::*, ray::Ray, vec3::*, Num};

use self::{bvh_node::BvhNode, hittable_list::HittableList, sphere::Sphere};
pub mod bvh_node;
pub mod hittable_list;
pub mod sphere;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Box<Material>,
    pub t: Num,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(_p: Point3, _t: Num, _m: Box<Material>) -> HitRecord {
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
    fn bounding_box(&self) -> AABB;
}

pub enum HittableObject {
    Sphere(Sphere),
    BvhNode(BvhNode),
    HittableList(HittableList),
}
impl Hittable for HittableObject {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(r, ray_t),
            Self::BvhNode(n) => n.hit(r, ray_t),
            Self::HittableList(l) => l.hit(r, ray_t),
        }
    }
    fn bounding_box(&self) -> AABB {
        match self {
            Self::Sphere(s) => s.bounding_box(),
            Self::BvhNode(n) => n.bounding_box(),
            Self::HittableList(l) => l.bounding_box(),
        }
    }
}
