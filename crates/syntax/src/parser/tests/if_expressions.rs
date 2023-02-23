use super::*;

#[test]
fn if_expression() {
	parses_to! {
		parser: MistParser,
		input: "if 0{1}",
		rule: Rule::if_expression,
		tokens: [
			if_expression(0, 7, [
				kw_if(0, 2),
				WHITESPACE(2, 3),
				expression(3, 4, [
					expression_without_block(3, 4, [
						literal(3, 4, [
							literal_integer(3, 4)
						])
					])
				]),
				block(4, 7, [
					b_curly_l(4, 5),
					statements(5, 6, [
						expression_without_block(5, 6, [
							literal(5, 6, [
								literal_integer(5, 6)
							])
						])
					]),
					b_curly_r(6, 7)
				])
			])
		]
	};
}

#[test]
fn if_else_expression() {
	parses_to! {
		parser: MistParser,
		input: "if 0{1} else{2}",
		rule: Rule::if_expression,
		tokens: [
			if_expression(0, 15, [
				kw_if(0, 2),
				WHITESPACE(2, 3),
				expression(3, 4, [
					expression_without_block(3, 4, [
						literal(3, 4, [
							literal_integer(3, 4)
						])
					])
				]),
				block(4, 7, [
					b_curly_l(4, 5),
					statements(5, 6, [
						expression_without_block(5, 6, [
							literal(5, 6, [
								literal_integer(5, 6)
							])
						])
					]),
					b_curly_r(6, 7)
				]),
				WHITESPACE(7, 8),
				kw_else(8, 12),
				else_clause(12, 15, [
					block(12, 15, [
						b_curly_l(12, 13),
						statements(13, 14, [
							expression_without_block(13, 14, [
								literal(13, 14, [
									literal_integer(13, 14)
								])
							])
						]),
						b_curly_r(14, 15)
					])
				])
			])
		]
	};
}

#[test]
fn chained_if_expression() {
	parses_to! {
		parser: MistParser,
		input: "if 0{1} else if 2{3}",
		rule: Rule::if_expression,
		tokens: [
			if_expression(0, 20, [
				kw_if(0, 2),
				WHITESPACE(2, 3),
				expression(3, 4, [
					expression_without_block(3, 4, [
						literal(3, 4, [
							literal_integer(3, 4)
						])
					])
				]),
				block(4, 7, [
					b_curly_l(4, 5),
					statements(5, 6, [
						expression_without_block(5, 6, [
							literal(5, 6, [
								literal_integer(5, 6)
							])
						])
					]),
					b_curly_r(6, 7)
				]),
				WHITESPACE(7, 8),
				kw_else(8, 12),
				WHITESPACE(12, 13),
				else_clause(13, 20, [
					if_expression(13, 20, [
						kw_if(13, 15),
						WHITESPACE(15, 16),
						expression(16, 17, [
							expression_without_block(16, 17, [
								literal(16, 17, [
									literal_integer(16, 17)
								])
							])
						]),
						block(17, 20, [
							b_curly_l(17, 18),
							statements(18, 19, [
								expression_without_block(18, 19, [
									literal(18, 19, [
										literal_integer(18, 19)
									])
								])
							]),
							b_curly_r(19, 20)
						]),
					])
				])
			])
		]
	};

	parses_to! {
		parser: MistParser,
		input: "if 0{1} else if 2{3}else{4}",
		rule: Rule::if_expression,
		tokens: [
			if_expression(0, 27, [
				kw_if(0, 2),
				WHITESPACE(2, 3),
				expression(3, 4, [
					expression_without_block(3, 4, [
						literal(3, 4, [
							literal_integer(3, 4)
						])
					])
				]),
				block(4, 7, [
					b_curly_l(4, 5),
					statements(5, 6, [
						expression_without_block(5, 6, [
							literal(5, 6, [
								literal_integer(5, 6)
							])
						])
					]),
					b_curly_r(6, 7)
				]),
				WHITESPACE(7, 8),
				kw_else(8, 12),
				WHITESPACE(12, 13),
				else_clause(13, 27, [
					if_expression(13, 27, [
						kw_if(13, 15),
						WHITESPACE(15, 16),
						expression(16, 17, [
							expression_without_block(16, 17, [
								literal(16, 17, [
									literal_integer(16, 17)
								])
							])
						]),
						block(17, 20, [
							b_curly_l(17, 18),
							statements(18, 19, [
								expression_without_block(18, 19, [
									literal(18, 19, [
										literal_integer(18, 19)
									])
								])
							]),
							b_curly_r(19, 20)
						]),
						kw_else(20, 24),
						else_clause(24, 27, [
							block(24, 27, [
								b_curly_l(24, 25),
								statements(25, 26, [
									expression_without_block(25, 26, [
										literal(25, 26, [
											literal_integer(25, 26)
										])
									])
								]),
								b_curly_r(26, 27)
							])
						])
					])
				])
			])
		]
	};
}
