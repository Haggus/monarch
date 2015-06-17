extern crate image;

use std::path::Path;
use image::{GenericImage, Rgba};

fn main() {
    let img = image::open(&Path::new("small.png")).unwrap();
    let mut pallete: Vec<Rgba<u8>> = Vec::new();

    let (width, height) = img.dimensions();

    for pixel in img.pixels() {
        let (_,_,pix) = pixel;
        if is_unique(&mut pallete, &pix) {
            println!("pixel: {:?}", pixel);
            //println!("UNIQUE");
            pallete.push(pix.clone());
        }
    }

    println!("color: {:?}", img.color());

    println!("width: {:?}, height: {:?}", width, height);
    println!("number of total pixels:  {}", width * height);
    println!("number of unique pixels: {}", pallete.len());
}

fn is_unique(pallete: &mut Vec<Rgba<u8>>, pixel: &Rgba<u8>) -> bool {
    let mut does_exist: bool = false;
    for pix in pallete.iter() {
        if pix == pixel {
            does_exist = true;
        }
    }
    !does_exist
}
