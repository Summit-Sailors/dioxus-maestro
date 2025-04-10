use std::fmt::{Disaply, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
	Light,
	Dark,
	Auto,
}

impl Theme {
	pub fn as_str(&self) -> &'static str {
		match self {
			Theme::Light => "light",
			Theme::Dark => "dark",
			Theme::Auto => "auto",
		}
	}

	pub fn from_str_slice(s: &str) -> Self {
		match s {
			"light" => Theme::Light,
			"dark" => Theme::Dark,
			_ => Theme::Auto,
		}
	}

	pub fn resolve(&self, system_prefers_dark: bool) -> ResolvedTheme {
		match self {
			Theme::Light => ResolvedTheme::Light,
			Theme::Dark => ResolvedTheme::Dark,
			Theme::Auto =>
				if system_prefers_dark {
					ResolvedTheme::Dark
				} else {
					ResolvedTheme::Light
				},
		}
	}
}

impl Display for Theme {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

/// Actual them applied (light or dark, not system)
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedTheme {
	Light,
	Dark,
}

impl ResolvedTheme {
	pub fn as_str(&self) -> &'static str {
		match self {
			ResolvedTheme::Light => "light",
			ResolvedTheme::Dark => "dark",
		}
	}

	pub fn as_class(&self) -> &'static str {
		match self {
			ResolvedTheme::Light => "light",
			ResolvedTheme::Dark => "", // default is dark
		}
	}

	pub fn from_str_slice(s: &str) -> Self {
		match s {
			"light" => ResolvedTheme::Light,
			"dark" => ResolvedTheme::Dark,
			_ => ResolvedTheme::Light,
		}
	}
}
