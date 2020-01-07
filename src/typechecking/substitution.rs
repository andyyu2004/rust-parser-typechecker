use std::collections::HashMap;
use super::{TyKind, Constraint, Type, Ty};
use crate::error::Error;
use crate::map;

pub type Substitution = HashMap<u64, Ty>;

pub(crate) fn solve(constraint: Constraint) -> Result<Substitution, Error> {
    match constraint {
        Constraint::Empty => Ok(HashMap::new()),
        Constraint::And(box c, box mut d) => {
            let s = solve(c)?;
            d.apply(&s); // Apply substitution to constraint before continuing to avoid inconsistencies
            let t = solve(d)?;
            Ok(compose(s, t))
        }
        Constraint::Eq(t, u) => unify(t, u)
    }
}

fn unify(t: Ty, u: Ty) -> Result<Substitution, Error> {
    match (t.kind, u.kind) {
        (TyKind::Infer(i), y) => bind(i, Ty::new(t.span, y)),
        (x, TyKind::Infer(j)) => bind(j, Ty::new(u.span, x)),
        (TyKind::Arrow(box l, box r), TyKind::Arrow(box t, box u)) => {
            solve(Constraint::And(
                box Constraint::Eq(l, t),
                box Constraint::Eq(r, u),
            ))
        }
        (TyKind::Tuple(xs), TyKind::Tuple(ys)) => {
            let cs = xs.into_iter()
                .zip(ys)
                .fold(Constraint::Empty, |acc, (t, u)| Constraint::And(box acc, box Constraint::Eq(t, u)));
            solve(cs)
        },
        (t, u) if t == u => Ok(HashMap::new()),
        (x, y) => Err(Error::new(t.span.merge(u.span), format!("Failed to unify type {} with {}", x, y))),
    }
}

/// Performs occurs check and if it passes, return mapping from inference variable to the type
fn bind(i: u64, t: Ty) -> Result<Substitution, Error> {
    if TyKind::Infer(i) != t.kind && t.ftv().contains(&i) { Err(Error::new(t.span, format!("Occurs check failed: {} occurs in {}", i, t))) }
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
    use crate::parsing::Span;

    fn to_ty(kind: TyKind) -> Ty {
        Ty::new(Span::single(0, 0), kind)
    }

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
            5 => to_ty(TyKind::Bool)
        };
        let b = map! {
            5 => to_ty(TyKind::I64)
        };
        assert_eq!(map! {
           5 => to_ty(TyKind::Bool)
        }, compose(a, b));
    }

    #[test]
    fn combine() {
        let a = map! { 0 => to_ty(TyKind::Bool) };
        let b = map! { 1 => to_ty(TyKind::I64) };
        assert_eq!(map! {
            0 => to_ty(TyKind::Bool),
            1 => to_ty(TyKind::I64)
        }, compose(a, b));
    }

    #[test]
    fn test_compose() {
        let a = map! {
            1 => to_ty(TyKind::Infer(12)),
            5 => to_ty(TyKind::I64)
        };
        let b = map! {
            12 => to_ty(TyKind::Bool),
            5  => to_ty(TyKind::F64)
        };
        assert_eq!(map! {
            1  => to_ty(TyKind::Bool),
            5  => to_ty(TyKind::I64),
            12 => to_ty(TyKind::Bool)
        }, compose(a, b));
    }
}











