use std::env;

use clap::{Parser, Subcommand};
use config::Config;

pub mod check;
pub mod config;
pub mod push;
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
	/// Checks source files for conformance with piscine limitations.
	check {
		/// File to check.
		files: Vec<String>,
	},

	/// Formats source files.
	format {
		/// Files to format.
		files: Vec<String>,
	},

	/// Runs a set of files or the default target.
	run {
		/// Files to run.
		files: Vec<String>,
	},

	/// Runs tests contained within a particular test file or the default test file.
	test {
		/// Wether to capture standard output or not.
		#[clap(short, long)]
		capture: bool,

		/// Files to run tests from.
		files: Vec<String>,
		// /// Specific tests to run.
		// #[clap(short, long)]
		// tests: Vec<String>,
	},

	/// Watches changes to the project included files and runs a command on changes.
	watch {
		#[clap(short)]
		files: Option<Vec<String>>,
		/// command to run on changes (ex: "pi test")
		command: String,
	},

	/// Initializes a project directory configuration, useful for custom flags, includes and custop push messages.
	init {
		/// Identifier for the automated tests.
		prefix: String,
		/// Path to the folder containing the project.
		path: Option<String>,
		/// e
		#[clap(short, long)]
		tests: bool,
	},

	/// Pushes changes to the git server with a custom tag.
	push { message: Option<String> },
}

fn append_includes(list: &mut Vec<String>) {
	list.extend(
		Config::get_local_or_default()
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
	if Config::get_local_or_default().strict_mode() {
		args.push("-Werror".to_string());
	}
	args
}

fn main() {
	let args: Arguments = Parser::parse();

	match args.command {
		Commands::check { files } => check::main(files),

		Commands::format { files } => check::format(files),

		Commands::run { mut files } => {
			if files.is_empty() {
				files.push(Config::get_local_or_default().main_file().to_string());
			}
			append_includes(&mut files);
			let args = compilation_args();
			run::main(files, args);
		}

		Commands::test {
			capture,
			mut files,
			// tests,
		} => {
			if files.is_empty() {
				files.push(Config::get_local_or_default().test_file().to_string());
			}
			append_includes(&mut files);
			let args = compilation_args();
			test::main(capture, files, args)
		}
		Commands::watch { command, files } => {
			let mut files = files.unwrap_or_default();
			append_includes(&mut files);
			watch::main(files, command);
		}

		Commands::init {
			path,
			prefix,
			tests,
		} => {
			let path =
				path.unwrap_or_else(|| env::current_dir().unwrap().to_str().unwrap().to_string());
			config::create(path.clone(), prefix);
			if tests {
				config::create_test(path);
			}
		}

		Commands::push { message } => {
			push::main(message);
		}
	}
}
