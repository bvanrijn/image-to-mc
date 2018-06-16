extern crate color_thief;
extern crate enigo;
extern crate image;
extern crate image_utils;
extern crate rgb;
extern crate rprompt;
extern crate serde;
extern crate serde_json;
extern crate tempfile;

#[macro_use]
extern crate serde_derive;

mod difference;
mod dominant;
mod keyboard;
mod resize;
mod util;
mod values;

use image::GenericImage;

fn main() {
    let image_location = rprompt::prompt_reply_stdout("Image location on disk: ").unwrap();
    let coordinates = rprompt::prompt_reply_stdout("Coordinates (x;y;z): ").unwrap();

    let resized_image = resize::resize_image(image_location);
    let size = resized_image.dimensions();

    println!("Focus on the Minecraft window.");
    println!("This is very important. Waiting 3 seconds ...");

    util::sleep(3.0);
}
