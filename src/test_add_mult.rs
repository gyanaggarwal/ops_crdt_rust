use std::collections::HashMap;

use crate::message_data::UserUpdateMsg;
use crate::node_instance::{NodeInstance, get_rand};

// SDP behavior
// causally_stable - normal case
pub fn test1() {
    let mut ni0 = NodeInstance::new(0).unwrap();
    let mut ni1 = NodeInstance::new(1).unwrap();
    let mut ni2 = NodeInstance::new(2).unwrap();

    let crdt_instance = ni0.get_crdt_instance_add_mult();  

    let ops_value0 = get_rand(1, 20) as i32;
    let ops_instance0 = ni0.get_add_ops_add_mult(ops_value0);
    let user_update_msg0 = UserUpdateMsg::new(crdt_instance.clone(), ops_instance0);
    let result0 = ni0.process_local_msg_add_mult(user_update_msg0.clone()).unwrap();

    let ops_value1 = get_rand(1, 20) as i32;
    let ops_instance1 = ni1.get_mult_ops_add_mult(ops_value1);
    let user_update_msg1 = UserUpdateMsg::new(crdt_instance.clone(), ops_instance1);
    let result1 = ni1.process_local_msg_add_mult(user_update_msg1.clone()).unwrap();

    let ops_value2 = get_rand(1, 20) as i32;
    let ops_instance2 = ni2.get_add_ops_add_mult(ops_value2);
    let user_update_msg2 = UserUpdateMsg::new(crdt_instance.clone(), ops_instance2);
    let result2 = ni2.process_local_msg_add_mult(user_update_msg2.clone()).unwrap();

    for (pnode, pmsg_list) in result0 {
        let _ = match pnode {
            0 => ni0.process_peer_msg_add_mult(pmsg_list).unwrap(),
            1 => ni1.process_peer_msg_add_mult(pmsg_list).unwrap(),
            2 => ni2.process_peer_msg_add_mult(pmsg_list).unwrap(),
            _ => HashMap::new()};
    }  
 
    for (pnode, pmsg_list) in result1 {
        let _ = match pnode {
            0 => ni0.process_peer_msg_add_mult(pmsg_list).unwrap(),
            1 => ni1.process_peer_msg_add_mult(pmsg_list).unwrap(),
            2 => ni2.process_peer_msg_add_mult(pmsg_list).unwrap(),
            _ => HashMap::new()};
    }

    for (pnode, pmsg_list) in result2 {
        let _ = match pnode {
            0 => ni0.process_peer_msg_add_mult(pmsg_list).unwrap(),
            1 => ni1.process_peer_msg_add_mult(pmsg_list).unwrap(),
            2 => ni2.process_peer_msg_add_mult(pmsg_list).unwrap(),
            _ => HashMap::new()};
    }

    println!("\n\nni0 user_update_msg {:?}", user_update_msg0);
    println!("\nni0 query           {:?}", ni0.query_add_mult());
    println!("\nni0 crdt            {:?}", ni0.add_mult_crdt);

    println!("\n\nni1 user_update_msg {:?}", user_update_msg1);
    println!("\nni1 query           {:?}", ni1.query_add_mult());
    println!("\nni1 crdt            {:?}", ni1.add_mult_crdt);

    println!("\n\nni2 user_update_msg {:?}", user_update_msg2);
    println!("\nni2 query           {:?}", ni2.query_add_mult());
    println!("\nni2 crdt            {:?}", ni2.add_mult_crdt);
}