use std::collections::HashMap;

use dotenvy::dotenv;

use crate::NodeType;
use crate::constants::NODE_LIST;
use crate::vector_clock::VectorClockError;

use crate::node_instance::NodeInstance;

#[derive(Debug)]
pub struct NodeState{
    pub node_instance_list: HashMap<NodeType, NodeInstance>
}

impl NodeState {
    pub fn new() -> Result<Self, VectorClockError> {
        dotenv().ok();
        let node_list = NODE_LIST.to_owned();
        let mut node_instance_list = HashMap::new();
        for node in node_list {
            let node_instance = NodeInstance::new(node)?;
            node_instance_list.insert(node, node_instance);
        }
        Ok(Self{node_instance_list})
    }

    pub fn get_node_instance(&self, node: NodeType) -> &NodeInstance {
        self.node_instance_list.get(&node).unwrap()
    }

    pub fn get_node_len(&self) -> u16 {
        self.node_instance_list.len() as u16
    }
}
