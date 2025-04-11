mod _toml;
mod css_vars;
mod tailwind;

pub use {crate::designer::state::DesignerState, _toml::export_to_toml, css_vars::export_to_css_vars, tailwind::export_to_tailwind};

#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
	TailwindConfig,
	CssVariables,
	TomlConfig,
}

#[derive(Debug, Clone)]
pub struct ExportResult {
	/// Generated code or config
	pub content: String,
	pub filename: String,  // suggested file name
	pub mime_type: String, // MIME type
}

pub fn export_theme(state: &DesignerState, format: ExportFormat) -> ExportResult {
	match format {
		ExportFormat::TailwindConfig => export_to_tailwind(state),
		ExportFormat::CssVariables => export_to_css_vars(state),
		ExportFormat::TomlConfig => export_to_toml(state),
	}
}
