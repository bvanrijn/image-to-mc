#[macro_use]
extern crate serde_derive;

mod colordifference;
mod datavalues;

fn main() {
    println!("{}", colordifference::color_difference(&mut [0, 0, 0], &mut [255, 255, 255]));
    println!("{:?}", datavalues::get())
}
