use std::io;
use std::path::Path;
use std::collections::HashSet;
use image::{GenericImage, ImageBuffer, Rgba};

pub struct Palette {
    pixels: HashSet<Rgba<u8>>
}

impl Palette {
    pub fn new() -> Palette {
        Palette {
            pixels: HashSet::new()
        }
    }

    pub fn insert(&mut self, pixel: Rgba<u8>) {
        self.pixels.insert(pixel);
    }

    pub fn generate(&mut self) -> Result<(), io::Error> {
        let size:u32;

        match self.pixels.len() {
            0...2 => size = 2,
            _ => {
                let length = self.pixels.len() as f32;
                size = length.sqrt().ceil() as u32;
            }
        }

        let mut new_img = ImageBuffer::new(size, size);

        let mut current_x:u32 = 0;
        let mut current_y:u32 = 0;

        for pix in self.pixels.iter() {
            new_img.put_pixel(current_x, current_y, *pix);
            current_x += 1;
            if current_x == size {
                current_x = 0;
                current_y += 1;
            }
        }

        new_img.save(&Path::new("test_palette.png"))
    }
}
