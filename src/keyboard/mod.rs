use enigo::{Enigo, KeyboardControllable};
use util;

pub fn get_set_block_command(x: i32, y: i32, z: i32, block: String, data_value: u32) -> String {
    return format!("/setblock {} {} {} minecraft:{} {}", x, y, z, block, data_value)
}

fn send_command(command: String) {
    let mut enigo = Enigo::new();

    enigo.key_sequence("T");
    util::sleep(0.1);
    enigo.key_sequence(&command);
    util::sleep(0.3);
}

pub fn set_block(x: i32, y: i32, z: i32, block: String, data_value: u32) {
    let command = get_set_block_command(x, y, z, block, data_value);
    
    send_command(command);
}