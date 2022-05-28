use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod tokens;
mod lexer;

fn usage() {
    println!("usage: Jaye [help, run] [file]");
    println!("Run \"Jaye help\" for more");
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
            let file = File::open(&args[2]).unwrap();
            let reader = BufReader::new(file);

            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap(); 
                let mut lexer = lexer::Lexer::new(line);
                lexer.lex();
            }
        },

        _ => usage(),
    }
}
