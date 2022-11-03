use mist_syntax::ParseError;
use mist_syntax::Parser;
use mist_syntax::SyntaxKind;
use mist_syntax::SyntaxNode;
use num::BigUint;
use num::Num;

use crate::ast::*;
use crate::Db;

#[salsa::tracked]
pub fn parse_file(db: &dyn Db, input: InputFile) -> Result<Root, ParseError> {
	let cst = Parser::new(input.text(db).as_str()).parse_file()?;
	Ok(Root::file_module(db, input, cst).expect("invalid ast"))
}

impl Root {
	pub fn file_module(db: &dyn Db, input: InputFile, syntax: SyntaxNode) -> Option<Self> {
		let kind = match syntax.kind() {
			SyntaxKind::Root => {
				if let Some(module) = syntax.children().find_map(|c| Module::parse(db, c)) {
					RootKind::File(input, module)
				} else {
					panic!("not a file module")
				}
			},
			_ => return None,
		};
		Some(Root { syntax, kind })
	}
}

pub trait Ast: Sized {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self>;
}

impl Ast for Ident {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::TokIdentifier => Some(Ident::new(db, syntax)),
			_ => None,
		}
	}
}

// ===== Expression =====

impl Ast for Expression {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::Expression => {
				let list = syntax
					.children()
					.filter_map(|c| ExpressionWithoutBlock::parse(db, c))
					.collect();
				let trailing = syntax
					.children()
					.find_map(|c| ExpressionWithBlock::parse(db, c));
				Some(Expression {
					syntax,
					list,
					trailing,
				})
			},
			_ => None,
		}
	}
}

impl Ast for Block {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::BlockExpression => {
				let statements = syntax
					.children()
					.filter_map(|c| Statement::parse(db, c))
					.collect();
				Some(Block { syntax, statements })
			},
			_ => None,
		}
	}
}

impl Ast for ExpressionWithBlock {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::ExpressionWithBlock => {
				let kind = syntax
					.children()
					.find_map(|c| Block::parse(db, c))
					.map(ExpressionWithBlockKind::Block)
					.or_else(|| {
						syntax
							.children()
							.find_map(|c| If::parse(db, c))
							.map(ExpressionWithBlockKind::If)
					})
					.or_else(|| {
						syntax
							.children()
							.find_map(|c| Match::parse(db, c))
							.map(ExpressionWithBlockKind::Match)
					})
					.expect("invalid expression with block");

				Some(ExpressionWithBlock { syntax, kind })
			},
			_ => None,
		}
	}
}

impl Ast for ExpressionWithoutBlock {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::ExpressionWithoutBlock => {
				let kind = syntax
					.children()
					.find_map(|c| Literal::parse(db, c))
					.map(ExpressionWithoutBlockKind::Literal)
					.or_else(|| {
						syntax
							.children()
							.find_map(|c| TupleExpression::parse(db, c))
							.map(ExpressionWithoutBlockKind::Tuple)
					})
					.or_else(|| {
						syntax
							.children()
							.find_map(|c| Variable::parse(db, c))
							.map(ExpressionWithoutBlockKind::Variable)
					})
					.expect("invalid expression without block");

				Some(ExpressionWithoutBlock { syntax, kind })
			},
			_ => None,
		}
	}
}

impl Ast for If {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::IfExpression => {
				let condition = Box::new(
					syntax
						.children()
						.find_map(|c| Expression::parse(db, c))
						.expect("no condition in if expression"),
				);
				let consequence = syntax
					.children()
					.find_map(|c| Block::parse(db, c))
					.expect("no consequence in if expression");
				let or_else = syntax.children().find_map(|c| Else::parse(db, c));
				Some(If {
					syntax,
					condition,
					consequence,
					or_else,
				})
			},
			_ => None,
		}
	}
}

impl Ast for Else {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::ElseExpression => {
				let kind = syntax
					.children()
					.find_map(|c| Block::parse(db, c))
					.map(ElseKind::Block)
					.or_else(|| {
						syntax
							.children()
							.find_map(|c| If::parse(db, c))
							.map(Box::new)
							.map(ElseKind::If)
					})
					.expect("invalid else expression");
				Some(Else { syntax, kind })
			},
			_ => None,
		}
	}
}

