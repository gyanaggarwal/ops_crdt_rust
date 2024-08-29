/*
node_instance.rs
  node
  node_list
  map -> (crdt_type, crdt_instance_no) : new crdt_instance_id (node, crdt_type, instance_num)
  map -> (crdt_instance_id, crdt_instance) : get crdt_instance/create new crdt_instance
  it will convert user_msg to node_msg by adding vector_clock
  it will serialize the outgoing messages
  it will deserialize incoming messages
*/

use crdt::EDFlag;

pub type LCType            = u32;
pub type NodeType          = u16; //must implement Copy trait
pub type CRDTNumType       = u16;

pub type IntMultCrdtValue  = i32;
pub type IntMultOpsValue   = i32;
pub type EDFlagCrdtValue   = EDFlag;
pub type EDFlagOpsValue    = EDFlag;
pub type PNCntOpsValue     = u32;

pub mod vector_clock;

pub mod trcb;

pub mod message_data;

pub mod message_list;

pub mod anti_entropy;

pub mod node_state;

pub mod crdt;

pub mod constants;






