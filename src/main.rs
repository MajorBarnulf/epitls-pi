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
		// disables fsanitize
		#[clap(short, long)]
		disable_sanitize: bool,
		/// Files to run.
		files: Vec<String>,
		#[clap(short, long)]
		pass: Vec<String>,
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
		/// Path to the folder containing the project.
		path: String,
		/// Identifier for the automated tests.
		prefix: Option<String>,
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

fn compilation_args(sanitize: bool) -> Vec<String> {
	let mut args = vec![
		"-Wall".to_string(),
		"-Wextra".to_string(),
		"-std=c99".to_string(),
		"-g".to_string(),
		"-pedantic".to_string(),
	];
	if sanitize {
		args.push("-fsanitize=address".to_string())
	}
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

		Commands::run {
			disable_sanitize,
			mut files,
			pass,
		} => {
			if files.is_empty() {
				files.push(Config::get_local_or_default().main_file());
			}
			append_includes(&mut files);
			let args = compilation_args(!disable_sanitize);
			run::main(files, args, pass);
		}

		Commands::test {
			capture,
			mut files,
			// tests,
		} => {
			if files.is_empty() {
				files.push(Config::get_local_or_default().test_file());
			}
			let mut includes = vec![];
			append_includes(&mut includes);
			let args = compilation_args(true);
			test::main(capture, files, includes, args)
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
			let prefix = prefix.unwrap_or_else(|| ".".to_string());
			let prefix = prefix.trim().trim_end_matches('*');
			config::create(path.clone(), prefix.to_string());
			if tests {
				config::create_test(path);
			}
		}

		Commands::push { message } => {
			push::main(message);
		}
	}
}
