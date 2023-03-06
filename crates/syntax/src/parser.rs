pub use pest::iterators::Pair;

#[cfg(test)]
mod tests;

#[derive(Debug, Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MistParser;

pub type ParseResult<T> = Result<T, String>;

pub trait Parse {
	fn parse<'i>(pair: Pair<'i, Rule>) -> ParseResult<Self>
	where
		Self: Sized;
}
