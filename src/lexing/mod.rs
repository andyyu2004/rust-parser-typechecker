mod token_kind;
use regex::Regex;
use regexlexer::{LexSyntax, TokenKind, Token};
use crate::util::Dummy;
use crate::map;

impl<'a> Dummy for Token<'a> {
    fn dummy() -> Self {
        Token {
            kind: TokenKind::Unknown,
            lexeme: "",
            index: std::usize::MAX,
            line: std::usize::MAX,
            col: std::usize::MAX,
        }
    }
}

pub fn gen_syntax() -> LexSyntax {
    LexSyntax {
        symbols: vec! [
            /* token classes */
            (Regex::new(r#"^(0|[1-9][0-9]*)"#).unwrap(),     TokenKind::Integral),
            (Regex::new(r#"^([a-z][a-zA-Z0-9]*)"#).unwrap(), TokenKind::Identifier),
            (Regex::new(r#"^([A-Z][A-Za-z]*)"#).unwrap(),    TokenKind::Typename),
            (Regex::new(r#"^".*?""#).unwrap(),               TokenKind::Str), // Non-greedy match *?

            /* symbols */
            (Regex::new(r#"^->"#).unwrap(),    TokenKind::RArrow),
            (Regex::new(r#"^,"#).unwrap(),     TokenKind::Comma),
            (Regex::new(r#"^=>"#).unwrap(),    TokenKind::RFArrow),
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

        // This works because all keywords will either match identifier or typename
        keywords: map! {
            "let"   => TokenKind::Let,
            "false" => TokenKind::False,
            "true"  => TokenKind::True,
            "in"    => TokenKind::In,
            "fn"    => TokenKind::Fn,
            "Bool"  => TokenKind::Bool,
            "Int"   => TokenKind::Int
        },

        comments: vec! [
            Regex::new(r#"^//.*(\n|\z)"#).unwrap(),
            Regex::new(r#"^/\*.*\*/"#).unwrap()
        ]
    }
}




