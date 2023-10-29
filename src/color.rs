use crate::{interval::Interval, vec3::Vec3, Num};

pub type Color = Vec3;

#[inline(always)]
fn linear_to_gamma(linear_component: Num) -> Num {
    linear_component.sqrt()
}

pub fn write_color<W: std::io::Write>(f: &mut W, pixel_color: &Color, samplex_per_pixel: i32) {
    let scale = 1.0 / samplex_per_pixel as Num;
    let r = linear_to_gamma(pixel_color.x * scale);
    let g = linear_to_gamma(pixel_color.y * scale);
    let b = linear_to_gamma(pixel_color.z * scale);

    const INTENSITY: Interval = Interval {
        min: 0.000,
        max: 0.999,
    };
    write!(
        f,
        "{} {} {}\n",
        (255.999 * INTENSITY.clamp(r)) as i32,
        (255.999 * INTENSITY.clamp(g)) as i32,
        (255.999 * INTENSITY.clamp(b)) as i32
    )
    .unwrap();
}
