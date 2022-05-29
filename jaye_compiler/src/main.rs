use std::env;
use std::fs;

mod tokens;
mod lexer;
mod parser;
mod statements;

fn usage() {
    println!("usage: jaye [help, run] [file]");
    println!("Run \"jaye help\" for more");
    std::process::exit(0x0000);
}

fn help() {
    println!("Jaye Help");
    println!("==================");
    println!("run  - If you supply a file it run that file.");
    println!("help - Explains the commands.");

    std::process::exit(0x0000);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
    }

    match &args[1][..] {
        "help" => help(),
        "run" => {
            let contents = fs::read_to_string(&args[2])
                .expect("Something went wrong reading the file");

            let mut lexer = lexer::Lexer::new(contents);
            parser::Parser::parse(lexer.lex());
        },

        _ => usage(),
    }
}
