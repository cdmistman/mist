use std::collections::HashSet;

use crate::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenSet(HashSet<Token>);

impl TokenSet {
	pub fn contains(&self, token: &Token) -> bool {
		self.0.contains(&token)
	}
}

impl<const N: usize> From<[Token; N]> for TokenSet {
	fn from(value: [Token; N]) -> Self {
		Self(value.into())
	}
}
