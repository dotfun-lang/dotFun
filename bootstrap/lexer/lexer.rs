use std::fmt::Error;

use crate::lexer::token::{Token, TokenType};

pub struct Lexer {
    source: String,
    start: usize,
    current: usize,
    line: i64,
    column: i64,
    tokens: Vec<Token>
}

impl Lexer {
	pub fn new(source: String) -> Self {
		Lexer {
			source,
			start: 0,
			current: 0,
			line: 1,
			column: 1,
			tokens: Vec::new()
		}
	}

	pub fn lex(&mut self) -> Result<&Vec<Token>, String> {

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_tokens()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            line: self.line,
            column: self.column,
        });

        Ok(&self.tokens)
    }

	fn add_token(&mut self, token_type: TokenType) -> Result<(), String> {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            line: self.line,
            column: self.column,
        });

        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<(), String> {
        let c = self.advance();

        match c {
            b' ' | b'\r' | b'\t' => Ok(()),
            b'\n' => {
                self.line += 1;
                self.column = 1;
                Ok(())
            }

            b'/' => {
                if self.match_char(b'/') {
                    while !self.is_at_end() && self.peek() != b'\n' {
                        self.advance();
                    }
                    Ok(())
                } else if self.match_char(b'*') {
                    self.block_comment()
                } else {
                    self.add_token(TokenType::Slash)
                }
            }

            b'"' | b'\'' => self.string(),

            b'+' => {
                if self.match_char(b'+') { self.add_token(TokenType::PlusPlus) }
                else { self.add_token(TokenType::Plus) }
            }
            b'-' => {
                if self.match_char(b'-') { self.add_token(TokenType::MinusMinus) }
                else if self.match_char(b'>') { self.add_token(TokenType::Arrow) }
                else { self.add_token(TokenType::Minus) }
            }
            b'*' => self.add_token(TokenType::Star),
            b'%' => self.add_token(TokenType::Percent),

            b'=' => {
                if self.match_char(b'=') { self.add_token(TokenType::EqualEqual) }
                else { self.add_token(TokenType::Equal) }
            }
            b'!' => {
                if self.match_char(b'=') { self.add_token(TokenType::NotEqual) }
                else if self.match_char(b'!') { self.add_token(TokenType::BangBang) }
                else { self.add_token(TokenType::NotBang) }
            }
            b'>' => {
                if self.match_char(b'=') { self.add_token(TokenType::GreaterEqual) }
                else if self.match_char(b'>') { self.add_token(TokenType::ShiftRight) }
                else { self.add_token(TokenType::Greater) }
            }
            b'<' => {
                if self.match_char(b'=') { self.add_token(TokenType::LessEqual) }
                else if self.match_char(b'<') { self.add_token(TokenType::ShiftLeft) }
                else { self.add_token(TokenType::Less) }
            }

            b'&' => {
                if self.match_char(b'&') { self.add_token(TokenType::AndAnd) }
                else { self.add_token(TokenType::BitAnd) }
            }
            b'|' => {
                if self.match_char(b'|') { self.add_token(TokenType::OrOr) }
                else { self.add_token(TokenType::BitOr) }
            }

            b':' => {
                if self.match_char(b':') { self.add_token(TokenType::ColonColon) }
                else { self.add_token(TokenType::Colon) }
            }
            b'.' => {
                if self.match_char(b'.') {
                    if self.match_char(b'.') {
                        self.add_token(TokenType::Ellipsis)
                    } else {
                        Err(format!("Expected third '.' for ellipsis at line {}", self.line))
                    }
                } else {
                    self.add_token(TokenType::Dot)
                }
            }

            b'?' => self.add_token(TokenType::Question),
            b',' => self.add_token(TokenType::Comma),
            b';' => self.add_token(TokenType::Semicolon),
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b'[' => self.add_token(TokenType::LeftBracket),
            b']' => self.add_token(TokenType::RightBracket),
            b'$' => self.add_token(TokenType::Dollar),
            b'@' => self.add_token(TokenType::AT),

            b'0'..=b'9' => self.number(),

            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identifier(),

            _ => Err(format!(
                "Unexpected character '{}' at line {} column {}",
                c as char, self.line, self.column
            )),
        }
    }

	fn string(&mut self) -> Result<(), String> {
		loop {
			if self.is_at_end() {
				return Err(format!("Unterminated string literal at line {}", self.line));
			}
			if self.peek() == b'"' {
				self.advance();
				break;
			}
            if self.peek() == b'\n' {
                self.line += 1;
                self.column = 0;
            }
			self.advance();
		}
		return self.add_token(TokenType::StringLiteral);
	}

	pub fn number(&mut self) -> Result<(), String> {
		while is_digit(self.peek()) {
			self.advance();
		}

		if self.peek() == b'.' && is_digit(self.peek_next()) {
			self.advance();

			while is_digit(self.peek()) {
				self.advance();
			}
		}

		self.add_token(TokenType::NumberLiteral)
	}

	pub fn identifier(&mut self) -> Result<(), String> {
		while is_alpha_numeric(self.peek()) {
			self.advance();
		}

		let text = &self.source[self.start..self.current];
		let token_type = lookup_keyword(text);

		self.add_token(token_type)
	}

	fn block_comment(&mut self) -> Result<(), String> {
        loop {
            if self.is_at_end() {
                return Err(format!("Unterminated block comment at line {}", self.line));
            }
            if self.peek() == b'*' && self.peek_next() == b'/' {
                self.advance();
                self.advance();
                break;
            }
            if self.peek() == b'\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }
        Ok(())
    }

	pub fn is_at_end(&self) -> bool {
		self.current >= self.source.len() as usize
	}

	pub fn advance(&mut self) -> u8 {
        if self.is_at_end() {
            return 0;
        }

        let c = self.source.as_bytes()[self.current];

        self.current += 1;
        self.column += 1;
        c
    }

    pub fn match_char(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.as_bytes()[self.current] != expected {
            return false;
        }

        self.current += 1;
        self.column += 1;
        true
    }

    pub fn peek(&self) -> u8 {
        if self.is_at_end() {
            return 0;
        }
        self.source.as_bytes()[self.current]
    }

    pub fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return 0;
        }
        self.source.as_bytes()[self.current + 1]
    }
}

