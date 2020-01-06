#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate colour;

mod parsing;
mod lexing;
mod error;
mod util;
mod macros;
mod typechecking;


use regexlexer::{Lexer, LexSyntax};
use lexing::gen_syntax;
use typechecking::Typechecker;

pub use error::{Error, Formatter};
pub use parsing::{Parser, Expr, ExprKind};
pub use regexlexer::{Token, TokenKind};
pub use typechecking::{Ty, TyKind};

/// Generate ast using the default syntax provided from this crate
pub fn generate_ast<'a>(src: &'a str) -> Result<(Ty, Expr), Vec<Error>> {
    generate_ast_with_syntax(src, &gen_syntax())
}

pub fn generate_ast_with_syntax<'a, 'b>(src: &'a str, syntax: &'b LexSyntax) -> Result<(Ty, Expr), Vec<Error>> {
    let lexer = Lexer::new(src, &syntax);
    let tokens = match lexer.lex() {
        Ok(ts) => ts,
        Err(errors) => {
            eprintln!("{}", errors.join("\n"));
            std::process::exit(1);
        }
    };
    let mut parser = Parser::new(&tokens);
    let mut typechecker = Typechecker::new();
    let expr = parser.parse()?;
    let ty = typechecker.typecheck(&expr)?;

    Ok((ty, expr))
}

pub fn generate_ast_with_err_handling(src: &str) -> (Ty, Expr) {
    let syntax = gen_syntax();
    let lexer = Lexer::new(src, &syntax);
    let tokens = match lexer.lex() {
        Ok(ts) => ts,
        Err(errors) => {
            eprintln!("{}", errors.join("\n"));
            std::process::exit(1);
        }
    };

    println!("{:?}", tokens);
    let mut parser = Parser::new(&tokens);
    let mut typechecker = Typechecker::new();
    let formatter = Formatter::new(src);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(errors) => {
            formatter.write(errors);
            std::process::exit(1);
        }
    };

    let ty = match typechecker.typecheck(&ast) {
        Ok(ast) => ast,
        Err(errors) => {
            formatter.write(errors);
            std::process::exit(1);
        }
    };

    (ty, ast)
}















