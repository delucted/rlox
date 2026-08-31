/* rlox
** A Lox implementation in Rust
** @author Daniel Shapovalov
*/
pub mod language;
pub mod util;

use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use language::lexer::Lexer;
use crate::language::interpreter::Interpreter;
use crate::language::parser::Parser;

fn run(source: String, interpreter: &Interpreter) -> Result<&'static str, &'static str> {
    let mut lexer = Lexer::new(source);
    lexer.scan_tokens();
    
    if lexer.had_error {
        return Err("rlox lexer failed")
    }

    let mut parser = Parser::new(lexer.tokens);

    let expression = parser.parse()?;

    let interpreted = interpreter.interpret(expression)?;

    println!("{interpreted:?}");

    Ok("rlox successfully executed source")
}

fn run_repl(interpreter: &Interpreter) -> Result<(), String> {
    println!("rlox REPL - Welcome.");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut source = String::new();

        let res = io::stdin()
            .read_line(&mut source);

        match res {
            Err(e) => return Err(e.to_string()),
            _ => {}
        }

        match run(source, interpreter) {
            _ => {  }
        }
    }
}

fn run_file(path: &str, interpreter: &Interpreter) -> Result<&'static str, &'static str> {
    // TODO: convert into buffer to save memory
    run(match fs::read_to_string(path) {
        Ok(source) => source,
        Err(e) => panic!("Unable to read file \"{path}\": {e}")
    }, interpreter)
}

fn main() -> Result<(), String> {
    let interpreter = Interpreter {};
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_repl(&interpreter)?,
        2 => { run_file(&args[1], &interpreter)?; },
        _ => eprintln!("Usages:\n\trlox\n\trlox [file]")
    };

    Ok(())
}
