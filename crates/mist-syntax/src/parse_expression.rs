use crate::parse_pattern::START_PATTERN;
use crate::parser::*;

static START_EXPRESSION: Lazy<TokenSet> = Lazy::new(|| tokens![]);

impl<'source> Parser<'source> {
	#[doc(alias = "try_parse_expression")]
	pub(crate) fn parse_expression<const ACCEPT_BLOCK_ARG: bool>(&mut self) -> ParseResult<bool> {
		if let Some(has_block) = self.try_parse_expression::<ACCEPT_BLOCK_ARG>()? {
			Ok(has_block)
		} else {
			Err(ParseError::ExpectedSet {
				parsing_ctx: SyntaxKind::Expression,
				actual:      self.peek0.to_static(),
				expected:    START_EXPRESSION.clone(),
			})
		}
	}

	/// ```ebnf
	/// Expression<ACCEPT_BLOCK_ARG> =
	/// 	ExpressionWithoutBlock* ExpressionWithBlock<ACCEPT_BLOCK_ARG>?
	/// ```
	///
	/// # Returns
	/// - None if the expression failed to parse
	/// - Some(false) if the expression was parsed and didn't end with an
	///   ExpressionWithBlock
	/// - Some(true) if the expression was parsed and ended with an
	///   ExpressionWithBlock
	pub(crate) fn try_parse_expression<const ACCEPT_BLOCK_ARG: bool>(
		&mut self,
	) -> ParseResult<Option<bool>> {
		let checkpoint = self.checkpoint();

		let mut is_parsed = false;
		while self.try_parse_expression_without_block()? {
			is_parsed = true;
			self.eat_trivia();
		}

		let is_blocked = if is_parsed {
			self.try_parse_expression_with_block::<ACCEPT_BLOCK_ARG>()?
		} else {
			self.try_parse_expression_with_block::<true>()?
		};
		is_parsed |= is_blocked;

		if is_parsed {
			self.insert_node_at(checkpoint, SyntaxKind::Expression);
			Ok(Some(is_blocked))
		} else {
			Ok(None)
		}
	}

	/// ```ebnf
	/// BlockExpression =
	/// 	"{" Statement* "}"
	/// ```
	pub(crate) fn parse_block_expression(&mut self) -> ParseResult {
		let ctx = SyntaxKind::BlockExpression;
		self.node(ctx, |p| {
			p.expect_token(ctx, T!["{"])?;
			p.eat_trivia();
			while let Some(is_implicit_return) = p.try_parse_statement()? {
				p.eat_trivia();
				if is_implicit_return {
					break;
				}
			}
			p.expect_token(ctx, T!["}"])?;
			Ok(())
		})
	}

	/// ```ebnf
	/// ExpressionWithBlock<true> =
	/// 	BlockExpression
	/// 	| ExpressionWithBlock<false>
	///
	/// ExpressionWithBlock<false> =
	/// 	IfExpression
	/// 	| MatchExpression
	/// ```
	pub(crate) fn try_parse_expression_with_block<const ACCEPT_BLOCK_ARG: bool>(
		&mut self,
	) -> ParseResult<bool> {
		let parse_fn = match self.peek0.token {
			T!["{"] if ACCEPT_BLOCK_ARG => Self::parse_block_expression,
			T![if] => Self::parse_if_expression,
			T![match] => Self::parse_match_expression,
			_ => return Ok(false),
		};

		self.node(SyntaxKind::ExpressionWithBlock, parse_fn)?;
		Ok(true)
	}

