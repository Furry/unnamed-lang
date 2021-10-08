use crate::lexer::Lexer;

pub mod structures;
pub mod ast;

pub struct Parser {
    pub(crate) lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) {
        Parser { lexer };
    }

    pub fn bind(&self) {
        
    }
}