use super::*;

#[test]
fn empty() {
	parses_to! {
		parser: MistParser,
		input: "()",
		rule: Rule::expression_group,
		tokens: [
			expression_group(0, 2, [
				b_paren_l(0, 1),
				b_paren_r(1, 2)
			])
		]
	};
}

#[test]
fn invoke() {
	parses_to! {
		parser: MistParser,
		input: "(1)",
		rule: Rule::expression_group,
		tokens: [
			expression_group(0, 3, [
				b_paren_l(0, 1),
				invoke(1, 2, [
					expression_list(1, 2, [
						literal(1, 2, [
							literal_integer(1, 2)
						])
					])
				]),
				b_paren_r(2, 3)
			])
		]
	};
}

#[test]
fn expressions_with_block() {
	parses_to! {
		parser: MistParser,
		input: "(if (1) { 2 })",
		rule: Rule::expression_group,
		tokens: [
			expression_group(0, 14, [
				b_paren_l(0, 1),
				expression_with_block(1, 13, [
					if_expression(1, 13, [
						kw_if(1, 3),
						WHITESPACE(3, 4),
						call(4, 8, [
							expression_list(4, 8, [
								expression_group(4, 7, [
									b_paren_l(4, 5),
									invoke(5, 6, [
										expression_list(5, 6, [
											literal(5, 6, [
												literal_integer(5, 6)
											])
										])
									]),
									b_paren_r(6, 7)
								]),
								WHITESPACE(7, 8)
							]),
						]),
						block(8, 13, [
							b_curly_l(8, 9),
							WHITESPACE(9, 10),
							statements(10, 12, [
								call(10, 12, [
									expression_list(10, 12, [
										literal(10, 12, [
											literal_integer(10, 12, [
												WHITESPACE(11, 12)
											])
										]),
									]),
								]),
							]),
							b_curly_r(12, 13)
						])
					])
				]),
				b_paren_r(13, 14)
			])
		]
	}
}
