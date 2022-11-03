use std::path::PathBuf;

use mist_syntax::SyntaxNode;
use num::BigUint;

#[salsa::interned]
pub struct Ident {
	pub syntax: SyntaxNode,
}

// ===== Top Level =====

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Root {
	pub syntax: SyntaxNode,
	pub kind:   RootKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RootKind {
	File(InputFile, Module),
}

#[salsa::input]
pub struct InputFile {
	#[return_ref]
	pub path: PathBuf,

	#[return_ref]
	pub text: String,
}

// ===== Expression ======

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expression {
	pub syntax:   SyntaxNode,
	pub list:     Vec<ExpressionWithoutBlock>,
	pub trailing: Option<ExpressionWithBlock>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpressionWithBlock {
	pub syntax: SyntaxNode,
	pub kind:   ExpressionWithBlockKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionWithBlockKind {
	Block(Block),
	If(If),
	Match(Match),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpressionWithoutBlock {
	pub syntax: SyntaxNode,
	pub kind:   ExpressionWithoutBlockKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionWithoutBlockKind {
	Literal(Literal),
	Tuple(TupleExpression),
	Variable(Variable),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
	pub syntax:     SyntaxNode,
	pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If {
	pub syntax:      SyntaxNode,
	pub condition:   Box<Expression>,
	pub consequence: Block,
	pub or_else:     Option<Else>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Else {
	pub syntax: SyntaxNode,
	pub kind:   ElseKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElseKind {
	Block(Block),
	If(Box<If>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Literal {
	pub syntax: SyntaxNode,
	pub kind:   LiteralKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiteralKind {
	Bool(bool),
	Int(BigUint),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
	pub syntax: SyntaxNode,
	pub eval:   Box<Expression>,
	pub arms:   Vec<MatchArm>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchArm {
	pub syntax:  SyntaxNode,
	pub pattern: Pattern,
	pub eval:    Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TupleExpression {
	pub syntax: SyntaxNode,
	pub items:  Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
	pub syntax: SyntaxNode,
	pub name:   Ident,
}

// ===== Function =====

#[salsa::tracked]
pub struct Function {
	pub syntax: SyntaxNode,

	#[id]
	pub name: Ident,

	#[return_ref]
	pub signature: FunctionSignature,
	#[return_ref]
	pub body:      FunctionBody,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionBody {
	pub syntax: SyntaxNode,
	pub kind:   FunctionBodyKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FunctionBodyKind {
	Block(Block),
	Expression(Expression),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionSignature {
	pub syntax:      SyntaxNode,
	pub parameters:  Option<FunctionParameters>,
	pub return_type: Option<FunctionReturnType>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionParameters {
	pub syntax:     SyntaxNode,
	pub parameters: Vec<Pattern>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionReturnType {
	pub syntax:      SyntaxNode,
	pub return_type: TypeSpec,
}

// ===== Module =====

#[salsa::tracked]
pub struct Module {
	pub syntax: SyntaxNode,

	#[return_ref]
	pub items: Vec<ModuleItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModuleItem {
	Function(Function),
}

// ===== Pattern =====

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern {
	pub syntax: SyntaxNode,
	pub kind:   PatternKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PatternKind {
	Binding(BindingPattern),
	Constant(Literal),
	Tuple(TuplePattern),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BindingPattern {
	pub syntax:    SyntaxNode,
	pub ident:     Ident,
	pub type_spec: Option<TypeSpec>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TuplePattern {
	pub syntax: SyntaxNode,
	pub items:  Vec<Pattern>,
}

// ===== Statement =====

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement {
	pub syntax: SyntaxNode,
	pub kind:   StatementKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatementKind {
	Expression(ExpressionStatement),
	ImplicitReturn(ImplicitReturnStatement),
	LetStatement(LetStatement),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpressionStatement {
	pub syntax:     SyntaxNode,
	pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImplicitReturnStatement {
	pub syntax:     SyntaxNode,
	pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
	pub syntax:     SyntaxNode,
	pub binding:    Pattern,
	pub expression: Expression,
}

// ===== Type =====

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeSpec {
	pub syntax: SyntaxNode,
	pub kind:   TypeSpecKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeSpecKind {
	Primitive(PrimitiveType),
	Tuple(TupleTypeSpec),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
	Bool,
	Int,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TupleTypeSpec {
	pub syntax:     SyntaxNode,
	pub item_types: Vec<TypeSpec>,
}
