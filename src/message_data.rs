use std::fmt::Display;

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
pub struct OpsInstance <OpsValue: Display+Clone+PartialEq> {
    pub ops: SDPOpsType,
    pub ops_value: OpsValue
}
impl <OpsValue: Display+Clone+PartialEq> OpsInstance<OpsValue> {
    pub fn new(ops: SDPOpsType, ops_value: OpsValue) -> Self {
        Self {ops, ops_value}
    }

    pub fn check(&self, check_value: &Option<OpsValue>) -> bool {
        self.ops == SDPOpsType::SDPMult && match check_value {
                                                    Some(value) => *value == self.ops_value,
                                                    None => true
                                                }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserUpdateMsg <OpsValue: Display+Clone+PartialEq> {
    pub crdt_instance: CrdtInstance,
    pub ops_instance: OpsInstance<OpsValue>
}
impl <OpsValue: Display+Clone+PartialEq> UserUpdateMsg<OpsValue> {
    pub fn new(crdt_instance: CrdtInstance, ops_instance: OpsInstance<OpsValue>) -> Self {
        Self {crdt_instance, ops_instance}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeUpdateMsg <OpsValue: Display+Clone+PartialEq> {
    pub node: NodeType,
    pub node_vector_clock: VectorClock,
    pub user_update_msg: UserUpdateMsg<OpsValue>
}
impl <OpsValue: Display+Clone+PartialEq> NodeUpdateMsg<OpsValue> {
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
pub enum PeerNodeMsg <OpsValue: Display+Clone+PartialEq> {
    VectorClockNodeMsg(NodeVectorClockMsg),
    UpdateNodeMsg(NodeUpdateMsg<OpsValue>)
}





