use super::*;

#[test]
fn just_expression_group() {
	parses_to! {
		parser: MistParser,
		input: "(1)",
		rule: Rule::expression_list,
		tokens: [
			expression_list(0, 3, [
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
			])
		]
	};
}

#[test]
fn just_identifier() {
	parses_to! {
		parser: MistParser,
		input: "a",
		rule: Rule::expression_list,
		tokens: [
			expression_list(0, 1, [
				identifier(0, 1)
			])
		]
	};
}

#[test]
fn just_literal() {
	parses_to! {
		parser: MistParser,
		input: "1",
		rule: Rule::expression_list,
		tokens: [
			expression_list(0, 1, [
				literal(0, 1, [
					literal_integer(0, 1)
				])
			])
		]
	};
}

#[test]
fn c_style_simple_call() {
	parses_to! {
		parser: MistParser,
		input: "a(1)",
		rule: Rule::expression_list,
		tokens: [
			expression_list(0, 4, [
				identifier(0, 1),
				expression_group(1, 4, [
					b_paren_l(1, 2),
					invoke(2, 3, [
						expression_list(2, 3, [
							literal(2, 3, [
								literal_integer(2, 3)
							])
						])
					]),
					b_paren_r(3, 4)
				])
			])
		]
	};
}

#[test]
fn ml_style_simple_call() {
	parses_to! {
		parser: MistParser,
		input: "a 1",
		rule: Rule::expression_list,
		tokens: [
			expression_list(0, 3, [
				identifier(0, 1),
				WHITESPACE(1, 2),
				literal(2, 3, [
					literal_integer(2, 3)
				])
			])
		]
	};
}

#[test]
fn mixed_call() {
	parses_to! {
		parser: MistParser,
		input: "a(1) b",
		rule: Rule::expression_list,
		tokens: [
			expression_list(0, 6, [
				identifier(0, 1),
				expression_group(1, 4, [
					b_paren_l(1, 2),
					invoke(2, 3, [
						expression_list(2, 3, [
							literal(2, 3, [
								literal_integer(2, 3)
							])
						])
					]),
					b_paren_r(3, 4)
				]),
				WHITESPACE(4, 5),
				identifier(5, 6)
			])
		]
	}
}
