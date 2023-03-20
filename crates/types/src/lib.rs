mod engine;
mod types;

use std::hash::Hash;

use string_interner::symbol::SymbolUsize as Symbol;

#[derive(Default)]
pub struct Engine {
	types: Vec<Type>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct TypeId(u32);

pub struct Type {
	symbol:    Symbol,
	instances: Vec<Instance>,
}

pub struct InstanceId(u32);

pub struct Instance {
	type_id: TypeId,
}
