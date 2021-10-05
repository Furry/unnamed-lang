pub mod token;

use token::Token;
use super::utils::string;

pub struct Lexer {
    input: String,
}

pub struct LexerIntoIter {
    lexer: Lexer,
    position: usize,
}

impl IntoIterator for Lexer {
    type Item = Token;
    type IntoIter = LexerIntoIter;
    
    fn into_iter(self) -> Self::IntoIter {
        LexerIntoIter {
            lexer: self,
            position: 0,
        }
    }
}

impl Iterator for LexerIntoIter {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let input = self.lexer.input.clone();

        // Consume characters until we find a token.
        // let mut chars = input.chars().skip(self.position);

        // self.position += 1;
        // match input.chars().nth(self.position) {
        //     Some(c) => Token::from_char(c),
        //     None => None,
        // }

        let mut string_cache = String::new();
        while let Some(c) = input.chars().nth(self.position) {
            self.position += 1;
            // println!("indx: {}, char: {}", self.position, c);
            if Token::from_char(c).is_some() {
                if Token::is_token(c) {
                    // println!("token: {}", c);
                    if string_cache.len() > 1 {
                        return Some(Token::from_str(string_cache));
                    } else {
                        return Token::from_char(c);
                    }
                } else {
                    string_cache.push(c);
                }
            } else {
                return None
            }
        }

        None
        // Token::from_char(string_cache.chars().nth(0).unwrap())

        // let mut token_cache = String::new();
        // while let Some(c) = chars.next() {
        //     match c {
        //         ' ' | '\n' | '\t' => {
        //             if token_cache.len() > 0 {
        //                 break;
        //             }
        //         },
        //         _ => {
        //             token_cache.push(c);
        //         },
        //     }
        //     self.position += 1;
        // }

        // // Return the token.
        // if token_cache.len() == 1 {
        //     // First element from tokenCache
        //     Token::from_char(token_cache.chars().next().unwrap())
        // } else {
        //     Some(Token::from_str(token_cache))
        // }

        
    }
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Lexer {
        Lexer {
            input: input.into(),
        }
    }
}