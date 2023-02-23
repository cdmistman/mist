mod if_expressions;
mod literals;

mod test_prelude {
	pub use pest::*;

	pub use crate::parser::*;
}

use test_prelude::*;

#[test]
fn identifiers() {
	parses_to! {
		parser: MistParser,
		input: "a",
		rule: Rule::identifier,
		tokens: [identifier(0, 1)]
	};

	parses_to! {
		parser: MistParser,
		input: "ab",
		rule: Rule::identifier,
		tokens: [identifier(0, 2)]
	};

	parses_to! {
		parser: MistParser,
		input: "_a",
		rule: Rule::identifier,
		tokens: [identifier(0, 2)]
	};

	parses_to! {
		parser: MistParser,
		input: "_ab",
		rule: Rule::identifier,
		tokens: [identifier(0, 3)]
	};
}