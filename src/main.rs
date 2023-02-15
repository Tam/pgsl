mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::path::PathBuf;
use clap::Parser;

#[derive(clap::Parser)]
struct Cli {
	/// The starting input file
	///   Defaults to _schema.psl or schema/_schema.psl
	#[arg(value_hint=clap::ValueHint::AnyPath)]
	input : Option<PathBuf>,

	/// Runs the debug stuff
	#[arg(long,default_value_t=false)]
	debug : bool,

	/// Postgres hostname
	#[arg(long)]
	host : Option<String>,

	/// Postgres database name
	#[arg(short,long)]
	database : Option<String>,

	/// Postgres username
	#[arg(short,long)]
	username : Option<String>,

	/// Postgres password (you should use an env var for this)
	#[arg(short,long)]
	password : Option<String>,

	/// Postgres port
	#[arg(long,default_value_t=5432)]
	port : u16,
}

fn main() {
	let cli = Cli::parse();

	if cli.debug { parser::debug(cli.input); }
}

