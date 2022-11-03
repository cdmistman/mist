use mist_tokens::lexer::Lexer;
pub(crate) use mist_tokens::Tok;
pub(crate) use mist_tokens::Token;
pub(crate) use mist_tokens::TokenSet;
pub(crate) use once_cell::sync::Lazy;
pub(crate) use rowan::Checkpoint;
use rowan::GreenNodeBuilder;

pub(crate) use crate::syntax_kind::SyntaxKind;
use crate::SyntaxNode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
	ExpectedSet {
		parsing_ctx: SyntaxKind,
		actual:      Tok<'static>,
		expected:    TokenSet,
	},
	ExpectedToken {
		parsing_ctx: SyntaxKind,
		actual:      Tok<'static>,
		expected:    Token,
	},
}

pub type ParseResult<T = ()> = std::result::Result<T, ParseError>;

static TRIVIA: Lazy<TokenSet> = Lazy::new(|| tokens!['whitespace]);

pub struct Parser<'source> {
	builder: GreenNodeBuilder<'static>,
	lexer:   Lexer<'source>,

	pub(crate) peek0: Tok<'source>,
}

impl<'source> Parser<'source> {
	pub fn new(input: &'source str) -> Self {
		let builder = GreenNodeBuilder::new();
		let mut lexer = Lexer::new(input);
		let peek0 = lexer.next().unwrap();

		Self {
			builder,
			lexer,
			peek0,
		}
	}

	pub fn parse_file(mut self) -> ParseResult<SyntaxNode> {
		self.node(SyntaxKind::Root, Self::parse_file_module)?;

		Ok(SyntaxNode::new_root(self.builder.finish()))
	}
}

impl<'source> Parser<'source> {
	pub(crate) fn bump(&mut self) -> Tok<'source> {
		let new_peek = self.lexer.next().unwrap();
		let tok = std::mem::replace(&mut self.peek0, new_peek);
		if !tok.is_eof() {
			self.builder
				.token(SyntaxKind::from(tok.token).into(), tok.text.as_ref())
		}
		return tok;
	}
}

impl<'source> Parser<'source> {
	pub(crate) fn checkpoint(&mut self) -> Checkpoint {
		self.builder.checkpoint()
	}

	pub(crate) fn finish_node(&mut self) {
		self.builder.finish_node()
	}

	pub(crate) fn start_node(&mut self, kind: SyntaxKind) {
		self.builder.start_node(kind.into())
	}

	pub(crate) fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
		self.builder.start_node_at(checkpoint, kind.into())
	}

	pub(crate) fn node<T: Sized, F: FnOnce(&mut Self) -> ParseResult<T>>(
		&mut self,
		kind: SyntaxKind,
		f: F,
	) -> ParseResult<T> {
		self.start_node(kind);
		let res = f(self);
		self.finish_node();
		res
	}

	pub(crate) fn node_at<T: Sized, F: FnOnce(&mut Self) -> ParseResult<T>>(
		&mut self,
		checkpoint: Checkpoint,
		kind: SyntaxKind,
		f: F,
	) -> ParseResult<T> {
		self.start_node_at(checkpoint, kind);
		let res = f(self);
		self.finish_node();
		res
	}

	pub(crate) fn insert_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
		self.start_node_at(checkpoint, kind);
		self.finish_node();
	}
}

impl<'source> Parser<'source> {
	pub(crate) fn eat_set(&mut self, expected: &TokenSet) -> Option<Tok<'source>> {
		if expected.contains(&self.peek0.token) {
			Some(self.bump())
		} else {
			None
		}
	}

	pub(crate) fn eat_token(&mut self, expected: Token) -> Option<Tok<'source>> {
		if self.peek0.is(expected) {
			Some(self.bump())
		} else {
			None
		}
	}

	pub(crate) fn eat_trivia(&mut self) -> bool {
		let checkpoint = self.checkpoint();

		self.eat_set(&TRIVIA).is_some() && {
			self.start_node_at(checkpoint, SyntaxKind::Trivia);
			while self.eat_set(&TRIVIA).is_some() {}
			self.finish_node();
			true
		}
	}

	pub(crate) fn expect_set(
		&mut self,
		ctx: SyntaxKind,
		expected: &TokenSet,
	) -> ParseResult<Tok<'source>> {
		self.eat_set(expected)
			.ok_or_else(|| ParseError::ExpectedSet {
				parsing_ctx: ctx,
				actual:      self.peek0.to_static(),
				expected:    expected.clone(),
			})
	}

	pub(crate) fn expect_token(
		&mut self,
		ctx: SyntaxKind,
		expected: Token,
	) -> ParseResult<Tok<'source>> {
		self.eat_token(expected)
			.ok_or_else(|| ParseError::ExpectedToken {
				parsing_ctx: ctx,
				actual: self.peek0.to_static(),
				expected,
			})
	}
}
