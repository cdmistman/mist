#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Op {
	// constants
	Const { index: usize },
	ConstValFalse,
	ConstValTrue,

	// stack manipulation
	Dup,
	Pop { count: u8 },

	// variables
	GetLocal { index: usize },
	SetLocal { index: usize },

	// jumps
	Jump { len: i32 },
	JumpIfFalse { len: i32 },

	// arithmetic
	Add,
	Div,
	Mul,
	Sub,

	// comparison
	Equal,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,
	NotEqual,

	// logical
	And,
	Not,
	Or,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Frame {
	pub code: Vec<Op>,
}
