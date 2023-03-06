mod blocks;
mod expression_groups;
mod expression_lists;
mod expressions_with_block;
mod fn_defs;
mod fn_param_lists;
mod fn_return_types;
mod if_expressions;
mod invocations;
mod literals;
mod statements;

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
