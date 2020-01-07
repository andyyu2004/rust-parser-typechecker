#![feature(box_syntax)]
use rustyline::{Editor};
use rustyline::error::{ReadlineError};
use std::env;
use parserlib::{generate_ast_with_err_handling, generate_ast};
use parserlib::Formatter;

fn main() {
    let mut rl = Editor::<()>::new();

    if env::args().len() > 2 {
        println!("[usage] <file>");
        std::process::exit(1)
    }

    if env::args().len() == 2 {
        let contents = std::fs::read_to_string(&env::args().collect::<Vec<String>>()[1]).expect("Failed to read file");
        let (ty, ast) = generate_ast_with_err_handling(&contents);
        println!("{:?}", ast);
        println!("{}", ast);
        println!("{}", ty);
        std::process::exit(0);
    }

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("λ ");
        let line = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                rl.save_history("history.txt").unwrap();
                if line.is_empty() { continue }
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

        match generate_ast(&line) {
            Ok((ty, ast)) => {
                println!("{:?}", ast);
                println!("{}", ast);
                println!("{}", ty);
            }
            Err(errors) => {
                let formatter = Formatter::new(&line);
                formatter.write(errors);
            }
        };


    }
}


