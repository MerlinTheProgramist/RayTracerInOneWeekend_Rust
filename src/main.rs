use std::cmp::max;

pub mod hittable;
pub mod interval;
pub mod ray;
pub mod vec3;

use hittable::{hittable_list::HittableList, sphere::Sphere, Hittable};
use interval::Interval;
use ray::*;
use vec3::*;

type Num = f64;

fn ray_color(r: &Ray, hittable: &dyn Hittable) -> Color {
    if let Some(rec) = hittable.hit(r, Interval::new(0., Ray::INFINITY)) {
        return 0.5 * (rec.normal + Color::new(1., 1., 1.));
    }

    let normal = normalize(r.direction());
    let a = 0.5 * (normal.y + 1.0);
    return (1. - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = max(1, (image_width as Num / aspect_ratio) as i32);

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Camera
    let focal_lenght = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as Num / image_height as Num);
    let camera_center = Point3::new(0., 0., 0.);

    // horizontal and vertiacal delta vectors of viewport edges
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // horizontal and vertiacal delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as Num;
    let pixel_delta_v = viewport_v / image_height as Num;

    // location of the uper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0., 0., focal_lenght) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    print!("P3\n{} {} \n255\n", image_width, image_height);

    for i in 0..image_height {
        for j in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_v * i as Num + pixel_delta_u * j as Num;
            let ray_direction = pixel_center - camera_center;

            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            pixel_color.write_color();
        }
    }
}
