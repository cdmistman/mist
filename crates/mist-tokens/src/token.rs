#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
	// braces
	LCurly,
	RCurly,
	LParen,
	RParen,
	LSquare,
	RSquare,

	// symbols
	Arrow,
	Carat,
	Colon,
	Comma,
	Dot,
	Equal,
	Semicolon,
	Underscore,

	// keywords
	KwBool,
	KwElse,
	KwFalse,
	KwFun,
	KwIf,
	KwInt,
	KwLet,
	KwMatch,
	KwReturn,
	KwTrue,

	// user variable input
	Identifier,
	Integer,

	Whitespace,
	Eof,
	Error,
}
