pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod ray;
pub mod vec3;

use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable::{hittable_list::HittableList, sphere::Sphere};
use material::*;
use vec3::*;

type Num = f64;

fn main() {
    // World
    let mut world = HittableList::new();
    let material_ground = Rc::new(materials::Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(materials::Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(materials::Dielectric::new(&1.5));
    let material_right = Rc::new(materials::Metal::new(&Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_center.clone(),
    )));

    // Hollow glass sphere
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.4,
        material_left.clone(),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right.clone(),
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);
}
