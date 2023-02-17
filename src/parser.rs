use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use pest::iterators::Pair;
use pest::Parser;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Parser)]
#[grammar = "pgsl.pest"]
struct PGSLParser;

#[derive(Debug, Default)]
pub struct PGSLColumn {
	pub name : String,
	pub type_of: String,
	pub attributes : String,
	pub comments : Vec<String>,
}

#[derive(Debug, Default)]
pub struct PGSLTable {
	pub name : String,
	pub schema : Option<String>,
	pub extends : Vec<String>,
	pub columns : Vec<PGSLColumn>,
}

#[derive(Debug, Default)]
pub struct PGSLInterface {
	pub name : String,
	pub columns : Vec<PGSLColumn>,
}

#[derive(Debug, Default)]
pub struct PGSLGrant {
	pub privilege : String,
	pub roles : Vec<String>,
}

#[derive(Debug, Default)]
pub struct PGSLSchema {
	pub name : String,
	pub grants : Vec<PGSLGrant>,
}

#[derive(Debug, Default)]
pub struct PGSLArgument {
	pub name : String,
	pub type_of : String,
	pub default : Option<String>,
}

#[derive(Debug, Default)]
pub struct PGSLEnd {
	pub language : String,
	pub stability : String,
	pub security : Option<String>,
}

#[derive(Debug, Default)]
pub struct PGSLFunction {
	pub schema : Option<String>,
	pub name : String,
	pub returns : Option<String>,
	pub accept : Vec<PGSLArgument>,
	pub declare : Vec<PGSLArgument>,
	pub body : String,
	pub end : PGSLEnd,
}

#[derive(Debug, Default)]
pub struct PGSLTrigger {
	pub when : String,
	pub event : Vec<String>,
	pub interface : bool,
	pub schema : Option<String>,
	pub name : String,
	pub declare : Vec<PGSLArgument>,
	pub body : String,
	pub end : PGSLEnd,
}

#[derive(Debug, Default)]
pub struct PGSLView {
	pub schema : Option<String>,
	pub name : String,
	pub columns : Vec<String>,
	pub body : String,
}

#[derive(Debug, Default)]
pub struct PGSLData {
	pub requires : Vec<PathBuf>,
	pub schemas : Vec<PGSLSchema>,
	pub interfaces : HashMap<String, PGSLInterface>,
	pub tables : Vec<PGSLTable>,
	pub triggers : Vec<PGSLTrigger>,
	pub functions : Vec<PGSLFunction>,
	pub views : Vec<PGSLView>,
}

/// Parses the files at the given path into PGSLData
pub fn parse (path : PathBuf) -> Result<PGSLData> {
	lazy_static! {
		static ref RX : Regex = Regex::new(r"(?i)\.pgs?l$").unwrap();
	}

	let mut path_as_str = path.display().to_string();
	if !RX.is_match(path_as_str.as_str()) {
		path_as_str.push_str(".pgl");

		if fs::try_exists(path_as_str.clone()).is_err() {
			path_as_str = path_as_str.replace(".pgl", ".pgsl");
		}
	}

	let unparsed_file = fs::read_to_string(PathBuf::from(path_as_str.clone()))
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
			Rule::table => data.tables.push(parse_table(line)),
			Rule::schema => data.schemas.push(parse_schema(line)),
			Rule::trigger => data.triggers.push(parse_trigger(line)),
			Rule::function => data.functions.push(parse_function(line)),
			Rule::view => data.views.push(parse_view(line)),
			_ => unreachable!("Rule::{:?}", line.as_rule()),
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

/// Parses the schema rule
fn parse_schema (lines : Pair<Rule>) -> PGSLSchema {
	let mut schema = PGSLSchema::default();

	for line in lines.into_inner() {
		match line.as_rule() {
			Rule::schema_name => schema.name = line.as_str().to_string(),
			Rule::grant => schema.grants.push(parse_grant(line)),
			_ => unreachable!(),
		}
	}

	schema
}

/// Parses the table rule
fn parse_table (lines : Pair<Rule>) -> PGSLTable {
	let mut table = PGSLTable::default();

	for line in lines.into_inner() {
		match line.as_rule() {
			Rule::schema_name => table.schema = Some(line.as_str().to_string()),
			Rule::table_name => table.name = line.as_str().to_string(),
			Rule::columns => table.columns = parse_columns(line),
			Rule::extends => table.extends = parse_extends(line),
			_ => unreachable!(),
		}
	}

	table
}

/// Parses the grant rule
fn parse_grant (lines : Pair<Rule>) -> PGSLGrant {
	let mut grant = PGSLGrant::default();

	for line in lines.into_inner() {
		match line.as_rule() {
			Rule::privilege => grant.privilege = line.as_str().to_string(),
			Rule::role_name => grant.roles.push(line.as_str().to_string()),
			_ => unreachable!(),
		}
	}

	grant
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
			Rule::type_name => column.type_of = line.as_str().to_string(),
			Rule::column_attributes => column.attributes = line.as_str().to_string(),
			Rule::column_comment => column.comments.push(line.as_str().to_string()),
			_ => unreachable!(),
		}
	}

	column
}

