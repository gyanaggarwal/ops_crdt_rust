use std::collections::{HashMap, HashSet};
use rand::prelude::*;

use crate::message_data::{OpsInstance, PeerNodeMsg, UserUpdateMsg};
use crate::{NodeType, 
            IntMultCrdtValue, IntMultOpsValue,
            EDFlagCrdtValue, EDFlagOpsValue,
            ARSetOpsValue,
            PNCntOpsValue};
use crate::crdt::{CRDT, CrdtInstance, CrdtType};
use crate::pncnt_crdt::{PNCounter, PNCounterData};
use crate::arset_crdt::{AWSet, RWSet};
use crate::vector_clock::VectorClockError;
use crate::edflag_crdt::{EDFlag, EWFlag, DWFlag};
use crate::add_mult_crdt::AddMult;

#[derive(Debug)]
pub struct NodeInstance {
    pub node:          NodeType,
    pub add_mult_crdt: CRDT<IntMultCrdtValue, IntMultOpsValue, AddMult>,
    pub ewflag_crdt:   CRDT<EDFlagCrdtValue, EDFlagOpsValue, EWFlag>,
    pub dwflag_crdt:   CRDT<EDFlagCrdtValue, EDFlagOpsValue, DWFlag>,
    pub awset_crdt:    CRDT<HashSet<ARSetOpsValue>, ARSetOpsValue, AWSet>,
    pub rwset_crdt:    CRDT<HashSet<ARSetOpsValue>, ARSetOpsValue, RWSet>,
    pub pncnt_crdt:    CRDT<PNCounterData, PNCntOpsValue, PNCounter>
}

impl NodeInstance {
    pub fn new(node: NodeType) -> Result<Self, VectorClockError> {
        let add_mult_crdt: CRDT<IntMultCrdtValue, IntMultOpsValue, AddMult> = CRDT::new(node, 0)?;
        let ewflag_crdt:   CRDT<EDFlagCrdtValue, EDFlagOpsValue, EWFlag> = CRDT::new(node, EDFlag::Enabled)?;
        let dwflag_crdt:   CRDT<EDFlagCrdtValue, EDFlagOpsValue, DWFlag> = CRDT::new(node, EDFlag::Disabled)?;
        let awset_crdt:    CRDT<HashSet<ARSetOpsValue>, ARSetOpsValue, AWSet> = CRDT::new(node, HashSet::<ARSetOpsValue>::new())?;
        let rwset_crdt:    CRDT<HashSet<ARSetOpsValue>, ARSetOpsValue, RWSet> = CRDT::new(node, HashSet::<ARSetOpsValue>::new())?;
        let pncnt_crdt:    CRDT<PNCounterData, PNCntOpsValue, PNCounter> = CRDT::new(node, PNCounterData::new())?;
        Ok(Self{node, add_mult_crdt, ewflag_crdt, dwflag_crdt, awset_crdt, rwset_crdt, pncnt_crdt})
    }

