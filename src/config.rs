pub struct UserConfig {
    main_file: Option<String>,
    test_file: Option<String>,
    includes: Option<Vec<String>>,
}

pub struct Config {
    main_file: String,
    test_file: String,
    includes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            main_file: "main.c".into(),
            test_file: "test.c".into(),
            includes: vec![],
        }
    }
}
