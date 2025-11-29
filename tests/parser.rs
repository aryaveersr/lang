use insta::{assert_yaml_snapshot, glob, with_settings};
use lang::{lexer::Lexer, parser::Parser};
use std::fs;

#[test]
fn test_parser() {
    glob!("parser/*.lang", |path| {
        with_settings!({sort_maps => true}, {
            let source = fs::read_to_string(path).unwrap();
            let lexer = Lexer::new(&source);
            let result = Parser::new(lexer).parse();

            assert_yaml_snapshot!(result);
        });
    });
}
