use std::sync::Arc;

use crate::ray::Ray;
use crate::HittableObject;

use super::*;

pub struct HittableList {
    pub objects: Vec<Arc<HittableObject>>,
    bbox: AABB,
}

impl From<HittableObject> for HittableList {
    fn from(h: HittableObject) -> Self {
        Self {
            bbox: h.bounding_box(),
            objects: vec![Arc::new(h)],
        }
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add_ref(&mut self, object: Arc<HittableObject>) {
        self.bbox = AABB::union(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
    pub fn add(&mut self, object: HittableObject) {
        self.add_ref(Arc::new(object));
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
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

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
