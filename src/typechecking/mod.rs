mod typing;
mod typechecker;
mod env;
mod constraint;
mod substitution;
mod tyscheme;

pub use typing::{Ty, TyKind};
pub use typechecker::Typechecker;
pub(crate) use tyscheme::TyScheme;
pub(crate) use env::Env;
pub(crate) use constraint::Constraint;
pub(crate) use substitution::{Substitution, solve};


use std::collections::HashSet;

pub trait Type {
    fn apply(&mut self, s: &Substitution);
    fn ftv(&self) -> HashSet<u64>;
}


