use crate::parser::*;

pub static START_PATTERN: Lazy<TokenSet> = Lazy::new(|| tokens!["(", _, false, true, 'ident, 'int]);

impl<'source> Parser<'source> {
	#[doc(alias = "try_parse_pattern")]
	pub(crate) fn parse_pattern(&mut self) -> ParseResult {
		if self.try_parse_pattern()? {
			Ok(())
		} else {
			Err(ParseError::ExpectedSet {
				parsing_ctx: SyntaxKind::Pattern,
				actual:      self.peek0.to_static(),
				expected:    START_PATTERN.clone(),
			})
		}
	}

	/// ```ebnf
	/// Pattern =
	/// 	BindingPattern
	/// 	| ConstantPattern
	/// 	| TuplePattern
	/// ```
	pub(crate) fn try_parse_pattern(&mut self) -> ParseResult<bool> {
		let parse_fn = match self.peek0.token {
			T!["("] => Self::parse_pattern_tuple,
			T![_] | T!['ident] => Self::parse_pattern_binding,
			T![false] | T![true] | T!['int] => Self::parse_pattern_constant,
			_ => return Ok(false),
		};

		self.node(SyntaxKind::Pattern, parse_fn)?;
		Ok(true)
	}
}

impl<'source> Parser<'source> {
	/// ```ebnf
	/// BindingPattern =
	/// 	Ident (":" TypeSpec)
	/// ```
	fn parse_pattern_binding(&mut self) -> ParseResult {
		let ctx = SyntaxKind::PatternBinding;

		// TODO: destructors
		self.node(ctx, |p| {
			p.expect_set(ctx, &tokens![_, 'ident])?;
			p.eat_trivia();
			if p.eat_token(T![:]).is_some() {
				p.eat_trivia();
				p.parse_type_spec()
			} else {
				Ok(())
			}
		})
	}

	/// ```ebnf
	/// ConstantPattern =
	/// 	"false"
	/// 	| "true"
	/// 	| IntegerLiteral
	/// ```
	fn parse_pattern_constant(&mut self) -> ParseResult {
		let ctx = SyntaxKind::PatternConstant;
		self.node(ctx, |p| p.expect_set(ctx, &tokens![false, true, 'int]))?;
		Ok(())
	}

	/// ```ebnf
	/// TuplePattern =
	/// 	"(" (Pattern ("," Pattern)* ","?)? ")"
	/// ```
	fn parse_pattern_tuple(&mut self) -> ParseResult {
		let ctx = SyntaxKind::PatternTuple;
		self.node(ctx, |p| {
			p.expect_token(ctx, T!["("])?;
			loop {
				p.eat_trivia();
				if !p.try_parse_pattern()? {
					break;
				}
				p.eat_trivia();
				if !p.eat_token(T![,]).is_some() {
					break;
				}
			}

			p.expect_token(ctx, T![")"]).map(|_| ())
		})
	}
}
