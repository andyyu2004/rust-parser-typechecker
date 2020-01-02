#![feature(box_syntax)]

mod parsing;
mod lexing;
mod error;
mod util;
mod macros;
mod typechecking;

use rustyline::{Editor};
use rustyline::error::{ReadlineError};
use std::env;
use lexing::gen_syntax;
use regexlexer::Lexer;
use parsing::Parser;

fn main() {
    let mut rl = Editor::<()>::new();
    let lexical_syntax = gen_syntax();

    if env::args().len() > 2 {
        println!("[usage] <file>");
        std::process::exit(1)
    }

    if env::args().len() == 2 {
        let contents = std::fs::read_to_string(&env::args().collect::<Vec<String>>()[1]).expect("Failed to read file");
        let lexer = Lexer::new(&contents, &lexical_syntax);
        println!("{:?}", lexer.lex());
        std::process::exit(1);
    }

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("λ ");
        let line = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                line
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        };

        let lexer = Lexer::new(&line, &lexical_syntax);
        let tokens = match lexer.lex() {
            Ok(tokens) => tokens,
            Err(err)   => {
                eprintln!("{}", err.join("\n"));
                continue;
            }
        };

        println!("tokens: {:?}", tokens);

        let mut parser = Parser::new(&tokens);
        let res = parser.parse().map(|expr| {
            println!("{:?}", expr);
            println!("{}", expr);
            expr
        });

        if let Err(err) = res { eprintln!("{:?}", err) }


        rl.save_history("history.txt").unwrap();


    }
}


