use super::*;

#[test]
fn empty() {
	parses_to! {
		parser: MistParser,
		input: "()",
		rule: Rule::fn_param_list,
		tokens: [
			fn_param_list(0, 2, [
				b_paren_l(0, 1),
				b_paren_r(1, 2)
			])
		]
	};
}

#[test]
fn single_untyped_binding() {
	parses_to! {
		parser: MistParser,
		input: "(a)",
		rule: Rule::fn_param_list,
		tokens: [
			fn_param_list(0, 3, [
				b_paren_l(0, 1),
				binding(1, 2, [
					identifier(1, 2)
				]),
				b_paren_r(2, 3)
			])
		]
	};
}

#[test]
fn single_typed_binding() {
	parses_to! {
		parser: MistParser,
		input: "(a: bool)",
		rule: Rule::fn_param_list,
		tokens: [
			fn_param_list(0, 9, [
				b_paren_l(0, 1),
				binding(1, 8, [
					identifier(1, 2),
					sym_colon(2, 3),
					WHITESPACE(3, 4),
					type_ref(4, 8, [
						identifier(4, 8)
					])
				]),
				b_paren_r(8, 9)
			])
		]
	};
}

#[test]
fn multiple_bindings() {
	parses_to! {
		parser: MistParser,
		input: "(a, b: bool)",
		rule: Rule::fn_param_list,
		tokens: [
			fn_param_list(0, 12, [
				b_paren_l(0, 1),
				binding(1, 2, [
					identifier(1, 2)
				]),
				sym_comma(2, 3),
				WHITESPACE(3, 4),
				binding(4, 11, [
					identifier(4, 5),
					sym_colon(5, 6),
					WHITESPACE(6, 7),
					type_ref(7, 11, [
						identifier(7, 11)
					])
				]),
				b_paren_r(11, 12)
			])
		]
	};
}
