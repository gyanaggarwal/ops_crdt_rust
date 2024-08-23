use std::fmt;

use crate::vector_clock::{VectorClock, VectorClockError, VCOrdering};
use crate::message_data::NodeUpdateMsg;

pub fn remove_causally_stable<OpsValue: fmt::Display+Clone>(cs_vc: &VectorClock, msg_list: Vec<NodeUpdateMsg<OpsValue>>) 
        -> Result<Vec<NodeUpdateMsg<OpsValue>>, VectorClockError> {

    let mut new_list = Vec::new();

    for msg in msg_list {
        let cmp_csvc = cs_vc.cmp_vc(&msg.node_vector_clock)?;
        if cmp_csvc == VCOrdering::VCCN || cmp_csvc == VCOrdering::VCGR {
            new_list.push(msg);
        }
    }
    Ok(new_list)    
}