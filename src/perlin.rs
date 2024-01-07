use rand::{seq::SliceRandom, thread_rng};

use crate::{
    vec3::{dot, Point3, Vec3},
    Num,
};

pub struct Perlin {
    ran_vec: [Vec3; Perlin::POINT_COUNT],
    perm_x: [i32; Perlin::POINT_COUNT],
    perm_y: [i32; Perlin::POINT_COUNT],
    perm_z: [i32; Perlin::POINT_COUNT],
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            ran_vec: core::array::from_fn(|_i| Vec3::random_range(-1.0, 1.0)),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn noise(&self, p: &Point3) -> Num {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = Default::default();
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ran_vec[self.perm_x[(i + di as i32) as usize & 255]
                        as usize
                        ^ self.perm_y[(j + dj as i32) as usize & 255] as usize
                        ^ self.perm_z[(k + dk as i32) as usize & 255] as usize];
                }
            }
        }
        Self::perlin_interp(c, u, v, w)
    }

    fn perlin_generate_perm() -> [i32; Self::POINT_COUNT] {
        let mut arr = core::array::from_fn(|i| i as i32);
        arr.shuffle(&mut thread_rng());
        arr
    }

    // #[allow(unused)]
    // fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    //     let mut accum = 0.0;
    //     for i in 0..2 {
    //         for j in 0..2 {
    //             for k in 0..2 {
    //                 accum += (i as Num * u + (1.0 - i as Num) * (1.0 - u))
    //                     * (j as Num * v + (1.0 - j as Num) * (1.0 - v))
    //                     * (k as Num * w + (1.0 - k as Num) * (1.0 - w))
    //                     * c[i][j][k];
    //             }
    //         }
    //     }
    //     accum
    // }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: Num, v: Num, w: Num) -> Num {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as Num, v - j as Num, w - k as Num);
                    accum += (i as Num * uu + (1.0 - i as Num) * (1.0 - uu))
                        * (j as Num * vv + (1.0 - j as Num) * (1.0 - vv))
                        * (k as Num * ww + (1.0 - k as Num) * (1.0 - ww))
                        * dot(&c[i as usize][j as usize][k as usize], &weight_v);
                }
            }
        }
        return accum;
    }

    pub fn turb(&self, mut p: Point3, depth: i32) -> Num {
        let mut accum = 0.0;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(&p);
            weight *= 0.5;
            p *= 2.0;
        }
        return accum.abs();
    }
}
