mod utils;
mod lexer;
mod parser;

#[cfg(test)]
mod tests;

fn main() {
    // Read file tests/run.ul
    let contents = std::fs::read_to_string("tests/run.ul").unwrap();
    let lexer = lexer::Lexer::new(contents);

    for token in lexer {
        println!("{:?}", token);
    }
}
