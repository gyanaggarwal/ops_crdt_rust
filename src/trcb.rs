use std::collections::HashMap;
use std::cmp;
use anyhow::Result;

use crate::{LCType, NodeType};
use crate::vector_clock::{VectorClock, VectorClockError, VCStatus};

#[derive(Debug)]
pub struct TRCBData {
    pub node: NodeType,
    pub node_vector_clock: VectorClock,
    pub node_trcb: HashMap<NodeType, VectorClock>
}

impl TRCBData {
    pub fn new(node: NodeType, node_list: Vec<NodeType>) -> Result<Self, VectorClockError> {
        if !&node_list.contains(&node) {
            return Err(VectorClockError::InconsistentInputTRBC(node, node_list));
        }

        let node_vector_clock = VectorClock::new(node_list.clone())?;

        let mut node_trcb = HashMap::new();

        for pnode in node_list {
            if pnode != node {
                node_trcb.insert(pnode, node_vector_clock.clone());
            }
        }
        
        Ok(Self {
            node,
            node_vector_clock,
            node_trcb
        })
    }

    pub fn next_vc(&mut self) -> Result<VectorClock, VectorClockError> {
        self.node_vector_clock.next_vc(&self.node)?;
        Ok(self.node_vector_clock.clone())
    }

    pub fn add_peer_vc(&mut self, peer_node: NodeType, peer_vc: VectorClock) -> Result<VCStatus, VectorClockError> {
        let peer_vc_status = self.node_vector_clock.is_next_vc(&peer_node, &peer_vc)?;

        if peer_vc_status == VCStatus::INORDER {
            self.node_vector_clock.next_vc(&peer_node)?;
            self.node_trcb.insert(peer_node, peer_vc);
        }
    
        Ok(peer_vc_status)
    }

    pub fn add_peer_vcmsg(&mut self, peer_node: NodeType, peer_vc: VectorClock) -> Result<(), VectorClockError> {
        let cvc = self.node_trcb.get(&peer_node).ok_or(VectorClockError::NodeNotFound)?;
        let mvc = cvc.max_vc(&peer_vc)?;
        self.node_trcb.insert(peer_node, mvc);
        Ok(())
    }

    pub fn causally_stable(&self) -> Result<VectorClock, VectorClockError> {
        let mut cs_map: HashMap<NodeType, LCType> = HashMap::new();

        for (nnode, nlc) in self.node_vector_clock.vcmap.iter() {
            let mut mlc = nlc;
            for (_, pvc) in &self.node_trcb {
                let plc = pvc.vcmap.get(nnode).ok_or(VectorClockError::UnexpectedError("trcb.causally_stable 67".to_owned()))?;
                mlc = cmp::min(mlc, plc);
            }
            cs_map.insert(*nnode, *mlc);
        };

        Ok(VectorClock{vcmap: cs_map})
    }
}