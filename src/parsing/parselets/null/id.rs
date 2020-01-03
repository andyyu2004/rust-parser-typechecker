use crate::parsing::{Parser, Expr, ExprKind};
use regexlexer::Token;
use crate::error::Error;

pub(crate) fn parse_id<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    let kind = ExprKind::Id { name: token };
    Ok(Expr::new(
        token,
        kind,
        parser.gen_type_var(),
        parser.gen_id()
    ))
}
