extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename="type")]
    item_type: u32,
    meta: u32,
    name: String,
    text_type: String,
}

fn get_items_text() -> String {
    let mut contents = String::new();

    let mut file = match File::open("items/items.json") {
        Err(why) => panic!("couldn't open items/items.json: {}", why),
        Ok(file) => file,
    };

    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {},
    }

    return contents
}

fn get_items() -> Vec<Item> {
    let items_text = get_items_text();
    let items: Vec<Item> = match serde_json::from_str(&items_text) {
        Ok(items) => items,
        Err(why) => panic!("{}", why),
    };

    return items
}

pub fn get() -> HashMap<String, Item> {
    let items = get_items();
    let mut h = HashMap::new();
    
    for item in items {
        // colorthief
        // cargo add palette and image
        h.insert(item.text_type.to_string(), item);
    }

    return h
}