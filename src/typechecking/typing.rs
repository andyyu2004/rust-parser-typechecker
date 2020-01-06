use std::fmt::{self, Display, Formatter, Debug};
use super::{Type, Substitution};
use std::collections::HashSet;
use crate::set;
use crate::parsing::Span;

#[derive(Clone, Debug, Eq)]
pub struct Ty {
    pub span: Span,
    pub kind: TyKind,
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool { self.kind == other.kind }
}

impl Ty {
    pub fn new(span: Span, kind: TyKind) -> Self {
        Self { span, kind }
    }
}

impl Type for Ty {
    fn apply(&mut self, s: &Substitution) { self.kind.apply(s) }
    fn ftv(&self) -> HashSet<u64> { self.kind.ftv() }
}

impl Display for Ty {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "{}", self.kind) }
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum TyKind {
    Bool,
    I64,
    F64,
    Infer(u64), // Unification type variable
    Tuple(Vec<Ty>),
    Arrow(Box<Ty>, Box<Ty>),
}

impl Type for TyKind {
    fn apply(&mut self, s: &Substitution) {
        match self {
            Self::Infer(i) => if let Some(t) = s.get(i) { *self = t.kind.clone() }
            Self::Tuple(xs) => xs.iter_mut().for_each(|t| t.apply(s)),
            Self::Arrow(box l, box r) => { l.apply(s); r.apply(s); }
            Self::Bool | Self::F64 | Self::I64 => {},
        }
    }

    fn ftv(&self) -> HashSet<u64> {
        match self {
            Self::Infer(i) => set! { *i },
            // Doesn't seem to have a good way to union without copying the set in some way
            // But u64 copy cost isn't too bad anyhow
            Self::Tuple(xs) => xs.iter()
                .map(|x| x.ftv())
                .fold(HashSet::new(), |acc, x| &acc | &x),
            Self::Arrow(l, r) => &l.ftv() & &r.ftv(),
            Self::Bool | Self::F64 | Self::I64 => HashSet::new(),
        }
    }
}


impl Display for TyKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Infer(i)     => write!(f, "{}", i),
            Self::I64          => write!(f, "i64"),
            Self::F64          => write!(f, "f64"),
            Self::Bool         => write!(f, "bool"),
            Self::Tuple(xs)    => write!(f, "({})", xs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
            Self::Arrow(box l, r)  => match l.kind {
                Self::Arrow(..) => write!(f, "({}) -> {}", l, r),
                _               => write!(f, "{} -> {}", l, r),
            }
        }
    }
}


