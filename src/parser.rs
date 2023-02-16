use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use pest::iterators::Pair;
use pest::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
#[grammar = "pgsl.pest"]
struct PGSLParser;

#[derive(Debug, Default)]
pub struct PGSLColumn {
	pub name : String,
	pub type_name : String,
	pub attributes : String,
	pub comments : Vec<String>,
}

#[derive(Debug, Default)]
pub struct PGSLTable {
	pub name : String,
	pub schema : String,
	pub extends : Vec<String>,
	pub columns : Vec<PGSLColumn>,
}

#[derive(Debug, Default)]
pub struct PGSLInterface {
	pub name : String,
	pub columns : Vec<PGSLColumn>,
}

#[derive(Debug, Default)]
pub struct PGSLData {
	pub requires : Vec<PathBuf>,
	pub interfaces : HashMap<String, PGSLInterface>,
	pub tables : Vec<PGSLTable>,
}

/// Parses the files at the given path into PGSLData
pub fn parse (path : PathBuf) -> Result<PGSLData> {
	let path_as_str = path.display().to_string();
	let unparsed_file = fs::read_to_string(path)
		.with_context(|| format!("Unable to read file {path_as_str}"))?;

	let file = PGSLParser::parse(Rule::pgsl, &unparsed_file)
		.with_context(|| format!("Failed to parse {path_as_str}"))?
		.next().unwrap();

	let mut data = PGSLData::default();

	for line in file.into_inner() {
		match line.as_rule() {
			Rule::EOI => (),
			Rule::require => {
				let mut requires = parse_require(line);
				data.requires.append(&mut requires);
			},
			Rule::interface => {
				let interface = parse_interface(line);
				data.interfaces.insert(interface.name.clone(), interface);
			},
			Rule::table => {
				let table = parse_table(line);
				data.tables.push(table);
			},
			_ => unreachable!(),
		}
	}

	Ok(data)
}

/// Parses the require rule
fn parse_require (lines : Pair<Rule>) -> Vec<PathBuf> {
	let mut requires = Vec::new();

	for line in lines.into_inner() {
		if line.as_rule() == Rule::path {
			requires.push(PathBuf::from(line.as_str()));
		} else { unreachable!() }
	}

	requires
}

/// Parses the interface rule
fn parse_interface (lines : Pair<Rule>) -> PGSLInterface {
	let mut interface = PGSLInterface::default();

	for line in lines.into_inner() {
		match line.as_rule() {
			Rule::interface_name => interface.name = line.as_str().to_string(),
			Rule::columns => interface.columns = parse_columns(line),
			_ => unreachable!(),
		}
	}

	interface
}

/// Parses the table rule
fn parse_table (lines : Pair<Rule>) -> PGSLTable {
	let mut table = PGSLTable::default();

	for line in lines.into_inner() {
		match line.as_rule() {
			Rule::schema_name => table.schema = line.as_str().to_string(),
			Rule::table_name => table.name = line.as_str().to_string(),
			Rule::columns => table.columns = parse_columns(line),
			Rule::extends => table.extends = parse_extends(line),
			_ => unreachable!(),
		}
	}

	table
}

/// Parses the extends rule
fn parse_extends (lines : Pair<Rule>) -> Vec<String> {
	let mut names = Vec::new();

	for line in lines.into_inner() {
		if line.as_rule() == Rule::interface_name {
			names.push(line.as_str().to_string());
		} else { unreachable!() }
	}

	names
}

/// Parses the columns rule
fn parse_columns (lines : Pair<Rule>) -> Vec<PGSLColumn> {
	let mut columns = Vec::new();

	for line in lines.into_inner() {
		if line.as_rule() == Rule::column {
			columns.push(parse_column(line));
		} else { unreachable!() }
	}

	columns
}

/// Parses a single column
fn parse_column (lines : Pair<Rule>) -> PGSLColumn {
	let mut column = PGSLColumn::default();

	for line in lines.into_inner() {
		match line.as_rule() {
			Rule::column_name => column.name = line.as_str().to_string(),
			Rule::type_name => column.type_name = line.as_str().to_string(),
			Rule::column_attributes => column.attributes = line.as_str().to_string(),
			Rule::column_comment => column.comments.push(line.as_str().to_string()),
			_ => unreachable!(),
		}
	}

	column
}

// region: Debug

pub fn debug(path : Option<PathBuf>) {
	let unparsed_file = fs::read_to_string(
		path.unwrap_or(PathBuf::from("test.pgl"))
	).expect("failed to read file");

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

// endregion
