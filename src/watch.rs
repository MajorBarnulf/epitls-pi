use std::{path::Path, sync::mpsc, time::Duration};

use notify_debouncer_mini::new_debouncer;

use crate::{
	tasks::{CompileTask, RunTask},
	utils::{log_failure, log_process, log_success},
};

pub struct Repeater {
	op: Box<dyn Fn()>,
}

impl Repeater {
	pub fn new(op: impl Fn() + 'static) -> Self {
		let op = Box::new(op);
		Self { op }
	}

	pub fn repeat(&self) -> Option<()> {
		(self.op)();
		log_process("waiting for changes...");
		Some(())
	}
}

pub fn main(files: Vec<String>, op: impl Fn() + 'static) {
	log_process(&format!("watching files '{files:?}'"));
	let repeater = Repeater::new(op);
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
