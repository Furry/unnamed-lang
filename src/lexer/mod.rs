use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    OpenParenthesis,         // (
    CloseParenthesis,        // )
    OpenSquareBracket,       // [
    CloseSquareBracket,      // ]
    OpenCurlyBracket,        // {
    CloseCurlyBracket,       // }

    // Operators
    PlusSign,                // +
    MinusSign,               // -
    Asterisk,                // *
    ForwardSlash,            // /
    BackSlash,               // \
    PercentSign,             // %
    EqualsSign,              // =
    LessThanSign,            // <
    GreaterThanSign,         // >
    Ampersand,               // &
    VerticalBar,             // |
    Caret,                   // ^
    ExclamationMark,         // !
    Tilde,                   // ~
    QuestionMark,            // ?
    Colon,                   // :
    Semicolon,               // ;
    Comma,                   // ,
    Dot,                     // .
    DoubleQuote,             // "
    SingleQuote,             // '
    BackTick,               // `
    Space,                   // ' '
    
    Unknown(char),           // Unknown

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lexer {
    source: String,
    position: usize,
}

impl Token {
    pub fn is_operator(&self) -> bool {
        match self {
            Token::Unknown(_) => false,
            _ => true
        }
    }
}

impl Into<char> for Token {
    fn into(self) -> char {
        match self {
            Token::OpenParenthesis => '(',
            Token::CloseParenthesis => ')',
            Token::OpenSquareBracket => '[',
            Token::CloseSquareBracket => ']',
            Token::OpenCurlyBracket => '{',
            Token::CloseCurlyBracket => '}',
            Token::Ampersand => '&',
            Token::VerticalBar => '|',
            Token::Caret => '^',
            Token::ExclamationMark => '!',
            Token::Tilde => '~',
            Token::QuestionMark => '?',
            Token::Colon => ':',
            Token::Semicolon => ';',
            Token::Comma => ',',
            Token::Dot => '.',
            Token::DoubleQuote => '"',
            Token::SingleQuote => '\'',
            Token::BackTick => '`',
            Token::PlusSign => '+',
            Token::MinusSign => '-',
            Token::Asterisk => '*',
            Token::ForwardSlash => '/',
            Token::BackSlash => '\\',
            Token::PercentSign => '%',
            Token::EqualsSign => '=',
            Token::LessThanSign => '<',
            Token::GreaterThanSign => '>',
            Token::Space => ' ',
            Token::Unknown(c) => c,
        }
    }
}

impl Lexer {
    pub fn new<S: Into<String>>(source: S) -> Lexer {
        Lexer {
            source: source.into(),
            position: 0,
        }
    }

    pub fn segment(&self, start: usize, end: Token) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        // let mut tokens = Vec::new();
        self.clone().into_iter()
            .skip(start)
            .take_while(|t| *t != end)
            .for_each(|t| tokens.push(t));
        tokens
    }

    pub fn forward(&mut self, token: Token) -> usize {
        let mut size: usize = 0.try_into().unwrap();

        let itr = self.clone()
            .into_iter()
            .skip(self.position);

        for item in itr {
            if item == token {
                size += 1;
            } else {
                return size;
            }
        }
        size
    }
}

pub struct LexerIntoIterator {
    lexer: Lexer,
    index: usize,
}

impl IntoIterator for Lexer {
    type Item = Token;
    type IntoIter = LexerIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        LexerIntoIterator {
            lexer: self,
            index: 0,
        }
    }
}

impl Iterator for LexerIntoIterator {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let mut token = None;
        if self.index < self.lexer.source.len() {
            let c = self
                .lexer
                .source
                .chars()
                .nth(self.index)
                .unwrap();
            self.index += 1;
            token = Some(match c {
                '(' => Token::OpenParenthesis,
                ')' => Token::CloseParenthesis,
                '[' => Token::OpenSquareBracket,
                ']' => Token::CloseSquareBracket,
                '{' => Token::OpenCurlyBracket,
                '}' => Token::CloseCurlyBracket,
                '+' => Token::PlusSign,
                '-' => Token::MinusSign,
                '*' => Token::Asterisk,
                '/' => Token::ForwardSlash,
                '\\' => Token::BackSlash,
                '%' => Token::PercentSign,
                '=' => Token::EqualsSign,
                '<' => Token::LessThanSign,
                '>' => Token::GreaterThanSign,
                '&' => Token::Ampersand,
                '|' => Token::VerticalBar,
                '^' => Token::Caret,
                '!' => Token::ExclamationMark,
                '~' => Token::Tilde,
                '?' => Token::QuestionMark,
                ':' => Token::Colon,
                ';' => Token::Semicolon,
                ',' => Token::Comma,
                '.' => Token::Dot,
                '"' => Token::DoubleQuote,
                '\'' => Token::SingleQuote,
                '`' => Token::BackTick,
                ' ' => Token::Space,
                _ => Token::Unknown(c),
            });
        }
        token
    }
}