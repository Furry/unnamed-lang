use crate::lexer::Lexer;

pub mod consumer;
pub mod span;
pub mod node;
pub mod ast;

pub struct Parser {
    pub(crate) lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) {
        Parser { lexer };
    }

    pub fn bind(&self, name: String) {

    }
}