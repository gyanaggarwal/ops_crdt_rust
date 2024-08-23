use std::fmt;

use serde::{Serialize, Deserialize};

use crate::{NodeType, CRDTNumType};
use crate::trcb;
use crate::vector_clock::VectorClockError;
use crate::message_data::{NodeUpdateMsg, NodeVectorClockMsg};
use crate::message_list;

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
pub enum EDFlag {
    Enabled,
    Disabled
}
impl fmt::Display for EDFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            EDFlag::Enabled => "Enabled",
            EDFlag::Disabled => "Disabled"
        };
        write!(f, "{}", printable)
    }
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
pub struct CRDT <CrdtValue: Clone, OpsValue: fmt::Display+Clone, State> {
    pub trcb: trcb::TRCBData,
    pub msg_list: Vec<NodeUpdateMsg<OpsValue>>,
    pub crdt_value: CrdtValue,
    pub state: std::marker::PhantomData<State>
}

impl <CrdtValue: Clone, OpsValue: fmt::Display+Clone, State> CRDT<CrdtValue, OpsValue, State> {
    pub fn new(node: NodeType, node_list: Vec<NodeType>, crdt_value: CrdtValue) -> Result<Self, VectorClockError> {
        let trcb = trcb::TRCBData::new(node, node_list)?;
        let msg_list = Vec::new();
        Ok(Self{trcb, msg_list, crdt_value, state: std::marker::PhantomData::<State>})
    }

    pub fn add_msg(&mut self, msg: NodeUpdateMsg<OpsValue>) {
        self.msg_list.push(msg);
    }

    pub fn process_vc_msg(&mut self, msg: NodeVectorClockMsg) -> Result<(), VectorClockError> {
        self.trcb.add_peer_vcmsg(msg.node, msg.node_vector_clock)
    }

    pub fn causally_stable(&mut self) -> Result<(), VectorClockError> {
        let cs_vc = self.trcb.causally_stable()?;
        let new_list = message_list::remove_causally_stable(&cs_vc, self.msg_list.clone())?;
        self.msg_list = new_list;
        Ok(())
    }

    pub fn query(&self) -> CrdtValue {
        self.crdt_value.clone()
    }
}

impl <CrdtValue: Clone, OpsValue: fmt::Display+Clone> CRDT<CrdtValue, OpsValue, AddMult> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<OpsValue>) {
        todo!();
    }
}

impl <CrdtValue: Clone, OpsValue: fmt::Display+Clone> CRDT<CrdtValue, OpsValue, EWFlag> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<OpsValue>) {
        todo!();
    }
}

impl <CrdtValue: Clone, OpsValue: fmt::Display+Clone> CRDT<CrdtValue, OpsValue, DWFlag> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<OpsValue>) {
        todo!();
    }
}

impl <CrdtValue: Clone, OpsValue: fmt::Display+Clone> CRDT<CrdtValue, OpsValue, AWSet> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<OpsValue>) {
        todo!()
    }
}

impl <CrdtValue: Clone, OpsValue: fmt::Display+Clone> CRDT<CrdtValue, OpsValue, RWSet> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<OpsValue>) {
        todo!()
    }
}

impl <CrdtValue: Clone, OpsValue: fmt::Display+Clone> CRDT<CrdtValue, OpsValue, PNCounter> {
    pub fn process_msg(&mut self, _msg: &NodeUpdateMsg<OpsValue>) {
        todo!()
    }
}