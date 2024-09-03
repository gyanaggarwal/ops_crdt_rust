use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::Result;

use crate::NodeType;

use crate::crdt::CRDT;
use crate::{EDFlagCrdtValue, EDFlagOpsValue};
use crate::message_data::{NodeUpdateMsg, PeerNodeMsg, OpsInstance, SDPOpsType};
use crate::vector_clock::{VCStatus, VectorClockError};
use crate::message_list;

#[derive(Debug)]
pub struct EWFlag;
#[derive(Debug)]
pub struct DWFlag;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EDFlag {
    Enabled,
    Disabled
}

impl CRDT<EDFlagCrdtValue, EDFlagOpsValue, EWFlag> {
    pub fn process_local_msg(&mut self, msg: NodeUpdateMsg<EDFlagOpsValue>) -> 
        Result<HashMap<NodeType, Vec<PeerNodeMsg<EDFlagOpsValue>>>, VectorClockError> {
        self.process_msg(&msg)?;
        self.general_process_local_msg(msg)
    }

    pub fn process_peer_msg(&mut self, pmsg_list: Vec<PeerNodeMsg<EDFlagOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<EDFlagOpsValue>>>, VectorClockError> {
        for msg in pmsg_list {
            match msg {
                PeerNodeMsg::VectorClockNodeMsg(vmsg) =>  
                    self.general_process_vc_msg(vmsg)?,
                PeerNodeMsg::UpdateNodeMsg(umsg)   =>  
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

    pub fn process_msg(&mut self, msg: &NodeUpdateMsg<EDFlag>) -> Result<(), VectorClockError> {
        match msg.user_update_msg.ops_instance.ops_type {
            SDPOpsType::SDPAdd  =>      {   let clist 
                                                = message_list::concurrent_msg_list(&msg.node_vector_clock, 
                                                    &self.msg_list, self.get_option_value())?;
                                            if clist.len() == 0 {
                                                self.crdt_value = msg.user_update_msg.ops_instance.ops_value.clone();
                                            }
                                        },
            SDPOpsType::SDPMult =>      self.crdt_value = msg.user_update_msg.ops_instance.ops_value.clone()
        };
        Ok(())
    }

    pub fn get_option_value(&self) -> Option<EDFlagOpsValue> {
        Some(EDFlag::Enabled)
    }

    pub fn get_add_ops(&self) -> OpsInstance<EDFlagOpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, EDFlag::Disabled)
    }

    pub fn get_mult_ops(&self) -> OpsInstance<EDFlagOpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, EDFlag::Enabled)
    }
}

impl CRDT<EDFlagCrdtValue, EDFlagOpsValue, DWFlag> {
    pub fn process_local_msg(&mut self, msg: NodeUpdateMsg<EDFlagOpsValue>) -> 
        Result<HashMap<NodeType, Vec<PeerNodeMsg<EDFlagOpsValue>>>, VectorClockError> {
        self.process_msg(&msg)?;
        self.general_process_local_msg(msg)
    }

    pub fn process_peer_msg(&mut self, pmsg_list: Vec<PeerNodeMsg<EDFlagOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<EDFlagOpsValue>>>, VectorClockError> {
        for msg in pmsg_list {
            match msg {
                PeerNodeMsg::VectorClockNodeMsg(vmsg) =>  
                    self.general_process_vc_msg(vmsg)?,
                PeerNodeMsg::UpdateNodeMsg(umsg)   =>  
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

    pub fn process_msg(&mut self, msg: &NodeUpdateMsg<EDFlag>) -> Result<(), VectorClockError>{
        match msg.user_update_msg.ops_instance.ops_type {
            SDPOpsType::SDPAdd  =>  {   let clist 
                                            = message_list::concurrent_msg_list(&msg.node_vector_clock, 
                                                    &self.msg_list, self.get_option_value())?;
                                        if clist.len() == 0 {
                                            self.crdt_value = msg.user_update_msg.ops_instance.ops_value.clone();
                                        }
                                   },
            SDPOpsType::SDPMult =>  self.crdt_value = msg.user_update_msg.ops_instance.ops_value.clone()
        };
        Ok(())
    }

    pub fn get_option_value(&self) -> Option<EDFlagOpsValue> {
        Some(EDFlag::Disabled)
    }

    pub fn get_add_ops(&self) -> OpsInstance<EDFlagOpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, EDFlag::Enabled)
    }

    pub fn get_mult_ops(&self) -> OpsInstance<EDFlagOpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, EDFlag::Disabled)
    }
}


