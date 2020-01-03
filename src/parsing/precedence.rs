use std::ops::{Sub};
use regexlexer::Token;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Precedence {
    ZERO    = 0,
    ASSIGN  = 1,
    OR      = 2,
    AND     = 3,
    EQ      = 4,
    CMP     = 5,
    BITOR   = 6,
    BITXOR  = 7,
    BITAND  = 8,
    SHIFT   = 9,
    TERM    = 10,
    FACTOR  = 11,
    EXPO    = 12,
    CAST    = 13,
    UNARY   = 14,
    CALL    = 15,
    PRIMARY = 16,
}

use regexlexer::TokenKind::*;
impl Precedence {
    /// Precedence of left denotation parselets
    pub fn of_left(token: Token) -> Self {
        match token.kind {
            Plus | Minus        => Precedence::TERM,
            Star | Slash        => Precedence::FACTOR,
            DStar               => Precedence::EXPO,
            DEqual | BangEqual  => Precedence::EQ,
            GT | GTE | LT | LTE => Precedence::CMP,
            EOF                 => Precedence::ZERO,
            _                   => Precedence::ZERO,
        }
    }
}


impl From<i32> for Precedence {
    fn from(item: i32) -> Self {
        match item {
            0  => Self::ZERO,
            1  => Self::ASSIGN,
            2  => Self::OR,
            3  => Self::AND,
            4  => Self::EQ,
            5  => Self::CMP,
            6  => Self::BITOR,
            7  => Self::BITXOR,
            8  => Self::BITAND,
            9  => Self::SHIFT,
            10 => Self::TERM,
            11 => Self::FACTOR,
            12 => Self::EXPO,
            13 => Self::CAST,
            14 => Self::UNARY,
            15 => Self::CALL,
            16 => Self::PRIMARY,
            _ => panic!("Invalid integer for precedence"),
        }
    }
}

impl<T> Sub<T> for Precedence where T : Into<i32> {
    type Output = Precedence;

    fn sub(self, r: T) -> Self::Output {
        (self as i32 - r.into()).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sub() {
        let primary = Precedence::PRIMARY;
        assert_eq!(primary - 15, Precedence::ZERO);
    }

    #[test]
    fn test_cast() {
        assert_eq!(Into::<Precedence>::into(6), Precedence::BITOR)
    }

    #[test]
    fn test_cmp() {
        assert!(Precedence::EQ < Precedence::CMP)
    }
}







