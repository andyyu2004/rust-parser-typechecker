use crate::parsing::{Parser, Expr, Precedence, ExprKind};
use regexlexer::{Token, TokenKind};
use crate::error::Error;

/// Returns the precedence accounting for associativity
fn precedence(token: Token) -> Precedence {
    Precedence::of_left(token) - if right_associative(token) { 1 } else { 0 }
}

fn right_associative(token: Token) -> bool {
    match token.kind {
        TokenKind::Caret => true,
        _                => false,
    }
}

pub(crate) fn parse_binary<'a>(parser: &mut Parser<'a>, left: Expr<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    let right = box parser.parse_expression(precedence(token))?;
    let exprkind = ExprKind::Binary { op: token.kind, left: box left, right };
    Ok(Expr::new(token, exprkind, parser.gen_id()))
}
