use crate::parsing::{Parser, ExprKind};
use regexlexer::Token;
use crate::error::Error;
use crate::typechecking::Ty;

pub(crate) fn parse_str<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<(ExprKind, Option<Ty>), Error> {
    // Trim surrounding quotes
    let string = token.lexeme[1..token.lexeme.len() - 1].to_owned();
    Ok((ExprKind::Str { string }, None))
}
