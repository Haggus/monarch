extern crate image;
extern crate time;

mod palette;

use time::now;
use image::GenericImage;
use palette::Palette;

fn main() {
    let img = image::open("test.png").unwrap();
    let mut palette = Palette::new();

    let (width, height) = img.dimensions();

    let total_pixels = (width * height) as usize;

    let start = time::now();
    for (i, (_, _, pix)) in img.pixels().enumerate() {
        palette.insert(pix);
        let percentage = ((i + 1) * 100) / total_pixels;
        //print!("\r{}% | {} / {}", percentage, current_pixel, total_pixels);
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
    let total_time = end - start;
    println!("total time: {:?}", total_time);
}
