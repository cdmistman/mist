{
	open Lexing
	open Parser

	exception SyntaxError of string

	let string_of_char = String.make 1
}

let digit = ['0'-'9']
(* TODO: use xid rules *)
let letter = ['a'-'z' 'A'-'Z']
let newline = '\r' | '\n' | "\r\n"
let whitespace = [' ' '\t']+

let ident = letter ('_' | digit | letter)*

rule read =
	parse
	| whitespace { read lexbuf }
	| newline { Lexing.new_line lexbuf; read lexbuf }
	| '{' { L_CURLY }
	| '}' { R_CURLY }
	| '(' { L_PAREN }
	| ')' { R_PAREN }
	| '=' { EQ }
	| ';' { SEMICOLON }
	| "else" { KW_ELSE }
	| "false" { KW_FALSE }
	| "fun" { KW_FUN }
	| "if" { KW_IF }
	| "let" { KW_LET }
	| "true" { KW_TRUE }
	| ident { IDENT @@ Lexing.lexeme lexbuf }
	| digit+ { INT @@ int_of_string (Lexing.lexeme lexbuf) }
	| '"' { read_string (Lexing.lexeme_start lexbuf) (Buffer.create 8) lexbuf }
	| eof { EOF }
	| _ as ch { raise @@ SyntaxError ("unexpected character `" ^ string_of_char ch ^ "`") }

and read_string start_p buf =
	parse
	| '"' { STRING @@ Buffer.contents buf }
	| '\\' { read_char_escape start_p buf lexbuf }
	| [^ '"' '\\']+ {
		Lexing.lexeme lexbuf |> Buffer.add_string buf;
		read_string start_p buf lexbuf
	}
	| eof {
		let end_p = lexbuf.lex_curr_pos in
		raise @@ SyntaxError "unclosed string"
	}
	| _ as ch { raise @@ SyntaxError ("unhandled string char `" ^ string_of_char ch ^ "`") }

and read_char_escape start_p buf =
	parse
	| 'n' { Buffer.add_char buf '\n'; read_string start_p buf lexbuf }
	| 'r' { Buffer.add_char buf '\r'; read_string start_p buf lexbuf }
	| 't' { Buffer.add_char buf '\t'; read_string start_p buf lexbuf }
	| '\\' { Buffer.add_char buf '\\'; read_string start_p buf lexbuf }
	| '"' { Buffer.add_char buf '"'; read_string start_p buf lexbuf }
	| _ as ch { raise @@ SyntaxError "unhandled string escape sequence" }
