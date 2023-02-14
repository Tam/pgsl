extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "pgsl.pest"]
pub struct PGSLParser;

fn main() {
	let unparsed_file = fs::read_to_string("test.pgl").expect("failed to read file");
	let file = PGSLParser::parse(Rule::pgsl, &unparsed_file)
		.expect("failed to parse")
		.next().unwrap();

	println!("{:?}", file);
}
