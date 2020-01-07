use super::{Ty, Type, Substitution};
use std::fmt::{self, Formatter, Display};
use std::collections::HashSet;

#[derive(Clone)]
pub enum Constraint {
    Empty,
    Eq(Ty, Ty),
    And(Box<Constraint>, Box<Constraint>)
}

use Constraint::*;

impl Type for Constraint {
    fn ftv(&self) -> HashSet<u64> {
        match self {
            Empty     => HashSet::new(),
            Eq(t, u)  => &t.ftv() | &u.ftv(),
            And(c, d) => &c.ftv() | &d.ftv(),
        }
    }

    fn apply(&mut self, s: &Substitution) {
        match self {
            Empty     => {}
            Eq(t, u)  => { t.apply(s); u.apply(s) },
            And(c, d) => { c.apply(s); d.apply(s) },
        }
    }

}

impl Constraint {
    pub fn conj(cs: Vec<Constraint>) -> Constraint {
        cs.into_iter().fold(Self::Empty, |acc, x| Self::And(box acc, box x))
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Empty                     => write!(f, ""),
            Eq(t, u)                  => write!(f, "{} ~ {}", t, u),
            And(box Empty, box Empty) => write!(f, ""),
            And(box c, box Empty)     => write!(f, "{}", c),
            And(box Empty, box c)     => write!(f, "{}", c),
            And(box b, box c)         => write!(f, "{} & {}", b, c),
        }
    }
}
