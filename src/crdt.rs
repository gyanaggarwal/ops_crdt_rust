use serde::{Serialize, Deserialize};

use crate::{NodeType, CRDTNumType};
use crate::trcb;
use crate::vector_clock::VectorClockError;
use crate::message_data::{NodeUpdateMsg, NodeVectorClockMsg};

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
    PNCounterCrdt
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
pub struct CRDT <CrdtValue, OpsValue, State> {
    pub trcb: trcb::TRCBData,
    pub msg_list: Vec<NodeUpdateMsg<OpsValue>>,
    pub crdt_value: CrdtValue,
    pub state: std::marker::PhantomData<State>
}

impl <CrdtValue, OpsValue, State> CRDT<CrdtValue, OpsValue, State> {
    pub fn new(node: NodeType, node_list: Vec<NodeType>, crdt_value: CrdtValue) -> Result<Self, VectorClockError> {
        let trcb = trcb::TRCBData::new(node, node_list)?;
        let msg_list = Vec::new();
        Ok(Self{trcb, msg_list, crdt_value, state: std::marker::PhantomData::<State>})
    }

    pub fn add_msg(&mut self, msg: NodeUpdateMsg<OpsValue>) {
        self.msg_list.push(msg);
    }

    pub fn process_vc_msg(&mut self, _msg: NodeVectorClockMsg) {
        todo!();
    }
}

impl <CrdtValue, OpsValue> CRDT<CrdtValue, OpsValue, AddMult> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<OpsValue>) {
        todo!();
    }
}

impl <CrdtValue, OpsValue> CRDT<CrdtValue, OpsValue, EWFlag> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<OpsValue>) {
        todo!();
    }
}