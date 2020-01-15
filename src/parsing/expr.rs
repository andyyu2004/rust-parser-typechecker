use regexlexer::TokenKind;
use std::fmt::{self, Display, Formatter, Debug};
use crate::typechecking::Ty;
use crate::parsing::Span;

#[derive(PartialEq, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub ty: Ty,
    pub node_id: u64,
}

impl Expr {
    pub fn new(span: Span, kind: ExprKind, ty: Ty, node_id: u64) -> Self {
        Expr { span, kind, ty, node_id }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "{}", self.kind) }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "{:?}", self.kind) }
}

#[derive(Clone, PartialEq)]
pub struct Binder {
    pub span: Span,
    pub name: String,
    pub ty: Ty,
}

impl Binder {
    pub fn new(span: Span, name: String, ty: Ty) -> Self {
        Self { span, name, ty }
    }
}

impl Display for Binder {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

impl Debug for Binder { fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "{}", self) } }

#[derive(Clone, PartialEq)]
pub enum ExprKind {
    Unary { op: TokenKind, expr: Box<Expr> },
    Integral { value: i64 },
    Bool { b: bool },
    Id { name: String },
    Str { string: String },
    Binary { op: TokenKind, left: Box<Expr>, right: Box<Expr> },
    Grouping { expr: Box<Expr> },
    Let { binder: Binder, bound: Box<Expr> },
    Block { exprs: Vec<Expr>, suppressed: bool },
    Lambda { params: Vec<Binder>, ret: Ty, body: Box<Expr> },
    App { f: Box<Expr>, args: Vec<Expr> },
    Tuple { elems: Vec<Expr> },
}

pub fn fmt_vec<T>(xs: &Vec<T>, sep: &str) -> String where T : Display {
    xs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(sep)
}

pub fn fmt_vec_debug<T>(xs: &Vec<T>, sep: &str) -> String where T : Debug {
    xs.iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>().join(sep)
}

impl Display for ExprKind {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::Unary { op, expr }           => write!(fmt, "{}{}", op, expr),
            Self::Integral { value }           => write!(fmt, "{}", value),
            Self::Id { name }                  => write!(fmt, "{}", name),
            Self::Binary { op, left, right }   => write!(fmt, "{} {} {}", left, op, right),
            Self::Grouping { expr }            => write!(fmt, "({})", expr),
            Self::Bool { b }                   => write!(fmt, "{}", b),
            Self::Str { string }               => write!(fmt, "{}", string),
            Self::Let { binder, bound }        => write!(fmt, "let {} = {}", binder, bound),
            Self::Block { exprs, suppressed }  => write!(fmt, "{{ {}{} }}", fmt_vec(exprs, "; "), if *suppressed { ";" } else {""}),
            Self::Lambda { params, ret, body } => write!(fmt, "fn ({}) -> {} => {}", fmt_vec(params, ", "), ret, body),
            Self::App { f, args }              => write!(fmt, "{}({})", f, fmt_vec(args, ", ")),
            Self::Tuple { elems }              => write!(fmt, "({})", fmt_vec(elems, ", ")),
        }
    }
}

impl Debug for ExprKind {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::Unary { op, expr }           => write!(fmt, "({}{:?})", op, expr),
            Self::Integral { value }           => write!(fmt, "{}", value),
            Self::Id { name }                  => write!(fmt, "{}", name),
            Self::Binary { op, left, right }   => write!(fmt, "({} {:?} {:?})", op, left, right),
            Self::Grouping { expr }            => write!(fmt, "{:?}", expr),
            Self::Bool { b }                   => write!(fmt, "{}", b),
            Self::Str { string }               => write!(fmt, "{}", string),
            Self::Let { binder, bound }        => write!(fmt, "(let [{} = {:?}])", binder, bound),
            Self::Block { exprs, suppressed }  => write!(fmt, "{{ {}{} }}", fmt_vec_debug(exprs, "; "), if *suppressed { ";" } else {""}),
            Self::Lambda { params, ret, body } => write!(fmt, "(lambda ({}) -> {} => {:?})", fmt_vec_debug(params, ", "), ret, body),
            Self::App { f, args }              => write!(fmt, "({} {})", f, fmt_vec_debug(args, " ")),
            Self::Tuple { elems }              => write!(fmt, "({})", fmt_vec_debug(elems, ", ")),
        }
    }

}



















