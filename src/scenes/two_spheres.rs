use crate::{
    camera::Camera,
    color::Color,
    hittable::{bvh_node::BvhNode, hittable_list::HittableList, sphere::Sphere},
    material::Material,
    texture::TextureType,
    vec3::{Point3, Vec3},
};
use std::{fs, sync::Arc};

pub fn two_spheres(f: &mut fs::File) {
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
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = (20.0f64).to_radians();
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.initialize();
    cam.render(f, Arc::new(world));
}
