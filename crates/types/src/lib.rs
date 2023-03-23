#![feature(trait_alias)]

mod engine;
mod traits;
mod types;

use std::hash::BuildHasher;
use std::hash::Hash;

use string_interner::symbol::SymbolUsize as Symbol;

pub use self::engine::*;

pub type StringInterner<B, H> = string_interner::StringInterner<B, H>;
pub trait StringInternerBackend = string_interner::backend::Backend<Symbol = Symbol>;

#[derive(Default)]
pub struct Engine {
	traits: Vec<Trait>,
	types:  Vec<Type>,
	impls:  Vec<TraitImplementation>,

	constraints: Vec<Constraint>,
}

pub struct Constraint {
	left:  TypeId,
	right: TypeId,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct TraitId(u32);

#[derive(Debug)]
pub struct Trait {
	name: Symbol,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct TraitImplementationId(u32);

#[derive(Debug)]
pub struct TraitImplementation {
	trait_: TraitId,
	on:     TypeId,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct TypeId(u32);

#[derive(Debug, Clone)]
pub struct Type {
	name: Symbol,
	kind: TypeKind,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
	Boolean,
	Integer {
		/// The size of the integer in bits.
		///
		/// If `None`, the size is the native integer size (`usize` in Rust).
		size:       Option<u8>,
		signedness: Signedness,
	},
	Unit,
}

#[derive(Debug, Clone)]
pub enum Signedness {
	Signed,
	Unsigned,
}
