use edflag_crdt::EDFlag;

pub type LCType            = u32;
pub type NodeType          = u16; //must implement Copy trait
pub type CRDTNumType       = u16;

pub type IntMultCrdtValue  = i64;
pub type IntMultOpsValue   = i64;
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

pub mod node_instance;

pub mod crdt;

pub mod add_mult_crdt;

pub mod edflag_crdt;

pub mod arset_crdt;

pub mod pncnt_crdt;

pub mod constants;

pub mod rand_crdt;

pub mod rand_add_mult;

pub mod rand_ewflag;







