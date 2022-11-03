use super::*;

const fn eof(input_len: usize) -> Tok<'static> {
	Tok {
		token: T!['eof],
		span:  Span {
			ch_start: input_len,
			ch_end:   input_len,
		},
		text:  Cow::<'static, str>::Borrowed(""),
		data:  None,
	}
}

#[rstest]
#[case(" ", vec![
	Tok { token: T!['whitespace], span: Span { ch_start: 0, ch_end: 1 }, text: " ".into(), data: None },
	eof(1),
])]
#[case("\t", vec![
	Tok { token: T!['whitespace], span: Span { ch_start: 0, ch_end: 1 }, text: "\t".into(), data: None },
	eof(1),
])]
#[case("\t \t", vec![
	Tok { token: T!['whitespace], span: Span { ch_start: 0, ch_end: 3 }, text: "\t \t".into(), data: None },
	eof(3),
])]
#[case("{", vec![
	Tok { token: T!["{"], span: Span { ch_start: 0, ch_end: 1 }, text: "{".into(), data: None },
	eof(1),
])]
#[case("}", vec![
	Tok { token: T!["}"], span: Span { ch_start: 0, ch_end: 1 }, text: "}".into(), data: None },
	eof(1),
])]
#[case("()", vec![
	Tok { token: T!["("], span: Span { ch_start: 0, ch_end: 1 }, text: "(".into(), data: None },
	Tok { token: T![")"], span: Span { ch_start: 1, ch_end: 2 }, text: ")".into(), data: None },
	eof(2),
])]
#[case(" \t[\t(\t]\t)\t ", vec![
	Tok { token: T!['whitespace], span: Span { ch_start: 0, ch_end: 2 }, text: " \t".into(), data: None },
	Tok { token: T!["["], span: Span { ch_start: 2, ch_end: 3 }, text: "[".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 3, ch_end: 4 }, text: "\t".into(), data: None },
	Tok { token: T!["("], span: Span { ch_start: 4, ch_end: 5 }, text: "(".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 5, ch_end: 6 }, text: "\t".into(), data: None },
	Tok { token: T!["]"], span: Span { ch_start: 6, ch_end: 7 }, text: "]".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 7, ch_end: 8 }, text: "\t".into(), data: None },
	Tok { token: T![")"], span: Span { ch_start: 8, ch_end: 9 }, text: ")".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 9, ch_end: 11 }, text: "\t ".into(), data: None },
	eof(11)
])]
#[case("->", vec![
	Tok { token: T![->], span: Span { ch_start: 0, ch_end: 2 }, text: "->".into(), data: None },
	eof(2)
])]
#[case("^", vec![
	Tok { token: T![^], span: Span { ch_start: 0, ch_end: 1 }, text: "^".into(), data: None },
	eof(1)
])]
#[case(":", vec![
	Tok { token: T![:], span: Span { ch_start: 0, ch_end: 1 }, text: ":".into(), data: None },
	eof(1)
])]
#[case(",", vec![
	Tok { token: T![,], span: Span { ch_start: 0, ch_end: 1 }, text: ",".into(), data: None },
	eof(1)
])]
#[case(".", vec![
	Tok { token: T![.], span: Span { ch_start: 0, ch_end: 1 }, text: ".".into(), data: None },
	eof(1)
])]
#[case("=", vec![
	Tok { token: T![=], span: Span { ch_start: 0, ch_end: 1 }, text: "=".into(), data: None },
	eof(1)
])]
#[case(";", vec![
	Tok { token: T![;], span: Span { ch_start: 0, ch_end: 1 }, text: ";".into(), data: None },
	eof(1)
])]
#[case("_,___", vec![
	Tok { token: T![_], span: Span { ch_start: 0, ch_end: 1 }, text: "_".into(), data: None },
	Tok { token: T![,], span: Span { ch_start: 1, ch_end: 2 }, text: ",".into(), data: None },
	Tok { token: T![_], span: Span { ch_start: 2, ch_end: 5 }, text: "___".into(), data: None },
	eof(5),
])]
#[case("bool _bool bool_", vec![
	Tok { token: T![bool], span: Span { ch_start: 0, ch_end: 4 }, text: "bool".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 4, ch_end: 5 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 5, ch_end: 10 }, text: "_bool".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 10, ch_end: 11 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 11, ch_end: 16 }, text: "bool_".into(), data: None },
	eof(16),
])]
#[case("else _else else_", vec![
	Tok { token: T![else], span: Span { ch_start: 0, ch_end: 4 }, text: "else".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 4, ch_end: 5 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 5, ch_end: 10 }, text: "_else".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 10, ch_end: 11 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 11, ch_end: 16 }, text: "else_".into(), data: None },
	eof(16),
])]
#[case("false _false false_", vec![
	Tok { token: T![false], span: Span { ch_start: 0, ch_end: 5 }, text: "false".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 5, ch_end: 6 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 6, ch_end: 12 }, text: "_false".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 12, ch_end: 13 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 13, ch_end: 19 }, text: "false_".into(), data: None },
	eof(19),
])]
#[case("fun _fun fun_", vec![
	Tok { token: T![fun], span: Span { ch_start: 0, ch_end: 3 }, text: "fun".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 3, ch_end: 4 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 4, ch_end: 8 }, text: "_fun".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 8, ch_end: 9 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 9, ch_end: 13 }, text: "fun_".into(), data: None },
	eof(13),
])]
#[case("if _if if_", vec![
	Tok { token: T![if], span: Span { ch_start: 0, ch_end: 2 }, text: "if".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 2, ch_end: 3 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 3, ch_end: 6 }, text: "_if".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 6, ch_end: 7 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 7, ch_end: 10 }, text: "if_".into(), data: None },
	eof(10),
])]
#[case("int _int int_", vec![
	Tok { token: T![int], span: Span { ch_start: 0, ch_end: 3 }, text: "int".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 3, ch_end: 4 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 4, ch_end: 8 }, text: "_int".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 8, ch_end: 9 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 9, ch_end: 13 }, text: "int_".into(), data: None },
	eof(13),
])]
#[case("let _let let_", vec![
	Tok { token: T![let], span: Span { ch_start: 0, ch_end: 3 }, text: "let".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 3, ch_end: 4 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 4, ch_end: 8 }, text: "_let".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 8, ch_end: 9 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 9, ch_end: 13 }, text: "let_".into(), data: None },
	eof(13),
])]
#[case("match _match match_", vec![
	Tok { token: T![match], span: Span { ch_start: 0, ch_end: 5 }, text: "match".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 5, ch_end: 6 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 6, ch_end: 12 }, text: "_match".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 12, ch_end: 13 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 13, ch_end: 19 }, text: "match_".into(), data: None },
	eof(19),
])]
#[case("return _return return_", vec![
	Tok { token: T![return], span: Span { ch_start: 0, ch_end: 6 }, text: "return".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 6, ch_end: 7 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 7, ch_end: 14 }, text: "_return".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 14, ch_end: 15 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 15, ch_end: 22 }, text: "return_".into(), data: None },
	eof(22),
])]
#[case("true _true true_", vec![
	Tok { token: T![true], span: Span { ch_start: 0, ch_end: 4 }, text: "true".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 4, ch_end: 5 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 5, ch_end: 10 }, text: "_true".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 10, ch_end: 11 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 11, ch_end: 16 }, text: "true_".into(), data: None },
	eof(16),
])]
#[case("main _foo foo'''", vec![
	Tok { token: T!['ident], span: Span { ch_start: 0, ch_end: 4 }, text: "main".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 4, ch_end: 5 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 5, ch_end: 9 }, text: "_foo".into(), data: None },
	Tok { token: T!['whitespace], span: Span { ch_start: 9, ch_end: 10 }, text: " ".into(), data: None },
	Tok { token: T!['ident], span: Span { ch_start: 10, ch_end: 16 }, text: "foo'''".into(), data: None },
	eof(16)
])]
#[case("0,1,20,21,100,101", vec![
	Tok { token: T!['int], span: Span { ch_start: 0, ch_end: 1 }, text: "0".into(), data: Some(TokenDatum::IntLit { n: 0usize.into(), base: 10 }) },
	Tok { token: T![,], span: Span { ch_start: 1, ch_end: 2 }, text: ",".into(), data: None },
	Tok { token: T!['int], span: Span { ch_start: 2, ch_end: 3 }, text: "1".into(), data: Some(TokenDatum::IntLit { n: 1usize.into(), base: 10 }) },
	Tok { token: T![,], span: Span { ch_start: 3, ch_end: 4 }, text: ",".into(), data: None },
	Tok { token: T!['int], span: Span { ch_start: 4, ch_end: 6 }, text: "20".into(), data: Some(TokenDatum::IntLit { n: 20usize.into(), base: 10 }) },
	Tok { token: T![,], span: Span { ch_start: 6, ch_end: 7 }, text: ",".into(), data: None },
	Tok { token: T!['int], span: Span { ch_start: 7, ch_end: 9 }, text: "21".into(), data: Some(TokenDatum::IntLit { n: 21usize.into(), base: 10 }) },
	Tok { token: T![,], span: Span { ch_start: 9, ch_end: 10 }, text: ",".into(), data: None },
	Tok { token: T!['int], span: Span { ch_start: 10, ch_end: 13 }, text: "100".into(), data: Some(TokenDatum::IntLit { n: 100usize.into(), base: 10 }) },
	Tok { token: T![,], span: Span { ch_start: 13, ch_end: 14 }, text: ",".into(), data: None },
	Tok { token: T!['int], span: Span { ch_start: 14, ch_end: 17 }, text: "101".into(), data: Some(TokenDatum::IntLit { n: 101usize.into(), base: 10 }) },
	eof(17),
])]
#[case("0b010,0o010,0x010", vec![
	Tok { token: T!['int], span: Span { ch_start: 0, ch_end: 5 }, text: "0b010".into(), data: Some(TokenDatum::IntLit { n: 2usize.into(), base: 2 }) },
	Tok { token: T![,], span: Span { ch_start: 5, ch_end: 6 }, text: ",".into(), data: None },
	Tok { token: T!['int], span: Span { ch_start: 6, ch_end: 11 }, text: "0o010".into(), data: Some(TokenDatum::IntLit { n: 8usize.into(), base: 8 }) },
	Tok { token: T![,], span: Span { ch_start: 11, ch_end: 12 }, text: ",".into(), data: None },
	Tok { token: T!['int], span: Span { ch_start: 12, ch_end: 17 }, text: "0x010".into(), data: Some(TokenDatum::IntLit { n: 16usize.into(), base: 16 }) },
	eof(17),
])]
fn lexing(#[case] input: &'static str, #[case] expect: Vec<Tok<'static>>) {
	let actual = Lexer::new(input).take(expect.len()).collect::<Vec<_>>();
	assert_eq!(expect, actual, "expect != actual")
}
