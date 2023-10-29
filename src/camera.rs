use crate::{
    color::{write_color, Color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::*,
    Num,
};
use rand::Rng;
use std::{cmp::max, io::Write};

pub struct Camera {
    pub aspect_ratio: Num,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32,         // Maximum number of ray bounces into scene

    pub vfov: Num,        // Vertical view angle (field of view)
    pub lookfrom: Point3, // Point camera is looking from
    pub lookat: Point3,   // Point camera is looking at
    pub vup: Vec3,        // Camera-relative "up" direction

    pub defocus_angle: Num,
    pub focus_dist: Num,

    image_height: i32,   // Rendered image heihgt
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0,0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
    u: Vec3,             // Camera frame basis vectors
    v: Vec3,
    w: Vec3,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            image_height: 100,
            max_depth: 10,

            vfov: std::f64::consts::PI / 2.,
            lookfrom: Vec3::new(0., 0., -1.),
            lookat: Vec3::ZERO,
            vup: Vec3::new(0., 1., 0.),
            defocus_angle: 0.,
            focus_dist: 10.,

            center: Vec3::ZERO,
            pixel00_loc: Vec3::ZERO,
            pixel_delta_u: Vec3::ZERO,
            pixel_delta_v: Vec3::ZERO,
            u: Vec3::ZERO,
            v: Vec3::ZERO,
            w: Vec3::ZERO,

            defocus_disk_u: Vec3::ZERO,
            defocus_disk_v: Vec3::ZERO,
        }
    }

    pub fn render<W: Write>(&mut self, f: &mut W, world: &dyn Hittable) {
        self.initialize();

        write!(f, "P6\n{} {}\n255\n", self.image_width, self.image_height).unwrap();

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::ZERO;
                // multiple samples per pixel
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world)
                }
                write_color(f, &pixel_color, self.samples_per_pixel);
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
            // let direction = rec.normal + Vec3::random_unit_sphere();
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                return attenuation * &self.ray_color(&scattered, depth - 1, world);
            }
            return Color::ZERO;
            // return 0.5 * &self.ray_color(&Ray::new(rec.p, direction), depth - 1, world);
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

        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
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

        self.center = self.lookfrom;

        // Determine viewport dimensions
        let h = (self.vfov / 2.).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as Num / self.image_height as Num);

        // Calculate the u,v,w unit basis vectors for the camera coordicate frame
        self.w = normalize(&(self.lookfrom - self.lookat));
        self.u = normalize(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        // horizontal and vertiacal delta vectors of viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // horizontal and vertiacal delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as Num;
        self.pixel_delta_v = viewport_v / self.image_height as Num;

        // location of the uper left pixel
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_unit_in_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}
