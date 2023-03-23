use super::*;

impl Engine {
	pub fn alloc_type(&mut self, name: Symbol, kind: TypeKind) -> TypeId {
		let id = TypeId(self.types.len() as u32);
		self.types.push(Type { name, kind });

		id
	}

	pub fn add_constraint(&mut self, left: TypeId, right: TypeId) {
		self.constraints.push(Constraint { left, right });
	}
}
