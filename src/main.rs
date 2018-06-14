mod colordifference;

fn main() {
    println!("{}", colordifference::color_difference(&mut [0, 0, 0], &mut [255, 255, 255]));
}
