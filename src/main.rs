use rand::prelude::*;

use ops_crdt_rust::crdt::{CrdtInstance, CrdtType};
use ops_crdt_rust::node_state::NodeState;
use ops_crdt_rust::message_data::UserUpdateMsg;

fn main() {
    let node_state = NodeState::new().unwrap();
    let nlen = node_state.get_node_len();
    let crdt_node = get_crdt_index(nlen);
    let ops_index = get_ops_index();
    let crdt_instance = get_crdt_instance();
    let node_instance = node_state.get_node_instance(crdt_node);
    let ops_value = get_rand(1, 20) as i32;
    let ops_instance = if ops_index == 0 {node_instance.get_add_ops(ops_value)} else {node_instance.get_mult_ops(ops_value)};
    let user_update_msg = UserUpdateMsg::new(crdt_instance, ops_instance);

    println!("msg  =>   {:?}", user_update_msg);
    println!("node =>   {:?}", node_instance);
}

fn get_crdt_instance() -> CrdtInstance {
    CrdtInstance::new(0, 1, CrdtType::AddMultCrdt)
}

fn get_ops_index() -> u16 {
    get_rand(0,100)%2
}

fn get_crdt_index(high: u16) -> u16 {
    get_rand(0, high-1)
}

fn get_rand(low: u16, high: u16) -> u16 {
    rand::thread_rng().gen_range(low..=high)
}



