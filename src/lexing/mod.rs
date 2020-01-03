mod token_kind;
use regex::Regex;
use regexlexer::{LexSyntax, TokenKind, Token};
use crate::util::Dummy;

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
            (Regex::new(r#"^false"#).unwrap(), TokenKind::False),
            (Regex::new(r#"^true"#).unwrap(),  TokenKind::True),
            (Regex::new(r#"^in"#).unwrap(),    TokenKind::In),

            /* token classes */
            (Regex::new(r#"^(0|[1-9][0-9]*)"#).unwrap(), TokenKind::Integral),
            (Regex::new(r#"^[a-z]"#).unwrap(),           TokenKind::Identifier),
            (Regex::new(r#"^".*?""#).unwrap(),           TokenKind::Str), // Non-greedy match *?

            /* symbols */
            (Regex::new(r#"^ "#).unwrap(),     TokenKind::Space),
            (Regex::new(r#"^;"#).unwrap(),     TokenKind::SemiColon),
            (Regex::new(r#"^:"#).unwrap(),     TokenKind::Colon),
            (Regex::new(r#"^\\"#).unwrap(),    TokenKind::Backslash),
            (Regex::new(r#"^\("#).unwrap(),    TokenKind::LParen),
            (Regex::new(r#"^\)"#).unwrap(),    TokenKind::RParen),
            (Regex::new(r#"^\{"#).unwrap(),    TokenKind::LBrace),
            (Regex::new(r#"^\}"#).unwrap(),    TokenKind::RBrace),
            (Regex::new(r#"^\+"#).unwrap(),    TokenKind::Plus),
            (Regex::new(r#"^-"#).unwrap(),     TokenKind::Minus),
            (Regex::new(r#"^/"#).unwrap(),     TokenKind::Slash),
            (Regex::new(r#"^\*\*"#).unwrap(),  TokenKind::DStar),
            (Regex::new(r#"^\*"#).unwrap(),    TokenKind::Star),
            (Regex::new(r#"^\."#).unwrap(),    TokenKind::Dot),
            (Regex::new(r#"^~"#).unwrap(),     TokenKind::Tilde),
            (Regex::new(r#"^!="#).unwrap(),    TokenKind::BangEqual),
            (Regex::new(r#"^!"#).unwrap(),     TokenKind::Bang),
            (Regex::new(r#"^=="#).unwrap(),    TokenKind::DEqual),
            (Regex::new(r#"^="#).unwrap(),     TokenKind::Equal),
            (Regex::new(r#"<="#).unwrap(),     TokenKind::LTE),
            (Regex::new(r#"<"#).unwrap(),      TokenKind::LT),
            (Regex::new(r#">="#).unwrap(),     TokenKind::GTE),
            (Regex::new(r#">"#).unwrap(),      TokenKind::GT),
        ],

        comments: vec! [
            Regex::new(r#"^//.*(\n|\z)"#).unwrap(),
            Regex::new(r#"^/\*.*\*/"#).unwrap()
        ]
    }
}




