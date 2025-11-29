use super::*;

impl<'a> Parser<'a> {
    pub(super) fn next(&mut self) -> Token<'a> {
        self.lexer.next().expect("Unexpected EOF.")
    }

    pub(super) fn eat(&mut self, kind: TokenKind) -> Option<Token<'a>> {
        self.lexer.next_if(|i| i.kind == kind)
    }

    pub(super) fn eat_map<T>(&mut self, f: impl Fn(TokenKind) -> Option<T>) -> Option<T> {
        self.lexer
            .next_if(|token| f(token.kind).is_some())
            .map(|t| t.kind)
            .and_then(f)
    }

    pub(super) fn expect(&mut self, kind: TokenKind, err: &str) -> Token<'a> {
        self.eat(kind).expect(err)
    }
}
