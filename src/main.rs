/* rlox
** A Lox implementation in Rust
** @author Daniel Shapovalov
*/
pub mod language;
pub mod util;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;
use language::lexer::Lexer;
use crate::language::interpreter::Interpreter;
use crate::language::parser::Parser;
use crate::util::errors::LoxError;

fn run(source: String, interpreter: &mut Interpreter) -> Result<(), LoxError> {
    let tokens = Lexer::new(source).scan_tokens()?;
    let statements = Parser::new(tokens).parse()?;
    interpreter.interpret(statements)?;

    Ok(())
}

fn run_repl(interpreter: &mut Interpreter) {
    println!("rlox REPL - Welcome.");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut source = String::new();

        let res = io::stdin()
            .read_line(&mut source);

        match res {
            Err(e) => { eprintln!("{e}"); break; },
            Ok(0) => break,
            Ok(_) => {}
        }

        if let Err(err) = run(source, interpreter) {
            eprintln!("{err}");
        }
    }
}

fn run_file(path: &str, interpreter: &mut Interpreter) -> ExitCode {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Unable to read \"{path}\": {e}");
            return ExitCode::from(66);
        }
    };
    match run(source, interpreter) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(e.exit_code())
        }
    }
}

fn main() -> ExitCode {
    let mut interpreter = Interpreter::default();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => { run_repl(&mut interpreter); return ExitCode::SUCCESS } ,
        2 => { return run_file(&args[1], &mut interpreter) },
        _ => { eprintln!("Usages:\n\trlox\n\trlox [file]"); ExitCode::SUCCESS }
    }
}
