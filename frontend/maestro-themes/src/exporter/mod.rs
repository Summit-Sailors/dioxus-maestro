mod tailwind;

pub use tailwind::{generate_rust_theme, generate_tailwind_config, generate_tailwind_v4_css, generate_theme_css};

/// Theme integration options for exporters
#[derive(Default)]
pub struct ThemeOptions {
	/// Enable light/dark theme mode support
	pub with_themes: bool,
}

/// Export theme to various formats
pub fn export_theme(state: &crate::designer::state::DesignerState, theme_options: Option<ThemeOptions>) -> std::collections::HashMap<String, String> {
	let options = theme_options.unwrap_or_default();

	let mut exports = std::collections::HashMap::new();

	// standard theme exports
	exports.insert("tailwind.css".to_string(), tailwind::generate_tailwind_v4_css(state, options.with_themes));
	exports.insert("theme.css".to_string(), tailwind::generate_theme_css(state, options.with_themes));
	exports.insert("tailwind.config.js".to_string(), tailwind::generate_tailwind_config(state, options.with_themes));
	exports.insert("theme.rs".to_string(), tailwind::generate_rust_theme(state));
	exports
}
