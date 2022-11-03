use crate::parser::*;

impl<'source> Parser<'source> {
	/// ```ebnf
	/// FileModule =
	/// 	ModuleItem*
	/// ```
	pub(crate) fn parse_file_module(&mut self) -> ParseResult {
		let ctx = SyntaxKind::FileModule;

		self.node(ctx, |p| {
			p.eat_trivia();
			p.parse_module_items()
		})
	}
}

impl<'source> Parser<'source> {
	/// ```ebnf
	/// ModuleItem =
	/// 	Function
	/// ```
	fn parse_module_items(&mut self) -> ParseResult {
		let ctx = SyntaxKind::ModuleItem;

		loop {
			let parse_fn = match self.peek0.token {
				T![fun] => Self::parse_function_item,
				T!['eof] => break Ok(()),
				t => unreachable!("unexpected token {t:?}"),
			};

			self.node(ctx, parse_fn)?;
			self.eat_trivia();
		}
	}
}
