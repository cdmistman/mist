use super::*;

#[test]
fn block() {
	parses_to! {
		parser: MistParser,
		input: "{}",
		rule: Rule::expression_with_block,
		tokens: [
			expression_with_block(0, 2, [
				block(0, 2, [
					b_curly_l(0, 1),
					b_curly_r(1, 2)
				])
			])
		]
	};
}

#[test]
fn if_expression() {
	parses_to! {
		parser: MistParser,
		input: "if true {}",
		rule: Rule::expression_with_block,
		tokens: [
			expression_with_block(0, 10, [
				if_expression(0, 10, [
					kw_if(0, 2),
					WHITESPACE(2, 3),
					call(3, 8, [
						expression_list(3, 8, [
							literal(3, 7, [
								kw_true(3, 7),
							]),
							WHITESPACE(7, 8)
						])
					]),
					block(8, 10, [
						b_curly_l(8, 9),
						b_curly_r(9, 10)
					])
				])
			])
		]
	};
}
