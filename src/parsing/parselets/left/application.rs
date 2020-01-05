use crate::parsing::{Parser, Expr, Precedence, ExprKind};
use regexlexer::Token;
use crate::error::Error;
use crate::parsing::parselets::parse_tuple;

pub(crate) fn parse_application<'a>(parser: &mut Parser<'a>, left: Expr<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    let args = parse_tuple(parser, |p| Parser::parse_expression(p, Precedence::CALL))?;
    let exprkind = ExprKind::App { f: box left, args };
    Ok(Expr::new(token, exprkind, parser.gen_type_var(), parser.gen_id()))
}
