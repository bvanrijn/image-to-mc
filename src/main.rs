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

fn main() {
    let image_location = rprompt::prompt_reply_stdout("Image location on disk: ").unwrap();
    let coordinates = rprompt::prompt_reply_stdout("Coordinates (x;y;z): ").unwrap();

    let resized_image = resize::resize_image(image_location);
    let rgb_image = resized_image.to_rgb();

    let size = rgb_image.dimensions();

    let mut building_blocks: Vec<Vec<values::Item>> = Vec::new();

    for x in 0..size.0 {
        let mut row = Vec::new();

        for y in 0..size.1 {
            let color = rgb_image.get_pixel(x, y);
            let item = values::get_item_for_color(util::image_rgb_to_rgb_struct(*color));

            row.push(item.unwrap());
        }

        building_blocks.push(row);
    }

    println!("Focus on the Minecraft window.");
    println!("This is very important. Waiting 3 seconds ...");

    util::sleep(3.0);

    println!("{:?}", building_blocks);
}
