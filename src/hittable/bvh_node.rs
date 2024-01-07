use rand::Rng;

use crate::{aabb::AABB, interval::Interval, ray::Ray};
use std::{cmp::Ordering, sync::Arc};

use super::{hittable_list::HittableList, Hittable, HittableObject};

pub struct BvhNode {
    left: Arc<HittableObject>,
    right: Arc<HittableObject>,
    bbox: AABB,
}

impl BvhNode {
    pub fn from_list(list: &mut HittableList) -> HittableObject {
        BvhNode::new(&mut list.objects)
    }

    pub fn new(objects: &mut [Arc<HittableObject>]) -> HittableObject {
        let (left, right);

        let axis = rand::thread_rng().gen_range(0..=2);
        match objects.len() {
            1 => {
                right = objects[0].clone();
                left = right.clone();
            }
            2 => {
                if Self::box_compare(&objects[0], &objects[1], axis).is_le() {
                    left = objects[0].clone();
                    right = objects[1].clone();
                } else {
                    left = objects[1].clone();
                    right = objects[0].clone();
                }
            }
            _ => {
                // more than 2 elements
                objects.sort_by(|a, b| Self::box_compare(a, b, axis));
                let mid = objects.len() / 2;
                left = Arc::new(Self::new(&mut objects[..mid]));
                right = Arc::new(Self::new(&mut objects[mid..]));
            }
        }

        let bbox = AABB::union(&left.bounding_box(), &right.bounding_box());
        HittableObject::BvhNode(Self { bbox, left, right })
    }

    fn box_compare(a: &Arc<HittableObject>, b: &Arc<HittableObject>, axis_index: i32) -> Ordering {
        assert!(0 <= axis_index && axis_index <= 2);
        b.bounding_box()
            .axis(axis_index)
            .min
            .partial_cmp(&a.bounding_box().axis(axis_index).min)
            .unwrap_or(Ordering::Greater)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<super::HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t);

        let right_t = match &hit_left {
            Some(hit) => hit.t,
            None => ray_t.max,
        };

        let hit_right = self.right.hit(r, Interval::new(ray_t.min, right_t));

        if hit_right.is_some() {
            hit_right
        } else if hit_left.is_some() {
            hit_left
        } else {
            None
        }
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
