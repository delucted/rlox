/* rlox
** A Lox implementation in Rust
** @author Daniel Shapovalov
*/
pub mod language;
pub mod util;

use std::env;
use std::fs;
use std::io::{self, Write};
use language::lexer::Lexer;

fn run(source: String) -> Result<&'static str, &'static str> {
    let mut lexer = Lexer::new(source);
    lexer.scan_tokens();

    for token in lexer.iter() {
        println!("{token:?}")
    }

    Ok("rlox successfully executed source")
}

fn run_repl() {
    loop {
        println!("rlox REPL - Welcome.");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut source = String::new();

        let res = io::stdin()
            .read_line(&mut source);

        match res {
            Err(e) => eprintln!("{}", e),
            _ => {}
        }

        run(source).expect("");
    }
}

fn run_file(path: &str) -> Result<&str, &str> {
    // TODO: convert into buffer to save memory
    run(match fs::read_to_string(path) {
        Ok(source) => source,
        Err(e) => panic!("Unable to read file \"{path}\": {e}")
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_repl(),
        2 => { run_file(&args[1]).expect("File execution failed."); },
        _ => eprintln!("Usages:\n\trlox\n\trlox [file]")
    }
}
