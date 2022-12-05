#[cfg(test)]
#[macro_use]
extern crate rstest;

#[cfg(test)]
mod tests;

ocaml::import!(
	fn it_works();
);
