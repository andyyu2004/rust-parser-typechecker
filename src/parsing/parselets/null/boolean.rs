use crate::parsing::{Parser, ExprKind};
use crate::typechecking::{Ty, TyKind};
use regexlexer::Token;
use crate::error::Error;

pub(crate) fn parse_bool<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<(ExprKind, Option<Ty>), Error> {
    let b = token.lexeme.parse::<bool>().unwrap();
    let kind = ExprKind::Bool { b };
    let ty = Ty::new(parser.get_single_span(), TyKind::Bool);
    Ok((kind, Some(ty)))
}
