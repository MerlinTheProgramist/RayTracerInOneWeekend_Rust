use crate::{
    vec3::{Point3, Vec3},
    Num,
};

#[derive(Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: Num,
}

impl Ray {
    pub const INFINITY: Num = Num::MAX;
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self {
            orig,
            dir,
            tm: Num::default(),
        }
    }
    pub fn new_timed(orig: Point3, dir: Vec3, tm: Num) -> Self {
        Self { orig, dir, tm }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }
    pub fn time(&self) -> &Num {
        &self.tm
    }
    pub fn at(&self, t: Num) -> Point3 {
        self.orig + self.dir * t
    }
}
