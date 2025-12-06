use insta::{assert_yaml_snapshot, glob};
use itertools::Itertools;
use lang::lexer::Lexer;
use std::fs;

#[test]
fn test_lexer() {
    glob!("lexer/*.lang", |path| {
        let source = fs::read_to_string(path).unwrap();
        let lexer = Lexer::new(&source);

        assert_yaml_snapshot!(lexer.collect_vec());
    });
}
