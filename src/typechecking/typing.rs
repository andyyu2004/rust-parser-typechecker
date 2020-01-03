use std::fmt::{self, Display, Formatter, Debug};

#[derive(Clone, PartialEq)]
pub enum Ty {
    UVar(String), // Unification type variable
}

impl Display for Ty {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::UVar(t) => write!(f, "{}", t),
        }
    }
}
