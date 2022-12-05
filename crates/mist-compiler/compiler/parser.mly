%token L_CURLY R_CURLY L_PAREN R_PAREN
%token KW_LET
%token KW_IF KW_ELSE
%token <bool> BOOL
%token <Ast.big_float> FLOAT
%token <Ast.big_int> INT
%token <string> IDENT
%token <string> STRING
%token EQ SEMICOLON

// %start <Ast.repl> repl
%start <Ast.exp> expression
%%

let ident == id=IDENT; <`Id>

let pattern :=
	| id=ident; <`BindingPat>

let literal :=
	| b=BOOL; <`BoolLit>
	| f=FLOAT; <`FloatLit>
	| i=INT; <`IntLit>
	| s=STRING; <`StringLit>

let block ==
	L_CURLY; ss=list(statement); R_CURLY; <>

let statement :=
	| KW_LET; binding=pattern; EQ; result=expression; SEMICOLON; <`LetStm>
	| e=expression_with_block; <`InlineStm>
	| e=expression; SEMICOLON; <`ExpStm>

let expression :=
	| items=nonempty_list(expression_item); b=block; <`InvokeExp>
	| expression_noinvoke

let expression_noinvoke :=
	| items=nonempty_list(expression_item); <`AppExp>
	| expression_with_block

let expression_with_block :=
	| e=if_expression; <`IfExp>

let expression_item :=
	| l=literal; <`LitExp>
	| i=ident; <`VarExp>
	| L_PAREN; e=expression; R_PAREN; <>

let if_expression :=
	KW_IF; condition=expression_noinvoke; consequence=block; else_=else_; <>

let else_ :=
	| KW_ELSE; b=block; <`Else>
	| KW_ELSE; i=if_expression; <`ElseIf>
	| { `NoElse }

// let block :=
// 	L_CURLY; ss=list(statement); R_CURLY; <`Block>

// let statement :=
// 	| KW_LET; binding=pattern; EQ; result=call; SEMICOLON; <`LetStm>
// 	| =expression_with_block; <`InlineStm>
// 	| =call; SEMICOLON; <`ExpStm>

// let call :=
// 	| =nonempty_list(expression_item); =block; <`CallExp>
// 	| expression

// let expression :=
// 	| =expression_with_block; <`ExpWithBlock>
// 	| =nonempty_list(expression_item); <`ExpWithoutBlock>

// let expression_with_block :=
// 	| =if_expression; <`IfExp>

// let expression_item :=
// 	| L_PAREN; =expression; R_PAREN; <`GroupExp>
// 	| =literal; { `LitExp }

// let if_else :=
// 	| KW_ELSE; b=block; <`Else>
// 	| KW_ELSE; i=if_expression; <`ElseIf>
// 	| { `NoElse }

// let if_expression :=
// 	condition=expression;
// 	consequence=block;
// 	otherwise=if_else;
// 	<>

// let literal :=
// 	| b=BOOL; { `BoolLit b }
// 	| i=INT; { `IntLit i }

////// ============
// repl :
// 	| e=expression { Exp e }
// 	| s=statement { Stm s }

// ident : i=IDENT { `Ident i }

// pat :
// 	| L_PAREN p=pat R_PAREN { `GroupPat p }
// 	| i=ident { `VarPat i }

// block :
// 	| L_CURLY ss=statement* c=option(c=call { c }) R_CURLY { `Block (ss, c) }

// statement :
// 	| KW_LET binding=pat EQ evaluate=call SEMICOLON {
// 		`LetStm (binding, evaluate)
// 	}
// 	| c=call SEMICOLON { `CallStm c }
// 	| e=expression_with_block { `InlineStm e }

// call :
// 	| es=nonempty_list(e=expression_without_block { e }) tail_fn=block {
// 		`Call (es, tail_fn)
// 	}
// 	| e=expression { `Exp e }


// %inline expression :
// 	| b=block { `BlockExp b }
// 	| e=expression_with_block { `WithBlock e }
// 	| e=nonempty_list(e=expression_without_block { e }) { `WithoutBlock e }

// %inline expression_with_block :
// 	| i=if_expression { `IfExp i }

// %inline expression_without_block :
// 	| L_PAREN c=call R_PAREN { `GroupExp c }
// 	| l=literal { `LitExp l }

// if_else :
// 	| KW_ELSE b=block { `Else b }
// 	| KW_ELSE i=if_expression { `ElseIf i }
// 	| { `NoElse }


// if_expression :
// 	KW_IF condition=expression
// 				consequence=block
// 				else_=if_else {
// 		(condition, consequence, else_)
// 	}

// literal :
// 	| KW_FALSE { `BoolLit false }
// 	| KW_TRUE { `BoolLit true }
// 	| i=INT { `IntLit i }


////// ============

// exp :


// braced_exp :
// 	KW_IF e=exp


// block :
// 	L_CURLY R_CURLY { `Block }

// expr_list :
// 	| es=nonempty_list(e=expr_item { e }) { `App es }

// expr_item :
// 	| L_PAREN e=expr_list R_PAREN { `Parend e }
// 	| b=block { `BlockExpr b }
// 	| l=lit { `Lit l }
// 	| i=IDENT { `Var i }
// 	| e=if_expr { `If e }

// if_expr :
// 	| KW_IF cond=expr_list consequence=block els=else_expr { (cond, consequence, els) }

// else_expr :
// 	| KW_ELSE i=if_expr { `ElseIf i }
// 	| KW_ELSE b=block { `Else b }
// 	| { `None }

// lit :
// 	| KW_FALSE { `Bool false }
// 	| KW_TRUE { `Bool true }
// 	| i=INT { `Int i }
