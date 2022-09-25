use std::{fs, thread, time::Duration};

use crate::{
	tasks::{CompileTask, GenTask, RunTask},
	utils::{log_failure, log_process, log_success},
};

pub fn main(_capture: bool, files: Vec<String>, args: Vec<String>, _test: Option<Vec<String>>) {
	log_process("testing");
	for path in files {
		let content = fs::read_to_string(&path).unwrap();
		let tests = find_tests(content);
		for test in tests {
			log_process(&format!("running '{test}'"));
			let content = gen_test_main(fs::canonicalize(&path).unwrap().to_str().unwrap(), &test);
			let generated_code = GenTask::new(content).run();

			thread::sleep(Duration::from_millis(100));

			// compile with all files
			let mut task = CompileTask::new(vec![generated_code]);
			for flag in args.clone() {
				task = task.with_flag(flag);
			}
			let generated_bin = task.run().unwrap();

			// run
			if RunTask::new(generated_bin).run().is_err() {
				log_failure("test failed");
			}
		}
	}
	log_success("finished");
}

pub fn find_tests(source: String) -> Vec<String> {
	source
		.split([' ', '(', ')', ';'])
		.filter(|name| name.starts_with("test_"))
		.map(String::from)
		.collect()
}

pub fn gen_test_main(path: &str, test: &str) -> String {
	format!(
		"
void __pi_test();

int main(int argc, char** argv) {{
	(void)argc;
	(void)argv;
    
	__pi_test();
    return 0;
}}

#include \"{path}\"

void __pi_test() {{
    {test}();
}}
"
	)
}
