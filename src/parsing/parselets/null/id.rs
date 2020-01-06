use crate::parsing::{Parser, ExprKind};
use regexlexer::Token;
use crate::error::Error;
use crate::typechecking::Ty;

pub(crate) fn parse_id<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<(ExprKind, Option<Ty>), Error> {
    let kind = ExprKind::Id { name: token.lexeme.to_owned() };
    Ok((kind, None))
}
