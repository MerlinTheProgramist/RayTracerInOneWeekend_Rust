use crate::{
    vec3::{Point3, Vec3},
    Num,
};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub const INFINITY: Num = Num::MAX;
    pub fn new(o: Point3, d: Vec3) -> Ray {
        Ray { orig: o, dir: d }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }
    pub fn at(&self, t: Num) -> Point3 {
        self.orig + self.dir * t
    }
}
