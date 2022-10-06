use std::{
	env, fs,
	io::stdin,
	path::{Path, PathBuf},
};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::utils::{log_process, log_success, Apply};

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
			test_file: "test.ctest".into(),
			includes: vec![],
			strict_mode: false,
		}
	}

	pub fn write(&self, mut path: PathBuf) {
		path.extend([Self::CONFIG_FILE_NAME]);
		let content =
			ron::ser::to_string_pretty(self, PrettyConfig::default().struct_names(true)).unwrap();
		if path.exists() {
			log_process("config already exists, overwrite it?");
			let mut buff = String::new();
			stdin().read_line(&mut buff).unwrap();
		}
		fs::write(path, content).unwrap();
	}

	pub fn get_local() -> Option<Self> {
		let path = env::current_dir().unwrap();
		Self::get(&path)
	}

	pub fn get_local_or_default() -> Self {
		let path = env::current_dir().unwrap();
		Self::get(&path)
			.unwrap_or_else(|| Self::new(path.file_name().unwrap().to_str().unwrap().to_string()))
	}

	pub fn get_local_path() -> Option<PathBuf> {
		let path = env::current_dir().unwrap();
		Self::get_path(&path)
	}

	pub fn get(path: &Path) -> Option<Self> {
		let path = path.to_path_buf().canonicalize().unwrap();
		Self::try_get(&path).or_else(|| path.parent().and_then(Self::get))
	}

	pub fn get_path(path: &Path) -> Option<PathBuf> {
		let path = path.to_path_buf().canonicalize().unwrap();
		Self::try_get_path(&path).or_else(|| path.parent().and_then(Self::get_path))
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

	fn try_get_path(path: &Path) -> Option<PathBuf> {
		let path = path.to_path_buf().apply(|p| p.push(Self::CONFIG_FILE_NAME));
		fs::read_to_string(&path)
			.ok()
			.and_then(|content| ron::from_str::<Self>(&content).ok())
			.is_some()
			.then_some(path)
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

pub fn create_test(mut path: String) {
	const DEFAULT_CONTENT: &str = r#"
#include <stdlib.h>
#include <stdio.h>
#include <assert.h>

void test_it_works() {
	assert( (2 + 2) == (4) );
}
"#;
	path += "/test.ctest";
	fs::write(path, DEFAULT_CONTENT).unwrap();
}
