use std::cmp;

/// Contains offsets into the source file (maybe a sourcemap in the future) for error reporting purposes
#[derive(Clone, PartialEq, Debug, Eq, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
    pub line: usize,
}

impl Span {
    pub fn new(lo: usize, hi: usize, line: usize) -> Self {
        Self { lo, hi, line }
    }

    /// Span of width 1
    pub fn single(lo: usize, line: usize) -> Self {
        Span::new(lo, 1 + lo, line)
    }

    pub fn merge(self, other: Span) -> Span {
        let lo = cmp::min(self.lo, other.lo);
        let hi = cmp::max(self.hi, other.hi);
        let line = cmp::min(self.line, other.line);
        Span { lo, hi, line }
    }
}






