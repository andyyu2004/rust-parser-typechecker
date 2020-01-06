use crate::parsing::{Parser, ExprKind, Precedence};
use regexlexer::{Token, TokenKind};
use crate::error::Error;
use crate::typechecking::Ty;

pub(crate) fn parse_block(parser: &mut Parser, _token: Token) -> Result<(ExprKind, Option<Ty>), Error> {
    let mut exprs = vec![];
    let mut suppressed = false;
    loop {
        // If a } is matched, then either the block is empty or the final expression had a semicolon
        // And hence suppressed should be set
        if parser.matches(TokenKind::RBrace) { suppressed = true; break; }
        exprs.push(parser.parse_expression(Precedence::ZERO)?);
        if !parser.matches(TokenKind::SemiColon) { break }
    };

    // If the supppresed block didn't execute above, we must consume the closing } still
    if !suppressed { parser.expect(TokenKind::RBrace)?; };
    let kind = ExprKind::Block { exprs, suppressed };
    Ok((kind, None))
}
