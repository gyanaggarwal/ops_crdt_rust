use serde::{Serialize, Deserialize};

use crate::trcb::TRCBData;
use crate::vector_clock::VectorClock;
use crate::{NodeType, CRDTNumType};

pub struct AddMult;
pub struct EWFlag;
pub struct DWFlag;
pub struct AWSet;
pub struct RWSet;
pub struct PNCounter;
pub struct NoCrdt;

#[derive(Debug)]
pub struct CRDT <CrdtValue, MsgType, State = NoCrdt> {
    pub crdt_type: CrdtType,
    pub instance_num: CRDTNumType,
    pub trcb: TRCBData,
    pub msg_list: Vec<MsgType>,
    pub crdt_value: CrdtValue,
    pub state: std::marker::PhantomData<State>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SDPOpsType {
    SPDNonCommuAdd,
    SPDNonCommuMult,
    SPDCommu
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CrdtType {
    AddMultCrdt,
    EWFlagCrdt,
    DWFlagCrdt,
    PNCounterCrdt
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct OpsInstance <OpsValue> {
    pub ops: SDPOpsType,
    pub ops_value: OpsValue
}
impl <OpsValue> OpsInstance<OpsValue> {
    pub fn new(ops: SDPOpsType, ops_value: OpsValue) -> Self {
        Self {ops, ops_value}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMsg <OpsValue> {
    pub crdt_instance: CrdtInstance,
    pub ops_instance: OpsInstance<OpsValue>
}
impl <OpsValue> UserMsg<OpsValue> {
    pub fn new(crdt_instance: CrdtInstance, ops_instance: OpsInstance<OpsValue>) -> Self {
        Self {crdt_instance, ops_instance}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeMsg <OpsValue> {
    pub node: NodeType,
    pub node_vector_clock: VectorClock,
    pub user_msg: UserMsg<OpsValue>
}
impl <OpsValue> NodeMsg<OpsValue> {
    pub fn new(node:NodeType, node_vector_clock: VectorClock, user_msg: UserMsg<OpsValue>) -> Self {
        Self {node, node_vector_clock, user_msg}
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VectorClockMsg {
    pub node: NodeType,
    pub node_vector_clock: VectorClock
}
impl VectorClockMsg {
    pub fn new(node: NodeType, node_vector_clock: VectorClock) -> Self {
        Self {node, node_vector_clock}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PeerNodeMsg <OpsValue> {
    VCNodeMsg(VectorClockMsg),
    UserNodeMsg(NodeMsg<OpsValue>)
}





