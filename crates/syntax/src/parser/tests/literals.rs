use super::*;

#[test]
fn literal() {
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
fn literal_integer() {
	parses_to! {
		parser: MistParser,
		input: "0",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 1)]
	};

	parses_to! {
		parser: MistParser,
		input: "1",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 1)]
	};

	parses_to! {
		parser: MistParser,
		input: "10",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 2)]
	};

	parses_to! {
		parser: MistParser,
		input: "01",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 2)]
	};

	parses_to! {
		parser: MistParser,
		input: "0_",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 2)]
	};

	parses_to! {
		parser: MistParser,
		input: "01",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 2)]
	};

	parses_to! {
		parser: MistParser,
		input: "01_",
		rule: Rule::literal_integer,
		tokens: [literal_integer(0, 3)]
	};
}
