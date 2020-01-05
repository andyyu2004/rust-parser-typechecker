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
    Let { binder: Binder<'a>, bound: Box<Expr<'a>>, body: Box<Expr<'a>> },
    Block { exprs: Vec<Expr<'a>>, suppressed: bool },
    Lambda { params: Vec<Binder<'a>>, ret: Ty, body: Box<Expr<'a>> },
    App { f: Box<Expr<'a>>, args: Vec<Expr<'a>> },
}

pub fn fmt_vec<T>(xs: &Vec<T>, sep: &str) -> String where T : Display {
    xs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(sep)
}

pub fn fmt_vec_debug<T>(xs: &Vec<T>, sep: &str) -> String where T : Debug {
    xs.iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>().join(sep)
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ExprKind::Unary { op, expr }           => write!(fmt, "{}{}", op, expr),
            ExprKind::Integral { value }           => write!(fmt, "{}", value),
            ExprKind::Id { name }                  => write!(fmt, "{}", name),
            ExprKind::Binary { op, left, right }   => write!(fmt, "{} {} {}", left, op, right),
            ExprKind::Grouping { expr }            => write!(fmt, "({})", expr),
            ExprKind::Bool { b }                   => write!(fmt, "{}", b),
            ExprKind::Str { string }               => write!(fmt, "{}", string),
            ExprKind::Let { binder, bound, body }  => write!(fmt, "let {} = {} in {}", binder, bound, body),
            ExprKind::Block { exprs, suppressed }  => write!(fmt, "{{ {}{} }}", fmt_vec(exprs, ";"), if *suppressed { ";" } else {""}),
            ExprKind::Lambda { params, ret, body } => write!(fmt, "fn ({}) -> {} => {}", fmt_vec(params, ", "), ret, body),
            ExprKind::App { f, args }              => write!(fmt, "{}({})", f, fmt_vec(args, ", ")),
        }
    }
}

impl<'a> Debug for Expr<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ExprKind::Unary { op, expr }           => write!(fmt, "({}{:?})", op, expr),
            ExprKind::Integral { value }           => write!(fmt, "{}", value),
            ExprKind::Id { name }                  => write!(fmt, "{}", name),
            ExprKind::Binary { op, left, right }   => write!(fmt, "({} {:?} {:?})", op, left, right),
            ExprKind::Grouping { expr }            => write!(fmt, "{:?}", expr),
            ExprKind::Bool { b }                   => write!(fmt, "{}", b),
            ExprKind::Str { string }               => write!(fmt, "{}", string),
            ExprKind::Let { binder, bound, body }  => write!(fmt, "(let [{} = {:?}] ({:?}))", binder, bound, body),
            ExprKind::Block { exprs, suppressed }  => write!(fmt, "{{ {}{} }}", fmt_vec_debug(exprs, "; "), if *suppressed { ";" } else {""}),
            ExprKind::Lambda { params, ret, body } => write!(fmt, "(lambda ({}) -> {} => {:?})", fmt_vec(params, ", "), ret, body),
            ExprKind::App { f, args }              => write!(fmt, "({} {})", f, fmt_vec_debug(args, " ")),
        }
    }

}



















