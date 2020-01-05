use crate::parsing::{Parser, Expr, ExprKind, Binder, Precedence};
use super::super::parse_type;
use regexlexer::{Token, TokenKind};
use crate::error::Error;
use super::parse_binder;
use crate::parsing::parselets::parse_tuple;

pub(crate) fn parse_lambda<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    // Allows no paren for single argument lambda
    let params = if !parser.matches(TokenKind::LParen) {
        vec![parse_binder(parser)?]
    } else { parse_tuple(parser, parse_binder)? };

    parser.expect(TokenKind::RFArrow)?;
    let body = box parser.parse_expression(Precedence::ZERO)?;
    let ret = if parser.matches(TokenKind::RArrow) { parse_type(parser)? } else { body.ty.clone() };
    let kind = ExprKind::Lambda { params, ret, body };
    Ok(Expr::new(token, kind, parser.gen_type_var(), parser.gen_id()))
}
