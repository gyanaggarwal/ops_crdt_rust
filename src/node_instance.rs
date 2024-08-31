use std::collections::{HashMap, HashSet};

use crate::message_data::{OpsInstance, PeerNodeMsg, UserUpdateMsg};
use crate::{NodeType, 
            IntMultCrdtValue, IntMultOpsValue,
            EDFlagCrdtValue, EDFlagOpsValue,
            ARSetOpsValue,
            PNCntOpsValue};
use crate::crdt::{CRDT,
                  EDFlag,
                  PNCounterData,
                  AddMult, 
                  EWFlag,
                  DWFlag,
                  AWSet,
                  RWSet,
                  PNCounter};
use crate::vector_clock::VectorClockError;


#[derive(Debug)]
pub struct NodeInstance {
    pub node: NodeType,
    pub add_mult_crdt: CRDT<IntMultCrdtValue, IntMultOpsValue, AddMult>,
    pub ewflag_crdt: CRDT<EDFlagCrdtValue, EDFlagOpsValue, EWFlag>,
    pub dwflag_crdt: CRDT<EDFlagCrdtValue, EDFlagOpsValue, DWFlag>,
    pub awset_crdt: CRDT<HashSet<ARSetOpsValue>, ARSetOpsValue, AWSet>,
    pub rwset_crdt: CRDT<HashSet<ARSetOpsValue>, ARSetOpsValue, RWSet>,
    pub pncnt_crdt: CRDT<PNCounterData, PNCntOpsValue, PNCounter>
}

impl NodeInstance {
    pub fn new(node: NodeType) -> Result<Self, VectorClockError> {
        let add_mult_crdt: CRDT<IntMultCrdtValue, IntMultOpsValue, AddMult> = CRDT::new(node, 0)?;
        let ewflag_crdt: CRDT<EDFlagCrdtValue, EDFlagOpsValue, EWFlag> = CRDT::new(node, EDFlag::Enabled)?;
        let dwflag_crdt: CRDT<EDFlagCrdtValue, EDFlagOpsValue, DWFlag> = CRDT::new(node, EDFlag::Disabled)?;
        let awset_crdt: CRDT<HashSet<ARSetOpsValue>, ARSetOpsValue, AWSet> = CRDT::new(node, HashSet::<ARSetOpsValue>::new())?;
        let rwset_crdt: CRDT<HashSet<ARSetOpsValue>, ARSetOpsValue, RWSet> = CRDT::new(node, HashSet::<ARSetOpsValue>::new())?;
        let pncnt_crdt: CRDT<PNCounterData, PNCntOpsValue, PNCounter> = CRDT::new(node, PNCounterData::new())?;
        Ok(Self{node, add_mult_crdt, ewflag_crdt, dwflag_crdt, awset_crdt, rwset_crdt, pncnt_crdt})
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

    pub fn get_add_ops_add_mult(&self, value: IntMultOpsValue) -> OpsInstance<IntMultOpsValue> {
        self.add_mult_crdt.get_add_ops(value)
    }

    pub fn get_mult_ops_add_mult(&self, value: IntMultOpsValue) -> OpsInstance<IntMultOpsValue> {
        self.add_mult_crdt.get_add_ops(value)
    }
}