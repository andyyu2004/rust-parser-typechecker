use crate::parsing::{Parser, Expr, ExprKind};
use crate::typechecking::Ty;
use regexlexer::Token;
use crate::error::Error;

pub(crate) fn parse_integral<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    let value = token.lexeme.parse::<i64>().unwrap();
    let kind = ExprKind::Integral { value };
    Ok(Expr::new(
        token,
        kind,
        Ty::I64,
        parser.gen_id()
    ))
}
