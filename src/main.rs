pub mod aabb;
mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod rtw_image;
pub mod texture;
pub mod vec3;

use std::{env, fs, sync::Arc};

use camera::Camera;
use color::Color;
use hittable::{bvh_node::BvhNode, hittable_list::HittableList, sphere::Sphere, HittableObject};
use material::*;
use rand::Rng;
use rtw_image::RtwImage;
use texture::TextureType;
use vec3::*;

type Num = f64;

fn random_spheres(f: &mut fs::File) {
    let mut rand = rand::thread_rng();
    // World
    let mut world = HittableList::new();

    let checker = Arc::new(TextureType::new_checker(
        0.32,
        Arc::new(TextureType::new_solid(Color::new(0.2, 0.3, 0.1))),
        Arc::new(TextureType::new_solid(Color::new(0.9, 0.9, 0.9))),
    ));
    world.add(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Box::new(Material::new_lambertian_textured(checker.clone())),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand.gen::<Num>();
            let center = Point3::new(
                a as Num + 0.9 * rand.gen::<Num>(),
                0.2,
                b as Num + 0.9 * rand.gen::<Num>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).lenght() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * &Color::random();
                    let sphere_material = Box::new(Material::new_lambertian(albedo));
                    let center2 = center + Vec3::new(0., rand.gen_range(0.0..0.5), 0.);
                    world.add(Sphere::new_moving(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = rand.gen::<Num>();
                    let sphere_material = Box::new(Material::new_metal(albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Box::new(Material::new_dielectric(1.5));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    // let material1 = Box::new(Material::new_dielectric(1.5));
    let material1 = Box::new(Material::new_lambertian_textured(checker.clone()));
    world.add(Sphere::new(Vec3::new(0., 1., 0.), 1.0, material1));

    let material2 = Box::new(Material::new_lambertian(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Vec3::new(-4., 1., 0.), 1.0, material2));

    let material3 = Box::new(Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Vec3::new(4., 1., 0.), 1.0, material3));

    let world = BvhNode::from_list(&mut world);

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 as Num / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 50;
    cam.max_depth = 50;

    cam.vfov = std::f64::consts::PI / 9.0;
    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = std::f64::consts::PI / 9000.;
    cam.focus_dist = 10.0;

    cam.initialize();
    cam.render(f, Arc::new(world));
}

fn two_spheres(f: &mut fs::File) {
    let mut world = HittableList::new();

    let checker = Arc::new(TextureType::new_checker(
        0.01,
        Arc::new(TextureType::new_solid(Color::new(0.2, 0.3, 0.1))),
        Arc::new(TextureType::new_solid(Color::new(0.9, 0.9, 0.9))),
    ));

    world.add(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Box::new(Material::new_lambertian_textured(checker.clone())),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Box::new(Material::new_lambertian_textured(checker.clone())),
    ));

    let world = BvhNode::from_list(&mut world);
    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = (20.0f64).to_radians();
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.initialize();
    cam.render(f, Arc::new(world));
}

fn earth(f: &mut fs::File) {
    let earth_texture = Arc::new(TextureType::ImageTexture {
        image: RtwImage::new("./assets/earth/albedo.jpg"),
    });
    let earth_surface = Box::new(Material::new_lambertian_textured(earth_texture));
    let globe = Arc::new(Sphere::new(Vec3::default(), 2.0, earth_surface));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = (20.0f64).to_radians();
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    cam.initialize();
    cam.render(f, globe);
}

fn two_perlin_spheres(f: &mut fs::File) {
    let mut world = HittableList::new();

    let pertext = Arc::new(TextureType::new_noise(4.0));
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Material::new_lambertian_textured(pertext.clone())),
    ));
    world.add(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Box::new(Material::new_lambertian_textured(pertext)),
    ));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = (20.0f64).to_radians();
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    cam.initialize();
    cam.render(f, Arc::new(HittableObject::HittableList(world)));
}

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
        _ => panic!("Unknown scene selected"),
    }
}
