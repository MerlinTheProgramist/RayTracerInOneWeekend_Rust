use std::mem::swap;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::Num;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn default() -> Self {
        Self {
            x: Interval::default(),
            y: Interval::default(),
            z: Interval::default(),
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        println!("new bbox: {:?}", Self { x, y, z });
        Self { x, y, z }
    }

    pub fn padded(&self) -> Self {
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta {
            self.x
        } else {
            self.x.expand(delta)
        };
        let new_y = if self.y.size() >= delta {
            self.y
        } else {
            self.y.expand(delta)
        };
        let new_z = if self.z.size() >= delta {
            self.z
        } else {
            self.z.expand(delta)
        };
        Self::new(new_x, new_y, new_z)
    }

    pub fn union(a: &Self, b: &Self) -> Self {
        Self::new(
            Interval::union(&a.x, &b.x),
            Interval::union(&a.y, &b.y),
            Interval::union(&a.z, &b.z),
        )
    }

    pub fn from_corners(a: Point3, b: Point3) -> Self {
        Self {
            x: Interval::new(Num::min(a.x, b.x), Num::max(a.x, b.x)),
            y: Interval::new(Num::min(a.y, b.y), Num::max(a.y, b.y)),
            z: Interval::new(Num::min(a.z, b.z), Num::max(a.z, b.z)),
        }
    }

    pub fn axis(&self, n: i32) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        // println!(
        //     "trying to hit ({:?})-({:?}) ({:?})-({:?}) ({:?})-({:?})",
        //     self.axis(0).min,
        //     self.axis(0).max,
        //     self.axis(1).min,
        //     self.axis(1).max,
        //     self.axis(2).min,
        //     self.axis(2).max
        // );
        for a in 0..3 {
            let inv_dir = 1.0 / r.direction()[a] as Num;
            let orig = r.origin()[a];

            let mut t0 = (self.axis(a).min - orig) * inv_dir;
            let mut t1 = (self.axis(a).max - orig) * inv_dir;

            if inv_dir < 0.0 {
                swap(&mut t0, &mut t1);
            }

            ray_t.min = Num::max(ray_t.min, t0);
            ray_t.max = Num::min(ray_t.max, t1);

            if ray_t.max <= ray_t.min {
                // println!("{} <= {}", ray_t.max, ray_t.min);
                return false;
            }
        }
        // println!("Hit aabb");
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aabb_union() {
        let a = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        let b = AABB::new(
            Interval::new(1.0, 2.0),
            Interval::new(1.0, 2.0),
            Interval::new(1.0, 2.0),
        );
        let union = AABB::new(
            Interval::new(0.0, 2.0),
            Interval::new(0.0, 2.0),
            Interval::new(0.0, 2.0),
        );

        assert_eq!(union, AABB::union(&a, &b));
    }
}
