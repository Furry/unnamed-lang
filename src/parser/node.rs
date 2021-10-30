#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub node_type: NodeType,
    pub start: Option<Box<Node>>,
    pub body: Option<Box<Node>>,
    pub stop: Option<Box<Node>>
}

pub struct Collector {

}

#[derive(Debug, PartialEq, Eq)]
pub enum NodeType {
    Program,
    Function,
    String
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Node { node_type: node_type, start: None, body: None, stop: None }
    }
}

impl Collector {
    pub fn new() {
        
    }
}