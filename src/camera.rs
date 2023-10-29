use crate::{
    color::{write_color, Color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::*,
    Num,
};
use rand::Rng;
use std::cmp::max;

pub struct Camera {
    pub aspect_ratio: Num,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            image_height: 100,
            max_depth: 10,
            center: Vec3::ZERO,
            pixel00_loc: Vec3::ZERO,
            pixel_delta_u: Vec3::ZERO,
            pixel_delta_v: Vec3::ZERO,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        print!("P3\n{} {} \n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::ZERO;
                // multiple samples per pixel
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world)
                }
                write_color(&pixel_color, self.samples_per_pixel);
            }
        }
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::ZERO;
        }
        if let Some(rec) = world.hit(r, Interval::new(0.001, Ray::INFINITY)) {
            // let direction = Vec3::random_on_hemisphere(&rec.normal);
            // Lambertial distribution
            let direction = rec.normal + Vec3::random_unit_sphere();

            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), depth - 1, world);
            // return 0.5 * (rec.normal + Color::new(1., 1., 1.));
        }

        let normal = normalize(r.direction());
        let a = 0.5 * (normal.y + 1.0);
        return (1. - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as Num * self.pixel_delta_u) + (j as Num * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }
    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();

        let px = -0.5 + rng.gen::<Num>();
        let py = -0.5 + rng.gen::<Num>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    pub fn initialize(&mut self) {
        self.image_height = max(1, (self.image_width as Num / self.aspect_ratio) as i32);

        self.center = Vec3::ZERO;

        // Camera
        let focal_lenght = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as Num / self.image_height as Num);

        // horizontal and vertiacal delta vectors of viewport edges
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // horizontal and vertiacal delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as Num;
        self.pixel_delta_v = viewport_v / self.image_height as Num;

        // location of the uper left pixel
        let viewport_upper_left =
            self.center - Vec3::new(0., 0., focal_lenght) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }
}
