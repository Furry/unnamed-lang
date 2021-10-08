use super::super::lexer::Lexer;

pub fn parse<S: Into<String>>(input: S) {
    let lexer = Lexer::new(input);

    // This iterater handles the first pass.
    for item in lexer {
        
    }
}

// pub struct 