extern crate image;
extern crate time;

use std::path::Path;
use image::{GenericImage, ImageBuffer, Rgba};
use std::ops::Sub;
use time::*;

fn main() {
    let img = image::open(&Path::new("test.png")).unwrap();
    let mut palette: Vec<Rgba<u8>> = Vec::new();

    let (width, height) = img.dimensions();

    let total_pixels = img.pixels().count();
    let mut current_pixel = 1;

    let start = time::now();
    for pixel in img.pixels() {
        let (_,_,pix) = pixel;
        if is_unique(&mut palette, &pix) {
            // println!("pixel: {:?}", pixel);
            palette.push(pix.clone());
        }
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
    //println!("{:?} - {:?}", start, end);
    println!("total time: {:?}", total_time);
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

fn gen_palette(palette: &mut Vec<Rgba<u8>>) -> () {
    let width:u32;
    let height: u32;

    match palette.len() {
        0...2 => {
            width = 2;
            height = 2;
        },
        _ => {
            let length = palette.len() as f32;
            width = length.sqrt().ceil() as u32;
            height = length.sqrt().ceil() as u32;
        }
    }

    let mut new_img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            match palette.pop() {
                Some(s) => new_img.put_pixel(x, y, s),
                _ => continue
            }
        }
    }

    let _ = new_img.save(&Path::new("test_palette.png"));
}