fn parse_trigger (lines : Pair<Rule>) -> PGSLTrigger {
	let mut trigger = PGSLTrigger::default();

	for line in lines.into_inner() {
		match line.as_rule() {
			Rule::trigger_when => trigger.when = line.as_str().to_string(),
			Rule::trigger_event => trigger.event.push(line.as_str().to_string()),
			Rule::trigger_interface => trigger.interface = true,
			Rule::schema_name => trigger.schema = Some(line.as_str().to_string()),
			Rule::trigger_name => trigger.name = line.as_str().to_string(),
			Rule::declare => trigger.declare = parse_args(line),
			Rule::begin => trigger.body = parse_sql(line),
			Rule::end => trigger.end = parse_end(line),
			_ => unreachable!(),
		}
	}

	trigger
}

fn parse_function (lines : Pair<Rule>) -> PGSLFunction {
	let mut function = PGSLFunction::default();

	for line in lines.into_inner() {
		match line.as_rule() {
			Rule::schema_name => function.schema = Some(line.as_str().to_string()),
			Rule::function_name => function.name = line.as_str().to_string(),
			Rule::returns => function.returns = Some(line.as_str().to_string()),
			Rule::accept => function.accept = parse_args(line),
			Rule::declare => function.declare = parse_args(line),
			Rule::begin => function.body = parse_sql(line),
			Rule::end => function.end = parse_end(line),
			_ => unreachable!(),
		}
	}

	function
}

fn parse_argument (lines : Pair<Rule>) -> PGSLArgument {
	let mut arg = PGSLArgument::default();

	for line in lines.into_inner() {
		match line.as_rule() {
			Rule::argument_name => arg.name = line.as_str().to_string(),
			Rule::type_name => arg.type_of = line.as_str().to_string(),
			Rule::default_value => arg.default = Some(line.as_str().to_string()),
			_ => unreachable!()
		}
	}

	arg
}

fn parse_args (lines : Pair<Rule>) -> Vec<PGSLArgument> {
	let mut args = Vec::new();

	for line in lines.into_inner() {
		if line.as_rule() == Rule::argument {
			args.push(parse_argument(line));
		} else { unreachable!() }
	}

	args
}

fn parse_sql (lines : Pair<Rule>) -> String {
	let mut sql = String::new();

	for line in lines.into_inner() {
		if line.as_rule() == Rule::sql {
			sql.push_str(line.as_str());
			sql.push('\n');
		} else { unreachable!() }
	}

	sql.trim().to_string()
}

fn parse_end (lines : Pair<Rule>) -> PGSLEnd {
	let mut end = PGSLEnd::default();

	for line in lines.into_inner() {
		match line.as_rule() {
			Rule::language_name => end.language = line.as_str().to_string(),
			Rule::function_stability => end.stability = line.as_str().to_string(),
			Rule::function_security => end.security = Some(line.as_str().to_string()),
			_ => unreachable!(),
		}
	}

	end
}

fn parse_view (lines : Pair<Rule>) -> PGSLView {
	let mut view = PGSLView::default();

	for line in lines.into_inner() {
		match line.as_rule() {
			Rule::schema_name => view.schema = Some(line.as_str().to_string()),
			Rule::view_name => view.name = line.as_str().to_string(),
			Rule::column_name => view.columns.push(line.as_str().to_string()),
			Rule::view_body => view.body = parse_sql(line),
			_ => unreachable!(),
		}
	}

	view
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
