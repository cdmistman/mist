%token L_CURLY "{" R_CURLY "}"
%token L_PAREN "(" R_PAREN ")"
%token EQ "="
%token SEMICOLON ";"
%token KW_ELSE "else"
%token KW_FALSE "false"
%token KW_FUN "fun"
%token KW_IF "if"
%token KW_LET "let"
%token KW_TRUE "true"
%token <bool> BOOL
%token <Ast.float_lit> FLOAT
%token <Ast.int_lit> INT
%token <string> IDENT
%token <string> STRING
%token EOF

%start <Ast.definition list> program
%%

let program := defs=definition*; EOF; <>

let identifier == IDENT

let literal ==
	| "false"; { `BoolLit false }
	| "true"; { `BoolLit true }
	| f=FLOAT; <`FloatLit>
	| i=INT; <`IntLit>
	| s=STRING; <`StringLit>

let pattern :=
	| var=identifier; <`BindPat>

let definition :=
	| "fun"; name=identifier; args=pattern*; body=function_body; <`FunDef>

let function_body ==
	| b=block; <`BlkBody>
	| "="; e=expression; <`ExpBody>

let block == "{"; s=statement*; "}"; <>

let statement :=
	| "let"; p=pattern; "="; e=expression; ";"; <`LetStm>
	| e=expression; ";"; <`ExpStm>

let expression :=
	| es=expression_item+; b=block; <`InvokeExp>
	| expression_no_invoke

let expression_no_invoke :=
	| es=expression_item+; <`CallExp>
	| "("; e=expression; ")"; <>
	| e=if_expression; <`IfExp>

let expression_item :=
	| l=literal; <`ExpLit>
	| v=identifier; <`ExpVar>

let if_expression ==
	"if"; cond=expression_no_invoke; consequence=block; e=if_else; <>

let if_else :=
	| "else"; i=if_expression; <`ElseIf>
	| "else"; b=block; <`Else>
	| { `NoElse }
