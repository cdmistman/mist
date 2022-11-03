use mist_tokens::Token;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum SyntaxKind {
	// Tokens - braces
	TokLCurly,
	TokRCurly,
	TokLParen,
	TokRParen,
	TokLSquare,
	TokRSquare,

	// Tokens - keywords
	TokKwBool,
	TokKwElse,
	TokKwFalse,
	TokKwFun,
	TokKwIf,
	TokKwInt,
	TokKwLet,
	TokKwMatch,
	TokKwReturn,
	TokKwTrue,

	// Tokens - symbols
	TokArrow,
	TokCarat,
	TokColon,
	TokComma,
	TokDot,
	TokEqual,
	TokSemicolon,
	TokUnderscore,

	// Tokens - user variable input
	TokIdentifier,
	TokInteger,
	TokWhitespace,

	// Expression nodes
	Expression,
	ExpressionItem,
	ExpressionWithBlock,
	ExpressionWithoutBlock,
	BlockExpression,
	IfExpression,
	ElseExpression,
	LiteralExpression,
	MatchExpression,
	MatchBlock,
	MatchArm,
	TupleExpression,
	VariableExpression,

	// Function nodes
	Function,
	FunctionBody,
	FunctionParameters,
	FunctionReturnType,
	FunctionSignature,

	// Module nodes
	FileModule,
	ModuleItem,

	// Pattern nodes
	Pattern,
	PatternBinding,
	PatternConstant,
	PatternTuple,

	// Statement nodes
	Statement,
	ExpressionStatement,
	ImplicitReturnStatement,
	LetStatement,

	// Type nodes
	TypeSpec,
	PrimitiveType,
	TupleType,

	Trivia,
	Root,
	Error,
}

impl From<Token> for SyntaxKind {
	fn from(value: Token) -> Self {
		match value {
			T!["{"] => Self::TokLCurly,
			T!["}"] => Self::TokRCurly,
			T!["("] => Self::TokLParen,
			T![")"] => Self::TokRParen,
			T!["["] => Self::TokLSquare,
			T!["]"] => Self::TokRSquare,

			T![->] => Self::TokArrow,
			T![^] => Self::TokCarat,
			T![:] => Self::TokColon,
			T![,] => Self::TokComma,
			T![.] => Self::TokDot,
			T![=] => Self::TokEqual,
			T![;] => Self::TokSemicolon,
			T![_] => Self::TokUnderscore,

			T![bool] => Self::TokKwBool,
			T![else] => Self::TokKwElse,
			T![false] => Self::TokKwFalse,
			T![fun] => Self::TokKwFun,
			T![if] => Self::TokKwIf,
			T![int] => Self::TokKwInt,
			T![let] => Self::TokKwLet,
			T![match] => Self::TokKwMatch,
			T![return] => Self::TokKwReturn,
			T![true] => Self::TokKwTrue,

			T!['ident] => Self::TokIdentifier,
			T!['int] => Self::TokInteger,
			T!['whitespace] => Self::TokWhitespace,

			Token::Eof | Token::Error => Self::Error,
		}
	}
}

impl From<rowan::SyntaxKind> for SyntaxKind {
	fn from(value: rowan::SyntaxKind) -> Self {
		assert!(value.0 < Self::Error as u16);
		unsafe { std::mem::transmute(value.0) }
	}
}

impl Into<rowan::SyntaxKind> for SyntaxKind {
	fn into(self) -> rowan::SyntaxKind {
		rowan::SyntaxKind(self as u16)
	}
}
