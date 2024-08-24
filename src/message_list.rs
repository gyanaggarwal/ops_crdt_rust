use std::fmt;
use std::collections::HashMap;

use crate::vector_clock::{VectorClock, VectorClockError, VCOrdering};
use crate::message_data::NodeUpdateMsg;
use crate::{NodeType, LCType};

pub fn remove_causally_stable<OpsValue: fmt::Display+Clone>(cs_vc: &VectorClock, 
    msg_list: &HashMap<(NodeType, LCType), NodeUpdateMsg<OpsValue>>) -> 
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