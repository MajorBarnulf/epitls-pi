use std::fs;

use crate::tasks::{CompileTask, GenTask, RunTask};

pub fn main(_capture: bool, files: Vec<String>, _test: Option<Vec<String>>) {
	// let includes = files
	//     .iter()
	//     .cloned()
	//     .map(|p| PathBuf::from_str(&p).unwrap())
	//     .collect::<Vec<_>>();

	for path in files {
		let content = fs::read_to_string(&path).unwrap();
		let tests = find_tests(content);
		for test in tests {
			let content = gen_test_main(fs::canonicalize(&path).unwrap().to_str().unwrap(), &test);
			let generated_code = GenTask::new(content).run();

			// compile with all files
			//let files = includes.clone().apply(|v| v.insert(0, generated_code));
			let generated_bin = CompileTask::new(vec![generated_code]).run().unwrap();
			// run
			RunTask::new(generated_bin).run().unwrap();
		}
	}
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
void ____test();

int main() {{
    ____test();
    return 0;
}}

#include \"{path}\"

void ____test() {{
    {test}();
}}
"
	)
}
