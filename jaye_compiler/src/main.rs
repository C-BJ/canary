use std::env;
use std::fs;

mod tokens;
mod lexer;

fn usage(s: i32) {
    println!("usage: jayec [usage, run] [file]");
    println!("Run \"jayec help\" for more");
    std::process::exit(s);
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage(1);
    }

    match &args[1][..] {
        "usage" => usage(0),

        "run" => {
            let contents = fs::read_to_string(&args[2])
                .expect("Unable to open file for reading!");

            let mut lexer = lexer::Lexer::new(contents);

            println!("{:?}", lexer.lex());
        },

        _ => usage(1),
    }
}
