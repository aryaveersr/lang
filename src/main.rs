use lang::{hir_passes::TypeResolver, lexer::Lexer, parser::Parser};
use std::{
    fs,
    io::{self, Write, stdin, stdout},
};

fn compile(source: &str) {
    println!("== Tokens ==");

    for token in Lexer::new(source) {
        println!("{token:?}");
    }

    let mut hir = match Parser::new(Lexer::new(source)).parse() {
        Ok(hir) => hir,
        Err(err) => return println!("Parse Error:\n{err}"),
    };

    println!("\n== HIR ==");
    println!("{}", serde_yaml::to_string(&hir).unwrap());

    TypeResolver::new().resolve(&mut hir);
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
