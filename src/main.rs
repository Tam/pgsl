mod parser;
mod to_sql;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use std::path::PathBuf;

#[derive(clap::Parser)]
struct Cli {
    /// The starting input file
    ///   Defaults to _schema.psl or schema/_schema.psl
    #[arg(value_hint=clap::ValueHint::AnyPath)]
    input: Option<PathBuf>,

    /// Runs the debug stuff
    #[arg(long, default_value_t = false)]
    debug: bool,

    /// Postgres hostname
    #[arg(long, env = "PGHOST")]
    host: Option<String>,

    /// Postgres database name
    #[arg(short, long, env = "PGNAME")]
    database: Option<String>,

    /// Postgres username
    #[arg(short, long, env = "PGUSER")]
    username: Option<String>,

    /// Postgres password (you should use an env var for this)
    #[arg(short, long, env = "PGPASS")]
    password: Option<String>,

    /// Postgres port
    #[arg(long, default_value_t = 5432, env = "PGPORT")]
    port: u16,

	/// Perform a dry run to see what will change
	#[arg(long, default_value_t = false)]
	dryrun: bool,
}

fn main() -> Result<()> {
	use std::time::Instant;
	let now = Instant::now();
	
    dotenv().ok();
    let cli = Cli::parse();

    if cli.debug {
        parser::debug(cli.input);
        return Ok(());
    }

    let input = cli.input.unwrap_or(PathBuf::from("schema/_schema"));
    let ast = parser::parse(input)?;
    println!("{ast:#?}");
	
	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);
	
    Ok(())
}
