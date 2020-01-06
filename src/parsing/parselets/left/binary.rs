use crate::parsing::{Parser, Expr, Precedence, ExprKind};
use regexlexer::{Token, TokenKind};
use crate::error::Error;
use crate::typechecking::Ty;

/// Returns the precedence accounting for associativity
/// If an operator is right-associative, recursively parse expression with precedence of one less so it will parse itself
fn precedence(token: Token) -> Precedence {
    Precedence::of_left(token) - if right_associative(token) { 1 } else { 0 }
}

fn right_associative(token: Token) -> bool {
    // Assignment and exponentiation are right associative
    match token.kind {
        TokenKind::DStar => true,
        TokenKind::Equal => true,
        _                => false,
    }
}

pub(crate) fn parse_binary<'a>(parser: &mut Parser<'a>, left: Expr, token: Token<'a>) -> Result<(ExprKind, Option<Ty>), Error> {
    let right = box parser.parse_expression(precedence(token))?;
    let exprkind = ExprKind::Binary { op: token.kind, left: box left, right };
    Ok((exprkind, None))
}
