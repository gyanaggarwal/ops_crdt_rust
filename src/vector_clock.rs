use std::{cmp::max, cmp::min, collections::HashMap};
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::{LCType, NodeType};

pub const INITIAL_LC: LCType = 0;
pub const INC_LC:     LCType = 1;

#[derive(Debug)]
pub enum VectorClockError {
    EmptyNodeList,
    NodeNotFound,
    NonCompatibleVC,
    InconsistentInputTRBC(NodeType, Vec<NodeType>),
    UnexpectedError(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum VCOrdering {
    VCLE,
    VCEQ,
    VCGR,
    VCCN
}

#[derive(Debug, PartialEq, Clone)]
pub enum VCStatus {
    DUPLICATE,
    INORDER,
    OUTOFORDER
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorClock {
    pub vcmap: HashMap<NodeType, LCType>
}

impl VectorClock {
    pub fn new(node_list: Vec<NodeType>) -> Result<Self, VectorClockError> {
        if node_list.is_empty() {
            return Err(VectorClockError::EmptyNodeList);
        }
        
        let mut vcmap = HashMap::new();
        
        for node in node_list {
            vcmap.insert(node, INITIAL_LC);
        }

        Ok(Self{vcmap})
    }

    pub fn len(&self) -> usize {
        self.vcmap.len()
    }

    pub fn next_vc(&mut self, node: &NodeType) -> Result<(), VectorClockError> {
        let lc = self.vcmap.get_mut(node).ok_or(VectorClockError::NodeNotFound)?;
        *lc += INC_LC;
        Ok(())

    }

    pub fn is_next_vc(&self, node: &NodeType, peer_vc: &VectorClock) -> Result<VCStatus, VectorClockError> {
        let nlc = self.vcmap.get(node).ok_or(VectorClockError::NodeNotFound)?;
        let plc = peer_vc.vcmap.get(node).ok_or(VectorClockError::NodeNotFound)?;
        let vc_status = cmp_lc(*nlc+INC_LC, *plc);
        Ok(peer_vc_status(vc_status))
    }

    pub fn cmp_vc(&self, other: &VectorClock) -> Result<VCOrdering, VectorClockError> {
        if self.len() != other.len() {
            return Err(VectorClockError::NonCompatibleVC);
        }
        
        let mut vcords = VCOrdering::VCEQ;
        for (node, lc1) in self.vcmap.iter() {
            let lc2 = other.vcmap.get(node).ok_or(VectorClockError::NonCompatibleVC)?;
            let vcordo = cmp_lc(*lc1, *lc2);
            vcords = vc_order(vcords, vcordo);
        }

        Ok(vcords)
    }

    pub fn check_vc(&self, node: NodeType, other: &VectorClock) -> Result<VCOrdering, VectorClockError> {
        let lc1 = self.vcmap.get(&node).ok_or(VectorClockError::NodeNotFound)?+1;
        let lc2 = other.vcmap.get(&node).ok_or(VectorClockError::NodeNotFound)?+0;

        Ok(cmp_lc(lc1, lc2))
    }

    pub fn min_max_vc(&self, other: &VectorClock, f: fn(LCType, LCType) -> LCType) -> Result<VectorClock, VectorClockError> {
        let mut vcmap = HashMap::new();
        for (node, lc1) in self.vcmap.iter() {
            let lc2 = other.vcmap.get(node).ok_or(VectorClockError::NonCompatibleVC)?;
            let flc = f(*lc1, *lc2);
            vcmap.insert(*node, flc);
        }

        Ok(VectorClock{vcmap})
    }

    pub fn max_vc(&self, other: &VectorClock) -> Result<VectorClock, VectorClockError> {
        self.min_max_vc(other, max)
    }

    pub fn min_vc(&self, other: &VectorClock) -> Result<VectorClock, VectorClockError> {
        self.min_max_vc(other, min)
    }    
}

pub fn vc_order(st1: VCOrdering, st2: VCOrdering) -> VCOrdering {
    let st1c = st1.clone();
    let st2c = st2.clone();
    match (st1, st2) {
        (VCOrdering::VCEQ, _)                => st2c,
        (_, VCOrdering::VCEQ)                => st1c,
        (VCOrdering::VCCN, _)                => VCOrdering::VCCN,
        (_, VCOrdering::VCCN)                => VCOrdering::VCCN,
        (VCOrdering::VCLE, VCOrdering::VCLE) => VCOrdering::VCLE,
        (VCOrdering::VCGR, VCOrdering::VCGR) => VCOrdering::VCGR,
        (_, _)                               => VCOrdering::VCCN
    }
}

pub fn cmp_lc(lc1: LCType, lc2: LCType) -> VCOrdering {
    match lc1.cmp(&lc2) {
        Ordering::Less    => VCOrdering::VCLE,
        Ordering::Equal   => VCOrdering::VCEQ,
        Ordering::Greater => VCOrdering::VCGR
    }
}

pub fn peer_vc_status(pord: VCOrdering) -> VCStatus {
    match pord {
        VCOrdering::VCGR => VCStatus::DUPLICATE,
        VCOrdering::VCEQ => VCStatus::INORDER,
        _                => VCStatus::OUTOFORDER
    }
}