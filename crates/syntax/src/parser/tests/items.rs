use super::*;

#[test]
fn fn_def() {
	parses_to! {
		parser: MistParser,
		input: "fn main() {}",
		rule: Rule::item,
		tokens: [
			item(0, 12, [
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
			])
		]
	}
}
