mod value;

use mist_bytecode::Frame;
use mist_bytecode::Op;

pub use self::value::Value;

pub struct Vm {
	pub stack:     Vec<Value>,
	pub ip:        usize,
	pub code:      Vec<Op>,
	pub constants: Vec<Value>,
	pub globals:   Vec<Value>,
	pub locals:    Vec<Value>,
	pub frames:    Vec<Frame>,
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
	#[error("expected {expected}")]
	Expected { expected: &'static str },
}

impl Default for Vm {
	fn default() -> Self {
		Self {
			stack:     Vec::with_capacity(128),
			ip:        0,
			code:      Vec::new(),
			constants: Vec::new(),
			globals:   Vec::new(),
			locals:    Vec::new(),
			frames:    Vec::new(),
		}
	}
}

impl Vm {
	pub fn run(&mut self) -> Result<Option<Value>, RuntimeError> {
		loop {
			let op = match self.code.get(self.ip) {
				Some(op) => *op,
				None => return Ok(self.stack.pop()),
			};
			self.ip += 1;

			match op {
				Op::Const { index } => {
					let val = self.constants[index];
					self.stack.push(val);
				},
				Op::ConstValFalse => {
					self.stack.push(Value::Bool(false));
				},
				Op::ConstValTrue => {
					self.stack.push(Value::Bool(true));
				},

				Op::Dup => {
					let val = self.stack.last().unwrap();
					self.stack.push(*val);
				},
				Op::Pop { count } => {
					let _ = self.stack.drain(self.stack.len() - count as usize..);
				},

				Op::GetLocal { index } => {
					let val = self.locals[index];
					self.stack.push(val);
				},
				Op::SetLocal { index } => {
					let val = self.stack.pop().unwrap();
					self.locals[index] = val;
				},

				Op::Jump { len } => {
					self.ip = self.ip.overflowing_add(len as _).0;
				},
				Op::JumpIfFalse { len } => {
					let val = self.stack.pop().unwrap();
					if matches!(val, Value::Bool(false)) {
						self.ip = self.ip.overflowing_add(len as _).0;
					}
				},

				Op::Add => {
					let Value::Number(b) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					let Value::Number(a) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					self.stack.push(Value::Number(a + b));
				},
				Op::Div => {
					let Value::Number(b) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					let Value::Number(a) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					self.stack.push(Value::Number(a / b));
				},
				Op::Mul => {
					let Value::Number(b) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					let Value::Number(a) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					self.stack.push(Value::Number(a * b));
				},
				Op::Sub => {
					let Value::Number(b) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					let Value::Number(a) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					self.stack.push(Value::Number(a - b));
				},

				Op::Equal => {
					let b = self.stack.pop().unwrap();
					let a = self.stack.pop().unwrap();
					let result = match (a, b) {
						(Value::Number(a), Value::Number(b)) => a == b,
						(Value::Bool(a), Value::Bool(b)) => a == b,
						_ => false,
					};
					self.stack.push(Value::Bool(result));
				},
				Op::Greater => {
					let Value::Number(b) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					let Value::Number(a) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					self.stack.push(Value::Bool(a > b));
				},
				Op::GreaterEqual => {
					let Value::Number(b) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					let Value::Number(a) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					self.stack.push(Value::Bool(a >= b));
				},
				Op::Less => {
					let Value::Number(b) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					let Value::Number(a) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					self.stack.push(Value::Bool(a < b));
				},
				Op::LessEqual => {
					let Value::Number(b) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					let Value::Number(a) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "number",
						});
					};
					self.stack.push(Value::Bool(a <= b));
				},
				Op::NotEqual => {
					let b = self.stack.pop().unwrap();
					let a = self.stack.pop().unwrap();
					let result = match (a, b) {
						(Value::Number(a), Value::Number(b)) => a != b,
						(Value::Bool(a), Value::Bool(b)) => a != b,
						_ => false,
					};
					self.stack.push(Value::Bool(result));
				},

				Op::And => {
					let Value::Bool(b) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "bool",
						});
					};
					let Value::Bool(a) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "bool",
						});
					};
					self.stack.push(Value::Bool(a && b));
				},
				Op::Or => {
					let Value::Bool(b) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "bool",
						});
					};
					let Value::Bool(a) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "bool",
						});
					};
					self.stack.push(Value::Bool(a || b));
				},
				Op::Not => {
					let Value::Bool(a) = self.stack.pop().unwrap() else {
						return Err(RuntimeError::Expected {
							expected: "bool",
						});
					};
					self.stack.push(Value::Bool(!a));
				},
			}
		}
	}
}
