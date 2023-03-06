use super::*;

#[test]
fn empty_main() {
	parses_to! {
		parser: MistParser,
		input: "fn main() {}",
		rule: Rule::fn_def,
		tokens: [
			fn_def(0, 12, [
				kw_fn(0, 2),
				WHITESPACE(2, 3),
				identifier(3, 7),
				fn_param_list(7, 9, [
					b_paren_l(7, 8),
					b_paren_r(8, 9)
				]),
				WHITESPACE(9, 10),
				block(10, 12, [
					b_curly_l(10, 11),
					b_curly_r(11, 12)
				])
			])
		]
	};
}

#[test]
fn empty_main_explicit_return_type() {
	parses_to! {
		parser: MistParser,
		input: "fn main() -> () {}",
		rule: Rule::fn_def,
		tokens: [
			fn_def(0, 18, [
				kw_fn(0, 2),
				WHITESPACE(2, 3),
				identifier(3, 7),
				fn_param_list(7, 9, [
					b_paren_l(7, 8),
					b_paren_r(8, 9)
				]),
				WHITESPACE(9, 10),
				fn_return_type(10, 15, [
					sym_arrow(10, 12),
					WHITESPACE(12, 13),
					type_ref(13, 15, [
						type_ref_group(13, 15, [
							b_paren_l(13, 14),
							b_paren_r(14, 15)
						])
					])
				]),
				WHITESPACE(15, 16),
				block(16, 18, [
					b_curly_l(16, 17),
					b_curly_r(17, 18)
				])
			])
		]
	}
}
