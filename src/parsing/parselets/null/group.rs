use crate::parsing::{Parser, ExprKind, Precedence};
use regexlexer::{Token, TokenKind};
use crate::error::Error;
use crate::typechecking::Ty;

pub(crate) fn parse_group<'a>(parser: &mut Parser<'a>, _token: Token<'a>) -> Result<(ExprKind, Option<Ty>), Error> {
    parser.set_backtrack();
    parser.parse_expression(Precedence::ZERO).and_then(|expr| {
        let ty = expr.ty.clone(); // The group has the same ty as its inner expr; no point generating another variable
        let kind = ExprKind::Grouping { expr: box expr };
        if parser.matches(TokenKind::RParen) {
            Ok((kind, Some(ty)))
        } else {
            parser.backtrack();
            let (elems, _span) = parser.parse_tuple(|p| Parser::parse_expression(p, Precedence::ZERO))?;
            let kind = ExprKind::Tuple { elems };
            Ok((kind, Some(parser.gen_type_var())))
        }
    })
}