impl Ast for Literal {
	fn parse(_: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::LiteralExpression => {
				let child = syntax
					.children()
					.find(|c| {
						matches!(
							c.kind(),
							SyntaxKind::TokKwFalse | SyntaxKind::TokKwTrue | SyntaxKind::TokInteger
						)
					})
					.expect("no literal");
				let kind = match child.kind() {
					SyntaxKind::TokKwFalse => LiteralKind::Bool(false),
					SyntaxKind::TokKwTrue => LiteralKind::Bool(true),
					SyntaxKind::TokInteger => LiteralKind::Int({
						let token = child.first_token()?;
						BigUint::from_str_radix(token.text(), 10).unwrap()
					}),
					k => unreachable!("unexpected literal kind {k:?}"),
				};
				Some(Literal { syntax, kind })
			},
			_ => None,
		}
	}
}

impl Ast for Match {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::MatchExpression => {
				let eval = Box::new(
					syntax
						.children()
						.find_map(|c| Expression::parse(db, c))
						.expect("no expression in match expression"),
				);
				let arms = syntax
					.children()
					.filter_map(|c| MatchArm::parse(db, c))
					.collect();
				Some(Match { syntax, eval, arms })
			},
			_ => None,
		}
	}
}

impl Ast for MatchArm {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::MatchArm => {
				let pattern = syntax
					.children()
					.find_map(|c| Pattern::parse(db, c))
					.expect("no pattern in match arm");
				let eval = syntax
					.children()
					.find_map(|c| Expression::parse(db, c))
					.expect("no expression in match arm");
				Some(MatchArm {
					syntax,
					pattern,
					eval,
				})
			},
			_ => None,
		}
	}
}

impl Ast for TupleExpression {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::TupleExpression => {
				let items = syntax
					.children()
					.filter_map(|c| Expression::parse(db, c))
					.collect();
				Some(TupleExpression { syntax, items })
			},
			_ => None,
		}
	}
}

impl Ast for Variable {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::VariableExpression => {
				let name = syntax
					.children()
					.find_map(|c| Ident::parse(db, c))
					.expect("no Ident in variable expression");
				Some(Variable { syntax, name })
			},
			_ => None,
		}
	}
}

// ===== Function =====

impl Ast for Function {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::Function => {
				let name = syntax
					.children()
					.find_map(|c| Ident::parse(db, c))
					.expect("no name for function");
				let signature = syntax
					.children()
					.find_map(|c| FunctionSignature::parse(db, c))
					.expect("no signature for function");
				let body = syntax
					.children()
					.find_map(|c| FunctionBody::parse(db, c))
					.expect("no body for function");
				Some(Function::new(db, syntax, name, signature, body))
			},
			_ => None,
		}
	}
}

impl Ast for FunctionBody {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::FunctionBody => {
				let kind = syntax
					.children()
					.find_map(|c| Block::parse(db, c))
					.map(FunctionBodyKind::Block)
					.or_else(|| {
						syntax
							.children()
							.find_map(|c| Expression::parse(db, c))
							.map(FunctionBodyKind::Expression)
					})
					.expect("invalid function body form");
				Some(FunctionBody { syntax, kind })
			},
			_ => None,
		}
	}
}

impl Ast for FunctionParameters {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::FunctionParameters => {
				let parameters = syntax
					.children()
					.filter_map(|c| Pattern::parse(db, c))
					.collect();
				Some(FunctionParameters { syntax, parameters })
			},
			_ => None,
		}
	}
}

impl Ast for FunctionReturnType {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::FunctionReturnType => {
				let return_type = syntax
					.children()
					.find_map(|c| TypeSpec::parse(db, c))
					.expect("no TypeSpec in FunctionReturnType");
				Some(FunctionReturnType {
					syntax,
					return_type,
				})
			},
			_ => None,
		}
	}
}

impl Ast for FunctionSignature {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::FunctionSignature => {
				let parameters = syntax
					.children()
					.find_map(|c| FunctionParameters::parse(db, c));
				let return_type = syntax
					.children()
					.find_map(|c| FunctionReturnType::parse(db, c));
				Some(FunctionSignature {
					syntax,
					parameters,
					return_type,
				})
			},
			_ => None,
		}
	}
}

// ===== Module =====
impl Ast for Module {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::FileModule => {
				let items = syntax
					.children()
					.filter_map(|c| ModuleItem::parse(db, c))
					.collect();
				Some(Module::new(db, syntax, items))
			},
			_ => None,
		}
	}
}

impl Ast for ModuleItem {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::ModuleItem => Some(
				syntax
					.children()
					.find_map(|c| Function::parse(db, c))
					.map(ModuleItem::Function)
					.expect("invalid module item form"),
			),
			_ => None,
		}
	}
}

