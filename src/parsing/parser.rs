use super::{Expr, Precedence};
use regexlexer::{Token, TokenKind};
use crate::error::Error;
use super::parselets::*;
use crate::util::Assert;
use crate::typechecking::Ty;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    i: usize,
    node_id: u64,
    backtrack_index: usize,
}

type NullParseFn = for<'r, 'b> fn(&'r mut Parser<'b>, Token<'b>)           -> Result<Expr<'b>, Error>;
type LeftParseFn = for<'r, 'b> fn(&'r mut Parser<'b>, Expr<'b>, Token<'b>) -> Result<Expr<'b>, Error>;

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { tokens, i: 0, node_id: 0, backtrack_index: 0 }
    }

    pub(crate) fn gen_id(&mut self) -> u64 {
        self.node_id += 1;
        self.node_id
    }

    pub(crate) fn gen_type_var(&mut self) -> Ty { Ty::Infer(self.gen_id()) }

    pub fn parse(&mut self) -> Result<Expr<'a>, Vec<Error>> {
        self.parse_expression(Precedence::ZERO)
            .assert(|_| self.peek().map(|x| x.kind) == Ok(TokenKind::EOF),
                    || Error::new(self.curr_or_last(), format!("Did not consume all tokens (debug::currently on {:?})", self.peek())))
            .map_err(|err| vec![err])
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
            TokenKind::LParen     => Some(parse_group),
            TokenKind::Identifier => Some(parse_id),
            TokenKind::Str        => Some(parse_str),
            TokenKind::Let        => Some(parse_let),
            TokenKind::LBrace     => Some(parse_block),
            TokenKind::Fn         => Some(parse_lambda),
            TokenKind::False | TokenKind::True => Some(parse_bool),
            TokenKind::Plus | TokenKind::Minus | TokenKind::Tilde | TokenKind::Bang => Some(parse_prefix_op),
            _ => None
        }
    }

    fn get_left_denotation_rule(token_kind: TokenKind) -> LeftParseFn {
        match token_kind {
            TokenKind::LParen => parse_application,
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Slash
                | TokenKind::Star
                | TokenKind::LT
                | TokenKind::LTE
                | TokenKind::GT
                | TokenKind::GTE
                | TokenKind::DEqual
                | TokenKind::BangEqual
                | TokenKind::DStar => parse_binary,
            _ => unimplemented!()
        }
    }

    /// Returns ref to current token and pushes the index forward if the peek is succesful
    fn next(&mut self) -> Result<Token<'a>, Error> {
        self.peek().map(|tok| { self.i += 1; tok })
    }

    pub(crate) fn set_backtrack(&mut self) { self.backtrack_index = self.i }
    pub(crate) fn backtrack(&mut self) { self.i = self.backtrack_index }

    /// Returns ref to current token or an error if the current token is at EOF or even further
    fn peek(&self) -> Result<Token<'a>, Error> {
        if self.i < self.tokens.len() {
            Ok(self.tokens[self.i])
        } else {
            Err(Error::new(*self.tokens.last().unwrap(), "Ran out of tokens".to_owned()))
        }
    }

    /// Asserts the next token is the one given;
    pub(crate) fn expect(&mut self, kind: TokenKind) -> Result<Token<'a>, Error> {
        let curr = self.peek()?;
        if curr.kind == kind {
            self.i += 1;
            Ok(curr)
        } else {
            Err(Error::new(curr, format!("Expected `{}` found `{}`", kind, curr.kind)))
        }
    }

    /// Returns a boolean indicating whether the next token matches the one provided;
    /// If so, consumes the token;
    pub(crate) fn matches(&mut self, kind: TokenKind) -> bool {
        let is_match = self.peek().map(|t| t.kind) == Ok(kind);
        if is_match { self.i += 1 };
        is_match
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








