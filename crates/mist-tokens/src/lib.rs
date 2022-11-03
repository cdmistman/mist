#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
#[macro_use]
extern crate rstest;

pub mod lexer;
mod tok;
mod token;
mod token_set;

pub use tok::Tok;
pub use tok::TokenDatum;
pub use token::Token;
pub use token_set::TokenSet;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Span {
	pub ch_start: usize,
	pub ch_end:   usize,
}

#[macro_export]
macro_rules! T {
	// used for easy ddkp with vim, using this variant will result in an error
	[] => { $crate::Token:: };

	// braces
	["{"] => { $crate::Token::LCurly };
	["}"] => { $crate::Token::RCurly };
	["("] => { $crate::Token::LParen };
	[")"] => { $crate::Token::RParen };
	["["] => { $crate::Token::LSquare };
	["]"] => { $crate::Token::RSquare };

	// symbols
	[->] => { $crate::Token::Arrow };
	[^] => { $crate::Token::Carat };
	[:] => { $crate::Token::Colon };
	[,] => { $crate::Token::Comma };
	[.] => { $crate::Token::Dot };
	[=] => { $crate::Token::Equal };
	[;] => { $crate::Token::Semicolon };
	[_] => { $crate::Token::Underscore };

	// keywords
	[bool] => { $crate::Token::KwBool };
	[else] => { $crate::Token::KwElse };
	[false] => { $crate::Token::KwFalse };
	[fun] => { $crate::Token::KwFun };
	[if] => { $crate::Token::KwIf };
	[int] => { $crate::Token::KwInt };
	[let] => { $crate::Token::KwLet };
	[match] => { $crate::Token::KwMatch };
	[return] => { $crate::Token::KwReturn };
	[true] => { $crate::Token::KwTrue };

	['ident] => { $crate::Token::Identifier };
	['int]   => { $crate::Token::Integer };

	['whitespace] => { $crate::Token::Whitespace };
	['eof] => { $crate::Token::Eof };
}

#[macro_export]
macro_rules! tokens {
	[$($token:tt),* $(,)?] => { $crate::TokenSet::from([ $($crate::T![$token],)* ]) };
}
