use std::collections::HashMap;

use crate::NodeType;
use crate::message_data::{PeerNodeMsg, NodeVectorClockMsg};
use crate::crdt::CRDT;
use crate::vector_clock::VectorClockError;

impl <CrdtValue: Clone, OpsValue: Clone+PartialEq, State> CRDT<CrdtValue, OpsValue, State> {
    pub fn create_peer_msg_list(&self, msg_flag: bool) -> 
        Result<HashMap<NodeType, Vec<PeerNodeMsg<OpsValue>>>, VectorClockError> {
        let mut msg_map = HashMap::<NodeType, Vec<PeerNodeMsg<OpsValue>>>::new();
        let msg_vec = Vec::<PeerNodeMsg<OpsValue>>::new();
        let vc_flag = !msg_flag && self.msg_count_vc >= self.max_msg_count_vc;
        if vc_flag || msg_flag {
            let node_trcb = self.trcb.node_trcb.clone();
            for (pnode_key, pvc) in node_trcb {
                let mut msg_vec1 = msg_vec.clone();
                if vc_flag {
                    let vc_msg = PeerNodeMsg::VectorClockNodeMsg(NodeVectorClockMsg::new(self.trcb.node, self.trcb.node_vector_clock.clone()));
                    msg_vec1.push(vc_msg);
                }

                if msg_flag {
                    for (pvc_node_key, pvc_lc) in pvc.vcmap {
                        if pnode_key != pvc_node_key {
                            let lc0 = self.trcb.node_vector_clock.vcmap.get(&pvc_node_key).ok_or(VectorClockError::UnexpectedError)?;
                            for lc1 in pvc_lc+1..=*lc0 {
                                let msg_key = (pvc_node_key, lc1);
                                let msg = self.msg_list.get(&msg_key).ok_or(VectorClockError::UnexpectedError)?;
                                let msg = msg.clone();
                                msg_vec1.push(PeerNodeMsg::UpdateNodeMsg(msg));
                            }
                        }
                    }
                }
                msg_map.insert(pnode_key, msg_vec1);
            }
        }

        Ok(msg_map)
    }
}


