extern crate image;

use std::path::Path;
use image::{GenericImage, ImageBuffer, Rgba};

fn main() {
    let img = image::open(&Path::new("small.png")).unwrap();
    let mut palette: Vec<Rgba<u8>> = Vec::new();

    let (width, height) = img.dimensions();

    for pixel in img.pixels() {
        let (_,_,pix) = pixel;
        if is_unique(&mut palette, &pix) {
            println!("pixel: {:?}", pixel);
            palette.push(pix.clone());
        }
    }

    gen_palette(&mut palette, 100, 100);

    println!("color: {:?}", img.color());
    println!("width: {:?}, height: {:?}", width, height);
    println!("number of total pixels:  {}", width * height);
}

fn is_unique(palette: &mut Vec<Rgba<u8>>, pixel: &Rgba<u8>) -> bool {
    let mut does_exist: bool = false;
    for pix in palette.iter() {
        if pix == pixel {
            does_exist = true;
        }
    }
    !does_exist
}

fn gen_palette(palette: &mut Vec<Rgba<u8>>, width: u32, height: u32) {
    let mut new_img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            match palette.pop() {
                Some(s) => new_img.put_pixel(x, y, s),
                _ => continue
            }
        }
    }

    let _ = new_img.save(&Path::new("small_palette.png"));
}
