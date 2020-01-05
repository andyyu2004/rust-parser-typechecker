use std::collections::HashMap;
use super::{Ty, Constraint, Type};
use crate::error::Error;
use crate::map;
use crate::util::Dummy;
use regexlexer::Token;

pub type Substitution = HashMap<u64, Ty>;

pub(crate) fn solve(constraint: Constraint) -> Result<Substitution, Error> {
    match constraint {
        Constraint::Empty => Ok(HashMap::new()),
        Constraint::And(box c, box mut d) => {
            let s = solve(c)?;
            d.apply(&s);
            let t = solve(d)?;
            Ok(compose(s, t))
        }
        Constraint::Eq(t, u) => unify(t, u)
    }
}

fn unify(t: Ty, u: Ty) -> Result<Substitution, Error> {
    match (t, u) {
        (Ty::Infer(i), u) => bind(i, u),
        (t, Ty::Infer(j)) => bind(j, t),
        (Ty::Arrow(box l, box r), Ty::Arrow(box t, box u)) => {
            solve(Constraint::And(
                box Constraint::Eq(l, t),
                box Constraint::Eq(r, u),
            ))
        }
        (Ty::Tuple(xs), Ty::Tuple(ys)) => {
            let cs = xs.into_iter()
                .zip(ys)
                .fold(Constraint::Empty, |acc, (t, u)| Constraint::And(box acc, box Constraint::Eq(t, u)));
            solve(cs)
        },
        (t, u) => Err(Error::new(Token::dummy(), format!("Failed to unify type {} with {}", t, u))),
    }
}

/// Performs occurs check and if it passes, return mapping from inference variable to the type
fn bind(i: u64, t: Ty) -> Result<Substitution, Error> {
    if Ty::Infer(i) != t && t.ftv().contains(&i) { Err(Error::new(Token::dummy(), format!("Occurs check failed: {} occurs in {}", i, t))) }
    else { Ok(map! { i => t }) }
}



/// left -> composition
/// extend is right-biased
fn compose(mut s: Substitution, mut t: Substitution) -> Substitution {
    s.values_mut().map(|ty| ty.apply(&t)).count();
    t.extend(s);
    t
}

#[cfg(test)]
mod test {

    use crate::map;
    use super::*;

    #[test]
    fn test_right_bias_of_extend() {
        let mut a = map! { 0 => 0 };
        let b = map! { 0 => 1 };
        a.extend(b);
        assert_eq!(a, map! { 0 => 1 });
    }

    #[test]
    fn left_bias_of_compose() {
        let a = map! {
            5 => Ty::Bool
        };
        let b = map! {
            5 => Ty::I64
        };
        assert_eq!(map! {
           5 => Ty::Bool
        }, compose(a, b));
    }

    #[test]
    fn combine() {
        let a = map! { 0 => Ty::Bool };
        let b = map! { 1 => Ty::I64 };
        assert_eq!(map! {
            0 => Ty::Bool,
            1 => Ty::I64
        }, compose(a, b));
    }

    #[test]
    fn test_compose() {
        let a = map! {
            1 => Ty::Infer(12),
            5 => Ty::I64
        };
        let b = map! {
            12 => Ty::Bool,
            5  => Ty::F64
        };
        assert_eq!(map! {
            1  => Ty::Bool,
            5  => Ty::I64,
            12 => Ty::Bool
        }, compose(a, b));
    }
}











