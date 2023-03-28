use super::*;

impl Engine {
	pub fn alloc_type(&mut self, name: Symbol, kind: TypeKind) -> TypeId {
		let id = TypeId(self.types.len() as u32);
		self.types.push(Type { name, kind });

		id
	}

	pub fn instance(&mut self, type_: TypeId, span: SourceSpan) -> InstanceId {
		let instance = Instance {
			type_,
			span,
			constraints: Vec::new(),
		};

		let instance_id = InstanceId(self.instances.len() as u32);
		self.instances.push(instance);

		instance_id
	}
}
