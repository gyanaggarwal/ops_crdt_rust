use crdt::EDFlag;

pub type LCType            = u32;
pub type NodeType          = u16; //must implement Copy trait
pub type CRDTNumType       = u16;

pub type IntMultCrdtValue  = i32;
pub type IntMultOpsValue   = i32;
pub type EDFlagCrdtValue   = EDFlag;
pub type EDFlagOpsValue    = EDFlag;
pub type PNCntOpsValue     = u32;
pub type ARSetOpsValue     = i32;

pub mod vector_clock;

pub mod trcb;

pub mod message_data;

pub mod message_list;

pub mod anti_entropy;

pub mod node_state;

pub mod crdt;

pub mod constants;






