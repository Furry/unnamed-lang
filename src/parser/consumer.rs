use super::super::lexer::token::Token;

trait Consumer {
    fn consume(&mut self, input: Token) -> bool;
    fn supply(&mut self) -> Option<>;
}

trait Supplier {
    fn supply() -> Self;
}

pub struct Function {
    name: String,
    parameters: Vec<String>,
    modifiers: Vec<String>,
}
pub struct Emitter {
    name: String,
    tick_speed: i64,
}
