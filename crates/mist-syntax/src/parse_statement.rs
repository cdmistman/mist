use crate::parser::*;

impl<'source> Parser<'source> {
	/// ```ebnf
	/// Statement =
	/// 	ExpressionStatement
	/// 	| ImplicitReturnStatement
	/// 	| LetStatement
	///
	/// ExpressionStatement =
	/// 	ExpressionWithBlock
	/// 	| Expression<true> ";"
	/// ```
	///
	/// Note: to keep parsing simple, any expressions not followed by a ";" are
	/// actually in a special `StatementOrExpression` node. It's up to the AST
	/// generator to disambiguate `ExpressionWithBlock`s and
	/// `ExpressionWithoutBlock<true>`s in this circumstance.
	pub(crate) fn try_parse_statement(&mut self) -> ParseResult<Option<bool>> {
		let checkpoint1 = self.checkpoint();
		let checkpoint2 = self.checkpoint();
		let ctx = SyntaxKind::Statement;

		let is_implicit_return = if self.peek0.is(T![let]) {
			self.parse_let_statement()?;
			false
		} else if self.try_parse_expression_with_block::<false>()? {
			let kind = SyntaxKind::ExpressionStatement;
			self.eat_trivia();
			self.eat_token(T![;]);
			self.insert_node_at(checkpoint2, kind);
			false
		} else if let Some(_) = self.try_parse_expression::<true>()? {
			self.eat_trivia();
			if let Some(_) = self.eat_token(T![;]) {
				self.insert_node_at(checkpoint2, SyntaxKind::ExpressionStatement);
				false
			} else {
				self.insert_node_at(checkpoint2, SyntaxKind::ImplicitReturnStatement);
				true
			}
		} else {
			return Ok(None);
		};

		self.insert_node_at(checkpoint1, ctx);
		Ok(Some(is_implicit_return))
	}
}

impl<'source> Parser<'source> {
	/// ```ebnf
	/// LetStatment =
	/// 	"let" Pattern "=" Expression<true> ";"
	/// ```
	fn parse_let_statement(&mut self) -> ParseResult {
		let ctx = SyntaxKind::LetStatement;

		self.node(ctx, |p| {
			p.expect_token(ctx, T![let])?;
			p.eat_trivia();
			p.parse_pattern()?;
			p.eat_trivia();
			p.expect_token(ctx, T![=])?;
			p.eat_trivia();
			p.parse_expression::<true>()?;
			p.expect_token(ctx, T![;])?;
			Ok(())
		})
	}
}
