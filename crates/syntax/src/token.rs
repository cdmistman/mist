#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Token {
	L_CURLY,
	R_CURLY,
	L_PAREN,
	R_PAREN,

	EQ,
	SEMICOLON,

	KW_CONST,
	KW_ELSE,
	KW_FALSE,
	KW_FUN,
	KW_IF,
	KW_LET,
	KW_TRUE,

	FLOAT,
	IDENT,
	INT,
	STRING,

	EOF,
	Error,
}
