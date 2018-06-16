extern crate color_thief;
extern crate image;
extern crate image_utils;
extern crate rgb;
extern crate serde;
extern crate serde_json;
extern crate tempfile;

#[macro_use]
extern crate serde_derive;

mod difference;
mod dominant;
mod resize;
mod values;

use image::GenericImage;

fn main() {
    println!("{}", difference::color_difference(rgb::RGB{r: 0, g: 0, b: 0}, rgb::RGB{r: 255, g: 255, b: 255}));
    println!("{:?}", values::get_item_for_color(rgb::RGB{r: 0, g: 0, b: 0}));
    // println!("{:?}", values::get_data_values());
    println!("{:?}", dominant::get_color("items/images/1-0.png".to_string()));
    println!("{:?}", resize::resize_image("President_Barack_Obama.jpg".to_string()).dimensions());
}
