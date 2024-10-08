use serde::{Serialize, Deserialize};

use crate::vector_clock::VectorClock;
use crate::NodeType;
use crate::crdt::CrdtInstance;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SDPOpsType {
    SDPAdd,
    SDPMult,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpsInstance <OpsValue: Clone+PartialEq> {
    pub ops_type: SDPOpsType,
    pub ops_value: OpsValue
}
impl <OpsValue: Clone+PartialEq> OpsInstance<OpsValue> {
    pub fn new(ops_type: SDPOpsType, ops_value: OpsValue) -> Self {
        Self {ops_type, ops_value}
    }

    pub fn check(&self, check_value: &Option<OpsValue>) -> bool {
        self.ops_type == SDPOpsType::SDPMult && match check_value {
                                                    Some(value) => *value == self.ops_value,
                                                    None => true
                                                }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserUpdateMsg <OpsValue: Clone+PartialEq> {
    pub crdt_instance: CrdtInstance,
    pub ops_instance: OpsInstance<OpsValue>
}
impl <OpsValue: Clone+PartialEq> UserUpdateMsg<OpsValue> {
    pub fn new(crdt_instance: CrdtInstance, ops_instance: OpsInstance<OpsValue>) -> Self {
        Self {crdt_instance, ops_instance}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeUpdateMsg <OpsValue: Clone+PartialEq> {
    pub node: NodeType,
    pub node_vector_clock: VectorClock,
    pub user_update_msg: UserUpdateMsg<OpsValue>
}
impl <OpsValue: Clone+PartialEq> NodeUpdateMsg<OpsValue> {
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
pub enum PeerNodeMsg <OpsValue: Clone+PartialEq> {
    VectorClockNodeMsg(NodeVectorClockMsg),
    UpdateNodeMsg(NodeUpdateMsg<OpsValue>)
}





