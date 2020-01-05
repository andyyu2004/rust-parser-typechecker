use crate::parsing::Parser;
use regexlexer::TokenKind;
use crate::error::Error;
use crate::typechecking::Ty;
use super::parse_tuple;

pub(crate) fn parse_type<'a>(parser: &mut Parser<'a>) -> Result<Ty, Error> {
    if parser.matches(TokenKind::Bool) { Ok(Ty::Bool) }
    else if parser.matches(TokenKind::Int) { Ok(Ty::I64) }
    else if parser.matches(TokenKind::LParen) {
        parser.set_backtrack();
        let ty = parse_type(parser)?;
        // Parse single types within parens as a tuple
        if parser.matches(TokenKind::RParen) { return Ok(ty) }
        parser.backtrack();
        let types = parse_tuple(parser, parse_type)?;
        Ok(Ty::Tuple(types))
    } else if parser.matches(TokenKind::Fn) {
        parser.expect(TokenKind::LParen)?;
        let l = box Ty::Tuple(parse_tuple(parser, parse_type)?);
        parser.expect(TokenKind::RArrow)?;
        let r = box parse_type(parser)?;
        Ok(Ty::Arrow(l, r))
    } else if parser.matches(TokenKind::Typename) {
        unimplemented!();
    } else {
        unimplemented!();
    }
}
