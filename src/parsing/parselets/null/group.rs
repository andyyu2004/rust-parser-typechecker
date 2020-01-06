use crate::parsing::{Parser, ExprKind, Precedence};
use regexlexer::{Token, TokenKind};
use crate::error::Error;
use crate::typechecking::Ty;

pub(crate) fn parse_group<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<(ExprKind, Option<Ty>), Error> {
    parser.parse_expression(Precedence::ZERO).and_then(|expr| {
        let kind = ExprKind::Grouping { expr: box expr };
        parser.expect(TokenKind::RParen)?;
        Ok((kind, None))
    })
}
