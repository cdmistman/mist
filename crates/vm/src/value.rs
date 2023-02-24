#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
	Bool(bool),
	Number(u64),
}
