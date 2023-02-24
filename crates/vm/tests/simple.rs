#[macro_use]
extern crate pretty_assertions;

use mist_bytecode::Op;
use mist_vm::Value;
use mist_vm::Vm;

#[test]
fn chained_if() {
	let mut vm = Vm::default();
	vm.constants = vec![Value::Number(0), Value::Number(1), Value::Number(2)];
	vm.code = vec![
		// if false { 0 }
		Op::ConstValFalse,
		Op::JumpIfFalse { len: 2 },
		Op::Const { index: 0 },
		Op::Jump { len: 13 },
		// else if false && true { 1 }
		Op::ConstValFalse,
		Op::ConstValTrue,
		Op::And,
		Op::JumpIfFalse { len: 2 },
		Op::Const { index: 1 },
		Op::Jump { len: 7 },
		// else if false || true { 2 }
		Op::ConstValFalse,
		Op::ConstValTrue,
		Op::Or,
		Op::JumpIfFalse { len: 2 },
		Op::Const { index: 2 },
		Op::Jump { len: 1 },
		// else { 3 }
		Op::Const { index: 3 },
	];
	let result = vm.run().expect("runtime error");
	assert_eq!(result, Some(Value::Number(2)));
}
