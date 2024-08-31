use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};

use crate::{ARSetOpsValue, 
            EDFlagCrdtValue, 
            EDFlagOpsValue, 
            IntMultCrdtValue, 
            IntMultOpsValue, 
            PNCntOpsValue,
            LCType, 
            NodeType, 
            CRDTNumType};
use crate::trcb;
use crate::vector_clock::{VCStatus, VectorClockError, VectorClock, peer_vc_status};
use crate::message_data::{NodeUpdateMsg, 
                          NodeVectorClockMsg, 
                          PeerNodeMsg, 
                          SDPOpsType, 
                          OpsInstance, 
                          UserUpdateMsg};
use crate::message_list;
use crate::constants::{MAX_MSG_COUNT_CS, MAX_MSG_COUNT_VC, NODE_LIST};

#[derive(Debug)]
pub struct AddMult;
#[derive(Debug)]
pub struct EWFlag;
#[derive(Debug)]
pub struct DWFlag;
#[derive(Debug)]
pub struct AWSet;
#[derive(Debug)]
pub struct RWSet;
#[derive(Debug)]
pub struct PNCounter;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CrdtType {
    AddMultCrdt,
    EWFlagCrdt,
    DWFlagCrdt,
    AWSetCrdt,
    RWSetCrdt,
    PNCounterCrdt
}

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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EDFlag {
    Enabled,
    Disabled
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrdtInstance {
    instance_node_id: NodeType,
    instance_num: CRDTNumType,
    instance_type: CrdtType
}
impl CrdtInstance {
    pub fn new(instance_node_id: NodeType, instance_num: CRDTNumType, instance_type: CrdtType) -> Self {
        Self{instance_node_id, instance_num, instance_type}
    }
}

#[derive(Debug)]
pub struct CRDT <CrdtValue: Clone, OpsValue: Clone+PartialEq, State> {
    pub trcb: trcb::TRCBData,
    pub msg_list: HashMap<(NodeType, LCType), NodeUpdateMsg<OpsValue>>,
    pub crdt_value: CrdtValue,
    pub max_msg_count_vc: u16,
    pub max_msg_count_cs: u16,
    pub msg_count_vc: u16,
    pub msg_count_cs: u16,
    pub state: std::marker::PhantomData<State>
}

impl <CrdtValue: Clone, OpsValue: Clone+PartialEq, State> CRDT<CrdtValue, OpsValue, State> {
    pub fn new(node: NodeType, crdt_value: CrdtValue) -> Result<Self, VectorClockError> {
        let trcb = trcb::TRCBData::new(node, NODE_LIST.to_owned().clone())?;
        let msg_list = HashMap::new();
        Ok(Self{trcb, 
                msg_list, 
                crdt_value, 
                max_msg_count_vc: MAX_MSG_COUNT_VC.to_owned(),
                max_msg_count_cs: MAX_MSG_COUNT_CS.to_owned(),
                msg_count_vc: 0,
                msg_count_cs: 0,
                state: std::marker::PhantomData::<State>})
    }

    pub fn next_vc(&mut self) -> Result<VectorClock, VectorClockError> {
        self.trcb.next_vc()
    }

    pub fn get_node(&self) -> NodeType {
        self.trcb.node.clone()
    }

    pub fn create_local_msg(&mut self, user_update_msg: UserUpdateMsg<OpsValue>) -> 
        Result<NodeUpdateMsg<OpsValue>, VectorClockError> {
        let node = self.get_node();
        let node_vector_clock = self.next_vc()?.clone();
        Ok(NodeUpdateMsg::new(node, node_vector_clock, user_update_msg))
    }

    pub fn add_msg(&mut self, msg: NodeUpdateMsg<OpsValue>) -> Result<(), VectorClockError> {
        let lc = msg.node_vector_clock.vcmap.get(&msg.node).ok_or(VectorClockError::NodeNotFound)?;
        self.msg_list.insert((msg.node, lc.clone()), msg);
        Ok(())
    }

    pub fn general_process_local_msg(&mut self, msg: NodeUpdateMsg<OpsValue>) -> 
        Result<HashMap<NodeType, Vec<PeerNodeMsg<OpsValue>>>, VectorClockError> {
        self.msg_count_vc = 0;
        self.msg_count_cs += 1;
        self.add_msg(msg.clone())?;
        self.causally_stable()?;         
        self.create_peer_msg_list(true)
    }

    pub fn general_process_peer_msg(&mut self, msg: NodeUpdateMsg<OpsValue>) -> Result<VCStatus, VectorClockError>  {
        self.msg_count_vc += 1;
        let vc_ord = self.trcb.node_vector_clock.check_vc(msg.node, &msg.node_vector_clock)?;
        let vc_status = peer_vc_status(vc_ord);
    
        if vc_status == VCStatus::INORDER {
            self.msg_count_cs += 1;
            self.add_msg(msg.clone())?;
            self.trcb.add_peer_vc(msg.node, msg.node_vector_clock.clone())?;
        }
        Ok(vc_status)
    }
    pub fn general_process_vc_msg(&mut self, msg: NodeVectorClockMsg) -> Result<(), VectorClockError> {
        self.trcb.add_peer_vcmsg(msg.node, msg.node_vector_clock.clone())
    }

    pub fn causally_stable(&mut self) -> Result<(), VectorClockError> {
        if self.msg_count_cs >= self.max_msg_count_cs{
            let cs_vc = self.trcb.causally_stable()?;
            let new_list = message_list::remove_causally_stable(&cs_vc, &self.msg_list)?;
            self.msg_list = new_list;
            self.msg_count_cs = 0;
        }
        Ok(())
    }

    pub fn query(&self) -> CrdtValue {
        self.crdt_value.clone()
    }
}

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
                PeerNodeMsg::VectorClockNodeMsg(vmsg)   =>  self.general_process_vc_msg(vmsg)?,
                PeerNodeMsg::UpdateNodeMsg(umsg)        =>  {   let vc_status = self.general_process_peer_msg(umsg.clone())?;
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
                                        let m = clist.iter().fold(1, |acc, cmsg| acc*cmsg.user_update_msg.ops_instance.ops_value);
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
                PeerNodeMsg::VectorClockNodeMsg(vmsg)   =>  self.general_process_vc_msg(vmsg)?,
                PeerNodeMsg::UpdateNodeMsg(umsg)     =>  {   let vc_status = self.general_process_peer_msg(umsg.clone())?;
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
                PeerNodeMsg::VectorClockNodeMsg(vmsg)   =>  self.general_process_vc_msg(vmsg)?,
                PeerNodeMsg::UpdateNodeMsg(umsg)     =>  {   let vc_status = self.general_process_peer_msg(umsg.clone())?;
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
                PeerNodeMsg::VectorClockNodeMsg(vmsg)   =>  self.general_process_vc_msg(vmsg)?,
                PeerNodeMsg::UpdateNodeMsg(umsg)   =>  {   let vc_status = self.general_process_peer_msg(umsg.clone())?;
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
                PeerNodeMsg::VectorClockNodeMsg(vmsg)   =>  self.general_process_vc_msg(vmsg)?,
                PeerNodeMsg::UpdateNodeMsg(umsg)   =>  {   let vc_status = self.general_process_peer_msg(umsg.clone())?;
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
                PeerNodeMsg::VectorClockNodeMsg(vmsg) =>    self.general_process_vc_msg(vmsg)?,
                PeerNodeMsg::UpdateNodeMsg(umsg)      =>    {   let vc_status = self.general_process_peer_msg(umsg.clone())?;
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

