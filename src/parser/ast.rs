use super::super::lexer::Lexer;

pub fn parse<S: Into<String>>(input: S) {
    let lexer = Lexer::new(input);

    for item in lexer {
        
    }
}