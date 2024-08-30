use std::collections::HashMap;

use crate::message_data::{OpsInstance, PeerNodeMsg, UserUpdateMsg};
use crate::{NodeType, IntMultCrdtValue, IntMultOpsValue};
use crate::crdt::{CRDT, AddMult};
use crate::vector_clock::VectorClockError;


#[derive(Debug)]
pub struct NodeInstance {
    pub node: NodeType,
    pub add_mult_crdt: CRDT<IntMultCrdtValue, IntMultOpsValue, AddMult>
}

impl NodeInstance {
    pub fn new(node: NodeType) -> Result<Self, VectorClockError> {
        let add_mult_crdt: CRDT<IntMultCrdtValue, IntMultOpsValue, AddMult> = CRDT::new(node, 0)?;
        Ok(Self{node, add_mult_crdt})
    }

    pub fn process_local_msg(&mut self, user_update_msg: UserUpdateMsg<IntMultOpsValue>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<IntMultOpsValue>>>, VectorClockError> {
        let node_update_msg = self.add_mult_crdt.create_local_msg(user_update_msg)?;
        self.add_mult_crdt.process_local_msg(node_update_msg)
    }

    pub fn process_peer_msg(&mut self, pmsg_list: Vec<PeerNodeMsg<IntMultOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<IntMultOpsValue>>>, VectorClockError> {
        self.add_mult_crdt.process_peer_msg(pmsg_list)
    }

    pub fn get_add_ops(&self, value: IntMultOpsValue) -> OpsInstance<IntMultOpsValue> {
        self.add_mult_crdt.get_add_ops(value)
    }

    pub fn get_mult_ops(&self, value: IntMultOpsValue) -> OpsInstance<IntMultOpsValue> {
        self.add_mult_crdt.get_add_ops(value)
    }
}