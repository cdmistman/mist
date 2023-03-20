use super::*;

impl Engine {
	pub fn get_type(&self, sym: Symbol) -> TypeId {
		self.types
			.iter()
			.position(|t| t.symbol == sym)
			.map(|i| TypeId(i as u32))
			.unwrap_or_default()
	}
}
