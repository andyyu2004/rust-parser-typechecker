use crate::parsing::{Parser, Expr, ExprKind};
use regexlexer::Token;
use crate::error::Error;

pub(crate) fn parse_str<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    let string = &token.lexeme[1..token.lexeme.len() - 1];
    Ok(Expr::new(
        token,
        ExprKind::Str { string },
        parser.gen_type_var(),
        parser.gen_id()
    ))
}
