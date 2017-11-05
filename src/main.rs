mod parser;
mod interpreter;

use std::env;
use std::process;
use std::fs::File;
use std::io::Read;

use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let mut args = env::args();
    args.next();

    let mut brainfuck = String::new();
    if let Some(file_name) = args.next() {
        let mut input = match File::open(file_name) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("Error while reading file: {}.", e);
                process::exit(1);
            }
        };

        match input.read_to_string(&mut brainfuck) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error while reading file to String: {}.", e);
                process::exit(1);
            }
        };
    } else {
        eprintln!("Error: no file provied. Usage: `brainfuck [file]'.");
        process::exit(1);
    }

    let mut parser = Parser::new(brainfuck);
    parser.parse();

    match Interpreter::run(parser) {
        Ok(output) => print!("{}", output),
        Err(e) => {
            eprintln!("Error while running program: {}", e);
            process::exit(1);
        },
    };
}
