use super::*;

#[test]
fn bool() {
	parses_to! {
		parser: MistParser,
		input: "-> bool",
		rule: Rule::fn_return_type,
		tokens: [
			fn_return_type(0, 7, [
				sym_arrow(0, 2),
				WHITESPACE(2, 3),
				type_ref(3, 7, [
					identifier(3, 7)
				])
			])
		]
	};
}
