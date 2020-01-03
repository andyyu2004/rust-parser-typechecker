use regexlexer::{TokenKind, Token};
use std::fmt::{self, Display, Formatter, Debug};
use crate::typechecking::Ty;

#[derive(PartialEq, Clone)]
pub struct Expr<'a> {
    pub token: Token<'a>,
    pub kind: ExprKind<'a>,
    pub ty: Ty,
    pub node_id: u64,
}

impl<'a> Expr<'a> {
    pub fn new(token: Token<'a>, kind: ExprKind<'a>, ty: Ty, node_id: u64) -> Self {
        Expr { token, kind, ty, node_id }
    }

}

#[derive(Clone, PartialEq)]
pub struct Binder<'a> {
    pub name: Token<'a>,
    pub ty: Ty,
}

impl<'a> Display for Binder<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

#[derive(Clone, PartialEq)]
pub enum ExprKind<'a> {
    Unary { op: TokenKind, expr: Box<Expr<'a>> },
    Integral { value: i64 },
    Bool { b: bool },
    Id { name: Token<'a> },
    Str { string: &'a str },
    Binary { op: TokenKind, left: Box<Expr<'a>>, right: Box<Expr<'a>> },
    Grouping { expr: Box<Expr<'a>> },
    Let { binder: Binder<'a>, expr: Box<Expr<'a>>, body: Box<Expr<'a>> },
    Block { exprs: Vec<Expr<'a>>, suppressed: bool },
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ExprKind::Unary { op, expr }          => write!(f, "{}{}", op, expr),
            ExprKind::Integral { value }          => write!(f, "{}", value),
            ExprKind::Id { name }                 => write!(f, "{}", name),
            ExprKind::Binary { op, left, right }  => write!(f, "{} {} {}", left, op, right),
            ExprKind::Grouping { expr }           => write!(f, "({})", expr),
            ExprKind::Bool { b }                  => write!(f, "{}", b),
            ExprKind::Str { string }              => write!(f, "{}", string),
            ExprKind::Let { binder, expr, body }  => write!(f, "let {} = {} in {}", binder, expr, body),
            ExprKind::Block { exprs, suppressed } => write!(f, "{{ {}{} }}", exprs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("; "), if *suppressed { ";" } else {""})
        }
    }
}

impl<'a> Debug for Expr<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ExprKind::Unary { op, expr }          => write!(f, "({}{:?})", op, expr),
            ExprKind::Integral { value }          => write!(f, "{}", value),
            ExprKind::Id { name }                 => write!(f, "{}", name),
            ExprKind::Binary { op, left, right }  => write!(f, "({} {:?} {:?})", op, left, right),
            ExprKind::Grouping { expr }           => write!(f, "{:?}", expr),
            ExprKind::Bool { b }                  => write!(f, "{}", b),
            ExprKind::Str { string }              => write!(f, "{}", string),
            ExprKind::Let { binder, expr, body }  => write!(f, "(let [{} = {}] ({}))", binder, expr, body),
            ExprKind::Block { exprs, suppressed } => write!(f, "{{ {}{} }}", exprs.iter().map(|e| format!("{:?}", e)).collect::<Vec<_>>().join("; "), if *suppressed { ";" } else {""})
        }
    }

}



















