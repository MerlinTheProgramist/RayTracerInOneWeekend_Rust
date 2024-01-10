pub mod aabb;
mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod rtw_image;
mod scenes;
pub mod texture;
pub mod vec3;
use std::{env, fs};

use scenes::*;

type Num = f64;

fn main() {
    let mut f = None;
    let mut scene = 1;
    for arg in env::args() {
        if arg.ends_with(".ppm") {
            f = Some(fs::File::create(arg).expect("Cannot create file"));
        } else if let Result::Ok(s) = arg.parse() {
            scene = s;
        }
    }
    let f = f.get_or_insert(fs::File::create("./render.ppm").unwrap());

    match scene {
        1 => random_spheres(f),
        2 => two_spheres(f),
        3 => earth(f),
        4 => two_perlin_spheres(f),
        5 => quads(f),
        _ => panic!("Unknown scene selected"),
    }
}
