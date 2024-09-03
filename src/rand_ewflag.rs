use std::collections::HashMap;

use dotenvy::dotenv;

use crate::node_instance::NodeInstance;
use crate::constants::{TEST_MSG_COUNT, TEST_SLEEP_TIME_MS, NODE_LIST};
use crate::rand_crdt;
use crate::crdt::{CrdtInstance, CrdtType};
use crate::message_data::UserUpdateMsg;

pub fn test_random(){
    dotenv().ok();
    let msg_count = TEST_MSG_COUNT.to_owned();
    let msg_sleep_time = TEST_SLEEP_TIME_MS.to_owned();
    let node_list = NODE_LIST.to_owned().len() as u16;

    let crdt_instance = CrdtInstance::new_default(CrdtType::EWFlagCrdt);

    let mut ni0 = NodeInstance::new(0).unwrap();
    let mut ni1 = NodeInstance::new(1).unwrap();
    let mut ni2 = NodeInstance::new(2).unwrap();  
    let mut ni3 = NodeInstance::new(3).unwrap();
    let mut ni4 = NodeInstance::new(4).unwrap();  

    for _i in 0..msg_count {

        let ops_index = rand_crdt::get_bool_index();
        let node_index = rand_crdt::get_node_index(node_list);

        let ops_instance = if ops_index {ni0.ewflag_crdt.get_add_ops()} else {ni0.ewflag_crdt.get_mult_ops()};
        let user_update_msg = UserUpdateMsg::new(crdt_instance.clone(), ops_instance);

        let result = match node_index {
            0 => {let node_update_msg = ni0.ewflag_crdt.create_local_msg(user_update_msg).unwrap();
                      ni0.ewflag_crdt.process_local_msg(node_update_msg).unwrap()},
            1 => {let node_update_msg = ni1.ewflag_crdt.create_local_msg(user_update_msg).unwrap();
                      ni1.ewflag_crdt.process_local_msg(node_update_msg).unwrap()},
            2 => {let node_update_msg = ni2.ewflag_crdt.create_local_msg(user_update_msg).unwrap();
                      ni2.ewflag_crdt.process_local_msg(node_update_msg).unwrap()},
            3 => {let node_update_msg = ni3.ewflag_crdt.create_local_msg(user_update_msg).unwrap();
                      ni3.ewflag_crdt.process_local_msg(node_update_msg).unwrap()},
            4 => {let node_update_msg = ni4.ewflag_crdt.create_local_msg(user_update_msg).unwrap();
                      ni4.ewflag_crdt.process_local_msg(node_update_msg).unwrap()},
            _ => HashMap::new()
        };

        for (pnode, pmsg_list) in result {
            let _ = match pnode {
                0 => ni0.ewflag_crdt.process_peer_msg(pmsg_list).unwrap(),
                1 => ni1.ewflag_crdt.process_peer_msg(pmsg_list).unwrap(),
                2 => ni2.ewflag_crdt.process_peer_msg(pmsg_list).unwrap(),
                3 => ni3.ewflag_crdt.process_peer_msg(pmsg_list).unwrap(),
                4 => ni4.ewflag_crdt.process_peer_msg(pmsg_list).unwrap(),
                _ => HashMap::new()};
        }  

        rand_crdt::msg_sleep(msg_sleep_time);
    }

    println!("\nni0 node {:?} len {:?} value {:?} trcb {:?}",
        ni0.ewflag_crdt.get_node(), ni0.ewflag_crdt.msg_list_len(), 
        ni0.ewflag_crdt.crdt_value, ni0.ewflag_crdt.trcb);
    println!("\nni1 node {:?} len {:?} value {:?} trcb {:?}",
        ni1.ewflag_crdt.get_node(), ni1.ewflag_crdt.msg_list_len(), 
        ni1.ewflag_crdt.crdt_value, ni1.ewflag_crdt.trcb);
    println!("\nni2 node {:?} len {:?} value {:?} trcb {:?}", 
        ni2.ewflag_crdt.get_node(), ni2.ewflag_crdt.msg_list_len(), 
        ni2.ewflag_crdt.crdt_value, ni2.ewflag_crdt.trcb);
    println!("\nni3 node {:?} len {:?} value {:?} trcb {:?}",
        ni3.ewflag_crdt.get_node(), ni3.ewflag_crdt.msg_list_len(), 
        ni3.ewflag_crdt.crdt_value, ni3.ewflag_crdt.trcb);
    println!("\nni4 node {:?} len {:?} value {:?} trcb {:?}",
        ni4.ewflag_crdt.get_node(), ni4.ewflag_crdt.msg_list_len(), 
        ni4.ewflag_crdt.crdt_value, ni4.ewflag_crdt.trcb);
}