pub fn main(_capture: bool, _file: String, _test: Option<String>) {
    let content = todo!();
    let tests = find_tests(content);
    for test in tests {
        // compile
        // run
    }
}

pub fn find_tests(source: String) -> Vec<String> {
    source
        .split([' ', '(', ')', ';'])
        .filter(|name| &name[0..5] == "test_")
        .map(String::from)
        .collect()
}

pub fn gen_test_main(test_file: &str, test: &str) -> String {
    format!(
        "
#include \"{test_file}\"

int main() {{
    {test}();
    return 0;
    }}
"
    )
}
