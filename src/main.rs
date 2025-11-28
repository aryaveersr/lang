use lang::{builder::Builder, lexer::Lexer, parser::Parser};
use std::{
    fs,
    io::{self, Write, stdin, stdout},
};

fn compile(source: &str) {
    for token in Lexer::new(source) {
        println!("{token:?}");
    }

    let ast = Parser::new(Lexer::new(source)).parse().unwrap();

    println!("====");
    println!("=> AST:");
    println!("{}", serde_yaml::to_string(&ast).unwrap());

    let hir = Builder::new().build_hir(ast);

    println!("====");
    println!("=> HIR:");
    println!("{}", serde_yaml::to_string(&hir).unwrap());
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
