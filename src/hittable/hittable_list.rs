use crate::ray::Ray;

use super::*;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
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

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit = Option::<HitRecord>::None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(h) = &object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                hit = Some(h.clone());
                closest_so_far = h.t;
            }
        }
        hit
    }
}
