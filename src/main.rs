extern crate image;
extern crate time;

use std::path::Path;
use std::collections::HashSet;
use image::{GenericImage, ImageBuffer, Rgba};
use std::ops::Sub;
use time::*;

fn main() {
    let img = image::open(&Path::new("test.png")).unwrap();
    let mut palette:HashSet<Rgba<u8>> = HashSet::new();

    let (width, height) = img.dimensions();

    let total_pixels = img.pixels().count();
    let mut current_pixel = 1;

    let start = time::now();
    for pixel in img.pixels() {
        let (_,_,pix) = pixel;
        palette.insert(pix);
        let percentage = (current_pixel * 100) / total_pixels;
        print!("\r{}% | {} / {}", percentage, current_pixel, total_pixels);
        current_pixel += 1;
    }
    println!("");

    gen_palette(&mut palette);

    let end = time::now();
    let total_time = end.sub(start);
    println!("color: {:?}", img.color());
    println!("width: {:?}, height: {:?}", width, height);
    println!("number of total pixels:  {}", width * height);
    println!("total time: {:?}", total_time);
}

fn gen_palette(palette: &mut HashSet<Rgba<u8>>) -> () {
    let size:u32;

    match palette.len() {
        0...2 => {
            size = 2;
        },
        _ => {
            let length = palette.len() as f32;
            size = length.sqrt().ceil() as u32;
        }
    }

    let mut new_img = ImageBuffer::new(size, size);

    let mut current_x:u32 = 0;
    let mut current_y:u32 = 0;

    for pix in palette.iter() {
        new_img.put_pixel(current_x, current_y, *pix);
        current_x += 1;
        if current_x == size {
            current_x = 0;
            current_y += 1;
        }
    }

    let _ = new_img.save(&Path::new("test_palette.png"));
}
