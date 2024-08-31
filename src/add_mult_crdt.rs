use std::collections::HashMap;

use crate::{NodeType, IntMultCrdtValue, IntMultOpsValue};
use crate::crdt::CRDT;
use crate::message_data::{NodeUpdateMsg, PeerNodeMsg, SDPOpsType, OpsInstance};
use crate::vector_clock::{VCStatus, VectorClockError};
use crate::message_list;

#[derive(Debug)]
pub struct AddMult;

impl CRDT<IntMultCrdtValue, IntMultOpsValue, AddMult> {
    pub fn process_local_msg(&mut self, msg: NodeUpdateMsg<IntMultOpsValue>) -> 
        Result<HashMap<NodeType, Vec<PeerNodeMsg<IntMultOpsValue>>>, VectorClockError> {
        self.process_msg(&msg)?;
        self.general_process_local_msg(msg)
    }

    pub fn process_peer_msg(&mut self, pmsg_list: Vec<PeerNodeMsg<IntMultOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<IntMultOpsValue>>>, VectorClockError> {
        for msg in pmsg_list {
            match msg {
                PeerNodeMsg::VectorClockNodeMsg(vmsg) =>  
                    self.general_process_vc_msg(vmsg)?,
                PeerNodeMsg::UpdateNodeMsg(umsg)      =>  
                    {   let vc_status = self.general_process_peer_msg(umsg.clone())?;
                        if vc_status == VCStatus::INORDER {
                            self.process_msg(&umsg)?
                        }
                    }
            }
        }
        self.causally_stable()?;
        let msg_list = self.create_peer_msg_list(false)?;
        if msg_list.len() > 0 {
            self.msg_count_vc = 0;
        }
        Ok(msg_list)
    }

    pub fn process_msg(&mut self, msg: &NodeUpdateMsg<IntMultOpsValue>) -> Result<(), VectorClockError> {
        match msg.user_update_msg.ops_instance.ops_type {
            SDPOpsType::SDPAdd  =>  {   let clist 
                                            = message_list::concurrent_msg_list(&msg.node_vector_clock, 
                                                    &self.msg_list, self.get_option_value())?;
                                        let m = clist.iter()
                                                        .fold(1, 
                                                           |acc, cmsg| 
                                                              acc*cmsg.user_update_msg.ops_instance.ops_value);
                                        self.crdt_value += m*msg.user_update_msg.ops_instance.ops_value
                                    },
            SDPOpsType::SDPMult =>  self.crdt_value *= msg.user_update_msg.ops_instance.ops_value
        };
        Ok(())
    }

    pub fn get_option_value(&self) -> Option<IntMultOpsValue> {
        None
    }

    pub fn get_add_ops(&self, value: IntMultOpsValue) -> OpsInstance<IntMultOpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, value)
    }

    pub fn get_mult_ops(&self, value: IntMultOpsValue) -> OpsInstance<IntMultOpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, value)
    }
}

