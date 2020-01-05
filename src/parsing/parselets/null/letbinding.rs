use crate::parsing::{Parser, Expr, ExprKind, Binder, Precedence};
use regexlexer::{Token, TokenKind};
use crate::error::Error;
use super::super::parse_type;

pub(crate) fn parse_let<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    let binder = parse_binder(parser)?;
    parser.expect(TokenKind::Equal)?;
    let bound = box parser.parse_expression(Precedence::ZERO)?;
    parser.expect(TokenKind::In)?;
    let body = box parser.parse_expression(Precedence::ZERO)?;
    let kind = ExprKind::Let { binder, bound, body };
    Ok(Expr::new(token, kind, parser.gen_type_var(), parser.gen_id()))
}

pub(crate) fn parse_binder<'a>(parser: &mut Parser<'a>) -> Result<Binder<'a>, Error> {
    let name = parser.expect(TokenKind::Identifier)?;
    let ty = if parser.matches(TokenKind::Colon) { parse_type(parser)? }
    else { parser.gen_type_var() };
    Ok(Binder { name, ty })
}


