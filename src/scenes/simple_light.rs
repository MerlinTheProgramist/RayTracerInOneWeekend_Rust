use crate::{
    camera::Camera,
    color::Color,
    hittable::{bvh_node::BvhNode, hittable_list::HittableList, quad::Quad, sphere::Sphere},
    material::Material,
    texture::TextureType,
    vec3::{Point3, Vec3},
};
use std::{fs, sync::Arc};

pub fn simple_light(f: &mut fs::File) {
    let mut world = HittableList::new();

    let pertect = Arc::new(TextureType::new_noise(4.0));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Material::new_lambertian_textured(pertect.clone())),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Box::new(Material::new_lambertian_textured(pertect)),
    ));

    let difflight = Box::new(Material::new_solid_diffuse_light(Color::new(4.0, 4.0, 4.0)));
    world.add(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    ));
    world.add(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
        difflight,
    ));
    let world = BvhNode::from_list(&mut world);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = (20.0f64).to_radians();
    cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
    cam.lookat = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    cam.initialize();
    cam.render(f, Arc::new(world));
}
