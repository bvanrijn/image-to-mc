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
    let coordinates_input = rprompt::prompt_reply_stdout("Coordinates (x;y;z): ").unwrap();

    let coordinates = util::parse_coordinates(coordinates_input);

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

    for (n, row) in building_blocks.iter().enumerate() {
        for (o, item) in row.iter().enumerate() {
            println!("{}", keyboard::get_set_block_command(coordinates.x + n as i32, coordinates.y, coordinates.z + o as i32, item.text_type.to_string(), item.meta));
            // keyboard::set_block(
            //     coordinates.x + n as i32,
            //     coordinates.y,
            //     coordinates.z + o as i32,
            //     item.text_type.to_string(),
            //     item.meta,
            // )
        }
    }
}
