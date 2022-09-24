use crate::{
    tasks::{CompileTask, RunTask},
    utils::{log_failure, log_success},
};

pub fn main(files: Vec<String>) -> Option<()> {
    let source_file = files.into_iter().map(|f| f.into()).collect();
    let compiled = CompileTask::new(source_file)
        .with_flag("-Wall")
        .with_flag("-Wextra")
        .with_flag("-std=c99")
        .run()
        .map(Option::from)
        .unwrap_or_else(|_| {
            log_failure("compilation failed");
            None
        })?;
    log_success("compilation successful");
    RunTask::new(compiled)
        .run()
        .map(Option::from)
        .unwrap_or_else(|_| {
            log_failure("process failure");
            None
        })?;
    log_success("process exited successfully");
    Some(())
}
