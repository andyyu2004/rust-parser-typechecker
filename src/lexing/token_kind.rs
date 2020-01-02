#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum TokenKind {
    Plus, Minus, Star, Slash,
    Number,
    Eof,
    Let, Var,
    Bind,
    Less, LessEqual, Greater, GreaterEqual,
    Equal, BangEqual, Bang, DoubleEqual,
    DPlus, TPlus,
    Identifier
}
