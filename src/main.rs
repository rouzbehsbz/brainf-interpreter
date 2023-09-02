use std::{io, env, fs, process};

use interpreter::Interpreter;
use token::Token;

mod token;
mod interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: brainf <source>");
        process::exit(1);
    }

    let instructions = match fs::read_to_string(&args[1]) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read the source file. please check file permissions or its existance.");
            process::exit(1);
        }
    };

    let tokens = Token::tokenize(&instructions);
    let mut interpreter = Interpreter::default();
    let jump_table = interpreter.create_jump_table(&tokens);

    println!("{:?}", jump_table);

    interpreter.run(
        &tokens,
        &jump_table,
        io::stdout(),
        io::stdin()
    );
}