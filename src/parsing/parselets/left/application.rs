use crate::parsing::{Parser, Expr, Precedence, ExprKind};
use regexlexer::Token;
use crate::error::Error;
use crate::typechecking::Ty;

pub(crate) fn parse_application<'a>(parser: &mut Parser<'a>, left: Expr, token: Token<'a>) -> Result<(ExprKind, Option<Ty>), Error> {
    let (args, _) = parser.parse_tuple(|p| Parser::parse_expression(p, Precedence::CALL))?;
    let exprkind = ExprKind::App { f: box left, args };
    Ok((exprkind, None))
}
