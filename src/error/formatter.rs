use crate::error::Error;
use crate::parsing::Span;

pub struct Formatter<'a> {
    src: &'a str,
    lines: Vec<&'a str>,
}

impl<'a> Formatter<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, lines: src.lines().collect() }
    }

    pub fn write(&self, errors: Vec<Error>) {
        for error in errors {  self.write_err(error) }
    }

    fn slice(&self, span: &Span) -> &str {
        &self.src[span.lo..span.hi]
    }

    /// Returns a triple of the line and the two surrounding it
    /// Remember: the line count starts from 1 while index from 0
    fn surrounding(&self, line: usize) -> (&str, &str, &str) {
        let prev = if line > 1 { self.lines[line - 2] } else { "" };
        let curr = self.lines[line - 1];
        let next = if line < self.lines.len() { self.lines[line] } else { "" };
        (prev, curr, next)
    }

    pub fn write_err(&self, error: Error) {
        let Error { span, msg } = &error;
        self.print_relevant_src(&error);
        // let slice = self.slice(span);
        red!("{}", msg)
    }

    fn print_relevant_src(&self, error: &Error) {
        let line = error.span.line;
        let (prev, curr, next) = self.surrounding(line);
        green_ln!("Relevant source:");
        if !prev.is_empty() {
            blue!("{}: ", line - 1);
            println!("{}", prev);
        }
        blue!("{}: ", line);
        println!("{}", curr);
        if !next.is_empty() {
            blue!("{}: ", line + 1);
            println!("{}", next);
        }

    }
}
