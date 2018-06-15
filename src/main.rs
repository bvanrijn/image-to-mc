extern crate serde;
extern crate serde_json;
extern crate image;
extern crate color_thief;
extern crate rgb;
extern crate image_utils;
extern crate tempfile;

#[macro_use]
extern crate serde_derive;

mod difference;
mod values;
mod dominant;
mod resize;

use image::GenericImage;

fn main() {
    println!("{}", difference::color_difference(&mut [0, 0, 0], &mut [255, 255, 255]));
    println!("{:?}", values::get_data_values());
    println!("{:?}", dominant::get_color("items/images/1-0.png".to_string()));
    println!("{:?}", resize::resize_image("President_Barack_Obama.jpg".to_string()).dimensions());
}
