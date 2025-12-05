use insta::{assert_yaml_snapshot, glob, with_settings};
use lang::{lexer::Lexer, parser::Parser, type_resolver::TypeResolver};
use std::fs;

#[test]
fn test_type_resolver() {
    glob!("type_resolver/*.lang", |path| {
        with_settings!({sort_maps => true}, {
            let source = fs::read_to_string(path).unwrap();
            let lexer = Lexer::new(&source);
            let mut hir = Parser::new(lexer).parse().unwrap();
            let result = TypeResolver::new().resolve(&mut hir);

            assert_yaml_snapshot!(result);

            if result.is_ok() {
                assert_yaml_snapshot!(hir);
            }
        });
    });
}
