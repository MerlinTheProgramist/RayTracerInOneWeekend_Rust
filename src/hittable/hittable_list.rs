use crate::ray::Ray;

use super::*;

pub struct HittableList {
    objects: Vec<Hittable>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }
}

impl HittableList {
    pub fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit = Option::<HitRecord>::None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(h) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = h.t;
                hit = Some(h);
            }
        }
        hit
    }
}