// ===== Pattern =====

impl Ast for Pattern {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::Pattern => {
				let kind = syntax
					.children()
					.find_map(|c| BindingPattern::parse(db, c))
					.map(PatternKind::Binding)
					.or_else(|| {
						syntax
							.children()
							.find_map(|c| Literal::parse(db, c))
							.map(PatternKind::Constant)
					})
					.or_else(|| {
						syntax
							.children()
							.find_map(|c| TuplePattern::parse(db, c))
							.map(PatternKind::Tuple)
					})
					.expect("invalid pattern form");
				Some(Pattern { syntax, kind })
			},
			_ => None,
		}
	}
}

impl Ast for BindingPattern {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::PatternBinding => {
				let ident = syntax
					.children()
					.find_map(|c| Ident::parse(db, c))
					.expect("no variable to bind to in PatternBinding");
				let type_spec = syntax.children().find_map(|c| TypeSpec::parse(db, c));
				Some(BindingPattern {
					syntax,
					ident,
					type_spec,
				})
			},
			_ => None,
		}
	}
}

impl Ast for TuplePattern {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::PatternTuple => {
				let items = syntax
					.children()
					.filter_map(|c| Pattern::parse(db, c))
					.collect();
				Some(TuplePattern { syntax, items })
			},
			_ => None,
		}
	}
}

// ===== Statement =====

impl Ast for Statement {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::Statement => {
				let kind = syntax
					.children()
					.find_map(|c| ExpressionStatement::parse(db, c))
					.map(StatementKind::Expression)
					.or_else(|| {
						syntax
							.children()
							.find_map(|c| ImplicitReturnStatement::parse(db, c))
							.map(StatementKind::ImplicitReturn)
					})
					.or_else(|| {
						syntax
							.children()
							.find_map(|c| LetStatement::parse(db, c))
							.map(StatementKind::LetStatement)
					})
					.expect("invalid statement form");
				Some(Statement { syntax, kind })
			},
			_ => None,
		}
	}
}

impl Ast for ExpressionStatement {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::ExpressionStatement => {
				let expression = syntax
					.children()
					.find_map(|c| Expression::parse(db, c))
					.expect("no expression in expression statement");
				Some(ExpressionStatement { syntax, expression })
			},
			_ => None,
		}
	}
}

impl Ast for ImplicitReturnStatement {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::ImplicitReturnStatement => {
				let expression = syntax
					.children()
					.find_map(|c| Expression::parse(db, c))
					.expect("no expression in implicit return statement");
				Some(ImplicitReturnStatement { syntax, expression })
			},
			_ => None,
		}
	}
}

impl Ast for LetStatement {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::LetStatement => {
				let binding = syntax
					.children()
					.find_map(|c| Pattern::parse(db, c))
					.expect("no binding in Let statement");
				let expression = syntax
					.children()
					.find_map(|c| Expression::parse(db, c))
					.expect("no expression in Let statement");
				Some(LetStatement {
					syntax,
					binding,
					expression,
				})
			},
			_ => None,
		}
	}
}

// ===== Type =====

impl Ast for TypeSpec {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::TypeSpec => {
				let kind = syntax
					.children()
					.find_map(|c| PrimitiveType::parse(db, c))
					.map(TypeSpecKind::Primitive)
					.or_else(|| {
						syntax
							.children()
							.find_map(|c| TupleTypeSpec::parse(db, c))
							.map(TypeSpecKind::Tuple)
					})
					.expect("invalid type spec form");
				Some(TypeSpec { syntax, kind })
			},
			_ => None,
		}
	}
}

impl Ast for PrimitiveType {
	fn parse(_: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::PrimitiveType => {
				let child = syntax.first_child().expect("no primitive type");
				Some(match child.kind() {
					SyntaxKind::TokKwBool => PrimitiveType::Bool,
					SyntaxKind::TokInteger => PrimitiveType::Int,
					k => panic!("unexpected primitive type {k:?}"),
				})
			},
			_ => None,
		}
	}
}

impl Ast for TupleTypeSpec {
	fn parse(db: &dyn Db, syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			SyntaxKind::TupleType => {
				let item_types = syntax
					.children()
					.filter_map(|c| TypeSpec::parse(db, c))
					.collect();
				Some(TupleTypeSpec { syntax, item_types })
			},
			_ => None,
		}
	}
}
