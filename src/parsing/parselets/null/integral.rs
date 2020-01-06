use crate::parsing::{Parser, ExprKind};
use crate::typechecking::{Ty, TyKind};
use regexlexer::Token;
use crate::error::Error;

pub(crate) fn parse_integral<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<(ExprKind, Option<Ty>), Error> {
    let value = token.lexeme.parse::<i64>().unwrap();
    let kind = ExprKind::Integral { value };
    let ty = Ty::new(parser.get_single_span(), TyKind::I64);
    Ok((kind, Some(ty)))
}
