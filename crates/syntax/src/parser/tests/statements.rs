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
