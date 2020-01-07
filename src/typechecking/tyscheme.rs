use super::{Ty, TyKind, Substitution, Type};
use std::collections::HashSet;
use crate::util::Counter;
use std::fmt::{self, Formatter, Display};

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct TyScheme {
    ty: Ty,
    forall: HashSet<u64>,
}

impl TyScheme {
    pub fn new(ty: Ty, forall: HashSet<u64>) -> Self {
        Self { ty, forall }
    }

    pub fn instantiate(&self, name_gen: &mut Counter) -> Ty {
        let substitution: Substitution = self.forall.iter()
            .map(|t| (*t, Ty::new(self.ty.span, TyKind::Infer(name_gen.next())))).collect();
        let mut ty = self.ty.clone();
        ty.apply(&substitution);
        ty
    }
}

impl Display for TyScheme {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "∀{}.{}", self.forall.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","), self.ty)
    }
}

impl Type for TyScheme {
    fn ftv(&self) -> HashSet<u64> { self.ty.ftv() }
    fn apply(&mut self, s: &Substitution) { self.ty.apply(s) }
}

/// Creates new typescheme with no bound variables
impl From<Ty> for TyScheme {
    fn from(ty: Ty) -> Self {
        Self { ty, forall: HashSet::new() }
    }
}

impl From<&Ty> for TyScheme {
    fn from(ty: &Ty) -> Self {
        Self { ty: ty.clone(), forall: HashSet::new() }
    }
}
