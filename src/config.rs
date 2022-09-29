use std::{
	env, fs,
	path::{Path, PathBuf},
};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::utils::{log_success, Apply};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	identifier: String,
	main_file: String,
	test_file: String,
	includes: Vec<String>,
	strict_mode: bool,
}

impl Config {
	const CONFIG_FILE_NAME: &'static str = "pi.ron";
	pub fn new(identifier: String) -> Self {
		Self {
			identifier,
			main_file: "main.c".into(),
			test_file: "test.c".into(),
			includes: vec![],
			strict_mode: false,
		}
	}

	pub fn write(&self, mut path: PathBuf) {
		path.extend([Self::CONFIG_FILE_NAME]);
		let content =
			ron::ser::to_string_pretty(self, PrettyConfig::default().struct_names(true)).unwrap();
		fs::write(path, content).unwrap();
	}

	pub fn get_current() -> Self {
		let path = env::current_dir().unwrap();
		Self::get(&path)
			.unwrap_or_else(|| Self::new(path.file_name().unwrap().to_str().unwrap().to_string()))
	}

	pub fn get(path: &Path) -> Option<Self> {
		let path = path.to_path_buf().canonicalize().unwrap();
		Self::try_get(&path).or_else(|| path.parent().and_then(Self::get))
	}

	pub fn identifier(&self) -> &str {
		&self.identifier
	}

	pub fn main_file(&self) -> &str {
		&self.main_file
	}

	pub fn test_file(&self) -> &str {
		&self.test_file
	}

	pub fn includes(&self) -> &Vec<String> {
		&self.includes
	}

	pub fn strict_mode(&self) -> bool {
		self.strict_mode
	}

	fn try_get(path: &Path) -> Option<Self> {
		let path = path.to_path_buf().apply(|p| p.push(Self::CONFIG_FILE_NAME));
		fs::read_to_string(path)
			.ok()
			.and_then(|content| ron::from_str(&content).ok())
	}
}

pub fn create(path: String, identifier: String) {
	let absolute = fs::canonicalize(&path).unwrap();
	if !absolute.is_dir() {
		panic!("not a directory");
	}
	let config = Config::new(identifier);
	config.write(absolute.clone());
	let path = absolute
		.apply(|p| p.push(Config::CONFIG_FILE_NAME))
		.to_str()
		.unwrap()
		.to_string();
	log_success(&format!("created '{path}'"));
}
