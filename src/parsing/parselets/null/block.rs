use crate::parsing::{Parser, Expr, ExprKind, Binder, Precedence};
use regexlexer::{Token, TokenKind};
use crate::error::Error;

pub(crate) fn parse_block<'a>(parser: &mut Parser<'a>, token: Token<'a>) -> Result<Expr<'a>, Error> {
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
    Ok(Expr::new(token, kind, parser.gen_type_var(), parser.gen_id()))
}
