use std::collections::HashMap;
use anyhow::Result;

use crate::vector_clock::{VectorClock, VectorClockError, VCOrdering};
use crate::message_data::NodeUpdateMsg;
use crate::{NodeType, LCType};

pub fn remove_causally_stable<OpsValue: Clone+PartialEq>
    (cs_vc: &VectorClock, msg_list: &HashMap<(NodeType, LCType), NodeUpdateMsg<OpsValue>>) -> 
    Result<HashMap<(NodeType, LCType), NodeUpdateMsg<OpsValue>>, VectorClockError> {

    let mut new_list = HashMap::new();

    for (key, msg) in msg_list.iter() {
        let cmp_csvc = cs_vc.cmp_vc(&msg.node_vector_clock)?;
        if cmp_csvc == VCOrdering::VCCN || cmp_csvc == VCOrdering::VCGR {
            new_list.insert(*key, msg.clone());
        }
    }
    Ok(new_list)    
}

pub fn concurrent_msg_list<OpsValue: Clone+PartialEq>
    (msg_vc: &VectorClock, msg_list: &HashMap<(NodeType, LCType), NodeUpdateMsg<OpsValue>>, check_value: Option<OpsValue>) ->
    Result<Vec<NodeUpdateMsg<OpsValue>>, VectorClockError> {

    let mut clist = Vec::new();

    for msg in msg_list.values() {
        let cmp_msg = msg_vc.cmp_vc(&msg.node_vector_clock)? == VCOrdering::VCCN;
        let check_msg = msg.user_update_msg.ops_instance.check(&check_value);
        if cmp_msg && check_msg {
            clist.push(msg.clone());
        }
    }

    Ok(clist)
}