	/// ```ebnf
	/// ExpressionWithouBlock =
	/// 	LiteralExpression
	/// 	| TupleExpression
	/// 	| VariableExpression
	/// ```
	pub(crate) fn try_parse_expression_without_block(&mut self) -> ParseResult<bool> {
		let parse_fn = match self.peek0.token {
			T!["("] => Self::parse_tuple_expression,
			T![false] | T![true] | T!['int] => Self::parse_literal_expression,
			T!['ident] => Self::parse_variable_expression,
			_ => return Ok(false),
		};

		self.node(SyntaxKind::ExpressionWithoutBlock, parse_fn)?;
		Ok(true)
	}
}

impl<'source> Parser<'source> {
	/// ```ebnf
	/// IfExpression =
	/// 	"if" Expression<false> BlockExpression ElseExpression?
	///
	/// ElseExpression =
	/// 	"else" BlockExpression
	/// 	| "else" IfExpression
	/// ```
	fn parse_if_expression(&mut self) -> ParseResult {
		let if_ctx = SyntaxKind::IfExpression;
		let else_ctx = SyntaxKind::ElseExpression;

		let mut nesting = 0;
		loop {
			self.start_node(if_ctx);
			nesting += 1;

			self.expect_token(if_ctx, T![if])?;
			self.eat_trivia();
			self.parse_expression::<false>()?;
			self.eat_trivia();
			self.parse_block_expression()?;
			self.eat_trivia();

			let checkpoint = self.checkpoint();
			if self.eat_token(T![else]).is_none() {
				break;
			}
			self.start_node_at(checkpoint, else_ctx);
			nesting += 1;

			self.eat_trivia();
			match self.peek0.token {
				T!["{"] => {
					self.parse_block_expression()?;
					break;
				},
				T![if] => continue,
				_ => {
					let actual = self.peek0.to_static();
					return Err(ParseError::ExpectedSet {
						parsing_ctx: else_ctx,
						actual,
						expected: tokens!["{", if],
					});
				},
			}
		}

		while nesting > 0 {
			self.finish_node();
			nesting -= 1;
		}
		Ok(())
	}

	/// ```ebnf
	/// LiteralExpression =
	/// 	"false"
	/// 	| "true"
	/// 	| IntegerLiteral
	///
	/// IntegerLiteral =
	/// 	("0".."9")+
	/// ```
	fn parse_literal_expression(&mut self) -> ParseResult {
		let ctx = SyntaxKind::LiteralExpression;
		self.node(ctx, |p| p.expect_set(ctx, &tokens![false, true, 'int]))?;
		Ok(())
	}

	/// ```ebnf
	/// MatchExpression =
	/// 	"match" Expression<false> "{" MatchArms "}"
	///
	/// MatchArms =
	/// 	(Pattern "->" (ExpressionWithBlock ","? | ExpressionWithoutBlock<true> ","))*
	/// 	Pattern "->" Expression<true>
	/// ```
	fn parse_match_expression(&mut self) -> ParseResult {
		let ctx = SyntaxKind::MatchExpression;
		self.node(ctx, |p| {
			p.expect_token(ctx, T![match])?;
			p.eat_trivia();
			p.parse_expression::<false>()?;
			p.eat_trivia();
			p.expect_token(ctx, T!["{"])?;

			let arm_ctx = SyntaxKind::MatchArm;
			let mut has_arm = false;
			loop {
				p.eat_trivia();

				let checkpoint = p.checkpoint();
				if !p.try_parse_pattern()? {
					if has_arm {
						break;
					} else {
						return Err(ParseError::ExpectedSet {
							parsing_ctx: arm_ctx,
							actual:      p.peek0.to_static(),
							expected:    START_PATTERN.clone(),
						});
					}
				}
				has_arm = true;

				let can_continue = p.node_at(checkpoint, arm_ctx, |p| {
					p.eat_trivia();
					p.expect_token(ctx, T![->])?;
					p.eat_trivia();

					if p.parse_expression::<true>()? {
						p.eat_trivia();
						p.eat_token(T![,]);
						Ok(true)
					} else {
						p.eat_trivia();
						Ok(p.expect_token(ctx, T![,]).is_ok())
					}
				})?;

				if !can_continue {
					break;
				}
			}

			p.expect_token(ctx, T!["}"])?;
			Ok(())
		})
	}

	/// ```ebnf
	/// TupleExpression =
	/// 	"(" (Expression<true> ("," Expression<true>)* ","?)? ")"
	/// ```
	fn parse_tuple_expression(&mut self) -> ParseResult {
		let ctx = SyntaxKind::TupleExpression;
		self.node(ctx, |p| {
			p.expect_token(ctx, T!["("])?;
			loop {
				p.eat_trivia();
				if let None = p.try_parse_expression::<true>()? {
					break;
				}
				p.eat_trivia();
				if let None = p.eat_token(T![,]) {
					break;
				}
			}
			p.expect_token(ctx, T![")"])?;
			Ok(())
		})
	}

	/// ```ebnf
	/// VariableExpression =
	/// 	Ident
	/// ```
	fn parse_variable_expression(&mut self) -> ParseResult {
		let ctx = SyntaxKind::VariableExpression;
		self.node(ctx, |p| p.expect_token(ctx, T!['ident]))?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[rstest]
	#[case("biggie'", Ok(false))]
	#[case("if true {}", Ok(true))]
	#[case("if true {} else {}", Ok(true))]
	#[case("if true {} else if { true } {} else {}", Ok(true))]
	#[case("false true 36", Ok(false))]
	#[case(
		"foo match false {
		bing: bool -> true,
		bong: bool -> match bong { true -> () () }
		blam: bool -> foo { true },
		boing: bool -> false
	}",
		Ok(true)
	)]
	#[case("(() ,true, false ,(()),)", Ok(false))]
	fn expressions_accepting_block_arg(
		#[case] input: &'static str,
		#[case] expect: ParseResult<bool>,
	) {
		let actual = Parser::new(input).parse_expression::<true>();
		if let Err(
			ParseError::ExpectedSet {
				actual: Tok { span, .. },
				..
			}
			| ParseError::ExpectedToken {
				actual: Tok { span, .. },
				..
			},
		) = &actual
		{
			let ctx_start = span.ch_start - 5;
			let ctx_end = span.ch_end + 5;
			println!("error in: `{}`", &input[ctx_start..ctx_end]);
		}
		assert_eq!(expect, actual, "expect != actual")
	}
}
