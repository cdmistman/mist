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
				if_expression_else(8, 15, [
					kw_else(8, 12),
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
				if_expression_else(8, 20, [
					kw_else(8, 12),
					WHITESPACE(12, 13),
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

	// parses_to! {
	// 	parser: MistParser,
	// 	input: "if 0{1} elseif 2{3}else{4}",
	// 	rule: Rule::if_expression,
	// 	tokens: [
	// 		if_expression(0, 26, [
	// 			kw_if(0, 2),
	// 			WHITESPACE(2, 3),
	// 			expression(3, 4, [
	// 				expression_without_block(3, 4, [
	// 					literal(3, 4, [
	// 						literal_integer(3, 4)
	// 					])
	// 				])
	// 			]),
	// 			block(4, 7, [
	// 				b_curly_l(4, 5),
	// 				statements(5, 6, [
	// 					expression_without_block(5, 6, [
	// 						literal(5, 6, [
	// 							literal_integer(5, 6)
	// 						])
	// 					])
	// 				]),
	// 				b_curly_r(6, 7)
	// 			]),
	// 			WHITESPACE(7, 8),
	// 			if_expression_else(8, 26, [
	// 				kw_else(8, 12),
	// 				if_expression(12, 26, [
	// 					kw_if(12, 14),
	// 					WHITESPACE(14, 15),
	// 					expression(15, 16, [
	// 						expression_without_block(15, 16, [
	// 							literal(15, 16, [
	// 								literal_integer(15, 16)
	// 							])
	// 						])
	// 					]),
	// 					block(16, 20, [
	// 						b_curly_l(16, 17),
	// 						statements(17, 18, [
	// 							expression_without_block(17, 18, [
	// 								literal(17, 18, [
	// 									literal_integer(17, 18)
	// 								])
	// 							])
	// 						]),
	// 						b_curly_r(18, 19)
	// 					]),
	// 					if_expression_else(19, 26, [
	// 						kw_else(19, 23),
	// 						block(23, 26, [
	// 							b_curly_l(23, 24),
	// 							statements(24, 25, [
	// 								expression_without_block(24, 25, [
	// 									literal(24, 25, [
	// 										literal_integer(24, 25)
	// 									])
	// 								])
	// 							]),
	// 							b_curly_r(25, 26)
	// 						])
	// 					])
	// 				])
	// 			])
	// 		])
	// 	]
	// };
}
