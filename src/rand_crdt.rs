use rand::prelude::*;
use std::{thread, time};

pub fn get_rand(low: u16, high: u16) -> u16 {
    rand::thread_rng().gen_range(low..high)
}

pub fn get_bool_index() -> bool {
    get_rand(0, 100) % 2 == 0
}

pub fn get_node_index(node_list: u16) -> u16 {
    get_rand(0, node_list)
}

pub fn msg_sleep(stime: u64) {
    let time_millis = time::Duration::from_millis(stime);
    thread::sleep(time_millis);    
}
