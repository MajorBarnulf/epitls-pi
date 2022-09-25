use std::{fs, path::PathBuf, process::Command};

use chrono::Utc;
use termion::color;

pub fn tmp_file_path() -> PathBuf {
	let ms = Utc::now().timestamp_millis().to_string();
	let mut path: PathBuf = ["/", "tmp", "epitls-pi"].iter().collect();
	fs::create_dir_all(&path).unwrap();
	path.push(ms);
	path
}

pub trait Apply: Sized {
	fn apply<F, O>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut Self) -> O,
	{
		f(&mut self);
		self
	}
}
impl<T> Apply for T {}

fn log_pi_prefix() {
	print!(
		"{}[pi] {}",
		color::Fg(color::LightBlue),
		color::Fg(color::Reset)
	)
}

pub fn log_command_run(command: &Command) {
	log_pi_prefix();
	let prefix = format_process("running ");
	let value = format_variable(&format!("{command:?}"));
	let suffix = format_process(" ...");
	println!("{prefix}{value}{suffix}");
}

pub fn log_separator() {
	println!("────────────────")
}

pub fn log_separator_top() {
	println!("───────────────┐")
}

pub fn log_separator_bottom() {
	println!("───────────────┘")
}

pub fn log_failure(text: &str) {
	log_pi_prefix();
	let text = format!("{}{text}{}", color::Fg(color::Red), color::Fg(color::Reset));
	println!("{text}");
}

pub fn log_success(text: &str) {
	log_pi_prefix();
	let text = format_success(text);
	println!("{text}");
}

pub fn log_process(text: &str) {
	log_pi_prefix();
	let text = format_process(text);
	println!("{text}");
}

fn format_process(input: &str) -> String {
	format!(
		"{}{input}{}",
		color::Fg(color::Blue),
		color::Fg(color::Reset)
	)
}

fn format_success(input: &str) -> String {
	format!(
		"{}{input}{}",
		color::Fg(color::Green),
		color::Fg(color::Reset)
	)
}

fn format_variable(input: &str) -> String {
	format!(
		"{}{input}{}",
		color::Fg(color::White),
		color::Fg(color::Reset),
	)
}
