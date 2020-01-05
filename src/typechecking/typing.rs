use std::fmt::{self, Display, Formatter, Debug};
use super::{Type, Substitution};
use std::collections::HashSet;
use crate::set;

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Ty {
    Bool,
    I64,
    F64,
    Infer(u64), // Unification type variable
    Tuple(Vec<Ty>),
    Arrow(Box<Ty>, Box<Ty>),
}

impl Type for Ty {
    fn apply(&mut self, s: &Substitution) {
        match self {
            Self::Infer(i) => if let Some(t) = s.get(i) { *self = t.clone() }
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


impl Display for Ty {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Infer(i)      => write!(f, "{}", i),
            Self::I64          => write!(f, "i64"),
            Self::F64          => write!(f, "f64"),
            Self::Bool         => write!(f, "bool"),
            Self::Tuple(xs)    => write!(f, "({})", xs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
            Self::Arrow(box l, r)  => match l {
                Self::Arrow(..) => write!(f, "({}) -> {}", l, r),
                _               => write!(f, "{} -> {}", l, r),
            }
        }
    }
}


