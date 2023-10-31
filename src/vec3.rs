use rand::Rng;

use crate::Num;
use std::{fmt::Display, ops};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: Num,
    pub y: Num,
    pub z: Num,
}

impl Vec3 {
    pub fn new(x: Num, y: Num, z: Num) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn random() -> Vec3 {
        let mut rand = rand::thread_rng();
        Vec3 {
            x: rand.gen::<Num>(),
            y: rand.gen::<Num>(),
            z: rand.gen::<Num>(),
        }
    }
    pub fn random_range(min: Num, max: Num) -> Vec3 {
        let mut rand = rand::thread_rng();
        Vec3 {
            x: min + rand.gen::<Num>() * max,
            y: min + rand.gen::<Num>() * max,
            z: min + rand.gen::<Num>() * max,
        }
    }
    // Maybe it should be random_IN_unit_sphere, therefore not normalized
    pub fn random_unit_sphere() -> Vec3 {
        let mut rand = rand::thread_rng();
        // ON unit sphere
        let theta = rand.gen_range((0.)..std::f64::consts::PI * 2.);
        let phi = (rand.gen_range((-1.)..1.) as Num).acos();
        Vec3 {
            x: phi.sin() * theta.cos(),
            y: phi.sin() * theta.sin(),
            z: phi.cos(),
        }
        // IN unit sphere
        // loop {
        //     let p = Vec3::new(
        //         rand.gen_range((-1.)..1.),
        //         rand.gen_range((-1.)..1.),
        //         rand.gen_range((-1.)..1.),
        //     );
        //     if p.lenght_sqr() < 1. {
        //         return p;
        //     }
        // }
    }
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let res = Vec3::random_unit_sphere();
        if dot(&res, normal) > 0.0 {
            res
        } else {
            -res
        }
    }
    pub fn random_unit_in_disk() -> Vec3 {
        let mut rand = rand::thread_rng();
        // let theta = rand.gen::<Num>() * std::f64::consts::PI * 2.;
        // Vec3::new(theta.cos(), theta.sin(), 0.) * rand.gen::<Num>()
        loop {
            let p = Vec3::new(rand.gen_range((-1.)..1.), rand.gen_range((-1.)..1.), 0.);
            if p.lenght_sqr() < 1. {
                return p;
            }
        }
    }

    pub fn lenght_sqr(&self) -> Num {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn lenght(&self) -> Num {
        self.lenght_sqr().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        const S: Num = 1e-8;
        (self.x.abs() < S) && (self.y.abs() < S) && (self.z.abs() < S)
    }

    pub const ZERO: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Index<i32> for Vec3 {
    type Output = Num;
    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range"),
        }
    }
}
impl ops::IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, v: Self) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl ops::MulAssign<Num> for Vec3 {
    fn mul_assign(&mut self, t: Num) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl ops::DivAssign<Num> for Vec3 {
    fn div_assign(&mut self, t: Num) {
        self.x /= t;
        self.y /= t;
        self.z /= t;
    }
}

// Returninng operators
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} ", self.x, self.y, self.z)
    }
}
impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, v: Self) -> Self::Output {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}
impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, v: Self) -> Self::Output {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl ops::Mul<&Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, u: &Vec3) -> Self::Output {
        Vec3::new(self.x * u.x, self.y * u.y, self.z * u.z)
    }
}
impl ops::Mul<Num> for Vec3 {
    type Output = Self;
    fn mul(self, t: Num) -> Self::Output {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}
impl ops::Mul<Vec3> for Num {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        v * self
    }
}
impl ops::Mul<&Vec3> for Num {
    type Output = Vec3;
    fn mul(self, v: &Vec3) -> Self::Output {
        *v * self
    }
}
impl ops::Div<Num> for Vec3 {
    type Output = Vec3;
    fn div(self, t: Num) -> Self::Output {
        Vec3::new(self.x / t, self.y / t, self.z / t)
    }
}

pub fn dot(v: &Vec3, u: &Vec3) -> Num {
    v.x * u.x + v.y * u.y + v.z * u.z
}

pub fn cross(v: &Vec3, u: &Vec3) -> Vec3 {
    Vec3::new(
        v.y * u.z - v.z * u.y,
        v.z * u.x - v.x * u.z,
        v.x * u.y - v.y * u.x,
    )
}

pub fn normalize(v: &Vec3) -> Vec3 {
    *v / v.lenght()
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2. * dot(v, n) * n
}
#[inline(always)]
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: Num) -> Vec3 {
    let cos_theta = Num::min(dot(&-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (*uv + (cos_theta * n));
    let r_out_parallel = (1.0 - r_out_perp.lenght_sqr()).abs().sqrt() * -n;
    r_out_perp + r_out_parallel
}

pub type Point3 = Vec3;
