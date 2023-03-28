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
#[error("can't constrain incompatible types")]
pub struct ConstraintError {
	pub left:  SourceSpan,
	pub right: SourceSpan,
}

impl Engine {
	pub fn type_check<B, H>(
		&self,
		symbols: &StringInterner<B, H>,
	) -> Result<(), Vec<TypeCheckError>>
	where
		B: StringInternerBackend,
		H: BuildHasher,
	{
		let failed_constraints = self
			.instances
			.iter()
			.flat_map(|instance| {
				instance
					.constraints
					.iter()
					.filter_map(|constraint| match constraint {
						Constraint::Expect { to_be_type, .. } if instance.type_ == *to_be_type => {
							None
						},
						Constraint::Expect { to_be_type, span } => {
							Some(TypeCheckError::Constraint(ConstraintError {
								left:  instance.span,
								right: *span,
							}))
						},
					})
			})
			.collect::<Vec<_>>();

		match failed_constraints.len() {
			0 => Ok(()),
			_ => Err(failed_constraints),
		}
	}
}
