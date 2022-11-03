use std::borrow::Cow;

use num::BigUint;

use crate::Span;
use crate::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tok<'source> {
	pub token: Token,
	pub span:  Span,
	pub text:  Cow<'source, str>,
	pub data:  Option<TokenDatum>,
}

impl<'source> Tok<'source> {
	pub fn is(&self, token: Token) -> bool {
		self.token == token
	}

	pub fn is_eof(&self) -> bool {
		self.is(Token::Eof)
	}

	pub fn to_static(&self) -> Tok<'static> {
		Tok {
			token: self.token,
			span:  self.span,
			data:  self.data.clone(),
			text:  self.text.to_string().into(),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenDatum {
	IntLit { n: BigUint, base: u32 },
}
