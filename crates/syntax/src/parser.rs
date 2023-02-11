#[cfg(test)]
mod tests;

#[derive(Debug, Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MistParser;
