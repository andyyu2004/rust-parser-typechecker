use super::{Expr, Precedence};
use regexlexer::{Token, TokenKind};
use crate::error::Error;
use super::parselets::*;
use crate::util::Assert;

pub struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    i: usize,
    node_id: u64,
}

type NullParseFn = for<'r, 'b> fn(&'r mut Parser<'b>, Token<'b>)           -> Result<Expr<'b>, Error>;
type LeftParseFn = for<'r, 'b> fn(&'r mut Parser<'b>, Expr<'b>, Token<'b>) -> Result<Expr<'b>, Error>;

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Parser { tokens, i: 0, node_id: 0 }
    }

    pub fn gen_id(&mut self) -> u64 {
        self.node_id += 1;
        self.node_id
    }

    pub fn parse(&mut self) -> Result<Expr<'a>, Error> {
        self.parse_expression(Precedence::ZERO)
            .assert(|_| self.peek().map(|x| x.kind) == Ok(TokenKind::EOF),
                    || Error::new(self.curr_or_last(), format!("Did not consume all tokens (debug::currently on {:?})", self.peek())))
    }

    fn curr_precedence(&self) -> Precedence {
        self.peek().map(Precedence::of_left).unwrap_or(Precedence::ZERO)
    }

    pub(crate) fn parse_expression(&mut self, precedence: Precedence) -> Result<Expr<'a>, Error> {
        let token = self.next()?;
        let null_parse_fn = Parser::get_null_denotation_rule(token.kind)
            .ok_or(Error::new(token, format!("Failed to parse null denotation token `{}`", token)))?;
        let mut expr = null_parse_fn(self, token)?;

        // The left denotation lookup won't fail unlike the null denotation due to the precedence condition on the while loop
        // Bad operators will have zero precedence and hence never enter the loop
        while self.curr_precedence() > precedence {
            let token = self.next()?;
            let left_parse_fn = Parser::get_left_denotation_rule(token.kind);
            expr = left_parse_fn(self, expr, token)?;
        }

        Ok(expr)
    }

    /// Returns the relevant null denotation parse function for the tokenkind
    fn get_null_denotation_rule(token_kind: TokenKind) -> Option<NullParseFn> {
        match token_kind {
            TokenKind::Integral   => Some(parse_integral),
            TokenKind::Identifier => Some(parse_id),
            TokenKind::Plus | TokenKind::Minus | TokenKind::Tilde | TokenKind::Bang => Some(parse_prefix_op),
            _ => None
        }
    }

    fn get_left_denotation_rule(token_kind: TokenKind) -> LeftParseFn {
        match token_kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Slash | TokenKind::Star => parse_binary,
            _ => unimplemented!()
        }
    }

    /// Returns ref to current token and pushes the index forward if the peek is succesful
    fn next(&mut self) -> Result<Token<'a>, Error> {
        self.peek().map(|tok| { self.i += 1; tok })
    }

    /// Returns ref to current token or an error if the current token is at EOF or even further
    fn peek(&self) -> Result<Token<'a>, Error> {
        if self.i < self.tokens.len() {
            Ok(self.tokens[self.i])
        } else {
            Err(Error::new(*self.tokens.last().unwrap(), "Ran out of tokens".to_owned()))
        }
    }

    /// Convenience method for grabbing a token for error handling purposes
    fn curr_or_last(&self) -> Token {
        if self.i < self.tokens.len() { self.tokens[self.i] }
        else { *self.tokens.last().unwrap() }
    }
}


/*
The gen only typechecks if the vec! is returned immediately. If bound to a name then returned, it fails. Interesting
fn parse_prefix_op<'a>(parser: &mut Parser, token: Token) -> Result<Expr<'a>, Error> { unimplemented!() }
fn parse_identifier<'a>(parser: &mut Parser, token: Token) -> Result<Expr<'a>, Error> { unimplemented!() }
fn gen<'a>() -> Vec<Box<dyn Fn(&mut Parser, Token) -> Result<Expr<'a>, Error>>> {
    vec! [
        box parse_prefix_op,
        box parse_identifier,
    ]
}
*/








