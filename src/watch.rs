use std::{path::Path, sync::mpsc, time::Duration};

use notify::{Error, Event, Watcher};
use notify_debouncer_mini::new_debouncer;

use crate::{
    tasks::{CompileTask, RunTask},
    utils::{log_failure, log_process, log_success},
};

pub struct Repeater {
    file: String,
}

impl Repeater {
    pub fn new(file: String) -> Self {
        Self { file }
    }

    pub fn repeat(&self) -> Option<()> {
        let source = CompileTask::new(self.file.clone().into())
            .run()
            .map(Option::from)
            .unwrap_or_else(|_| {
                log_failure("failed compilation");
                None
            })?;

        log_success("compilation successful");
        RunTask::new(source)
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

pub fn main(file: String) {
    log_process(&format!("watching file '{file}'"));
    let repeater = Repeater::new(file.clone());
    repeater.repeat();

    let (send, rec) = mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_millis(100), None, send).unwrap();

    debouncer
        .watcher()
        .watch(Path::new(&file), notify::RecursiveMode::Recursive)
        .unwrap();

    for events in rec {
        for _ in events.unwrap() {
            repeater.repeat();
        }
    }
}
