use crate::parsing::{Parser, Expr, ExprKind, Precedence};
use regexlexer::{Token, TokenKind};
use crate::error::Error;

pub(crate) fn parse_group<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
    parser.parse_expression(Precedence::ZERO).and_then(|expr| {
        let kind = ExprKind::Grouping { expr: box expr };
        parser.expect(TokenKind::RParen)?;
        Ok(Expr::new(
            token,
            kind,
            parser.gen_type_var(),
            parser.gen_id()
        ))
    })
}
