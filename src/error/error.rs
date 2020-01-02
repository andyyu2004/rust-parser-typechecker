use regexlexer::Token;

#[derive(Clone, PartialEq, Debug)]
pub struct Error {
    line: usize,
    col: usize,
    msg: String,
}

impl Error {
    pub fn new(token: Token, msg: String) -> Self {
        let Token { line, col, .. } = token;
        Error {
            msg,
            line,
            col,
        }
    }
}
