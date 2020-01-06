use crate::parsing::{Parser, ExprKind, Precedence};
use regexlexer::{Token, TokenKind};
use crate::error::Error;
use crate::typechecking::Ty;

pub(crate) fn parse_lambda(parser: &mut Parser, token: Token) -> Result<(ExprKind, Option<Ty>), Error> {
    // Allows no paren for single argument lambda
    let params = if !parser.matches(TokenKind::LParen) {
        vec![parser.parse_binder()?]
    } else { parser.parse_tuple(Parser::parse_binder)?.0 };

    parser.expect(TokenKind::RFArrow)?;
    let body = box parser.parse_expression(Precedence::ZERO)?;
    let ret = if parser.matches(TokenKind::RArrow) { parser.parse_type()? } else { body.ty.clone() };
    let kind = ExprKind::Lambda { params, ret, body };
    Ok((kind, None))
}
