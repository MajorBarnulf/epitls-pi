use std::process::{exit, Command};

use chrono::Utc;

use crate::{
	config::Config,
	utils::{log_error, log_process, log_success},
};

pub fn add() {
	let path = Config::get_local_path().unwrap();
	let path = path.parent().unwrap();
	let path = path.to_str().unwrap();
	Command::new("git")
		.args(["add", path])
		.status()
		.unwrap()
		.success()
		.then_some(())
		.unwrap_or_else(|| exit(1));
}

pub fn main(message: Option<String>) {
	let message = message.unwrap_or_else(|| Utc::now().format("pi - %d/%m/%Y %H:%M").to_string());
	let timestamp = Utc::now().timestamp();
	let suffix = format!("pi-{timestamp}");
	let tag = Config::get_local()
		.unwrap_or_else(|| {
			log_error("no config file found.\nPlease initialize with 'pi init <tag-prefix>'");
			exit(1)
		})
		.identifier()
		.to_string()
		+ &suffix;

	// commit
	log_process("committing changes");
	Command::new("git")
		.args(["commit", "-m", &message])
		.status()
		.unwrap()
		.success()
		.then_some(())
		.unwrap_or_else(|| exit(1));

	// push
	Command::new("git").arg("push").status().unwrap();

	// tag
	log_process("tagging");
	Command::new("git")
		.args(["tag", "-a", &tag, "-m", ""])
		.status()
		.unwrap()
		.success()
		.then_some(())
		.unwrap_or_else(|| exit(1));

	// push tag
	Command::new("git")
		.args(["push", "--follow-tags"])
		.status()
		.unwrap()
		.success()
		.then_some(())
		.unwrap_or_else(|| exit(1));

	log_success(&format!("pushed with tag '{tag}'"));
}
