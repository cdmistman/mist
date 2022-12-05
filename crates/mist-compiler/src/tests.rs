use ocaml::Runtime;

#[fixture]
fn rt() -> ocaml::Runtime {
	ocaml::init()
}

#[rstest]
fn it_works(rt: Runtime) {
	unsafe { super::it_works(&rt) }.unwrap()
}