    pub fn process_local_msg_add_mult(&mut self, user_update_msg: UserUpdateMsg<IntMultOpsValue>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<IntMultOpsValue>>>, VectorClockError> {
        let node_update_msg = self.add_mult_crdt.create_local_msg(user_update_msg)?;
        self.add_mult_crdt.process_local_msg(node_update_msg)
    }

    pub fn process_peer_msg_add_mult(&mut self, pmsg_list: Vec<PeerNodeMsg<IntMultOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<IntMultOpsValue>>>, VectorClockError> {
        self.add_mult_crdt.process_peer_msg(pmsg_list)
    }

    pub fn process_local_msg_ewflag(&mut self, user_update_msg: UserUpdateMsg<EDFlagOpsValue>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<EDFlagOpsValue>>>, VectorClockError> {
        let node_update_msg = self.ewflag_crdt.create_local_msg(user_update_msg)?;
        self.ewflag_crdt.process_local_msg(node_update_msg)
    }

    pub fn process_peer_msg_ewflag(&mut self, pmsg_list: Vec<PeerNodeMsg<EDFlagOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<EDFlagOpsValue>>>, VectorClockError> {
        self.ewflag_crdt.process_peer_msg(pmsg_list)
    }

    pub fn process_local_msg_dwflag(&mut self, user_update_msg: UserUpdateMsg<EDFlagOpsValue>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<EDFlagOpsValue>>>, VectorClockError> {
        let node_update_msg = self.dwflag_crdt.create_local_msg(user_update_msg)?;
        self.dwflag_crdt.process_local_msg(node_update_msg)
    }

    pub fn process_peer_msg_dwflag(&mut self, pmsg_list: Vec<PeerNodeMsg<EDFlagOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<EDFlagOpsValue>>>, VectorClockError> {
        self.dwflag_crdt.process_peer_msg(pmsg_list)
    }

    pub fn process_local_msg_awset(&mut self, user_update_msg: UserUpdateMsg<ARSetOpsValue>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<ARSetOpsValue>>>, VectorClockError> {
        let node_update_msg = self.awset_crdt.create_local_msg(user_update_msg)?;
        self.awset_crdt.process_local_msg(node_update_msg)
    }

    pub fn process_peer_msg_awset(&mut self, pmsg_list: Vec<PeerNodeMsg<ARSetOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<ARSetOpsValue>>>, VectorClockError> {
        self.awset_crdt.process_peer_msg(pmsg_list)
    }

    pub fn process_local_msg_rwset(&mut self, user_update_msg: UserUpdateMsg<ARSetOpsValue>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<ARSetOpsValue>>>, VectorClockError> {
        let node_update_msg = self.rwset_crdt.create_local_msg(user_update_msg)?;
        self.rwset_crdt.process_local_msg(node_update_msg)
    }

    pub fn process_peer_msg_rwset(&mut self, pmsg_list: Vec<PeerNodeMsg<ARSetOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<ARSetOpsValue>>>, VectorClockError> {
        self.rwset_crdt.process_peer_msg(pmsg_list)
    }

    pub fn process_local_msg_pncnt(&mut self, user_update_msg: UserUpdateMsg<PNCntOpsValue>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<PNCntOpsValue>>>, VectorClockError> {
        let node_update_msg = self.pncnt_crdt.create_local_msg(user_update_msg)?;
        self.pncnt_crdt.process_local_msg(node_update_msg)
    }

    pub fn process_peer_msg_pncnt(&mut self, pmsg_list: Vec<PeerNodeMsg<PNCntOpsValue>>) ->
        Result<HashMap<NodeType, Vec<PeerNodeMsg<PNCntOpsValue>>>, VectorClockError> {
        self.pncnt_crdt.process_peer_msg(pmsg_list)
    }

    pub fn get_add_ops_add_mult(&self, value: IntMultOpsValue) -> OpsInstance<IntMultOpsValue> {
        self.add_mult_crdt.get_add_ops(value)
    }

    pub fn get_mult_ops_add_mult(&self, value: IntMultOpsValue) -> OpsInstance<IntMultOpsValue> {
        self.add_mult_crdt.get_mult_ops(value)
    }

    pub fn get_add_ops_ewflag(&self) -> OpsInstance<EDFlagOpsValue> {
        self.ewflag_crdt.get_add_ops()
    }

    pub fn get_mult_ops_ewflag(&self) -> OpsInstance<EDFlagOpsValue> {
        self.ewflag_crdt.get_mult_ops()
    }

    pub fn get_add_ops_dwflag(&self) -> OpsInstance<EDFlagOpsValue> {
        self.dwflag_crdt.get_add_ops()
    }

    pub fn get_mult_ops_dwflag(&self) -> OpsInstance<EDFlagOpsValue> {
        self.dwflag_crdt.get_mult_ops()
    }

    pub fn get_add_ops_awset(&self, value: ARSetOpsValue) -> OpsInstance<ARSetOpsValue> {
        self.awset_crdt.get_add_ops(value)
    }

    pub fn get_mult_ops_awset(&self, value: ARSetOpsValue) -> OpsInstance<ARSetOpsValue> {
        self.awset_crdt.get_mult_ops(value)
    }

    pub fn get_add_ops_rwset(&self, value: ARSetOpsValue) -> OpsInstance<ARSetOpsValue> {
        self.rwset_crdt.get_add_ops(value)
    }

    pub fn get_mult_ops_rwset(&self, value: ARSetOpsValue) -> OpsInstance<ARSetOpsValue> {
        self.rwset_crdt.get_mult_ops(value)
    }

    pub fn get_add_ops_pncnt(&self, value: PNCntOpsValue) -> OpsInstance<PNCntOpsValue> {
        self.pncnt_crdt.get_add_ops(value)
    }

    pub fn get_mult_ops_pncnt(&self, value: PNCntOpsValue) -> OpsInstance<PNCntOpsValue> {
        self.pncnt_crdt.get_mult_ops(value)
    }

    pub fn get_crdt_instance_add_mult(&self) -> CrdtInstance {
        CrdtInstance::new(0, 1, CrdtType::AddMultCrdt)
    }

    pub fn get_crdt_instance_ewflag(&self) -> CrdtInstance {
        CrdtInstance::new(0, 1, CrdtType::EWFlagCrdt)
    }

    pub fn get_crdt_instance_dwflag(&self) -> CrdtInstance {
        CrdtInstance::new(0, 1, CrdtType::DWFlagCrdt)
    }
    
    pub fn get_crdt_instance_awset(&self) -> CrdtInstance {
        CrdtInstance::new(0, 1, CrdtType::AWSetCrdt)
    }

    pub fn get_crdt_instance_rwset(&self) -> CrdtInstance {
        CrdtInstance::new(0, 1, CrdtType::RWSetCrdt)
    }

    pub fn get_crdt_instance_pncnt(&self) -> CrdtInstance {
        CrdtInstance::new(0, 1, CrdtType::PNCounterCrdt)
    }
}

pub fn get_rand(low: u16, high: u16) -> u16 {
    rand::thread_rng().gen_range(low..=high)
}

pub fn do_nothing() -> () {
    ()
}