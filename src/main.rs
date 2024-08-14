use ops_crdt_rust::vector_clock;
use ops_crdt_rust::trcb;
use ops_crdt_rust::NodeType;

fn main() {
    let node_list = create_node_list();
    let vc000 = vector_clock::VectorClock::new(node_list.clone()).unwrap();
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

    let trcb1 = trcb::TRCBData::new(1, node_list.clone()).unwrap();

    println!("trcb {:?}", trcb1);

}

fn create_node_list() -> Vec<NodeType> {
    vec![0,1,2]
}

