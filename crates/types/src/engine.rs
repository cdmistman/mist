use miette::Diagnostic;
use thiserror::Error;

use super::*;

impl Engine {
	pub fn impl_trait(&mut self, on: TypeId, trait_: TraitId) -> TraitImplementationId {
		let trait_impl_id = TraitImplementationId(self.impls.len() as u32);
		self.impls.push(TraitImplementation { trait_, on });

		trait_impl_id
	}
}

#[derive(Debug, Diagnostic, Error)]
pub enum TypeCheckError {
	#[error(transparent)]
	Constraint(#[from] ConstraintError),
}

#[derive(Debug, Diagnostic, Error)]
#[error("can't constrain incompatible types {left} and {right}")]
pub struct ConstraintError {
	pub left:  String,
	pub right: String,
}

impl Engine {
	pub fn type_check<B, H>(&self, symbols: &StringInterner<B, H>) -> Result<(), TypeCheckError>
	where
		B: StringInternerBackend,
		H: BuildHasher,
	{
		for Constraint {
			left: TypeId(left),
			right: TypeId(right),
		} in &self.constraints
		{
			if left == right {
				continue;
			}

			let left_type = &self.types[*left as usize];
			let right_type = &self.types[*right as usize];

			match (&left_type.kind, &right_type.kind) {
				(TypeKind::Boolean, TypeKind::Boolean) | (TypeKind::Unit, TypeKind::Unit) => (),
				(TypeKind::Integer { .. }, TypeKind::Integer { .. }) => {},
				_ => {
					return Err(ConstraintError {
						left:  symbols.resolve(left_type.name).unwrap().to_string(),
						right: symbols.resolve(right_type.name).unwrap().to_string(),
					}
					.into())
				},
			}
		}

		Ok(())
	}
}
