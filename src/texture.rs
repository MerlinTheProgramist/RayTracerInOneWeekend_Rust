use std::{hash::BuildHasher, sync::Arc};

use image::GenericImageView;

use crate::{color::Color, interval::Interval, rtw_image::RtwImage, vec3::Point3, Num};

pub enum TextureType {
    SolidColor {
        color_value: Color,
    },
    CheckerTexture {
        inv_scale: Num,
        even: Arc<TextureType>,
        odd: Arc<TextureType>,
    },
    ImageTexture {
        image: RtwImage,
    },
}

impl TextureType {
    pub fn value(&self, mut u: Num, mut v: Num, p: &Point3) -> Color {
        match self {
            Self::SolidColor { color_value } => *color_value,
            Self::CheckerTexture {
                inv_scale,
                even,
                odd,
            } => {
                let x_int = Num::floor(inv_scale * u) as i32;
                let y_int = Num::floor(inv_scale * v) as i32;
                // let z_int = Num::floor(inv_scale * p.z) as i32;

                let is_even = (x_int + y_int) % 2 == 0;
                if is_even {
                    even.value(u, v, p)
                } else {
                    odd.value(u, v, p)
                }
            }
            Self::ImageTexture { image } => {
                if image.height <= 0 {
                    return Color::new(0.0, 1.0, 1.0);
                }

                u = Interval::new(0.0, 1.0).clamp(u);
                v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

                let i = (u * image.width as Num) as usize;
                let j = (v * image.height as Num) as usize;
                let pixel = image.pixel_data(i, j);

                let color_scale = 1.0 / 255.0;
                Color::new(
                    color_scale * pixel[0] as Num,
                    color_scale * pixel[1] as Num,
                    color_scale * pixel[2] as Num,
                )
            }
        }
    }

    pub fn solid_new(color_value: Color) -> Self {
        Self::SolidColor { color_value }
    }
    pub fn solid_from_rgb(red: Num, green: Num, blue: Num) -> Self {
        Self::SolidColor {
            color_value: Color::new(red, green, blue),
        }
    }
    pub fn checker_new(scale: Num, even: Arc<TextureType>, odd: Arc<TextureType>) -> Self {
        Self::CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}