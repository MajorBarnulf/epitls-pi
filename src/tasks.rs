use std::{
    fs,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use crate::utils::{log_command_run, log_separator, tmp_file_path, Apply};

pub struct CompileTask {
    file: PathBuf,
    addition: Vec<String>,
    flags: Vec<String>,
}

// TODO: split compile & compile raw

impl CompileTask {
    pub fn new(file: PathBuf) -> Self {
        Self {
            file,
            addition: vec![],
            flags: vec![],
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

    pub fn run(self) -> Result<PathBuf, ExitStatus> {
        let proc_source = self.gen_source();
        let sources = vec![proc_source, self.file.clone()];
        self.compile(sources)
    }

    pub fn gen_source(&self) -> PathBuf {
        let mut output_path = tmp_file_path();
        // TODO: make use of supplement
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
        log_command_run(&command);
        log_separator();
        let status = command.status().unwrap();
        log_separator();
        status.success().then_some(output_path).ok_or(status)
    }
}

pub struct RunTask {
    file: PathBuf,
}

impl RunTask {
    pub fn new(file: PathBuf) -> Self {
        Self { file }
    }

    pub fn run(self) -> Result<(), ExitStatus> {
        let mut command = Command::new(self.file);
        log_command_run(&command);
        log_separator();
        let status = command.status().unwrap();
        log_separator();
        if status.success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
