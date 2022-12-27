type float_lit = Q.t

and identifier = string

and int_lit = Z.t

and literal =
  [ `BoolLit of bool
  | `FloatLit of float_lit
  | `IntLit of int_lit
  | `StringLit of string
  ]

and pattern = [ `BindPat of identifier ]

and definition = [ `FunDef of identifier * pattern list * function_body ]

and function_body =
  [ `BlkBody of block
  | `ExpBody of expression
  ]

and block = statement list

and statement =
  [ `LetStm of pattern * expression
  | `ExpStm of expression
  ]

and expression =
  [ `CallExp of expression_item list
  | `IfExp of if_expression
  | `InvokeExp of expression_item list * block
  ]

and expression_item =
  [ `ExpLit of literal
  | `ExpVar of identifier
  ]

and if_expression = expression * block * if_else

and if_else =
  [ `Else of block
  | `ElseIf of if_expression
  | `NoElse
  ]
