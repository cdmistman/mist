use super::*;

#[test]
fn kw_false() {
	parses_to! {
		parser: MistParser,
		input: "false",
		rule: Rule::literal,
		tokens: [
			literal(0, 5, [
				kw_false(0, 5)
			])
		]
	};
}

#[test]
fn kw_true() {
	parses_to! {
		parser: MistParser,
		input: "true",
		rule: Rule::literal,
		tokens: [
			literal(0, 4, [
				kw_true(0, 4)
			])
		]
	};
}

#[test]
fn literal() {
	parses_to! {
		parser: MistParser,
		input: "0",
		rule: Rule::literal,
		tokens: [
			literal(0, 1, [
				literal_integer(0, 1)
			])
		]
	}
}

#[test]
fn literal_integer_0() {
	parses_to! {
		parser: MistParser,
		input: "0",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 1)]
	};
}

#[test]
fn literal_integer_1() {
	parses_to! {
		parser: MistParser,
		input: "1",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 1)]
	};
}

#[test]
fn literal_integer_10() {
	parses_to! {
		parser: MistParser,
		input: "10",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 2)]
	};
}

#[test]
fn literal_integer_01() {
	parses_to! {
		parser: MistParser,
		input: "01",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 2)]
	};
}

#[test]
fn literal_integer_0_() {
	parses_to! {
		parser: MistParser,
		input: "0_",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 2)]
	};
}

#[test]
fn literal_integer_01_() {
	parses_to! {
		parser: MistParser,
		input: "01_",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 3)]
	};
}
