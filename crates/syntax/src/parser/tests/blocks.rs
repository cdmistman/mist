use super::*;

#[test]
fn empty_block() {
	parses_to! {
		parser: MistParser,
		input: "{}",
		rule: Rule::block,
		tokens: [
			block(0, 2, [
				b_curly_l(0, 1),
				b_curly_r(1, 2)
			])
		]
	};
}

#[test]
fn just_expression() {
	parses_to! {
		parser: MistParser,
		input: "{1}",
		rule: Rule::block,
		tokens: [
			block(0, 3, [
				b_curly_l(0, 1),
				statements(1, 2, [
					statement_expression(1, 2, [
						call(1, 2, [
							expression_list(1, 2, [
								literal(1, 2, [
									literal_integer(1, 2)
								])
							])
						])
					])
				]),
				b_curly_r(2, 3)
			])
		]
	};
}
