use super::*;

impl<'a> Parser<'a> {
    pub(super) fn next(&mut self) -> Result<Token<'a>> {
        self.lexer.next().ok_or(Pe::Eof)
    }

    pub(super) fn eat(&mut self, kind: TokenKind) -> Option<Token<'a>> {
        self.lexer.next_if(|i| i.kind == kind)
    }

    pub(super) fn expect(&mut self, kind: TokenKind, err: Pe) -> Result<Token<'a>> {
        self.eat(kind).ok_or(err)
    }

    pub(super) fn map<T>(&mut self, f: impl Fn(TokenKind) -> Option<T>) -> Option<T> {
        self.lexer
            .next_if(|token| f(token.kind).is_some())
            .map(|t| t.kind)
            .and_then(f)
    }
}
