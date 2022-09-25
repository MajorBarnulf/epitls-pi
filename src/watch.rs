use std::{path::Path, sync::mpsc, time::Duration};

use notify_debouncer_mini::new_debouncer;

use crate::{
	tasks::{CompileTask, RunTask},
	utils::{log_failure, log_process, log_success},
};

pub struct Repeater {
	files: Vec<String>,
}

impl Repeater {
	pub fn new(files: Vec<String>) -> Self {
		Self { files }
	}

	pub fn repeat(&self) -> Option<()> {
		let binary = CompileTask::new(self.files.clone().into_iter().map(|f| f.into()).collect())
			.run()
			.map(Option::from)
			.unwrap_or_else(|_| {
				log_failure("failed compilation");
				None
			})?;

		log_success("compilation successful");
		RunTask::new(binary)
			.run()
			.map(Option::from)
			.unwrap_or_else(|_| {
				log_failure("task failure");
				None
			})?;

		log_success("task successful");
		log_process("waiting for changes before re run");
		Some(())
	}
}

pub fn main(files: Vec<String>) {
	log_process(&format!("watching files '{files:?}'"));
	let repeater = Repeater::new(files.clone());
	repeater.repeat();

	let (send, rec) = mpsc::channel();
	let mut debouncer = new_debouncer(Duration::from_millis(100), None, send).unwrap();

	for file in files {
		debouncer
			.watcher()
			.watch(Path::new(&file), notify::RecursiveMode::Recursive)
			.unwrap();
	}

	for events in rec {
		for _ in events.unwrap() {
			repeater.repeat();
		}
	}
}
