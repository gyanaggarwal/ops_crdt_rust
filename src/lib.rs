/*
node_instance.rs
  node
  node_list
  map -> (crdt_type, crdt_instance_no) : new crdt_instance_id (node, crdt_type, instance_num)
  map -> (crdt_instance_id, crdt_instance) : get crdt_instance/create new crdt_instance

crdt.rs
  enum for crdt_type
  struct for crdt_instance_id
  logic for applying user message/peer update message/peer vector clock message

*/
pub type LCType       = u32;
pub type NodeType     = u16;
pub type CRDTNumType  = u16;

pub mod vector_clock;

pub mod trcb;

pub mod message_data;

pub mod anti_entropy;

pub mod node;

pub mod crdt;






