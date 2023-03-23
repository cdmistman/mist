use mist_types::*;
use string_interner::symbol::SymbolUsize;
use string_interner::DefaultBackend;
use string_interner::StringInterner;

type TypeCheckResult = Result<(), TypeCheckError>;

#[test]
fn bools() -> TypeCheckResult {
	let mut strings = StringInterner::<DefaultBackend<SymbolUsize>>::new();
	let mut engine = Engine::default();

	let bool_type = engine.alloc_type(strings.get_or_intern("bool"), TypeKind::Boolean);
	engine.add_constraint(bool_type, bool_type);

	engine.type_check(&strings)
}
