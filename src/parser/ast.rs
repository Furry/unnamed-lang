use crate::lexer::{ Lexer, token::Token };

pub fn parse<S: Into<String>>(input: S) {
    let lexer = Lexer::new(input);

    // This iterater handles the first pass.
    for item in lexer {
        
    }
}

pub struct AST {
    pub root: Node,
}

pub struct Node {
    pub token: Token,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(token: Token) -> Node {
        Node {
            token,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}
// pub struct 