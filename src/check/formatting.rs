pub type StaticConf = [(&'static str, &'static str)];

/// TODO: fill with appropriate rules
pub const FORMAT_CONFIG: &StaticConf = &[
	// (key, value)
	("BasedOnStyle", "GNU"),
	("IndentWidth", "4"),
];

pub fn formatted_config() -> String {
	let middle = FORMAT_CONFIG
		.into_iter()
		.map(|(key, value)| format!("{key}: {value}"))
		.collect::<Vec<_>>()
		.join(", ");
	format!("{{ {middle} }}")
}
