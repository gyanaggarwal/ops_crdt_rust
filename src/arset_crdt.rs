use std::collections::{HashMap, HashSet};
use anyhow::Result;

use crate::{NodeType, ARSetOpsValue};
use crate::crdt::CRDT;
use crate::message_list;
use crate::message_data::{NodeUpdateMsg, PeerNodeMsg, OpsInstance, SDPOpsType};
use crate::vector_clock::{VCStatus, VectorClockError};

#[derive(Debug)]
pub struct AWSet;
#[derive(Debug)]
pub struct RWSet;

impl CRDT<HashSet<ARSetOpsValue>, ARSetOpsValue, AWSet> {
    pub fn process_local_msg(&mut self, msg: NodeUpdateMsg<ARSetOpsValue>) -> 
        Result<HashMap<NodeType, Vec<PeerNodeMsg<ARSetOpsValue>>>, VectorClockError> {
        self.process_msg(&msg)?;
        self.general_process_local_msg(msg)
     }

    pub fn process_peer_msg(&mut self, pmsg_list: Vec<PeerNodeMsg<ARSetOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<ARSetOpsValue>>>, VectorClockError> {
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

    pub fn process_msg(&mut self, msg: &NodeUpdateMsg<ARSetOpsValue>) -> Result<(), VectorClockError>{
        let value = msg.user_update_msg.ops_instance.ops_value.clone();
        match msg.user_update_msg.ops_instance.ops_type {
            SDPOpsType::SDPAdd  =>  {   let clist 
                                            = message_list::concurrent_msg_list(&msg.node_vector_clock, 
                                                    &self.msg_list, self.get_option_value(value.clone()))?;
                                        if clist.len() == 0 {
                                            self.crdt_value.remove(&value);
                                        };
                                        true
                                    }
            SDPOpsType::SDPMult =>  self.crdt_value.insert(value.clone())
        };
        Ok(())
    }

    pub fn get_option_value(&self, value: ARSetOpsValue) -> Option<ARSetOpsValue> {
        Some(value)
    }

    pub fn get_add_ops(&self, value: ARSetOpsValue) -> OpsInstance<ARSetOpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, value)
    }

    pub fn get_mult_ops(&self, value: ARSetOpsValue) -> OpsInstance<ARSetOpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, value)
    }
}

impl CRDT<HashSet<ARSetOpsValue>, ARSetOpsValue, RWSet> {
    pub fn process_local_msg(&mut self, msg: NodeUpdateMsg<ARSetOpsValue>) -> 
        Result<HashMap<NodeType, Vec<PeerNodeMsg<ARSetOpsValue>>>, VectorClockError> {
        self.process_msg(&msg)?;
        self.general_process_local_msg(msg)
     }

    pub fn process_peer_msg(&mut self, pmsg_list: Vec<PeerNodeMsg<ARSetOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<ARSetOpsValue>>>, VectorClockError> {
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

    pub fn process_msg(&mut self, msg: &NodeUpdateMsg<ARSetOpsValue>)  -> Result<(), VectorClockError>{
        let value = msg.user_update_msg.ops_instance.ops_value.clone();
        match msg.user_update_msg.ops_instance.ops_type {
            SDPOpsType::SDPAdd  =>  {   let clist 
                                            = message_list::concurrent_msg_list(&msg.node_vector_clock, 
                                                    &self.msg_list, self.get_option_value(value.clone()))?;
                                        if clist.len() == 0 {
                                            self.crdt_value.insert(value.clone());
                                        };
                                        true
                                    }
            SDPOpsType::SDPMult =>  self.crdt_value.remove(&value)
        };
        Ok(())
    }

    pub fn get_option_value(&self, value: ARSetOpsValue) -> Option<ARSetOpsValue> {
        Some(value)
    }

    pub fn get_add_ops(&self, value: ARSetOpsValue) -> OpsInstance<ARSetOpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, value)
    }

    pub fn get_mult_ops(&self, value: ARSetOpsValue) -> OpsInstance<ARSetOpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, value)
    }
}
