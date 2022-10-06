use std::fs;

use termion::color;

use crate::{
	tasks::FormatTask,
	utils::{log_failure, log_process, log_success},
};

mod formatting;
mod testables;

pub fn main(files: Vec<String>) {
	for file in files {
		log_process(&format!("checking '{file}'"));
		check_formatting(file.to_string());
		let content = fs::read_to_string(&file).unwrap();
		for test in testables::tests() {
			if let Err(reason) = test.test(content.clone()) {
				let name = test.name();
				log_failure(&format!("'{file}': rule '{name}' fails:"));
				println!("{reason}");
			}
		}
	}
}

pub enum Diff {
	ToRemove { index: usize, content: String },
	ToAdd { index: usize, content: String },
	Keep { index: usize, content: String },
}

fn check_formatting(file: String) {
	let content = fs::read_to_string(&file).unwrap();
	let formatted = FormatTask::new(file.clone(), formatting::formatted_config()).run();
	let mut line_number = 0usize;
	let mut invalid = false;
	let differences = diff::lines(&content, &formatted)
		.into_iter()
		.map(|change| {
			match change {
				diff::Result::Left(_) | diff::Result::Both(_, _) => {
					line_number += 1;
				}
				_ => (),
			}
			(line_number, change)
		})
		.map(|(index, d)| match d {
			diff::Result::Left(content) => {
				invalid = true;
				Diff::ToRemove {
					index,
					content: content.into(),
				}
			}
			diff::Result::Both(content, _) => Diff::Keep {
				index,
				content: content.into(),
			},
			diff::Result::Right(content) => {
				invalid = true;
				Diff::ToAdd {
					index,
					content: content.into(),
				}
			}
		})
		.collect::<Vec<_>>();
	if invalid {
		log_failure(&format!("'{file}': invalid formatting:"));
		let red = color::Fg(color::Red);
		let green = color::Fg(color::Green);
		let reset = color::Fg(color::Reset);

		for difference in differences {
			match difference {
				Diff::ToRemove { index, content } => {
					println!("{red}{index:>3} -|{content}{reset}")
				}
				Diff::ToAdd { index, content } => {
					println!("{green}{index:>3} +|{content}{reset}")
				}
				Diff::Keep { index, content } => {
					println!("{index:>3}  |{content}")
				}
			}
		}
	}
}

pub fn format(files: Vec<String>) {
	for file in files {
		let mut formatted = FormatTask::new(file.clone(), formatting::formatted_config()).run();
		if !formatted.ends_with('\n') {
			formatted += "\n";
		}
		fs::write(&file, formatted).unwrap();
		log_success(&format!("formatted '{file}'"));
	}
}
