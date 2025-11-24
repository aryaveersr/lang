use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Consume until a condition is met, and return the slice upto that point.
    fn until(&mut self, condition: impl Fn(char) -> bool) -> &'a str {
        let (slice, source) = match self.source.find(condition) {
            Some(idx) => self.source.split_at(idx),
            None => (self.source, &self.source[self.source.len()..]),
        };

        self.source = source;
        slice
    }

    fn consume_whitespace(&mut self) {
        self.source = self.source.trim_ascii_start();

        // Consume all characters until newline if this is a single line comment (starts with '//').
        // While the next lines also begin with a comment, consume those as well.
        while self.source.len() > 1 && &self.source[0..2] == "//" {
            self.until(|i| i == '\n');
            self.source = self.source[1..].trim_ascii_start();
        }
    }

    fn consume_char(&mut self, kind: TokenKind) -> Token<'a> {
        let (slice, source) = self.source.split_at(1);
        self.source = source;
        Token { kind, slice }
    }

    fn consume_numeric(&mut self) -> Token<'a> {
        Token {
            kind: TokenKind::Numeric,
            slice: self.until(|i| !i.is_ascii_digit()),
        }
    }

    fn consume_identifier(&mut self) -> Token<'a> {
        let identifier = self.until(|i| !is_valid_in_identifier(i));
        Token {
            kind: match_kind(identifier),
            slice: identifier,
        }
    }

    fn consume_eq(&mut self, not_eq: TokenKind, eq: TokenKind) -> Token<'a> {
        let (idx, kind) = match self.source.chars().nth(1) {
            Some('=') => (2, eq),
            _ => (1, not_eq),
        };

        let (slice, source) = self.source.split_at(idx);
        self.source = source;
        Token { kind, slice }
    }
}

fn is_valid_in_identifier(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn match_kind(identifier: &str) -> TokenKind {
    match identifier {
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "and" => TokenKind::And,
        "or" => TokenKind::Or,
        "fun" => TokenKind::Fun,
        "return" => TokenKind::Return,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "loop" => TokenKind::Loop,
        "while" => TokenKind::While,
        "break" => TokenKind::Break,
        "let" => TokenKind::Let,

        _ => TokenKind::Identifier,
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_whitespace();

        Some(match self.source.chars().next()? {
            '{' => self.consume_char(TokenKind::LeftBrace),
            '}' => self.consume_char(TokenKind::RightBrace),
            '(' => self.consume_char(TokenKind::LeftParen),
            ')' => self.consume_char(TokenKind::RightParen),
            ';' => self.consume_char(TokenKind::Semicolon),
            ':' => self.consume_char(TokenKind::Colon),
            '+' => self.consume_char(TokenKind::Plus),
            '-' => self.consume_char(TokenKind::Minus),
            '*' => self.consume_char(TokenKind::Star),
            '/' => self.consume_char(TokenKind::Slash),

            '=' => self.consume_eq(TokenKind::Equal, TokenKind::EqualEqual),
            '<' => self.consume_eq(TokenKind::Lesser, TokenKind::LesserEqual),
            '>' => self.consume_eq(TokenKind::Greater, TokenKind::GreaterEqual),
            '!' => self.consume_eq(TokenKind::Not, TokenKind::NotEqual),

            c if c.is_ascii_digit() => self.consume_numeric(),
            c if is_valid_in_identifier(c) => self.consume_identifier(),

            _ => self.consume_char(TokenKind::Invalid),
        })
    }
}
