use crate::parsing::{Parser, Expr, ExprKind};
use regexlexer::Token;
use crate::error::Error;

pub(crate) fn parse_id<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    Ok(Expr::new(token, ExprKind::Id { name: token.lexeme }, parser.gen_id()))
}
