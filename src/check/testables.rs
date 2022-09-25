fn ends_with_newline(source: String) -> Result<(), String> {
	if !source.ends_with('\n') {
		Err("source does not end with newline".into())
	} else {
		Ok(())
	}
}

pub struct Rule {
	name: String,
	test: Box<dyn Fn(String) -> Result<(), String>>,
}

impl Rule {
	pub fn new(name: impl ToString, test: impl 'static + Fn(String) -> Result<(), String>) -> Self {
		let name = name.to_string();
		let test = Box::new(test);
		Self { name, test }
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn test(&self, source: String) -> Result<(), String> {
		(self.test)(source)
	}
}

/// TODO: fill with appropriate rules
pub fn tests() -> Vec<Rule> {
	vec![
		// rules
		Rule::new("ends_with_newline", ends_with_newline),
	]
}
