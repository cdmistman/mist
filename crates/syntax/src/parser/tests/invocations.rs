use super::*;

#[test]
fn just_expression_list() {
	parses_to! {
		parser: MistParser,
		input: "1",
		rule: Rule::invoke,
		tokens: [
			invoke(0, 1, [
				expression_list(0, 1, [
					literal(0, 1, [
						literal_integer(0, 1)
					])
				])
			])
		]
	};
}

#[test]
fn with_block() {
	parses_to! {
		parser: MistParser,
		input: "1 { 2 }",
		rule: Rule::invoke,
		tokens: [
			invoke(0, 7, [
				expression_list(0, 2, [
					literal(0, 2, [
						literal_integer(0, 2, [
							WHITESPACE(1, 2)
						])
					])
				]),
				block(2, 7, [
					b_curly_l(2, 3),
					WHITESPACE(3, 4),
					statements(4, 6, [
						statement_expression(4, 6, [
							call(4, 6, [
								expression_list(4, 6, [
									literal(4, 6, [
										literal_integer(4, 6, [
											WHITESPACE(5, 6)
										])
									])
								])
							])
						])
					]),
					b_curly_r(6, 7)
				])
			])
		]
	}
}
