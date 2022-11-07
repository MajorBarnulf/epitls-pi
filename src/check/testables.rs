pub type RuleResult = Result<(), String>;

pub struct Rule {
	name: String,
	test: Box<dyn Fn(String) -> RuleResult>,
}

impl Rule {
	pub fn new(name: impl ToString, test: impl 'static + Fn(String) -> RuleResult) -> Self {
		let name = name.to_string();
		let test = Box::new(test);
		Self { name, test }
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn test(&self, source: String) -> RuleResult {
		(self.test)(source)
	}
}

fn ends_with_newline(source: String) -> RuleResult {
	if !source.ends_with('\n') {
		Err("source does not end with newline".into())
	} else {
		Ok(())
	}
}

fn function_under_50l(source: String) -> RuleResult {
	for character in source.chars() {
		let _c: char = character;
	}

	Ok(())
}

/// TODO: fill with appropriate rules
pub fn tests() -> Vec<Rule> {
	vec![
		// rules
		Rule::new("ends_with_newline", ends_with_newline),
		Rule::new("function_under_50l", function_under_50l),
	]
}
