use super::string;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    BackTick,                // `
    Space,                   // ' '
    NewLine,                 // \n
    Return,                  // \r
    
    // Keywords
    Let,                     // let
    If,                      // if
    Else,                    // else
    While,                   // while
    For,                     // for
    In,                      // in
    Return_,                 // return
    Break,                   // break
    Continue,                // continue
    Struct,                  // struct
    Enum,                    // enum
    Fn,                      // fn
    True,                    // true
    False,                   // false
    Null,                    // null
    Emitter,                 // emitter

    Unknown(char),           // Unknown

    NumericSequence(String),    // NumericSequence
    AbstractSequence(String),   // AbstractSequence
    AlphabeticSequence(String), // AlphabeticSequence

}

impl Token {
    pub fn char_value(t: Token) -> Option<char> {
        Some(match t {
            Token::OpenParenthesis => '(',
            Token::CloseParenthesis => ')',
            Token::OpenSquareBracket => '[',
            Token::CloseSquareBracket => ']',
            Token::OpenCurlyBracket => '{',
            Token::CloseCurlyBracket => '}',
            Token::PlusSign => '+',
            Token::MinusSign => '-',
            Token::Asterisk => '*',
            Token::ForwardSlash => '/',
            Token::BackSlash => '\\',
            Token::PercentSign => '%',
            Token::EqualsSign => '=',
            Token::LessThanSign => '<',
            Token::GreaterThanSign => '>',
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
            Token::Space => ' ',
            Token::NewLine => '\n',
            Token::Return => '\r',
            Token::Unknown(c) => c,
            _ => return None,
        })
    }

    pub fn from_str<S: Into<String>>(input_: S) -> Token {
        let input: String = input_.into();
        let keyword_match = Some(match input.as_str() {
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "return" => Token::Return_,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "fn" => Token::Fn,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            "emitter" => Token::Emitter,
            _ => Token::Unknown('\0'),
        });

        if keyword_match.clone().unwrap() == Token::Unknown('\0') {
            if string::is_numeric(input.clone()) {
                return Token::NumericSequence(input);
            } else if string::is_alphabetic(input.clone()) {
                return Token::AlphabeticSequence(input);
            } else {
                return Token::AbstractSequence(input);
            }
        } else {
            return keyword_match.unwrap();
        }

    }

    pub fn from_char(c: char) -> Option<Token> {
        Some(match c {
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
            '\n' => Token::NewLine,
            '\r' => Token::Return,
            _ => Token::Unknown(c),
        })
    }

    pub fn is_token(c: char) -> bool {
        let token = Token::from_char(c);
        if token.is_some() {
            return match token.unwrap() {
                Token::Unknown(_) | Token::AbstractSequence(_) | Token::NumericSequence(_) | Token::AlphabeticSequence(_) => false,
                _ => true,
            }
        } else {
            return false;
        }
    }
}

