type big_float = Bignum.t
type big_int = Big_int.big_int

type id = [`Id of string]

type pat = [
  | `BindingPat of id
]

type lit = [
  | `BoolLit of bool
  | `FloatLit of big_float
  | `IntLit of big_int
  | `StringLit of string
]

type mod_def = [
  | `FileMod of mod_body
  | `InlineMod of id * mod_body
]

and mod_body = def list

and def = [
  | `FunDef of fun_def
]

and fun_def = id * pat list * fun_body

and fun_body = [
  | `BlockBody of block
  | `ExpBody of exp
]

and block = stm list

and stm = [
  | `ExpStm of exp
  | `InlineStm of exp
  | `LetStm of pat * exp
]

and exp = [
  | `AppExp of exp list
  | `IfExp of if_exp
  | `InvokeExp of exp list * block
  | `LitExp of lit
  | `VarExp of id
]

and if_exp = exp * block * else_kind

and else_kind = [
  | `Else of block
  | `ElseIf of if_exp
  | `NoElse
]

(* type ident = [`Ident of string]

and pat = [
  | `BindingPat of ident
  | `GroupPat of pat
]

and exp = [
  | `AppExp of app_exp
  | `WithBlockExp of exp_with_block
]

and exp_with_block = [
]

and app_exp = exp_without_block list

and exp_without_block = [
  | `GroupExp of exp
  | `LitExp of lit
  | `VarExp of ident
] *)

(* type ident = [`Ident of string]
and pat = [
  | `BindingPat of ident
  | `GroupPat of pat
]
and block = [`Block of stm list]
and stm = [
  | `ExpStm of exp
  | `InlineStm of exp
  | `LetStm of pat * exp
]
and exp = [
  | exp_noinvoke
  | invoke_exp
]
and exp_noinvoke = [#exp_with_block | #exp_without_block]
and exp_with_block = [
  | `IfExp of if_exp
]
and exp_without_block = [
  | #group_exp
  | `Foo
]
and group_exp = [ `GroupExp of exp ]
and if_exp = [`IfExp of exp]
and invoke_exp = [`InvokeExp of exp_without_block list * block] *)

(*
type ident = [`Ident of string]

and pat = [
  | `GroupPat of pat
  | `VarPat of ident
]

and block = [
  | `Block of stm list * call option
]

and stm = [
  | `CallStm of call
  | `InlineStm of exp_with_block
  | `LetStm of pat * call
]

and call = exp_item list * block option

and exp = [
  | `BlockExp of block
  | `WithBlock of exp_with_block
  | `WithoutBlock of exp_without_block list
]

and exp_with_block = [
  | `IfExp of if_exp
  | `CallExp of exp_item list * block option
]

and exp_item = [
  | `GroupExp of call
  | `LitExp of lit
]

and if_else = [
  | `Else of block
  | `ElseIf of if_exp
  | `NoElse
]

and if_exp = exp * block * if_else

and lit = [
  | `BoolLit of bool
  | `IntLit of big_int
]

type repl =
  | Exp of exp
  | Stm of stm *)





(* type block = [
  | `Block
]

and expr_list = [
  | `Exprs of expr list * block option
]

and expr = [
  | `App of expr list
  | `BlockExpr of block
  | `If of expr * block * else_expr
  | `Lit of lit
  | `Parend of expr
  | `Var of string
]

and else_expr = [
  | `ElseIf of expr * block * else_expr
  | `Else of block
  | `None
]

and lit = [
  | `Bool of bool
  | `Int of int64
] *)
