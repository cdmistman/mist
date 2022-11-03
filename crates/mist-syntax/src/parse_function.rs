use crate::parser::*;

impl<'source> Parser<'source> {
	/// ```ebnf
	/// FunctionItem =
	/// 	"fun" Ident FunctionSignature FunctionBody
	/// ```
	pub(crate) fn parse_function_item(&mut self) -> ParseResult {
		let ctx = SyntaxKind::Function;

		self.node(ctx, |p| {
			p.expect_token(ctx, T![fun])?;
			p.eat_trivia();
			p.expect_token(ctx, T!['ident])?;
			p.eat_trivia();
			p.parse_function_signature()?;
			p.eat_trivia();
			p.parse_function_body()
		})
	}
}

impl<'source> Parser<'source> {
	/// ```ebnf
	/// FunctionBody =
	/// 	BlockExpression
	/// 	| "=" Expression<true> ";"
	/// ```
	fn parse_function_body(&mut self) -> ParseResult {
		let ctx = SyntaxKind::FunctionBody;

		self.node(ctx, |p| match p.peek0.token {
			T!["{"] => p.parse_block_expression(),
			T![=] => {
				p.expect_token(ctx, T![=])?;
				p.eat_trivia();
				p.parse_expression::<true>()?;
				p.eat_trivia();
				p.expect_token(ctx, T![;])?;
				Ok(())
			},
			_ => Err(ParseError::ExpectedSet {
				parsing_ctx: ctx,
				actual:      p.peek0.to_static(),
				expected:    tokens!["{", =],
			}),
		})
	}

	/// ```ebnf
	/// FunctionSignature =
	/// 	FunctionParameters FunctionReturnType?
	///
	/// FunctionParameters = Pattern*
	///
	/// FunctionReturnType = "->" TypeSpec
	/// ```
	fn parse_function_signature(&mut self) -> ParseResult {
		let ctx = SyntaxKind::FunctionSignature;

		self.node(ctx, |p| {
			let checkpoint = p.checkpoint();
			let mut is_parsed = false;
			while p.try_parse_pattern()? {
				p.eat_trivia();
				is_parsed = true;
			}
			if is_parsed {
				p.insert_node_at(checkpoint, SyntaxKind::FunctionParameters);
			}

			let checkpoint = p.checkpoint();
			if p.eat_token(T![->]).is_some() {
				p.eat_trivia();
				p.node_at(
					checkpoint,
					SyntaxKind::FunctionReturnType,
					Self::parse_type_spec,
				)
			} else {
				Ok(())
			}
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[rstest]
	#[case("fun foo bar -> () = ();", Ok(()))]
	fn functions(#[case] input: &'static str, #[case] expect: ParseResult) {
		let actual = Parser::new(input).parse_function_item();
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
		assert_eq!(expect, actual, "expect != actual");
	}
}
