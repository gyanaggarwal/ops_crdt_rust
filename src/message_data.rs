use serde::{Serialize, Deserialize};

//use crate::trcb::TRCBData;
use crate::vector_clock::VectorClock;
//use crate::{NodeType, CRDTNumType};
use crate::NodeType;
//use crate::crdt::{CrdtType, CrdtInstance};
use crate::crdt::CrdtInstance;

/*
#[derive(Debug)]
pub struct CRDT <CrdtValue, OpsType, State = NoCrdt> {
    pub crdt_type: CrdtType,
    pub instance_num: CRDTNumType,
    pub trcb: TRCBData,
    pub msg_list: Vec<NodeUpdateMsg<OpsType>>,
    pub crdt_value: CrdtValue,
    pub state: std::marker::PhantomData<State>
}
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SDPOpsType {
    SPDNonCommuAdd,
    SPDNonCommuMult,
    SPDCommu
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpsInstance <OpsValue> {
    pub ops: SDPOpsType,
    pub ops_value: OpsValue
}
impl <OpsValue> OpsInstance<OpsValue> {
    pub fn new(ops: SDPOpsType, ops_value: OpsValue) -> Self {
        Self {ops, ops_value}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserUpdateMsg <OpsValue> {
    pub crdt_instance: CrdtInstance,
    pub ops_instance: OpsInstance<OpsValue>
}
impl <OpsValue> UserUpdateMsg<OpsValue> {
    pub fn new(crdt_instance: CrdtInstance, ops_instance: OpsInstance<OpsValue>) -> Self {
        Self {crdt_instance, ops_instance}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeUpdateMsg <OpsValue> {
    pub node: NodeType,
    pub node_vector_clock: VectorClock,
    pub user_update_msg: UserUpdateMsg<OpsValue>
}
impl <OpsValue> NodeUpdateMsg<OpsValue> {
    pub fn new(node:NodeType, node_vector_clock: VectorClock, user_update_msg: UserUpdateMsg<OpsValue>) -> Self {
        Self {node, node_vector_clock, user_update_msg}
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeVectorClockMsg {
    pub node: NodeType,
    pub node_vector_clock: VectorClock
}
impl NodeVectorClockMsg {
    pub fn new(node: NodeType, node_vector_clock: VectorClock) -> Self {
        Self {node, node_vector_clock}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PeerNodeMsg <OpsValue> {
    VectorClockNodeMsg(NodeVectorClockMsg),
    UpdateNodeMsg(NodeUpdateMsg<OpsValue>)
}





