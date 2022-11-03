#[macro_use]
extern crate mist_tokens;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
#[macro_use]
extern crate rstest;

mod parse_expression;
mod parse_function;
mod parse_module;
mod parse_pattern;
mod parse_statement;
mod parse_type;
mod parser;
mod syntax_kind;

pub use parser::ParseError;
pub use parser::Parser;
pub use syntax_kind::SyntaxKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Mist {}

impl rowan::Language for Mist {
	type Kind = syntax_kind::SyntaxKind;

	fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
		raw.into()
	}

	fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
		kind.into()
	}
}

pub type SyntaxElement = rowan::SyntaxElement<Mist>;
pub type SyntaxNode = rowan::SyntaxNode<Mist>;
pub type SyntaxToken = rowan::SyntaxToken<Mist>;
