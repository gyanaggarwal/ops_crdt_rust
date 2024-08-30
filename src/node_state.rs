use std::collections::HashMap;

use crate::NodeType;
use crate::crdt::{CrdtInstance, CRDT, CrdtType};
use crate::constants::NODE_LIST;
use crate::{IntMultCrdtValue, IntMultOpsValue};
use crate::crdt::AddMult;
use crate::vector_clock::VectorClockError;

#[derive(Debug)]
pub struct Node <CrdtValue: Clone, OpsValue: Clone+PartialEq, State>{
    pub crdt_instance: CrdtInstance,
    pub node_list: Vec<NodeType>,
    pub crdt_list: HashMap<NodeType, CRDT<CrdtValue, OpsValue, State>>,
    pub state: std::marker::PhantomData<State>
}

impl <CrdtValue: Clone, OpsValue: Clone+PartialEq, State> Node <CrdtValue, OpsValue, State> {
    pub fn new() -> Self {
        let crdt_instance = CrdtInstance::new(0, 0, CrdtType::AddMultCrdt);
        let node_list = NODE_LIST.to_owned();
        let crdt_list: HashMap<NodeType, CRDT<CrdtValue, OpsValue, State>> = HashMap::new();

        Self{crdt_instance, node_list, crdt_list, state: std::marker::PhantomData::<State>}
    }
}

impl Node<IntMultCrdtValue, IntMultOpsValue, AddMult> {
    pub fn populate(&mut self) -> Result<(), VectorClockError> {
        for n in self.node_list.clone() {
            let cins: CRDT<IntMultCrdtValue, IntMultOpsValue, AddMult>  = CRDT::new(n, 0)?;
            self.crdt_list.insert(n, cins);
        }
        Ok(())
    }

    pub fn get_crdt_instance(&self) -> CrdtInstance {
        self.crdt_instance.clone()
    }

    pub fn get_crdt(&mut self, node: u16) -> &CRDT<IntMultCrdtValue, IntMultOpsValue, AddMult> {
        self.crdt_list.get(&node).unwrap()
    }

    pub fn get_crdt_len(&self) -> u16 {
        self.node_list.len() as u16
    }
}