extern crate serde;
extern crate serde_json;
extern crate image;
extern crate color_thief;
extern crate rgb;

#[macro_use]
extern crate serde_derive;

mod colordifference;
mod datavalues;
mod dominant;

fn main() {
    println!("{}", colordifference::color_difference(&mut [0, 0, 0], &mut [255, 255, 255]));
    println!("{:?}", datavalues::get());
    println!("{:?}", dominant::get_color("items/images/1-0.png".to_string()));
}
