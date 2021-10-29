use std::{collections::HashMap, slice::SliceIndex};

use super::lexer::{self, token::Token};


/**
 * Tests each result from a lexed-token to insure it's the enum equivilant to it's text-based name.
 */
#[test]
fn lexer_tokens() -> Result<(), String> {
    let tokens = "()[]{}+-*/\\%=<>&|^!~?:;,.\"'` \n\r";
    let token_arr: Vec<char> = tokens.chars().collect();

    let mut index = 0;
    let lexer_instance = lexer::Lexer::new(tokens);
    for token in lexer_instance.into_iter() {
        let char = token_arr.get(index).unwrap();
        let expected_token = lexer::token::Token::from_char(char.clone()).unwrap();
        if token != lexer::token::Token::from_char(char.clone()).unwrap() {
            return Err(format!("Check failed on compare {} to {:?}", char, expected_token).to_string());
        }
        index += 1;
    }

    Ok(())
}

/**
 * Tests for each result from a lexed-string to insure it's the enum equivilant to it's text-based name.
 */
#[test]
fn lexer_keywords() -> Result<(), String> {
    let mut mapping: HashMap<&'static str, lexer::token::Token> = HashMap::new();

    mapping.insert("let", Token::Let);
    mapping.insert("if", Token::If);
    mapping.insert("else", Token::Else);
    mapping.insert("while", Token::While);
    mapping.insert("for", Token::For);
    mapping.insert("in", Token::In);
    mapping.insert("return", Token::Return_);
    mapping.insert("break", Token::Break);
    mapping.insert("continue", Token::Continue);
    mapping.insert("struct", Token::Struct);
    mapping.insert("enum", Token::Enum);
    mapping.insert("fn", Token::Fn);
    mapping.insert("true", Token::True);
    mapping.insert("false", Token::False);
    mapping.insert("null", Token::Null);
    mapping.insert("emitter", Token::Emitter);

    mapping.insert("0123456789", Token::NumericSequence("0123456789".to_string()));
    mapping.insert("abcdefg", Token::AlphabeticSequence("abcdefg".to_string()));
    mapping.insert("ABCDEFG", Token::AlphabeticSequence("ABCDEFG".to_string()));
    mapping.insert("aBcDeFg", Token::AlphabeticSequence("aBcDeFg".to_string()));
    
    for pair in mapping {
        let key = pair.0;
        let token = pair.1;

        if Token::from_str(key) != token {
            return Err(format!("Check failed on compare {} to {:?}", key, token).to_string());
        }
    };

    Ok(())
}

/**
 * Tests for each result from a sample function definition excerpt to insure each token and keyword are the enum equivilant.
 */
#[test]
fn function_definition() -> Result<(), String> {
    let function_def = "fn myFunction() {} !emit main;";
    let expected_sequence = vec![
        Token::Fn,
        Token::Space,
        Token::AlphabeticSequence("myFunction".to_string()),
        Token::OpenParenthesis,
        Token::CloseParenthesis,
        Token::Space,
        Token::OpenCurlyBracket,
        Token::CloseCurlyBracket,
        Token::Space,
        Token::ExclamationMark,
        Token::AlphabeticSequence("emit".to_string()),
        Token::Space,
        Token::AlphabeticSequence("main".to_string()),
        Token::Semicolon
    ];

    let lexer_instance = lexer::Lexer::new(function_def);
    let mut index = 0;

    for token in lexer_instance.into_iter() {
        let expected = expected_sequence.get(index).unwrap();
        if token != expected.clone() {
            return Err(format!("Check failed on expect {:?} for {:?}", expected, token).to_string());
        }
        index += 1;
    };

    Ok(())
}