pub mod camera;
pub mod hittable;
pub mod interval;
pub mod ray;
pub mod vec3;

use camera::Camera;
use hittable::{hittable_list::HittableList, sphere::Sphere};
use vec3::*;

type Num = f64;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.render(&world);
}
