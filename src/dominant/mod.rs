use color_thief;
use image;
use image::GenericImageView;
use rgb;
use std::vec::Vec;

fn open_image(file_name: String) -> Vec<u8> {
    let img = image::open(file_name).unwrap();
    let mut pixels = Vec::new();

    for pixel in img.pixels() {
        for data in &pixel.2.data {
            pixels.push(*data)
        }
    }

    pixels
}

pub fn get_color(file_name: String) -> rgb::RGB8 {
    let img = open_image(file_name);
    let palette = color_thief::get_palette(&img, color_thief::ColorFormat::Rgb, 10, 2).unwrap();
    let rgb = palette[0];

    rgb::RGB {
        r: rgb.r,
        g: rgb.g,
        b: rgb.b,
    }
}
