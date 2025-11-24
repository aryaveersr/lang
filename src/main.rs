use lang::lexer::Lexer;
use std::{
    fs,
    io::{self, Write, stdin, stdout},
};

fn compile(source: &str) {
    for token in Lexer::new(source) {
        println!("{token:?}");
    }
}

fn repl() -> io::Result<()> {
    loop {
        let mut line = String::new();

        print!("> ");
        stdout().flush()?;
        stdin().read_line(&mut line)?;

        if line.trim() == "exit" {
            return Ok(());
        }

        compile(&line);
    }
}

fn main() -> io::Result<()> {
    if let Some(path) = std::env::args().nth(1) {
        let source = fs::read_to_string(&path)?;
        compile(&source);
    } else {
        repl()?;
    }

    Ok(())
}
