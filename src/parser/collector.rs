use crate::lexer::{LexerIntoIter, token::Token};

use super::node::{ Node, NodeType };

/**
 * Implements collection logic for each parent-level node.
 */
pub struct Collector {
    pub active: bool,
    pub node: Node,
    
    pub put: Box<dyn Fn(&mut Collector, Token)>
}

impl Collector {
    pub fn new(node_type: NodeType) -> Self {

        let put: Option<Box<dyn Fn(&mut Collector, Token)>> = match node_type {
            NodeType::String => Some(Box::new(put_string)),
            NodeType::Function => Some(Box::new(put_function)),
            _ => None
        };

        Collector { 
            active: false,
            node: Node::new(node_type),
            put: put.unwrap()
        }
    }
}

pub fn put_string(c: &mut Collector, token: Token) {
    if c.active == false {
        
    }
}

pub fn put_function(c: &mut Collector, token: Token) {

}