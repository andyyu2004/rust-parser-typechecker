use crate::parsing::{Parser, ExprKind, Precedence};
use crate::typechecking::Ty;
use regexlexer::{Token, TokenKind};
use crate::error::Error;

pub(crate) fn parse_let<'a>(parser: &mut Parser<'a>, _token: Token<'a>) -> Result<(ExprKind, Option<Ty>), Error> {
    let binder = parser.parse_binder()?;
    parser.expect(TokenKind::Equal)?;
    let bound = box parser.parse_expression(Precedence::ZERO)?;
    parser.expect(TokenKind::In)?;
    let body = box parser.parse_expression(Precedence::ZERO)?;
    let kind = ExprKind::Let { binder, bound, body };
    Ok((kind, None))
}


