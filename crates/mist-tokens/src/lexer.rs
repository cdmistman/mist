#[cfg(test)]
mod tests;

use std::borrow::Cow;
use std::str::Chars;

use num::BigUint;
use num::Num;
use unicode_xid::UnicodeXID;

use crate::Span;
use crate::Tok;
use crate::Token;
use crate::TokenDatum;
use crate::T;

#[derive(Debug)]
pub struct Lexer<'source> {
	text:     &'source str,
	peek0:    char,
	peek1:    char,
	rest:     Chars<'source>,
	ch_start: usize,
	ch_end:   usize,
}

impl<'source> Lexer<'source> {
	pub fn new(text: &'source str) -> Self {
		let rest = text.chars();
		let mut lexer = Lexer {
			text,
			rest,
			peek0: '\0',
			peek1: '\0',
			ch_start: 0,
			ch_end: 0,
		};
		lexer.bump();
		lexer.bump();
		lexer.ch_start = 0;
		lexer.ch_end = 0;
		lexer
	}

	fn bump(&mut self) -> char {
		let res = self.peek0;
		self.peek0 = self.peek1;
		self.peek1 = self.rest.next().unwrap_or('\0');
		self.ch_end += 1;
		res
	}

	fn bump_and<F: FnOnce(&mut Self) -> Tok<'source>>(&mut self, f: F) -> Tok<'source> {
		self.bump();
		f(self)
	}

	fn bump_if(&mut self, ch: char) -> bool {
		self.peek0 == ch && {
			self.bump();
			true
		}
	}

