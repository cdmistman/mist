pub mod ast;
pub mod parse_cst;

#[salsa::jar(db = Db)]
pub struct Jar(
	ast::Ident,
	ast::InputFile,
	ast::Function,
	ast::Module,
	parse_cst::parse_file,
);

pub trait Db: salsa::DbWithJar<Jar> {}
