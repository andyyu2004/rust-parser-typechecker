use crate::parsing::{Parser, Expr, ExprKind};
use regexlexer::Token;
use crate::error::Error;

pub(crate) fn parse_integral<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    let value = token.lexeme.parse::<i64>().unwrap();
    let kind = ExprKind::Integral { value };
    Ok(Expr::new(
        token,
        kind,
        parser.gen_type_var(),
        parser.gen_id()
    ))
}
