fn main() {
	// Without this rerun directive, cargo only checks the *rust* source files.
	// That means when the compiler source changes it doesn't trigger a rebuild,
	// causing things to break.
	println!("cargo:rerun-if-changed=.");
	ocaml_build::Dune::new("crates/mist-compiler/compiler")
		.with_root("../..")
		.build();
}