fn lookup_keyword(text: &str) -> TokenType {
	match text {
		"class" => TokenType::Class,
		"interface" => TokenType::Interface,
		"import" => TokenType::Import,
		"package" => TokenType::Package,
		"enum" => TokenType::Enum,
		"struct" => TokenType::Struct,
		"protected" => TokenType::Protected,
		"private" => TokenType::Private,
		"override" => TokenType::Override,
		"this" => TokenType::This,
		"new" => TokenType::New,
		"super" => TokenType::Super,
		"constructor" => TokenType::Constructor,
		"data" => TokenType::Data,
		"typeof" => TokenType::Typeof,
		"annotation" => TokenType::Annotation,
		"if" => TokenType::If,
		"else" => TokenType::Else,
		"elif" => TokenType::Elif,
		"while" => TokenType::While,
		"for" => TokenType::For,
		"loop" => TokenType::Loop,
		"break" => TokenType::Break,
		"continue" => TokenType::Continue,
		"async" => TokenType::Async,
		"await" => TokenType::Await,
		"fn" => TokenType::Function,
		"return" => TokenType::Return,
		"true" => TokenType::True,
		"false" => TokenType::False,
		"null" => TokenType::Null,
		"mut" => TokenType::Mut,
		"val" => TokenType::Val,
		"and" => TokenType::And,
		"or" => TokenType::Or,
		"not" => TokenType::Not,
		"is" => TokenType::Is,
		"in" => TokenType::In,
		"of" => TokenType::Of,
		_ => TokenType::Identifier,
	}
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

fn is_alpha(c: u8) -> bool {
    (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_'
}

fn is_alpha_numeric(c: u8) -> bool {
    is_alpha(c) || is_digit(c)
}
