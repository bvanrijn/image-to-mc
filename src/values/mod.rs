use serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec::Vec;

use difference;
use dominant;
use rgb;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "type")]
    item_type: u32,

    meta: u32,
    name: String,
    text_type: String,

    #[serde(skip)]
    color: [u8; 3],
}

fn get_items_text() -> String {
    let mut contents = String::new();

    let mut file = match File::open("items/items.json") {
        Err(why) => panic!("couldn't open items/items.json: {}", why),
        Ok(file) => file,
    };

    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {}
    }

    return contents;
}

fn exclude_range(it: &mut Vec<u32>, start: u32, stop: u32) {
    for i in start..stop {
        it.push(i)
    }
}

fn get_items() -> Vec<Item> {
    let mut ignored_types = vec![
        0, 2, 6, 8, 9, 10, 11, 12, 13, 19, 20, 23, 26, 27, 28, 29, 30, 31, 32, 33, 34, 37, 38, 39,
        40, 44, 46, 50, 51, 52, 53, 54, 55, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
        72, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 89, 91, 92, 93, 94, 95, 96, 97, 101,
        102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 113, 114, 115, 116, 117, 118, 119, 120,
        122, 123, 124, 126, 127, 128, 130, 131, 132, 134, 135, 136, 137, 138, 139, 140, 141, 142,
        143, 144, 145, 146, 147, 148, 149, 150, 151, 154, 156, 157, 158, 160, 161, 163, 164, 166,
        167, 169, 170, 171, 174, 175, 176, 177, 178, 180, 182, 183, 184, 185, 186, 187, 188, 189,
        190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 203, 204, 205, 207, 208, 209, 210,
        211, 212, 213, 217, 218, 252,
    ];

    exclude_range(&mut ignored_types, 219, 234);
    exclude_range(&mut ignored_types, 235, 250);
    exclude_range(&mut ignored_types, 255, 2267);

    let items_text = get_items_text();
    let items: Vec<Item> = match serde_json::from_str(&items_text) {
        Ok(items) => items,
        Err(why) => panic!("{}", why),
    };

    let mut ret = Vec::new();

    for mut item in items {
        if ignored_types.contains(&item.item_type) {
            continue;
        }

        let file_name = get_file_name(&item);
        let color = dominant::get_color(file_name);

        item.color = [color.r, color.g, color.b];

        ret.push(item);
    }

    return ret;
}

fn get_file_name(item: &Item) -> String {
    let file_name = format!("items/images/{}-{}.png", item.item_type, item.meta);

    return file_name;
}

fn color_to_string(color: [u8; 3]) -> String {
    format!("{},{},{}", color[0], color[1], color[2])
}

fn get_data_values() -> HashMap<String, Item> {
    let path = Path::new("values-resolved.json");
    let mut items_text = String::new();

    if path.exists() {
        let _ = File::open(&path).unwrap().read_to_string(&mut items_text);

        let items: HashMap<String, Item> = match serde_json::from_str(&items_text) {
            Ok(items) => items,
            Err(why) => panic!("{}", why),
        };

        return items;
    }

    let items = get_items();
    let mut h = HashMap::new();

    for item in items {
        h.insert(color_to_string(item.color), item);
    }

    let json = match serde_json::to_string(&h) {
        Ok(json) => json,
        Err(why) => panic!("{}", why),
    };

    let mut file = File::create("values-resolved.json").unwrap();
    file.write(json.as_bytes()).unwrap();

    return h;
}

pub fn get_item_for_color(color: rgb::RGB8) -> Option<Item> {
    let data_values = get_data_values();

    let mut item = Item {
        item_type: 0,
        meta: 0,
        name: String::new(),
        text_type: String::new(),
        color: [0, 0, 0],
    };
    let mut smallest_diff: f64 = 0.0;

    for data_value in data_values {
        let _cv = data_value.0;
        let _c: Vec<&str> = _cv.split(",").collect();

        let data_value_color = rgb::RGB {
            r: _c[0].parse().unwrap(),
            g: _c[1].parse().unwrap(),
            b: _c[2].parse().unwrap(),
        };

        let diff = difference::color_difference(color, data_value_color);

        if diff < smallest_diff {
            item = data_value.1;
            smallest_diff = diff;
        }
    }

    if item.text_type == "" {
        return None;
    } else {
        return Some(item);
    }
}
