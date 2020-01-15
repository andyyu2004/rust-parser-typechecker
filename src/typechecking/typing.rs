use std::fmt::{self, Display, Formatter, Debug};
use super::{Type, Substitution, TyScheme, Env};
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

    pub(crate) fn generalize(self, env: &Env<&str, TyScheme>) -> TyScheme {
        let forall = &self.ftv() - &env.ftv();
        TyScheme::new(self, forall)
    }

    pub(crate) fn erased() -> Self {
        Self { span: Span::single(0, 0), kind: TyKind::Erased }
    }

    /// wraps Ty into singleton tuple
    pub(crate) fn singleton(self) -> Self {
        let span = self.span;
        Self::new(span, TyKind::Tuple(vec![self]))
    }

    pub(crate) fn take(&mut self) -> Self {
        std::mem::replace(self, Ty::erased())
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
    Erased,
    Bool,
    I64,
    F64,
    Infer(u64), // Unification type variable
    TyVar(String),
    Tuple(Vec<Ty>),
    Arrow(Box<Ty>, Box<Ty>),
}

impl TyKind {
    pub fn unit() -> Self { Self::Tuple(Vec::new()) }
}

impl Type for TyKind {
    fn apply(&mut self, s: &Substitution) {
        match self {
            Self::Infer(i) => if let Some(t) = s.get(i) { *self = t.kind.clone() }
            Self::Tuple(xs) => xs.iter_mut().for_each(|t| t.apply(s)),
            Self::Arrow(box l, box r) => { l.apply(s); r.apply(s); }
            Self::Bool | Self::F64 | Self::I64 | Self::Erased => {},
            Self::TyVar(_n) => unimplemented!(),
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
            Self::Arrow(l, r) => &l.ftv() | &r.ftv(),
            Self::Bool | Self::F64 | Self::I64 | Self::Erased => HashSet::new(),
            Self::TyVar(_n) => unimplemented!(),
        }
    }
}

impl Display for TyKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Infer(i)     => write!(f, "τ{}", i),
            Self::I64          => write!(f, "i64"),
            Self::F64          => write!(f, "f64"),
            Self::Bool         => write!(f, "bool"),
            Self::Tuple(xs)    => write!(f, "({})", xs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
            Self::TyVar(name)     => write!(f, "{}", name),
            Self::Erased       => write!(f, "τ"),
            Self::Arrow(box l, r)  => match l.kind {
                Self::Arrow(..) => write!(f, "({}) -> {}", l, r),
                _               => write!(f, "{} -> {}", l, r),
            },
        }
    }
}


/// Convenience method for testing
#[cfg(test)]
impl TyKind { pub fn to_ty(self) -> Ty { Ty::new(Span::single(0, 0), self) } }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ftv() {
        let a = TyKind::Infer(0).to_ty();
        let b = TyKind::Tuple(vec![TyKind::F64.to_ty(), TyKind::Infer(2).to_ty()]).to_ty();
        let f = TyKind::Arrow(box a, box b);
        assert_eq!(f.ftv(), set! { 0, 2 })
    }
}






