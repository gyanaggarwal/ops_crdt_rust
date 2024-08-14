// struct
// next_cd
// replace
// causally_stable
// new_causally_stable

use std::collections::HashMap;
use crate::NodeType;
use crate::vector_clock::{VectorClock, VectorClockError};

#[derive(Debug)]
pub struct TRCBData {
    pub node: NodeType,
    pub node_vector_clock: VectorClock,
    pub node_causally_stable_vc: VectorClock,
    pub new_causally_stable_vc: bool,
    pub node_trcb: HashMap<NodeType, VectorClock>
}

impl TRCBData {
    pub fn new(node: NodeType, node_list: Vec<NodeType>) -> Result<Self, VectorClockError> {
        let vc = VectorClock::new(node_list.clone())?;

        if !&node_list.contains(&node) {
            return Err(VectorClockError::InconsistentInputTRBC(node, node_list));
        }

        let mut node_trcb = HashMap::new();

        for pnode in node_list {
            if pnode != node {
                node_trcb.insert(pnode, vc.clone());
            }
        }
        
        Ok(Self {
            node,
            node_vector_clock: vc.clone(),
            node_causally_stable_vc: vc,
            new_causally_stable_vc: false,
            node_trcb
        })
    }
}