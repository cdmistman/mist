use miette::SourceSpan;
use mist_types::*;
use string_interner::symbol::SymbolUsize;
use string_interner::DefaultBackend;
use string_interner::StringInterner;

#[test]
fn constrain_bool() {
	let mut strings = StringInterner::<DefaultBackend<SymbolUsize>>::new();
	let mut engine = Engine::default();

	let bool_type = engine.alloc_type(strings.get_or_intern("bool"), TypeKind::Boolean);
	let bool_instance = engine.instance(bool_type, SourceSpan::from(0..4));
	engine.expect_type(bool_instance, bool_type, SourceSpan::from(0..4));

	engine.type_check().unwrap();
}

#[test]
#[should_panic]
fn constrain_bool_to_int() {
	let mut strings = StringInterner::<DefaultBackend<SymbolUsize>>::new();
	let mut engine = Engine::default();

	let bool_type = engine.alloc_type(strings.get_or_intern("bool"), TypeKind::Boolean);
	let int_type = engine.alloc_type(strings.get_or_intern("int"), TypeKind::Integer {
		size:       None,
		signedness: Signedness::Signed,
	});
	let bool_instance = engine.instance(bool_type, SourceSpan::from(0..4));
	engine.expect_type(bool_instance, int_type, SourceSpan::from(0..4));

	engine.type_check().unwrap();
}
