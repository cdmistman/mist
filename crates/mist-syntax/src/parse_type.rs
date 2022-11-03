use crate::parser::*;

static TYPE_SPEC_START: Lazy<TokenSet> = Lazy::new(|| tokens!["("]);

impl<'source> Parser<'source> {
	#[doc(alias = "try_parse_type_spec")]
	pub(crate) fn parse_type_spec(&mut self) -> ParseResult {
		if self.try_parse_type_spec()? {
			Ok(())
		} else {
			Err(ParseError::ExpectedSet {
				parsing_ctx: SyntaxKind::TypeSpec,
				actual:      self.peek0.to_static(),
				expected:    TYPE_SPEC_START.clone(),
			})
		}
	}

	/// ```ebnf
	/// TypeSpec =
	/// 	PrimitiveTypeSpec
	/// 	| TupleTypeSpec
	/// ```
	pub(crate) fn try_parse_type_spec(&mut self) -> ParseResult<bool> {
		let parse_fn = match self.peek0.token {
			T![bool] | T![int] => Self::parse_primitive_type,
			T!["("] => Self::parse_tuple_type,
			_ => return Ok(false),
		};

		self.node(SyntaxKind::TypeSpec, parse_fn)?;
		Ok(true)
	}
}

impl<'source> Parser<'source> {
	/// ```ebnf
	/// PrimitiveTypeSpec =
	/// 	"bool"
	/// 	| "int"
	/// ```
	fn parse_primitive_type(&mut self) -> ParseResult {
		let ctx = SyntaxKind::PrimitiveType;
		self.node(ctx, |p| p.expect_set(ctx, &tokens![bool, int]))?;
		Ok(())
	}

	/// ```ebnf
	/// TupleTypeSpec =
	/// 	"(" (TypeSpec ("," TypeSpec)*)? ","? ")"
	/// ```
	fn parse_tuple_type(&mut self) -> ParseResult {
		let ctx = SyntaxKind::TupleType;

		self.node(ctx, |p| {
			p.expect_token(ctx, T!["("])?;
			loop {
				p.eat_trivia();
				if !p.try_parse_type_spec()? {
					break;
				}

				p.eat_trivia();
				if p.eat_token(T![,]).is_none() {
					break;
				}
			}

			p.expect_token(ctx, T![")"])?;
			Ok(())
		})
	}
}
