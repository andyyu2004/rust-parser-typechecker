use crate::parsing::Span;

#[derive(Clone, PartialEq, Debug)]
pub struct Error {
    pub span: Span,
    pub msg: String,
}

impl Error {
    pub fn new(span: Span, msg: String) -> Self {
        Error { span, msg }
    }
}
