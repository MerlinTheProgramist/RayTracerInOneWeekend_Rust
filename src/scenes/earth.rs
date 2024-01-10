use crate::{
    camera::Camera,
    hittable::sphere::Sphere,
    material::Material,
    rtw_image::RtwImage,
    texture::TextureType,
    vec3::{Point3, Vec3},
};
use std::{fs, sync::Arc};

pub fn earth(f: &mut fs::File) {
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
