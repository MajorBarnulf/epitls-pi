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

	/// Runs tests contained within a particular test file or the default test file
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

	/// Watches changes to the project included files and runs a command on changes
	watch {
		#[clap(short)]
		files: Option<Vec<String>>,
		/// command to run on changes (ex: "pi test")
		command: String,
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
			if files.is_empty() {
				files.push(Config::get_current().main_file().to_string());
			}
			append_includes(&mut files);
			let args = compilation_args();
			run::main(files, args);
		}
		Commands::test {
			capture,
			mut files,
			tests,
		} => {
			if files.is_empty() {
				files.push(Config::get_current().test_file().to_string());
			}
			append_includes(&mut files);
			let args = compilation_args();
			let tests = (!tests.is_empty()).then_some(tests);
			test::main(capture, files, args, tests)
		}
		Commands::watch { command, files } => {
			let mut files = files.unwrap_or_default();
			append_includes(&mut files);
			watch::main(files, command);
		}

		Commands::init { path } => config::create(path),
	}
}
