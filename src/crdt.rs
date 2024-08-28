use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};

use crate::{CRDTNumType, LCType, NodeType, 
            PNCntOpsValue, 
            IntMultCrdtValue, IntMultOpsValue, 
            EDFlagCrdtValue, EDFlagOpsValue};
use crate::trcb;
use crate::vector_clock::VectorClockError;
use crate::message_data::{NodeUpdateMsg, NodeVectorClockMsg, SDPOpsType, OpsInstance};
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

    pub fn add_msg(&mut self, msg: NodeUpdateMsg<OpsValue>) -> Result<(), VectorClockError> {
        let lc = msg.node_vector_clock.vcmap.get(&msg.node).ok_or(VectorClockError::NodeNotFound)?;
        self.msg_list.insert((msg.node, lc.clone()), msg);
        Ok(())
    }

    pub fn process_vc_msg(&mut self, msg: NodeVectorClockMsg) -> Result<(), VectorClockError> {
        self.trcb.add_peer_vcmsg(msg.node, msg.node_vector_clock)
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

    //add vc_msg based on vc_msg_count
    pub fn send_msg_list(&mut self, _node: NodeType) -> Result<Vec<NodeUpdateMsg<OpsValue>>, VectorClockError> {
        todo!()
    }
}

// SDPAdd - get concurrent SDPMult msgs and add ops_value
// multiply with SDPAdd value then add it crdt_value
// let sum = a.iter().fold(0, |acc, x| acc + x);
// option = none
impl CRDT<IntMultCrdtValue, IntMultOpsValue, AddMult> {
    pub fn process_msg(&mut self, msg: &NodeUpdateMsg<i32>) -> Result<(), VectorClockError> {
        match msg.user_update_msg.ops_instance.ops {
            SDPOpsType::SDPAdd  => {let clist 
                                        = message_list::concurrent_msg_list(&msg.node_vector_clock, 
                                                                            &self.msg_list, None)?;
                                    let m = clist.iter().fold(1, |acc, cmsg| acc*cmsg.user_update_msg.ops_instance.ops_value);
                                    self.crdt_value += m*msg.user_update_msg.ops_instance.ops_value
                                   },
            SDPOpsType::SDPMult => self.crdt_value *= msg.user_update_msg.ops_instance.ops_value
        };
        Ok(())
    }

    pub fn get_option_value() -> Option<IntMultOpsValue> {
        None
    }

    pub fn get_add_ops(value: IntMultOpsValue) -> OpsInstance<IntMultOpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, value)
    }

    pub fn get_mult_ops(value: IntMultOpsValue) -> OpsInstance<IntMultOpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, value)
    }
}

// SDPAdd  - get concurrent SDPMult msgs if empty then disabled
// SDPAdd  - disable
// SDPMutl - enable
// option = Some(enable)
impl CRDT<EDFlagCrdtValue, EDFlagOpsValue, EWFlag> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<EDFlag>) {
        todo!();
    }

    pub fn get_option_value() -> Option<EDFlagOpsValue> {
        Some(EDFlag::Enabled)
    }

    pub fn get_add_ops() -> OpsInstance<EDFlagOpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, EDFlag::Disabled)
    }

    pub fn get_mult_ops() -> OpsInstance<EDFlagOpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, EDFlag::Enabled)
    }
}

// SDPAdd  - get concurrent SDPMult msgs if empty then enabled
// SDPAdd  - enable
// SDPMult - disable
// option = Some(disable)
impl CRDT<EDFlagCrdtValue, EDFlagOpsValue, DWFlag> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<EDFlag>) {
        todo!();
    }

    pub fn get_option_value() -> Option<EDFlagOpsValue> {
        Some(EDFlag::Disabled)
    }

    pub fn get_add_ops() -> OpsInstance<EDFlagOpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, EDFlag::Enabled)
    }

    pub fn get_mult_ops() -> OpsInstance<EDFlagOpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, EDFlag::Disabled)
    }
}

// SDPAdd - get concurrent SDPMult msg with value if empty then remove it
// option  = Some(value) of SDPMult
impl <OpsValue: Clone+PartialEq> CRDT<HashSet<OpsValue>, OpsValue, AWSet> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<OpsValue>) {
        todo!()
    }

    pub fn get_option_value(value: OpsValue) -> Option<OpsValue> {
        Some(value)
    }

    pub fn get_add_ops(value: OpsValue) -> OpsInstance<OpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, value)
    }

    pub fn get_mult_ops(value: OpsValue) -> OpsInstance<OpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, value)
    }
}

// SDPAdd - get concurrent SDPMult msg with value if empty then add it
// option = Some(value) of mult
impl <OpsValue: Clone+PartialEq> CRDT<HashSet<OpsValue>, OpsValue, RWSet> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<OpsValue>) {
        todo!()
    }

    pub fn get_option_value(value: OpsValue) -> Option<OpsValue> {
        Some(value)
    }

    pub fn get_add_ops(value: OpsValue) -> OpsInstance<OpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, value)
    }

    pub fn get_mult_ops(value: OpsValue) -> OpsInstance<OpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, value)
    }
}

impl CRDT<PNCounterData, PNCntOpsValue, PNCounter> {
    pub fn process_msg(&mut self, msg: &NodeUpdateMsg<PNCntOpsValue>) -> Result<(), VectorClockError>{
        match msg.user_update_msg.ops_instance.ops {
            SDPOpsType::SDPAdd  => self.crdt_value = 
                                   PNCounterData{pcount: self.crdt_value.pcount+msg.user_update_msg.ops_instance.ops_value,
                                                 ncount: self.crdt_value.ncount},
            SDPOpsType::SDPMult => self.crdt_value = 
                                   PNCounterData{pcount: self.crdt_value.pcount,
                                                 ncount: self.crdt_value.ncount+msg.user_update_msg.ops_instance.ops_value}
        };

        Ok(())
    }

    pub fn get_add_ops(value: PNCntOpsValue) -> OpsInstance<PNCntOpsValue> {
        OpsInstance::new(SDPOpsType::SDPAdd, value)
    }

    pub fn get_mult_ops(value: PNCntOpsValue) -> OpsInstance<PNCntOpsValue> {
        OpsInstance::new(SDPOpsType::SDPMult, value)
    }
}

