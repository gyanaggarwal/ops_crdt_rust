/*
node_instance.rs
  node
  node_list
  map -> (crdt_type, crdt_instance_no) : new crdt_instance_id (node, crdt_type, instance_num)
  map -> (crdt_instance_id, crdt_instance) : get crdt_instance/create new crdt_instance
  it will convert user_msg to node_msg by adding vector_clock
  it will serialize the outgoing messages
  it will deserialize incoming messages

crdt.rs
  add add_fn (crdt_value, ops_value) -> crdt_value
  add mult_fn (crdt_value, ops_value) -> crdt_value
  remove SDPCommutative from enum

  when receive a NodeUpdateMsg,
  if the msg is a local update 
    make msg_count_vc = 0
    make msg_count_cs = add 1
    update crdt
    run conditionally (causally_stable, make msg_count_cs = 0)
    make a send_list with no vc_msg
  else 
    make msg_count_vc = add 1
    if it is a duplicate msg, ignore it
    if it is inorder
      make msg_count_cs = add 1
      update crdt
      run conditionally (causally_stable, make msg_count_cs = 0)
    always make send_list with vc
    if send_list is non-empty make msg_count_vc = 0
*/

use crdt::EDFlag;

pub type LCType       = u32;
pub type NodeType     = u16; //must implement Copy trait
pub type CRDTNumType  = u16;

pub type IntMultCrdtValue  = i32;
pub type IntMultOpsValue   = i32;
pub type EDFlagOpsValue    = EDFlag;
pub type PNCounterOpsValue = u32;

pub mod vector_clock;

pub mod trcb;

pub mod message_data;

pub mod message_list;

pub mod anti_entropy;

pub mod node_state;

pub mod crdt;

pub mod constants;






