use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default, strum_macros::EnumIter, strum_macros::Display, strum_macros::EnumString)]
pub enum Theme {
	Light,
	Dark,
	#[default]
	Auto,
}

impl Theme {
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

/// Actual them applied (light or dark, not system)
#[derive(Debug, Clone, PartialEq, strum_macros::EnumIter, strum_macros::Display, strum_macros::EnumString, Default)]
pub enum ResolvedTheme {
	Light,
	#[default]
	Dark,
}
