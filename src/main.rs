use clap::{Parser, Subcommand};
use config::Config;

pub mod check;
pub mod config;
pub mod run;
pub mod tasks;
pub mod test;
pub mod utils;
pub mod watch;

#[derive(Parser)]
pub struct Arguments {
	#[clap(subcommand)]
	command: Commands,
}

#[allow(non_camel_case_types)]
#[derive(Subcommand)]
pub enum Commands {
	/// Checks a source file for conformance with piscine limitations.
	check {
		/// File to check.
		files: Vec<String>,
	},

	/// Runs a set of files or the default target.
	run {
		/// Files to run.
		files: Vec<String>,
	},

	/// Runs tests contained within a particular test file or
	test {
		/// Wether to capture standard output or not.
		#[clap(short, long)]
		capture: bool,

		/// Files to run tests from.
		files: Vec<String>,

		/// Specific tests to run.
		#[clap(short, long)]
		tests: Vec<String>,
	},

	/// Watches changes to source files and re run them
	watch {
		/// Files to run.
		files: Vec<String>,
	},

	///
	init { path: String },
}

fn append_includes(list: &mut Vec<String>) {
	list.extend(
		Config::get_current()
			.includes()
			.iter()
			.map(|f| f.to_string()),
	);
}

fn compilation_args() -> Vec<String> {
	let mut args = vec![
		"-Wall".to_string(),
		"-Wextra".to_string(),
		"-std=c99".to_string(),
	];
	if Config::get_current().fascist_mode() {
		args.push("-Werror".to_string());
	}
	args
}

fn main() {
	let args: Arguments = Parser::parse();

	match args.command {
		Commands::check { files } => check::main(files),
		Commands::run { mut files } => {
			append_includes(&mut files);
			let args = compilation_args();
			run::main(files, args);
		}
		Commands::test {
			capture,
			mut files,
			tests,
		} => {
			append_includes(&mut files);
			let args = compilation_args();
			let tests = (!tests.is_empty()).then_some(tests);
			test::main(capture, files, tests)
		}
		Commands::watch { mut files } => {
			append_includes(&mut files);
			let args = compilation_args();
			watch::main(files, args)
		}
		Commands::init { path } => config::create(path),
	}
}
