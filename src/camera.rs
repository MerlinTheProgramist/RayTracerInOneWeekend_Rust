use std::cmp::max;

use crate::{hittable::Hittable, interval::Interval, ray::Ray, vec3::*, Num};

pub struct Camera {
    pub aspect_ratio: Num,
    pub image_width: i32,
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
            image_height: 100,
            center: Vec3::ZERO,
            pixel00_loc: Vec3::ZERO,
            pixel_delta_u: Vec3::ZERO,
            pixel_delta_v: Vec3::ZERO,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        print!("P3\n{} {} \n255\n", self.image_width, self.image_height);

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + self.pixel_delta_v * i as Num
                    + self.pixel_delta_u * j as Num;
                let ray_direction = pixel_center - self.center;

                let r = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&r, world);
                pixel_color.write_color();
            }
        }
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(r, Interval::new(0., Ray::INFINITY)) {
            return 0.5 * (rec.normal + Color::new(1., 1., 1.));
        }

        let normal = normalize(r.direction());
        let a = 0.5 * (normal.y + 1.0);
        return (1. - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
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
        let pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }
}