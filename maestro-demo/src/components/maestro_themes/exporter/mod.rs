mod component_specific_styles;
mod tailwind;

use super::designer::DesignerState;

/// Theme Export Format
#[derive(Debug, PartialEq, Clone, Default, strum_macros::EnumIter, strum_macros::Display, strum_macros::EnumString)]
pub enum ExportFormat {
	#[default]
	TailwindConfig,
	CSSVariable,
	RustCode,
	TailwindCSS,
}

impl ExportFormat {
	pub fn extension(&self) -> &'static str {
		match self {
			ExportFormat::TailwindConfig => "js",
			ExportFormat::CSSVariable => "css",
			ExportFormat::RustCode => "rs",
			ExportFormat::TailwindCSS => "css",
		}
	}

	pub fn language(&self) -> &'static str {
		match self {
			ExportFormat::TailwindConfig => "javascript",
			ExportFormat::CSSVariable => "css",
			ExportFormat::RustCode => "rust",
			ExportFormat::TailwindCSS => "css",
		}
	}
}

/// Theme integration options for exporters
#[derive(Debug, PartialEq, Clone)]
pub struct ThemeOptions {
	/// Enable light/dark theme mode support
	pub with_doc_themes: bool,
	pub format: ExportFormat,
	pub components_id: String,
	pub stylesheet_path: String,
}

/// Export theme to various formats
pub fn export_theme(state: &DesignerState, theme_options: &ThemeOptions) -> String {
	let options = theme_options;

	match options.format {
		ExportFormat::TailwindConfig => tailwind::generate_tailwind_config(state, options.with_doc_themes),
		ExportFormat::CSSVariable => tailwind::generate_theme_variables(state),
		ExportFormat::RustCode => tailwind::generate_rust_theme(state),
		ExportFormat::TailwindCSS => tailwind::generate_tailwind_v4_css(state, options.clone()),
	}
}
