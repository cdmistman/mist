use super::*;

impl Engine {
	pub fn alloc_trait(&mut self, name: Symbol) -> TraitId {
		let id = TraitId(self.traits.len() as u32);
		self.traits.push(Trait { name });

		id
	}
}
