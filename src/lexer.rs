use crate::{
    position::Position,
    token::{Token, TokenKind},
};

pub struct Lexer<'src> {
    source: &'src str,
    pos: Position,
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_whitespace();

        Some(match self.source.chars().next()? {
            '{' => self.consume_char(TokenKind::LeftBrace),
            '}' => self.consume_char(TokenKind::RightBrace),
            '(' => self.consume_char(TokenKind::LeftParen),
            ')' => self.consume_char(TokenKind::RightParen),
            ';' => self.consume_char(TokenKind::Semicolon),
            ':' => self.consume_char(TokenKind::Colon),
            ',' => self.consume_char(TokenKind::Comma),
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

            _ => self.consume_char(TokenKind::Unknown),
        })
    }
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            pos: Position::default(),
        }
    }

    fn consume_whitespace(&mut self) {
        loop {
            while let Some(c) = self.source.chars().next()
                && c.is_ascii_whitespace()
            {
                self.source = &self.source[1..];
                self.pos.take_char(c);
            }

            if !self.source.starts_with("//") {
                break;
            }

            self.until(|i| i == '\n');

            if self.source.starts_with('\n') {
                self.source = &self.source[1..];
                self.pos.newline();
            }
        }
    }

    fn consume_char(&mut self, kind: TokenKind) -> Token<'src> {
        let pos = self.pos;
        let (slice, source) = self.source.split_at(1);

        self.pos.take_char(slice.chars().next().unwrap());
        self.source = source;

        Token::new(kind, slice, pos)
    }

    fn consume_numeric(&mut self) -> Token<'src> {
        let pos = self.pos;
        let slice = self.until(|i| !i.is_ascii_digit());

        Token::new(TokenKind::Numeric, slice, pos)
    }

    fn consume_identifier(&mut self) -> Token<'src> {
        let pos = self.pos;
        let identifier = self.until(|i| !is_valid_in_identifier(i));

        Token::new(match_kind(identifier), identifier, pos)
    }

    fn consume_eq(&mut self, not_eq: TokenKind, eq: TokenKind) -> Token<'src> {
        let pos = self.pos;
        let (idx, kind) = match self.source.chars().nth(1) {
            Some('=') => (2, eq),
            _ => (1, not_eq),
        };

        let (slice, source) = self.source.split_at(idx);

        self.pos.column += idx;
        self.source = source;

        Token::new(kind, slice, pos)
    }

    fn until(&mut self, condition: impl Fn(char) -> bool) -> &'src str {
        let (slice, source) = match self.source.find(condition) {
            Some(idx) => self.source.split_at(idx),
            None => (self.source, &self.source[self.source.len()..]),
        };

        slice.chars().for_each(|c| self.pos.take_char(c));

        self.source = source;
        slice
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