	fn chomp(&mut self, token: Token) -> Tok<'source> {
		let span = self.span();
		self.ch_start = self.ch_end;
		let text = Cow::Borrowed(&self.text[span.ch_start..span.ch_end]);
		Tok {
			token,
			span,
			text,
			data: None,
		}
	}

	fn is_at_word_end(&mut self) -> bool {
		!(self.peek0.is_xid_continue() || self.peek0 == '\'')
	}

	fn next_token(&mut self) -> Tok<'source> {
		let single_ch_tok_kind = match self.peek0 {
			'\0' => return self.chomp(T!['eof]),
			ch if ch.is_whitespace() => loop {
				self.bump();
				if self.peek0 == '\0' || !self.peek0.is_whitespace() {
					return self.chomp(T!['whitespace]);
				}
			},
			'(' => T!["("],
			')' => T![")"],
			',' => T![,],
			'-' => return self.bump_and(Self::start_dash),
			'.' => T![.],
			'0' => return self.start_0(),
			'1'..='9' => return self.start_int::<10>(),
			':' => T![:],
			';' => T![;],
			'=' => T![=],
			'[' => T!["["],
			']' => T!["]"],
			'^' => T![^],
			'_' => return self.bump_and(Self::start_underscore),
			'b' => return self.bump_and(Self::start_b),
			'e' => return self.bump_and(Self::start_e),
			'f' => return self.bump_and(Self::start_f),
			'i' => return self.bump_and(Self::start_i),
			'l' => return self.bump_and(Self::start_l),
			'm' => return self.bump_and(Self::start_m),
			'r' => return self.bump_and(Self::start_r),
			't' => return self.bump_and(Self::start_t),
			ch if ch.is_xid_start() => return self.bump_and(Self::ident),
			'{' => T!["{"],
			'}' => T!["}"],
			_ => Token::Error,
		};
		self.bump();
		self.chomp(single_ch_tok_kind)
	}

	fn span(&mut self) -> Span {
		Span {
			ch_start: self.ch_start,
			ch_end:   self.ch_end,
		}
	}

	fn ident(&mut self) -> Tok<'source> {
		while self.peek0.is_xid_continue() {
			self.bump();
		}
		while self.peek0 == '\'' {
			self.bump();
		}
		self.chomp(T!['ident])
	}
}

impl<'source> Iterator for Lexer<'source> {
	type Item = Tok<'source>;

	fn next(&mut self) -> Option<Self::Item> {
		Some(self.next_token())
	}
}

impl<'source> Lexer<'source> {
	fn start_dash(&mut self) -> Tok<'source> {
		match self.peek0 {
			'>' => self.bump_and(|l| l.chomp(T![->])),
			_ => todo!("non-arrow dashes"),
		}
	}

	fn start_b(&mut self) -> Tok<'source> {
		if self.bump_if('o') && self.bump_if('o') && self.bump_if('l') && self.is_at_word_end() {
			return self.chomp(T![bool]);
		}
		self.ident()
	}

	fn start_e(&mut self) -> Tok<'source> {
		if self.bump_if('l') && self.bump_if('s') && self.bump_if('e') && self.is_at_word_end() {
			return self.chomp(T![else]);
		}
		self.ident()
	}

	fn start_f(&mut self) -> Tok<'source> {
		match self.peek0 {
			'a' => {
				self.bump();
				self.start_fa()
			},
			'u' => {
				self.bump();
				self.start_fu()
			},
			_ => self.ident(),
		}
	}

	fn start_fa(&mut self) -> Tok<'source> {
		if self.bump_if('l') && self.bump_if('s') && self.bump_if('e') && self.is_at_word_end() {
			return self.chomp(T![false]);
		}
		self.ident()
	}

	fn start_fu(&mut self) -> Tok<'source> {
		if self.bump_if('n') && self.is_at_word_end() {
			return self.chomp(T![fun]);
		}
		self.ident()
	}

	fn start_i(&mut self) -> Tok<'source> {
		match self.peek0 {
			'f' => {
				self.bump();
				if self.is_at_word_end() {
					return self.chomp(T![if]);
				}
			},
			'n' => {
				self.bump();
				if self.bump_if('t') && self.is_at_word_end() {
					return self.chomp(T![int]);
				}
			},
			_ => (),
		}
		self.ident()
	}

	fn start_l(&mut self) -> Tok<'source> {
		if self.bump_if('e') && self.bump_if('t') && self.is_at_word_end() {
			return self.chomp(T![let]);
		}
		self.ident()
	}

	fn start_m(&mut self) -> Tok<'source> {
		if self.bump_if('a')
			&& self.bump_if('t')
			&& self.bump_if('c')
			&& self.bump_if('h')
			&& self.is_at_word_end()
		{
			return self.chomp(T![match]);
		}
		self.ident()
	}

	fn start_r(&mut self) -> Tok<'source> {
		if self.bump_if('e')
			&& self.bump_if('t')
			&& self.bump_if('u')
			&& self.bump_if('r')
			&& self.bump_if('n')
			&& self.is_at_word_end()
		{
			return self.chomp(T![return]);
		}
		self.ident()
	}

	fn start_t(&mut self) -> Tok<'source> {
		if self.bump_if('r') && self.bump_if('u') && self.bump_if('e') && self.is_at_word_end() {
			return self.chomp(T![true]);
		}
		self.ident()
	}

	fn start_underscore(&mut self) -> Tok<'source> {
		while self.bump_if('_') {}

		if self.is_at_word_end() {
			self.chomp(T![_])
		} else if self.peek0.is_xid_continue() {
			self.bump();
			self.ident()
		} else {
			todo!("other underscore applications?");
		}
	}

	fn start_0(&mut self) -> Tok<'source> {
		match self.peek1 {
			'b' => {
				self.bump();
				self.bump();
				self.start_int::<2>()
			},
			'o' => {
				self.bump();
				self.bump();
				self.start_int::<8>()
			},
			'x' => {
				self.bump();
				self.bump();
				self.start_int::<16>()
			},
			_ => self.start_int::<10>(),
		}
	}

	fn start_int<const BASE: u32>(&mut self) -> Tok<'source> {
		let mut digits = String::from(self.bump());
		loop {
			let digit = match self.peek0 {
				'_' => {
					self.bump();
					continue;
				},
				'0'..='1' => self.bump(),
				'2'..='7' if BASE > 2 => self.bump(),
				'8'..='9' if BASE > 8 => self.bump(),
				'a'..='z' | 'A'..='Z' if BASE > 10 => self.bump(),
				_ => break,
			};
			digits.push(digit);
		}
		let n = BigUint::from_str_radix(digits.as_str(), BASE).unwrap();
		let mut res = self.chomp(T!['int]);
		res.data = Some(TokenDatum::IntLit { n, base: BASE });
		res
	}
}
