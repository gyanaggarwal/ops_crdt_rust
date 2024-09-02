use std::collections::HashSet;

use crate::{NodeType, 
            IntMultCrdtValue, IntMultOpsValue,
            EDFlagCrdtValue, EDFlagOpsValue,
            ARSetOpsValue,
            PNCntOpsValue};
use crate::crdt::CRDT;
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
}

