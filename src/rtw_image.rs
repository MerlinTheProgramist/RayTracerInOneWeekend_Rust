use image::io::Reader;
pub struct RtwImage {
    bytes_per_pixel: usize,
    data: Vec<u8>,
    pub width: usize,
    pub height: usize,
    bytes_per_scanline: usize,
}

impl RtwImage {
    pub fn new(filename: &str) -> RtwImage {
        let image = Reader::open(filename).unwrap().decode().unwrap();
        RtwImage {
            bytes_per_pixel: 3,
            width: image.width() as usize,
            height: image.height() as usize,
            bytes_per_scanline: image.width() as usize * 3,
            data: image.into_bytes(),
        }
    }

    pub fn pixel_data(&self, mut x: usize, mut y: usize) -> [u8; 3] {
        // static magenta = [255,0,255];

        x = x.clamp(0, self.width - 1);
        y = y.clamp(0, self.height - 1);

        let index = y * self.bytes_per_scanline + x * self.bytes_per_pixel;
        [self.data[index], self.data[index + 1], self.data[index + 2]]
    }
}
