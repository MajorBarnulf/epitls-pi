use std::{
	fs,
	path::PathBuf,
	process::{Command, ExitStatus, Stdio},
};

use crate::utils::{
	log_command_run, log_separator_bottom, log_separator_top, tmp_file_path, Apply,
};

pub struct CompileTask {
	files: Vec<PathBuf>,
	addition: Vec<String>,
	flags: Vec<String>,
	verbose: bool,
}

impl CompileTask {
	pub fn new(files: Vec<PathBuf>) -> Self {
		Self {
			files,
			addition: vec![],
			flags: vec![],
			verbose: false,
		}
	}

	pub fn with_addition(mut self, code: impl ToString) -> Self {
		self.addition.push(code.to_string());
		self
	}

	pub fn with_flag(mut self, flag: impl ToString) -> Self {
		self.flags.push(flag.to_string());
		self
	}

	pub fn with_verbose(mut self) -> Self {
		self.verbose = true;
		self
	}

	pub fn run(self) -> Result<PathBuf, ExitStatus> {
		let proc_source = self.gen_source();
		let mut sources = self.files.clone();
		sources.push(proc_source);
		self.compile(sources)
	}

	pub fn gen_source(&self) -> PathBuf {
		let mut output_path = tmp_file_path();
		output_path.set_extension("c");
		fs::write(&output_path, "").unwrap();
		output_path
	}

	pub fn compile(&self, sources: Vec<PathBuf>) -> Result<PathBuf, ExitStatus> {
		let output_path = tmp_file_path().apply(|o| o.set_extension("b"));
		let output_path_ref = output_path.to_str().unwrap();
		let mut command = Command::new("gcc");
		command
			.args(["-o", output_path_ref])
			.args(self.flags.clone())
			.args(sources.iter().map(|s| s.to_str().unwrap()));
		if self.verbose {
			log_command_run(&command);
			log_separator_top();
		}
		let status = command.status().unwrap();
		if self.verbose {
			log_separator_bottom();
		}
		status.success().then_some(output_path).ok_or(status)
	}
}

pub struct RunTask {
	file: PathBuf,
	verbose: bool,
}

impl RunTask {
	pub fn new(file: PathBuf) -> Self {
		Self {
			file,
			verbose: false,
		}
	}

	pub fn with_verbose(mut self) -> Self {
		self.verbose = true;
		self
	}

	pub fn run(self) -> Result<(), ExitStatus> {
		let mut command = Command::new("sh");
		command.args(["-c", self.file.to_str().unwrap()]);
		if self.verbose {
			log_command_run(&command);
			log_separator_top();
		}
		let status = command.status().unwrap();
		if self.verbose {
			log_separator_bottom();
		}
		if status.success() {
			Ok(())
		} else {
			Err(status)
		}
	}
}

pub struct GenTask {
	content: String,
}

impl GenTask {
	pub fn new(content: String) -> Self {
		Self { content }
	}

	pub fn run(self) -> PathBuf {
		let output_path = tmp_file_path().apply(|o| o.set_extension("c"));
		let content = self.content;
		fs::write(&output_path, &content).unwrap();
		output_path
	}
}

pub struct FormatTask {
	file: String,
	config: String,
}

impl FormatTask {
	pub fn new(file: String, config: String) -> Self {
		Self { config, file }
	}

	pub fn run(self) -> String {
		let config = self.config;
		let mut command = Command::new("clang-format");
		command
			.arg(self.file)
			.arg(format!("-style={config}"))
			.stdout(Stdio::piped())
			.stderr(Stdio::piped());
		command.status().unwrap();

		let result = command.output().unwrap().stdout;
		String::from_utf8(result).unwrap()
	}
}

pub struct CmdTask {
	command: String,
}

impl CmdTask {
	pub fn new(command: String) -> Self {
		Self { command }
	}

	#[allow(clippy::result_unit_err)]
	pub fn run(self) -> Result<(), ()> {
		Command::new("sh")
			.arg("-c")
			.arg(self.command)
			.stderr(Stdio::inherit())
			.stdout(Stdio::inherit())
			.output()
			.map(|_| ())
			.map_err(|_| ())
	}
}
