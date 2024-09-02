use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::{LCType, 
            NodeType, 
            CRDTNumType};
use crate::trcb;
use crate::vector_clock::{VCStatus, VectorClockError, VectorClock, peer_vc_status};
use crate::message_data::{NodeUpdateMsg, 
                          NodeVectorClockMsg, 
                          PeerNodeMsg, 
                          UserUpdateMsg};
use crate::message_list;
use crate::constants::{MAX_MSG_COUNT_CS, MAX_MSG_COUNT_VC, NODE_LIST};

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
pub struct CrdtInstance {
    instance_node_id: NodeType,
    instance_num: CRDTNumType,
    instance_type: CrdtType
}
impl CrdtInstance {
    pub fn new(instance_node_id: NodeType, instance_num: CRDTNumType, instance_type: CrdtType) -> Self {
        Self{instance_node_id, instance_num, instance_type}
    }

    pub fn new_default(instance_type: CrdtType) -> Self {
        Self{instance_node_id:0, instance_num:0, instance_type}
    }
}

#[derive(Debug)]
pub struct CRDT <CrdtValue: Clone+Debug, OpsValue: Clone+PartialEq+Debug, State> {
    pub trcb: trcb::TRCBData,
    pub msg_list: HashMap<(NodeType, LCType), NodeUpdateMsg<OpsValue>>,
    pub crdt_value: CrdtValue,
    pub max_msg_count_vc: u16,
    pub max_msg_count_cs: u16,
    pub msg_count_vc: u16,
    pub msg_count_cs: u16,
    pub state: std::marker::PhantomData<State>
}

impl <CrdtValue: Clone+Debug, OpsValue: Clone+PartialEq+Debug, State: Debug> CRDT<CrdtValue, OpsValue, State> {
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

    pub fn next_vc(&mut self) -> Result<VectorClock, VectorClockError> {
        self.trcb.next_vc()
    }

    pub fn get_node(&self) -> NodeType {
        self.trcb.node.clone()
    }

    pub fn create_local_msg(&mut self, user_update_msg: UserUpdateMsg<OpsValue>) -> 
        Result<NodeUpdateMsg<OpsValue>, VectorClockError> {
        let node = self.get_node();
        let node_vector_clock = self.next_vc()?.clone();
        Ok(NodeUpdateMsg::new(node, node_vector_clock, user_update_msg))
    }

    pub fn add_msg(&mut self, msg: NodeUpdateMsg<OpsValue>) -> Result<(), VectorClockError> {
        let lc = msg.node_vector_clock.vcmap.get(&msg.node).ok_or(VectorClockError::NodeNotFound)?;
        self.msg_list.insert((msg.node, lc.clone()), msg);
        Ok(())
    }

    pub fn general_process_local_msg(&mut self, msg: NodeUpdateMsg<OpsValue>) -> 
        Result<HashMap<NodeType, Vec<PeerNodeMsg<OpsValue>>>, VectorClockError> {
        self.msg_count_vc = 0;
        self.msg_count_cs += 1;
        self.add_msg(msg.clone())?;
        self.causally_stable()?;         
        self.create_peer_msg_list(true)
    }

    pub fn general_process_peer_msg(&mut self, msg: NodeUpdateMsg<OpsValue>) -> Result<VCStatus, VectorClockError>  {
        self.msg_count_vc += 1;
        let vc_ord = self.trcb.node_vector_clock.check_vc(msg.node, &msg.node_vector_clock)?;
        let vc_status = peer_vc_status(vc_ord);
    
        if vc_status == VCStatus::INORDER {
            self.msg_count_cs += 1;
            self.add_msg(msg.clone())?;
            self.trcb.add_peer_vc(msg.node, msg.node_vector_clock.clone())?;
        }
        Ok(vc_status)
    }
    pub fn general_process_vc_msg(&mut self, msg: NodeVectorClockMsg) -> Result<(), VectorClockError> {
        self.trcb.add_peer_vcmsg(msg.node, msg.node_vector_clock.clone())?;
        self.msg_count_cs += 1;
        self.causally_stable()
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

    pub fn msg_list_len(&self) -> usize {
        self.msg_list.len()
    }
}

