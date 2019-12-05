use rustyline::{Editor};
use rustyline::error::{ReadlineError};
use lexer::lexing::{Lexer, LexSyntax};
use std::env;

#[derive(Copy, Clone, Debug, PartialEq)]
enum TokenType {
    Plus, Minus, Star, Slash,
    Number,
    Eof,
    Let,
    Bind,
    Less, LessEqual, Greater, GreaterEqual,
    Equal, Bang,
    DoubleEqual, BangEqual,
    Inc, Dec,
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
                m
        }
    };
);

fn generate_syntax() -> LexSyntax<'static, TokenType> {
    let symbols = map! {
        "+"   => TokenType::Plus,
        // "--"  => TokenType::Dec,
        "++"  => TokenType::Inc,
        "-"   => TokenType::Minus,
        "*"   => TokenType::Star,
        "/"   => TokenType::Slash,
        ">>=" => TokenType::Bind,
        "<"   => TokenType::Less,
        "<="  => TokenType::LessEqual,
        ">="  => TokenType::GreaterEqual,
        ">"   => TokenType::Greater,
        "="   => TokenType::Equal,
        "=="  => TokenType::DoubleEqual,
        "!="  => TokenType::BangEqual,
        "!"   => TokenType::Bang
    };

    let token_classes = map! {
        "<integer>" => TokenType::Number,
        "<float>" => TokenType::Number,
        "<eof>" => TokenType::Eof
    };

    let keywords = map! { "let" => TokenType::Let };

    LexSyntax::new(symbols, keywords, token_classes)
}

fn main() {
    let mut rl = Editor::<()>::new();
    let lexical_syntax = generate_syntax();

    if env::args().len() > 2 {
        println!("[usage] <file>");
        std::process::exit(1)
    }

    if env::args().len() == 2 {
        let contents = std::fs::read_to_string(&env::args().collect::<Vec<String>>()[1]).expect("Failed to read string");
        let mut lexer = Lexer::new(&contents, &lexical_syntax);
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

        let mut lexer = Lexer::new(&line, &lexical_syntax);

        match lexer.lex() {
            Ok(tokens) => println!("{:?}", tokens),
            Err(err) => println!("{:?}", err),
        }

        rl.save_history("history.txt").unwrap();
    }
}
