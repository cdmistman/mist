{
	open Lexing
	open Parser

	exception SyntaxError of string
}

rule read =
	parse
	| eof { EOF }