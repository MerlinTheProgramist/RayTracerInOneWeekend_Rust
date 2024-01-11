use rand::Rng;

use crate::{
    camera::Camera,
    color::Color,
    hittable::{bvh_node::BvhNode, hittable_list::HittableList, sphere::Sphere},
    material::Material,
    vec3::{Point3, Vec3},
    Num,
};
use std::{fs, sync::Arc};

pub fn random_spheres(f: &mut fs::File) {
    let mut rand = rand::thread_rng();
    // World
    let mut world = HittableList::new();

    world.add(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Box::new(Material::new_lambertian(Color::new(0.5, 0.5, 0.5))),
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

    let material1 = Box::new(Material::new_dielectric(1.5));
    world.add(Sphere::new(Vec3::new(0., 1., 0.), 1.0, material1));

    let material2 = Box::new(Material::new_lambertian(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Vec3::new(-4., 1., 0.), 1.0, material2));

    let material3 = Box::new(Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Vec3::new(4., 1., 0.), 1.0, material3));

    let world = BvhNode::from_list(&mut world);

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 as Num / 9.0;
    cam.image_width = 2560;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = std::f64::consts::PI / 9.0;
    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = std::f64::consts::PI / 9000.;
    cam.focus_dist = 10.0;

    cam.initialize();
    cam.render(f, Arc::new(world));
}
