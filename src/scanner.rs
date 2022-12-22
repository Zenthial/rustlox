#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub start: usize,
    pub content: String,
    pub length: usize,
    pub line: i32,
}

pub struct Scanner {
    source: Box<Vec<char>>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn init(source: String) -> Self {
        let mut scanner = Scanner {
            source: Box::new(source.chars().collect()),
            start: 0,
            current: 0,
            line: 1,
        };

        scanner.source.push('\0');
        return scanner;
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            t_type: TokenType::Error,
            start: 0,
            length: message.len(),
            content: message.to_string(),
            line: self.line,
        }
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let content = if token_type == TokenType::Eof {
            "".to_string()
        } else {
            self.source[self.start..self.current].iter().collect()
        };

        Token {
            t_type: token_type,
            start: self.start,
            length: self.current - self.start,
            content,
            line: self.line,
        }
    }

    fn at_end(&self) -> bool {
        return self.source[self.current] == '\0';
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.source[self.current - 1];
    }

    fn peak(&self) -> char {
        return self.source[self.current];
    }

    fn peak_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        return self.source[self.current + 1];
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peak();
            match c {
                ' ' | '\r' | '\t' => _ = self.advance(),
                '\n' => {
                    self.line += 1;
                    _ = self.advance();
                }
                '/' => {
                    if self.peak_next() == '/' {
                        while self.peak() != '\n' && !self.at_end() {
                            _ = self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn string(&mut self) -> Token {
        while self.peak() != '"' && !self.at_end() {
            if self.peak() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance();
        return self.make_token(TokenType::String);
    }

    fn number(&mut self) -> Token {
        while self.peak().is_digit(10) {
            self.advance();
        }

        if self.peak() == '.' && self.peak_next().is_digit(10) {
            self.advance();

            while self.peak().is_digit(10) {
                self.advance();
            }
        }

        return self.make_token(TokenType::Number);
    }

    fn match_keyword(&self, start: usize, rest: &str, token: TokenType) -> TokenType {
        if self.current - self.start != start + rest.len() {
            return TokenType::Identifier;
        }

        for (i, c) in rest.chars().enumerate() {
            if self.source[self.start + start + i] != c {
                return TokenType::Identifier;
            }
        }

        return token;
    }

    fn identifier_type(&self) -> TokenType {
        let c = self.source[self.start];
        match c {
            'a' => self.match_keyword(1, "nd", TokenType::And),
            'c' => self.match_keyword(1, "lass", TokenType::Class),
            'e' => self.match_keyword(1, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'a' => self.match_keyword(2, "lse", TokenType::False),
                        'o' => self.match_keyword(2, "r", TokenType::For),
                        'u' => self.match_keyword(2, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'i' => self.match_keyword(1, "f", TokenType::If),
            'n' => self.match_keyword(1, "il", TokenType::Nil),
            'o' => self.match_keyword(1, "r", TokenType::Or),
            'p' => self.match_keyword(1, "rint", TokenType::Print),
            'r' => self.match_keyword(1, "eturn", TokenType::Return),
            's' => self.match_keyword(1, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'h' => self.match_keyword(2, "is", TokenType::This),
                        'r' => self.match_keyword(2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'v' => self.match_keyword(1, "ar", TokenType::Var),
            'w' => self.match_keyword(1, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn identifier(&mut self) -> Token {
        while self.peak().is_alphabetic() || self.peak().is_digit(10) {
            self.advance();
        }

        return self.make_token(self.identifier_type());
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.at_end() {
            return self.make_token(TokenType::Eof);
        }

        match self.advance() {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            ';' => return self.make_token(TokenType::SemiColon),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '*' => return self.make_token(TokenType::Star),
            '/' => return self.make_token(TokenType::Slash),
            '!' => {
                if self.match_token('=') {
                    return self.make_token(TokenType::BangEqual);
                } else {
                    return self.make_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_token('=') {
                    return self.make_token(TokenType::EqualEqual);
                } else {
                    return self.make_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_token('=') {
                    return self.make_token(TokenType::LessEqual);
                } else {
                    return self.make_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_token('=') {
                    return self.make_token(TokenType::GreaterEqual);
                } else {
                    return self.make_token(TokenType::Greater);
                }
            }
            '"' => return self.string(),
            c if c.is_digit(10) => return self.number(),
            c if c.is_alphabetic() => return self.identifier(),
            _ => return self.error_token("Unexpected character."),
        }
    }
}
