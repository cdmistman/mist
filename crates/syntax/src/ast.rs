use std::ops::Range;

pub type Span = Range<usize>;

pub struct Binding {
	pub name:    Identifier,
	pub ty_spec: TyRef,
}

pub struct FnDef {
	pub name:   Identifier,
	pub params: FnParamList,
}

pub struct FnParamList {
	pub span:   Span,
	pub params: Vec<Binding>,
}

pub struct Item {
	pub span: Span,
	pub kind: ItemKind,
}

pub enum ItemKind {
	Fn(FnDef),
}

pub struct Identifier {
	pub span: Span,
	pub name: String,
}

pub struct TyRef {
	pub span: Span,
	pub kind: TyRefKind,
}

pub enum TyRefKind {
	Unit,
	Group(Box<TyRef>),
	Type(Identifier),
}
