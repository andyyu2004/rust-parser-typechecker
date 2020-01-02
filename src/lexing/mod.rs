mod token_kind;
use regex::Regex;
use regexlexer::{LexSyntax, TokenKind, Token};
use crate::util::Dummy;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
                m
        }
    };
);

impl<'a> Dummy for Token<'a> {
    fn dummy() -> Self {
        Token {
            kind: TokenKind::Unknown,
            lexeme: "",
            line: std::usize::MAX,
            col: std::usize::MAX,
        }
    }
}

pub fn gen_syntax() -> LexSyntax {
    LexSyntax {
        symbols: vec! [
            /* keywords */
            (Regex::new(r#"^let"#).unwrap(),   TokenKind::Let),
            (Regex::new(r#"^in"#).unwrap(),    TokenKind::In),

            /* token classes */
            (Regex::new(r#"^(0|[1-9][0-9]*)"#).unwrap(), TokenKind::Integral),
            (Regex::new(r#"^[a-z]"#).unwrap(),        TokenKind::Identifier),

            /* symbols */
            (Regex::new(r#"^ "#).unwrap(),     TokenKind::Space),
            (Regex::new(r#"^\\"#).unwrap(),    TokenKind::Backslash),
            (Regex::new(r#"^\("#).unwrap(),    TokenKind::LParen),
            (Regex::new(r#"^\)"#).unwrap(),    TokenKind::RParen),
            (Regex::new(r#"^\+"#).unwrap(),    TokenKind::Plus),
            (Regex::new(r#"^-"#).unwrap(),     TokenKind::Minus),
            (Regex::new(r#"^/"#).unwrap(),     TokenKind::Slash),
            (Regex::new(r#"^\*"#).unwrap(),    TokenKind::Star),
            (Regex::new(r#"^\."#).unwrap(),    TokenKind::Dot),
            (Regex::new(r#"^~"#).unwrap(),     TokenKind::Tilde),
            (Regex::new(r#"^!"#).unwrap(),     TokenKind::Bang),
        ],

        comments: vec! [
            Regex::new(r#"^//.*(\n|\z)"#).unwrap(),
            Regex::new(r#"^/\*.*\*/"#).unwrap()
        ]
    }
}




