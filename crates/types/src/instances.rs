use super::*;

impl Engine {
	pub fn expect_type(&mut self, instance: InstanceId, to_be_type: TypeId, span: SourceSpan) {
		let instance = &mut self.instances[instance.0 as usize];
		instance
			.constraints
			.push(Constraint::Expect { to_be_type, span });
	}
}
