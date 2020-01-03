use crate::parsing::{Parser, Expr, Precedence, ExprKind};
use regexlexer::Token;
use crate::error::Error;

pub(crate) fn parse_prefix_op<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    let expr = parser.parse_expression(Precedence::ZERO)?;
    let kind = ExprKind::Unary { op: token.kind, expr: box expr };
    Ok(Expr::new(
        token,
        kind,
        parser.gen_type_var(),
        parser.gen_id()
    ))

}
