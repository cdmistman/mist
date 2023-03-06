use super::*;

#[test]
fn just_call() {
	parses_to! {
		parser: MistParser,
		input: "a",
		rule: Rule::statements,
		tokens: [
			statements(0, 1, [
				call(0, 1, [
					expression_list(0, 1, [
						identifier(0, 1)
					])
				])
			])
		]
	};
}

#[test]
fn invoke_no_semicolon() {
	parses_to! {
		parser: MistParser,
		input: "a{1}",
		rule: Rule::statements,
		tokens: [
			statements(0, 4, [
				call(0, 1, [
					expression_list(0, 1, [
						identifier(0, 1)
					])
				]),
				block(1, 4, [
					b_curly_l(1, 2),
					statements(2, 3, [
						call(2, 3, [
							expression_list(2, 3, [
								literal(2, 3, [
									literal_integer(2, 3)
								])
							])
						])
					]),
					b_curly_r(3, 4)
				])
			])
		]
	};
}

#[test]
fn invoke_no_semicolon_and_call() {
	parses_to! {
		parser: MistParser,
		input: "a{1}b",
		rule: Rule::statements,
		tokens: [
			statements(0, 5, [
				call(0, 1, [
					expression_list(0, 1, [
						identifier(0, 1)
					])
				]),
				block(1, 4, [
					b_curly_l(1, 2),
					statements(2, 3, [
						call(2, 3, [
							expression_list(2, 3, [
								literal(2, 3, [
									literal_integer(2, 3)
								])
							])
						])
					]),
					b_curly_r(3, 4)
				]),
				statements(4, 5, [
					call(4, 5, [
						expression_list(4, 5, [
							identifier(4, 5)
						])
					])
				])
			])
		]
	}
}

#[test]
fn if_invoke_call() {
	parses_to! {
		parser: MistParser,
		input: "if true{a}b{c}d",
		rule: Rule::statements,
		tokens: [
			statements(0, 15, [
				expression_with_block(0, 10, [
					if_expression(0, 10, [
						kw_if(0, 2),
						WHITESPACE(2, 3),
						call(3, 7, [
							expression_list(3, 7, [
								literal(3, 7, [
									kw_true(3, 7),
								]),
							])
						]),
						block(7, 10, [
							b_curly_l(7, 8),
							statements(8, 9, [
								call(8, 9, [
									expression_list(8, 9, [
										identifier(8, 9)
									])
								])
							]),
							b_curly_r(9, 10)
						])
					])
				]),
				statements(10, 15, [
					call(10, 11, [
						expression_list(10, 11, [
							identifier(10, 11)
						])
					]),
					block(11, 14, [
						b_curly_l(11, 12),
						statements(12, 13, [
							call(12, 13, [
								expression_list(12, 13, [
									identifier(12, 13)
								])
							])
						]),
						b_curly_r(13, 14),
					]),
					statements(14, 15, [
						call(14, 15, [
							expression_list(14, 15, [
								identifier(14, 15)
							])
						])
					])
				])
			])
		]
	}
}
