use std::fs;

use termion::color;

use crate::{tasks::FormatTask, utils::log_failure};

const FORMAT_CONFIG: &str = r#"{BasedOnStyle: llvm}"#;

mod testables;

pub enum Diff {
	ToRemove { index: usize, content: String },
	ToAdd { index: usize, content: String },
	Keep { index: usize, content: String },
}

pub fn main(files: Vec<String>) {
	for file in files {
		let content = fs::read_to_string(&file).unwrap();
		let formatted = FormatTask::new(file, FORMAT_CONFIG.into()).run();
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
			log_failure("invalid formatting:");
			let red = color::Fg(color::Red);
			let green = color::Fg(color::Green);
			let reset = color::Fg(color::Reset);

			for difference in differences {
				match difference {
					Diff::ToRemove { index, content } => {
						println!("{red} {index} - | {content}{reset}")
					}
					Diff::ToAdd { index, content } => {
						println!("{green} {index} + | {content}{reset}")
					}
					Diff::Keep { index, content } => {
						println!(" {index}   | {content}")
					}
				}
			}
		}
	}
}
