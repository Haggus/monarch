extern crate image;
extern crate time;

mod palette;

use std::path::Path;
use image::{GenericImage};
use time::*;
use std::ops::Sub;
use palette::Palette;

fn main() {
    let img = image::open(&Path::new("test.png")).unwrap();
    let mut palette:Palette = Palette::new();

    let (width, height) = img.dimensions();

    let total_pixels = img.pixels().count();
    let mut current_pixel = 1;

    let start = time::now();
    for pixel in img.pixels() {
        let (_,_,pix) = pixel;
        palette.insert(pix);
        let percentage = (current_pixel * 100) / total_pixels;
        //print!("\r{}% | {} / {}", percentage, current_pixel, total_pixels);
        current_pixel += 1;
    }
    println!("");

    match palette.generate() {
        Ok(_) => {
            println!("color: {:?}", img.color());
            println!("width: {:?}, height: {:?}", width, height);
            println!("number of total pixels:  {}", width * height);
        },
        Err(e) => {
            println!("error: {}", e);
        }
    }

    let end = time::now();
    let total_time = end.sub(start);
    println!("total time: {:?}", total_time);
}
