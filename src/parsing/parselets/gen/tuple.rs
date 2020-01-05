use crate::parsing::Parser;
use regexlexer::TokenKind;
use crate::error::Error;

pub(crate) fn parse_tuple<'a, T>(parser: &mut Parser<'a>, parse_fn: impl Fn(&mut Parser<'a>) -> Result<T, Error>) -> Result<Vec<T>, Error> {
    let mut vec = vec![];
    while !parser.matches(TokenKind::RParen) {
        vec.push(parse_fn(parser)?);
        if !parser.matches(TokenKind::Comma) {
            parser.expect(TokenKind::RParen)?;
            break;
        }
    }
    Ok(vec)
}
