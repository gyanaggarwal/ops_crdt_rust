use std::collections::HashMap;
use std::fmt::Display;

use crate::NodeType;
use crate::message_data::{PeerNodeMsg, NodeVectorClockMsg};
use crate::crdt::CRDT;
use crate::vector_clock::VectorClockError;

impl <CrdtValue: Clone, OpsValue: Display+Clone, State> CRDT<CrdtValue, OpsValue, State> {
    pub fn create_peer_msg_list(&self, msg_flag: bool) -> 
        Result<HashMap<NodeType, Vec<PeerNodeMsg<OpsValue>>>, VectorClockError> {
        let mut msg_map = HashMap::<NodeType, Vec<PeerNodeMsg<OpsValue>>>::new();
        let vc_flag = self.msg_count_vc >= self.max_msg_count_vc;

        if vc_flag || msg_flag {
            if vc_flag {
                for node_key in self.trcb.node_trcb.keys() {
                    let vc_msg = PeerNodeMsg::VectorClockNodeMsg(NodeVectorClockMsg::new(self.trcb.node, self.trcb.node_vector_clock.clone()));
                    let mut msg_vec = Vec::<PeerNodeMsg<OpsValue>>::new();
                    msg_vec.push(vc_msg);
                    msg_map.insert(*node_key, msg_vec);
                }
            } 

            for (node_key, lc1) in self.trcb.node_vector_clock.vcmap.iter() {
                if *node_key != self.trcb.node || msg_flag {
                    let pvc = self.trcb.node_trcb.get(node_key).ok_or(VectorClockError::NodeNotFound)?;
                    for (onode_key, lc2) in pvc.vcmap.iter() {
                        if *onode_key != self.trcb.node && *onode_key != *node_key && lc1 > lc2 {
                            let mut omsg_vec = msg_map.get(onode_key).ok_or(VectorClockError::NodeNotFound)?;

                        }
                    }

                }

            }

        }

        Ok(msg_map)
        
    } 


/*    pub fn create_peer_msg_list(&self, mut msg_map: HashMap<NodeType, Vec<PeerNodeMsg<OpsValue>>>) -> 
        Result<(), VectorClockError> {
        todo!()
    }

    pub fn create_peer_vc_msg_list(&self, mut msg_map: HashMap<NodeType, Vec<PeerNodeMsg<OpsValue>>>) ->
        Result<(), VectorClockError> {
            if self.msg_count_vc >= self.max_msg_count_vc {
                self.add_peer_vc_msg(msg_map);
                self.create_peer_msg_list(msg_map)
            } else {
                Ok(())
            }
    }

    pub fn add_peer_vc_msg(&self, mut msg_map: HashMap<NodeType, Vec<PeerNodeMsg<OpsValue>>>) {
        for node_key in self.trcb.node_trcb.keys() {
            let vc_msg = PeerNodeMsg::VectorClockNodeMsg(NodeVectorClockMsg::new(self.trcb.node, self.trcb.node_vector_clock.clone()));
            let mut msg_vec = Vec::<PeerNodeMsg<OpsValue>>::new();
            msg_vec.push(vc_msg);
            msg_map.insert(*node_key, msg_vec);
        }
    }
*/
}
