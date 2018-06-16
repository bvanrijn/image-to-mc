use std::{thread, time};
use image;
use rgb;
use values;

pub fn exclude_range(it: &mut Vec<u32>, start: u32, stop: u32) {
    for i in start..=stop {
        it.push(i)
    }
}

pub fn color_to_string(color: [u8; 3]) -> String {
    format!("{},{},{}", color[0], color[1], color[2])
}

pub fn sleep(s: f64) {
    let ten_millis = time::Duration::from_millis((s * 1000.0) as u64);

    thread::sleep(ten_millis);
}

pub fn image_rgb_to_rgb_struct(color: image::Rgb<u8>) -> rgb::RGB<u8> {
    return rgb::RGB{
        r: color[0],
        g: color[1],
        b: color[2],
    }
}

fn parse_coordinates_to_vec(coordinates: &str) -> Vec<i32> {
    let coords: Vec<i32> = coordinates.split(";")
    .map(|coord| coord.parse().unwrap())
    .collect();

    return coords
}

pub struct Coordinates {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub fn parse_coordinates(coordinates: String) -> Coordinates {
    let c = parse_coordinates_to_vec(coordinates.as_str());

    return Coordinates{
        x: c[0],
        y: c[1],
        z: c[2]
    }
}

pub fn get_file_name(item: &values::Item) -> String {
    let file_name = format!("items/images/{}-{}.png", item.item_type, item.meta);

    return file_name;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exclude_range_len() {
        let mut it: Vec<u32> = vec![0, 1, 2, 3];
        exclude_range(&mut it, 4, 10);
    
        assert_eq!(it.len(), 11);
    }

    #[test]
    fn test_exclude_range_last_element() {
        let mut it: Vec<u32> = vec![0, 1, 2, 3];
        exclude_range(&mut it, 4, 10);

        assert_eq!(it[it.len()-1], 10);
    }

    #[test]
    fn test_color_to_string() {
        assert_eq!(color_to_string([12,34,56]), "12,34,56");
    }
}
