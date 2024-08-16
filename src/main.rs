use ops_crdt_rust::vector_clock;
use ops_crdt_rust::trcb;
use ops_crdt_rust::NodeType;

fn main() {
    test_vector_clock();
    test_base_trcb();
}

fn create_node_list() -> Vec<NodeType> {
    vec![0,1,2]
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

    println!("cmp11 {:?} cmp12 {:?} cmp21 {:?} cmp13 {:?} cmp31 {:?}", 
             cmp11, cmp12, cmp21, cmp13, cmp31);

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



