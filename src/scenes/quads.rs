use crate::{
    camera::Camera,
    color::Color,
    hittable::{bvh_node::BvhNode, hittable_list::HittableList, quad::Quad},
    material::Material,
    vec3::{Point3, Vec3},
};
use std::{fs, sync::Arc};

pub fn quads(f: &mut fs::File) {
    let mut world = HittableList::new();
    // Materials
    let left_red = Box::new(Material::new_lambertian(Color::new(1.0, 0.2, 0.2)));
    let back_green = Box::new(Material::new_lambertian(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Box::new(Material::new_lambertian(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Box::new(Material::new_lambertian(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Box::new(Material::new_lambertian(Color::new(0.2, 0.0, 0.8)));

    // Quads
    world.add(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    world.add(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    let world = Arc::new(BvhNode::from_list(&mut world));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = (80.0f64).to_radians();
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    cam.initialize();
    cam.render(f, world);
}
