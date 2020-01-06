use super::{Expr, Precedence, Span, ExprKind, Binder};
use regexlexer::{Token, TokenKind};
use crate::error::Error;
use super::parselets::*;
use crate::typechecking::{Ty, TyKind};

pub struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    i: usize, // Current index inside tokens
    id: u64,  // Counter to generate unique ids
    span_stack: Vec<usize>,
    backtrack_index: usize,
}

// Parser functions return a tuple of an exprkind as the fields of the expr can be filled out by the parser
// It also return a Option of a Type if there is a better option than just generating a new unification variable
type NullParseFn = for<'r, 'b> fn(&'r mut Parser<'b>, Token<'b>)       -> Result<(ExprKind, Option<Ty>), Error>;
type LeftParseFn = for<'r, 'b> fn(&'r mut Parser<'b>, Expr, Token<'b>) -> Result<(ExprKind, Option<Ty>), Error>;

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Parser { tokens, i: 0, id: 0, backtrack_index: 0, span_stack: Vec::new() }
    }

    pub(crate) fn gen_id(&mut self) -> u64 {
        self.id += 1;
        self.id
    }

    /// Returns the index into the src file the parser is currently at
    pub(crate) fn src_index(&self) -> usize { self.curr_or_last().index }
    pub(crate) fn src_line(&self) -> usize { self.curr_or_last().line }

    fn get_span(&mut self) -> Span { Span::new(self.span_stack.pop().unwrap(), self.src_index(), self.src_line()) }
    fn peek_span(&self) -> Span { Span::new(*self.span_stack.last().unwrap(), self.src_index(), self.src_line()) }
    pub(crate) fn get_single_span(&self) -> Span { Span::single(self.src_index(), self.src_line()) }

    pub(crate) fn gen_type_var(&mut self) -> Ty {
        Ty::new(Span::single(self.src_index(), self.src_line()), TyKind::Infer(self.gen_id()))
    }

    pub fn parse(&mut self) -> Result<Expr, Vec<Error>> {
        let expr = self.parse_expression(Precedence::ZERO).map_err(|err| vec![err])?;
        if self.peek().map(|x| x.kind) != Ok(TokenKind::EOF) {
            return Err(vec![Error::new(expr.span, format!("Did not consume all tokens (debug::currently on {:?})", self.peek()))]);
        }
        Ok(expr)
    }

    fn curr_precedence(&self) -> Precedence { self.peek().map(Precedence::of_left).unwrap_or(Precedence::ZERO) }

    pub(crate) fn parse_expression(&mut self, precedence: Precedence) -> Result<Expr, Error> {
        self.span_stack.push(self.src_index());
        let token = self.next()?;
        let null_parse_fn = Parser::get_null_denotation_rule(token.kind)
            .ok_or(Error::new(self.peek_span(), format!("Failed to parse null denotation token `{}`", token)))?;
        let (kind, ty) = null_parse_fn(self, token)?;
        let mut expr = Expr::new(self.get_span(), kind, ty.unwrap_or(self.gen_type_var()), self.gen_id());

        // The left denotation lookup won't fail unlike the null denotation due to the precedence condition on the while loop
        // Bad operators will have zero precedence and hence never enter the loop
        while self.curr_precedence() > precedence {
            self.span_stack.push(self.src_index());
            let token = self.next()?;
            let left_parse_fn = Parser::get_left_denotation_rule(token.kind);
            let (kind, ty) = left_parse_fn(self, expr, token)?;
            expr = Expr::new(self.get_span(), kind, ty.unwrap_or(self.gen_type_var()), self.gen_id())
        }
        Ok(expr)
    }

    pub(crate) fn parse_type(&mut self) -> Result<Ty, Error> {
        self.span_stack.push(self.src_index());
        if self.matches(TokenKind::Bool) {
            Ok(Ty::new(self.get_span(), TyKind::Bool))
        } else if self.matches(TokenKind::Int) {
            Ok(Ty::new(self.get_span(), TyKind::I64))
        } else if self.matches(TokenKind::LParen) {
            self.set_backtrack();
            let ty = self.parse_type()?;
            // Parse single types within parens as a tuple
            Ok(if self.matches(TokenKind::RParen) { ty } else {
                self.backtrack();
                let (types, span) = self.parse_tuple(Self::parse_type)?;
                Ty::new(span, TyKind::Tuple(types))
            })
        } else if self.matches(TokenKind::Fn) {
            self.expect(TokenKind::LParen)?;
            let (l, span) = self.parse_tuple(Self::parse_type)?;
            let ttuple = box Ty::new(span, TyKind::Tuple(l));
            self.expect(TokenKind::RArrow)?;
            let r = box self.parse_type()?;
            let kind = TyKind::Arrow(ttuple, r);
            Ok(Ty::new(self.get_span(), kind))
        } else if self.matches(TokenKind::Typename) {
            unimplemented!();
        } else {
            unimplemented!();
        }
    }

    pub(crate) fn parse_tuple<T>(&mut self, parse_fn: impl Fn(&mut Parser<'a>) -> Result<T, Error>) -> Result<(Vec<T>, Span), Error> {
        self.span_stack.push(self.src_index());
        let mut vec = vec![];
        while !self.matches(TokenKind::RParen) {
            vec.push(parse_fn(self)?);
            if !self.matches(TokenKind::Comma) {
                self.expect(TokenKind::RParen)?;
                break;
            }
        }
        Ok((vec, self.get_span()))
    }

    pub(crate) fn parse_binder(&mut self) -> Result<Binder, Error> {
        self.span_stack.push(self.src_index());
        let name = self.expect(TokenKind::Identifier)?.lexeme.to_owned();
        let ty = if self.matches(TokenKind::Colon) { self.parse_type()? }
        else { self.gen_type_var() };
        Ok(Binder::new(self.get_span(), name, ty))
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
            Err(Error::new(Span::single(self.src_index(), self.src_line()), "Ran out of tokens".to_owned()))
        }
    }

    /// Asserts the next token is the one given;
    pub(crate) fn expect(&mut self, kind: TokenKind) -> Result<Token<'a>, Error> {
        let curr = self.peek()?;
        if curr.kind == kind {
            self.i += 1;
            Ok(curr)
        } else {
            Err(Error::new(Span::single(self.src_index(), self.src_line()), format!("Expected `{}` found `{}`", kind, curr.kind)))
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
fn gen<'a>() -> Vec<Box<dyn Fn(&mut Parser, Token) -> Result<Expr, Error>>> {
    vec! [
        box parse_prefix_op,
        box parse_identifier,
    ]
}
*/








