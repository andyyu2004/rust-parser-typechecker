#![feature(box_syntax, box_patterns)]

mod parsing;
mod lexing;
mod error;
mod util;
mod macros;
mod typechecking;

use error::Error;
use util::Dummy;
use regexlexer::{Lexer, LexSyntax};
use lexing::gen_syntax;
use typechecking::Typechecker;

pub use parsing::{Parser, Expr, ExprKind};
pub use regexlexer::{Token, TokenKind};
pub use typechecking::Ty;

/// Generate ast using the default syntax provided from this crate
pub fn generate_ast<'a>(src: &'a str) -> Result<(Ty, Expr<'a>), Vec<Error>> {
    generate_ast_with_syntax(src, &gen_syntax())
}

pub fn generate_ast_with_syntax<'a, 'b>(src: &'a str, syntax: &'b LexSyntax) -> Result<(Ty, Expr<'a>), Vec<Error>> {
    let lexer = Lexer::new(src, &syntax);
    let tokens = lexer.lex()
        .map_err(|errors | errors.into_iter().map(|err| Error::new(Token::dummy(), err)).collect::<Vec<Error>>())?;
    // println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);
    let mut typechecker = Typechecker::new();
    let expr = parser.parse()?;
    let ty = typechecker.typecheck(&expr)?;

    Ok((ty, expr))
}
