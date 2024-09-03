use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::Result;

use crate::{NodeType, PNCntOpsValue};
use crate::crdt::CRDT;
use crate::message_data::{NodeUpdateMsg, PeerNodeMsg, OpsInstance, SDPOpsType};
use crate::vector_clock::{VectorClockError, VCStatus};

#[derive(Debug)]
pub struct PNCounter;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PNCounterData {
    pcount: PNCntOpsValue,
    ncount: PNCntOpsValue
}
impl PNCounterData {
    pub fn new() -> Self {
        Self{pcount:0, ncount:0}
    }
}

impl CRDT<PNCounterData, PNCntOpsValue, PNCounter> {
    pub fn process_local_msg(&mut self, msg: NodeUpdateMsg<PNCntOpsValue>) -> 
        Result<HashMap<NodeType, Vec<PeerNodeMsg<PNCntOpsValue>>>, VectorClockError> {
        self.process_msg(&msg)?;
        self.general_process_local_msg(msg)
    }

    pub fn process_peer_msg(&mut self, pmsg_list: Vec<PeerNodeMsg<PNCntOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<PNCntOpsValue>>>, VectorClockError> {
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

    pub fn process_msg(&mut self, msg: &NodeUpdateMsg<PNCntOpsValue>) -> Result<(), VectorClockError>{
        match msg.user_update_msg.ops_instance.ops_type {
            SDPOpsType::SDPAdd  => self.crdt_value = 
                                   PNCounterData{pcount: self.crdt_value.pcount+msg.user_update_msg.ops_instance.ops_value,
                                                 ncount: self.crdt_value.ncount},
            SDPOpsType::SDPMult => self.crdt_value = 
                                   PNCounterData{pcount: self.crdt_value.pcount,
                                                 ncount: self.crdt_value.ncount+msg.user_update_msg.ops_instance.ops_value}
        };
        Ok(())
    }

    pub fn get_add_ops(&self, value: PNCntOpsValue) -> OpsInstance<PNCntOpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, value)
    }

    pub fn get_mult_ops(&self, value: PNCntOpsValue) -> OpsInstance<PNCntOpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, value)
    }
}

