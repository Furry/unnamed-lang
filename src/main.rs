mod utils;
mod lexer;

fn main() {
    // Read file tests/run.ul
    // let contents = std::fs::read_to_string("tests/run.ul").unwrap();
    // let lexer = lexer::Lexer::new(contents);

    // for token in lexer {
    //     println!("{:?}", token);
    // }

    let lexer = lexer::Lexer::new("let x = 5;");
    for token in lexer {
        println!("{:?}", token);
    }
}
