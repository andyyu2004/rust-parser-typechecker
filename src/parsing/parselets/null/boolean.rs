use crate::parsing::{Parser, Expr, ExprKind};
use crate::typechecking::Ty;
use regexlexer::Token;
use crate::error::Error;

pub(crate) fn parse_bool<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    let b = token.lexeme.parse::<bool>().unwrap();
    let kind = ExprKind::Bool { b };
    Ok(Expr::new(
        token,
        kind,
        Ty::Bool,
        parser.gen_id()
    ))
}
