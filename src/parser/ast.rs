use crate::lexer::{ Lexer, token::Token };
use super::node::{Node, NodeKind};
use super::span::Span;

pub fn parse<S: Into<String>>(input: S) {
    let lexer = Lexer::new(input);

    // This iterater handles the first pass.
    for item in lexer {
        
    }
}

pub struct AST {
    pub depth: usize,
    pub root: Node,
}

impl AST {
    pub fn new() -> AST {
        AST {
            root: Node::new(NodeKind::Root, Span::new(0, 0)),
            depth: 0,
        }
    }
}