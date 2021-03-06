use std::io;

use clap::StructOpt;

use crate::cli::Args;
use crate::run::Result as RunResult;

mod cli;
mod config;
mod parser;
mod run;

fn main() -> RunResult {
	let args = Args::parse();

	let stdout = io::stdout();
	let handle = stdout.lock();

	run::run(handle, args)?;

	Ok(())
}
