/*
1. SDP behavior
2. anti entropy correctness
3. vector_clock msg generation
4. ignore duplicate msg
5. causally_stable - normal case
6. causally_stable - node not receiving user update
7. causally_stable - node receiving vector_clock msg
*/
use rand::prelude::*;

use ops_crdt_rust::crdt::{CrdtInstance, CrdtType};
use ops_crdt_rust::message_data::UserUpdateMsg;
use ops_crdt_rust::node_instance::NodeInstance;

fn main() {
    let ops_index = get_ops_index();
    let crdt_instance = get_crdt_instance();
    let mut ni0 = NodeInstance::new(0).unwrap();
    let mut ni1 = NodeInstance::new(1).unwrap();
    let mut ni2 = NodeInstance::new(2).unwrap();
   
    let ops_value = get_rand(1, 20) as i32;
    let ops_instance = if ops_index == 0 {ni0.get_add_ops_add_mult(ops_value)} else {ni0.get_mult_ops_add_mult(ops_value)};
    let user_update_msg = UserUpdateMsg::new(crdt_instance, ops_instance);
    let result = ni0.process_local_msg_add_mult(user_update_msg.clone()).unwrap();

    for (pnode, pmsg_list) in result {
        let _ = match pnode {
            0 => ni0.process_peer_msg_add_mult(pmsg_list),
            1 => ni1.process_peer_msg_add_mult(pmsg_list),
            2 => ni2.process_peer_msg_add_mult(pmsg_list),
            _ => panic!()};
     }

    println!("msg    => {:?}", user_update_msg);
    println!("\n\n");
    println!("ni0    => {:?}", ni0.add_mult_crdt);
    println!("\n\n");
    println!("ni1    => {:?}", ni1.add_mult_crdt);
    println!("\n\n");
    println!("ni2    => {:?}", ni2.add_mult_crdt);
}


fn get_crdt_instance() -> CrdtInstance {
    CrdtInstance::new(0, 1, CrdtType::AddMultCrdt)
}

fn get_ops_index() -> u16 {
    get_rand(0,100)%2
}

fn get_rand(low: u16, high: u16) -> u16 {
    rand::thread_rng().gen_range(low..=high)
}



