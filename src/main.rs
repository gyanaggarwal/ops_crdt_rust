//use serde_json;
//use std::collections::HashSet;
use dotenvy::dotenv;
use ops_crdt_rust::crdt::AddMult;
use ops_crdt_rust::node_state::Node;

//use ops_crdt_rust::crdt::{AWSet, AddMult, CrdtInstance, EDFlag, EWFlag, PNCounter, CRDT, PNCounterData};
//use ops_crdt_rust::message_data::OpsInstance;
//use ops_crdt_rust::message_data::UserUpdateMsg;
//use ops_crdt_rust::vector_clock;
//use ops_crdt_rust::message_data;
//use ops_crdt_rust::crdt;
//use ops_crdt_rust::trcb;
//use ops_crdt_rust::NodeType;
//use ops_crdt_rust::constants::{MAX_MSG_COUNT_CS, MAX_MSG_COUNT_VC, NODE_LIST};



fn main() {
    dotenv().ok();

    let mut node: Node<i32, i32, AddMult> = Node::new();
    node.populate().unwrap();

    println!("node {:?} {:?}", node, node.get_crdt_instance());
}
/*
fn main() {
    test_vector_clock();
    test_base_trcb();
    test_msg_serde();
    test_crdt();
    run_crdt();
}

fn run_crdt() {
    dotenv().ok();
    let max_msg_vc = MAX_MSG_COUNT_VC.to_owned();
    let max_msg_cs = MAX_MSG_COUNT_CS.to_owned();
    let node_list = NODE_LIST.to_owned();

    println!("vc {}, cs {}, nl {:?}", max_msg_vc, max_msg_cs, node_list);
}

fn create_node_list() -> Vec<NodeType> {
    vec![0,1,2]
}

fn test_crdt() {
    let crdt_instance1: CRDT<u32, u32, AddMult> = CRDT::new(0, 0).unwrap();
    println!("crdt_instance1 {:?} {:?}", crdt_instance1, crdt_instance1.query());

    let crdt_instance2: CRDT<EDFlag, EDFlag, EWFlag> = CRDT::new(0, EDFlag::Enabled).unwrap();
    println!("crdt_instance2 {:?} {:?}", crdt_instance2, crdt_instance2.query());

    let crdt_instance3: CRDT<HashSet<i32>, i32, AWSet> = CRDT::new(0, HashSet::new()).unwrap();
    println!("crdt_instance3 {:?} {:?}", crdt_instance3, crdt_instance3.query());

    let crdt_instance4: CRDT<crdt::PNCounterData, u32, PNCounter> = CRDT::new(0, PNCounterData::new()).unwrap();
    println!("crdt_instance4 {:?} {:?}", crdt_instance4, crdt_instance4.query());

    for i in 7..5 {
        println!("i75 {}", i);
    }

    for i in 7..=7 {
        println!{"i77 {}", i};
    }

    for i in 7..10 {
        println!("710 {}", i);
    }
}

fn test_msg_serde() {
    let node_list = create_node_list();
    let vc000 = vector_clock::VectorClock::new(node_list).unwrap();

    let node1 = &1;
    let mut vc010 = vc000.clone();
    vc010.next_vc(node1).unwrap();

    let vc_msg0 = message_data::NodeVectorClockMsg::new(0, vc000.clone());
    let pn_msg0: message_data::PeerNodeMsg<i32> = message_data::PeerNodeMsg::VectorClockNodeMsg(vc_msg0);

    let ops_instance: OpsInstance<i32> = OpsInstance::new(message_data::SDPOpsType::SDPAdd, 10);
    let crdt_instance = CrdtInstance::new(0, 0, crdt::CrdtType::AddMultCrdt);
    let user_msg: UserUpdateMsg<i32> = message_data::UserUpdateMsg::new(crdt_instance, ops_instance);
    let node_msg1 = message_data::NodeUpdateMsg::new(0, vc010.clone(), user_msg);
    let pn_msg1: message_data::PeerNodeMsg<i32> = message_data::PeerNodeMsg::UpdateNodeMsg(node_msg1);

    let mut msg_list = Vec::new();
    msg_list.push(pn_msg0.clone());
    msg_list.push(pn_msg1.clone());
    let tmsg_list = serde_json::to_string(&msg_list).unwrap();
    let fmsg_list: Vec<message_data::PeerNodeMsg<i32>> = serde_json::from_str(&tmsg_list).unwrap();
    println!("tmsg_list {:?}", tmsg_list);
    println!("fmsg_list {:?}", fmsg_list);

}

fn test_vector_clock() {
    let node_list = create_node_list();
    let vc000 = vector_clock::VectorClock::new(node_list).unwrap();
 
    let node0 = &0;
    let node1 = &1;
    let mut cc100 = vc000.clone();
    cc100.next_vc(node0).unwrap();

    let mut cc010 = vc000.clone();
    cc010.next_vc(node1).unwrap();

    let mut cc110 = cc100.clone();
    cc110.next_vc(node1).unwrap();

    let cc100r = &cc100;
    let cc010r = &cc010;
    let cc110r = &cc110;

    println!("vc000 {:?}", vc000); 
    println!("cc100 {:?}", cc100);
    println!("cc010 {:?}", cc010);
    println!("cc110 {:?}", cc110);

    let cmp11 = cc100r.cmp_vc(cc100r).unwrap();
    let cmp12 = cc100r.cmp_vc(cc010r).unwrap();
    let cmp21 = cc010r.cmp_vc(cc100r).unwrap();
    let cmp13 = cc100r.cmp_vc(cc110r).unwrap();
    let cmp31 = cc110r.cmp_vc(cc100r).unwrap();

    println!("cmp11 {:?} cmp12 {:?} cmp21 {:?} cmp13 {:?} cmp31 {:?}", cmp11, cmp12, cmp21, cmp13, cmp31);

}

fn test_base_trcb() {
    let node_list = create_node_list();
    let mut trcb1 = trcb::TRCBData::new(0, node_list.clone()).unwrap();
    println!("trcb1 {:?}", trcb1);

    trcb1.next_vc().unwrap();
    println!("trcb2 {:?}", trcb1);

    let vc000 = vector_clock::VectorClock::new(node_list).unwrap();
 
    let node0 = &0;
    let node1 = &1;
    let node2 = &2;
  
    let mut cc100 = vc000.clone();
    cc100.next_vc(node0).unwrap();

    let mut cc010 = vc000.clone();
    cc010.next_vc(node1).unwrap();
    let v010 = trcb1.add_peer_vc(*node1, cc010).unwrap();
    println!("v010 {:?}", v010);
    println!("t010 {:?}", trcb1);

    let mut cc001 = vc000.clone();
    cc001.next_vc(node2).unwrap();
    let v001 = trcb1.add_peer_vc(*node2, cc001.clone()).unwrap();
    println!("v001 {:?}", v001);
    println!("t001 {:?}", trcb1);

    let dv001 = trcb1.add_peer_vc(*node2, cc001.clone()).unwrap();
    println!("cc001 {:?}", cc001);
    println!("dv001 {:?}", dv001);
    println!("dt001 {:?}", trcb1);

    let mut cc003 = vc000.clone();
    cc003.next_vc(node2).unwrap();
    cc003.next_vc(node2).unwrap();
    cc003.next_vc(node2).unwrap();
    let v003 = trcb1.add_peer_vc(*node2, cc003.clone()).unwrap();
    println!("v003 {:?}", v003);
    println!("t003 {:?}", trcb1);

    let cs = trcb1.causally_stable();

    println!("cs {:?}", cs);

}
*/


