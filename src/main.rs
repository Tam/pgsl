extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "pgsl.pest"]
struct PGSLParser;

fn main() {
	debug();
}

fn debug() {
	let unparsed_file = fs::read_to_string("test.pgl")
		.expect("failed to read file");

	let file = PGSLParser::parse(Rule::pgsl, &unparsed_file)
		.expect("failed to parse")
		.next().unwrap();

	for line in file.into_inner() {
		debug_walk(line, 0);
	}
}

fn debug_walk(pair : Pair<Rule>, depth : usize) {
	if pair.as_rule() == Rule::EOI { return; }

	let rule = pair.as_rule();
	let value = pair.as_str();
	let mut pairs = pair.into_inner().peekable();

	if pairs.peek().is_some() {
		println!(
			"{:indent$}\x1b[96m{:?}\x1b[0m",
			"", rule,
			indent = depth * 2
		);
	} else {
		println!(
			"{:indent$}\x1b[36m{:?}\x1b[0m {}",
			"", rule, value,
			indent = depth * 2
		);
	}

	for line in pairs {
		debug_walk(line, depth + 1);
	}
}
