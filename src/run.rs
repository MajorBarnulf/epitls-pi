use crate::{
	tasks::{CompileTask, RunTask},
	utils::{log_failure, log_process},
};

pub fn main(files: Vec<String>, flags: Vec<String>, passed: Vec<String>) -> Option<()> {
	let source_file = files.into_iter().map(|f| f.into()).collect();
	log_process("compiling");
	let mut task = CompileTask::new(source_file);

	for flag in flags {
		task = task.with_flag(flag);
	}

	let compiled = task.run().map(Option::from).unwrap_or_else(|_| {
		log_failure("compilation failed");
		None
	})?;

	log_process("running");
	RunTask::new(compiled)
		.with_args(passed)
		.run()
		.map(Option::from)
		.unwrap_or_else(|_| {
			log_failure("process failure");
			None
		})?;
	Some(())
}
