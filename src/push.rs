use std::process::Command;

use chrono::Utc;

use crate::{config::Config, utils::log_success};

pub fn main(message: Option<String>) {
	let message = message.unwrap_or_else(|| Utc::now().format("pi - %d/%m/%Y %H:%M").to_string());

	// commit
	Command::new("git")
		.args(["commit", "-m", &message])
		.status()
		.unwrap();

	// push
	Command::new("git").arg("push").status().unwrap();

	// tag
	let timestamp = Utc::now().timestamp();
	let suffix = format!("pi-{timestamp}");
	let tag = Config::get_local().unwrap().identifier().to_string() + &suffix;
	Command::new("git")
		.args(["tag", "-a", &tag, "-m", ""])
		.status()
		.unwrap();

	// push tag
	Command::new("git")
		.args(["push", "--follow-tags"])
		.status()
		.unwrap();

	log_success(&format!("pushed with tag '{tag}'"));
}
