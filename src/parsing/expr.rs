use regexlexer::{TokenKind, Token};
use std::fmt::{self, Display, Formatter, Debug};
use crate::typechecking::Ty;

#[derive(PartialEq, Clone)]
pub struct Expr<'a> {
    token: Token<'a>,
    kind: ExprKind<'a>,
    ty: Option<Ty>,
    node_id: u64,
}

impl<'a> Expr<'a> {
    pub fn new(token: Token<'a>, kind: ExprKind<'a>, node_id: u64) -> Self {
        Expr { token, kind, ty: None, node_id }
    }

    pub fn with_type(token: Token<'a>, kind: ExprKind<'a>, ty: Ty, node_id: u64) -> Self {
        Expr { token, kind, ty: Some(ty), node_id }
    }

}

#[derive(Clone, PartialEq)]
pub enum ExprKind<'a> {
    Unary { op: TokenKind, expr: Box<Expr<'a>> },
    Integral { value: i64 },
    Id { name: &'a str },
    Binary { op: TokenKind, left: Box<Expr<'a>>, right: Box<Expr<'a>> }
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ExprKind::Unary { op, expr }         => write!(f, "{}{}", op, expr),
            ExprKind::Integral { value }         => write!(f, "{}", value),
            ExprKind::Id { name }                => write!(f, "{}", name),
            ExprKind::Binary { op, left, right } => write!(f, "{} {} {}", left, op, right),
        }
    }
}

impl<'a> Debug for Expr<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ExprKind::Unary { op, expr }         => write!(f, "({}{:?})", op, expr),
            ExprKind::Integral { value }         => write!(f, "{}", value),
            ExprKind::Id { name }                => write!(f, "{}", name),
            ExprKind::Binary { op, left, right } => write!(f, "({} {:?} {:?})", op, left, right),
        }
    }
}



















