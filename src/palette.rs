use std::io;
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
        let size: u32 = match self.pixels.len() {
            0...2 => 2,
            length => {
                (length as f64).sqrt().ceil() as u32
            }
        };

        let mut new_img = ImageBuffer::new(size, size);

        for (&new_pix, output_pix) in self.pixels.iter().zip(new_img.pixels_mut()) {
            *output_pix = new_pix;
        }

        new_img.save("test_palette.png")
    }
}
