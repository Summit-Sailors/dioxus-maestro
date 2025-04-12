mod tailwind;

pub use {crate::designer::state::DesignerState, tailwind::*};

#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
	TailwindV4,     // CSS first approach
	TailwindConfig, // Legacy (Version 3 and below)
	CssVariables,   // Pure CSS variables
	RustCode,       // Rust code rep
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
		ExportFormat::TailwindConfig =>
			ExportResult { content: generate_tailwind_config(state), filename: String::from("tailwind.config.js"), mime_type: String::from("application/javascript") },
		ExportFormat::CssVariables => ExportResult { content: generate_theme_css(state), filename: String::from("theme.css"), mime_type: String::from("text/css") },
		ExportFormat::RustCode => ExportResult { content: generate_rust_theme(state), filename: String::from("theme.rs"), mime_type: String::from("text/rust") },
		ExportFormat::TailwindV4 =>
			ExportResult { content: generate_tailwind_v4_css(state), filename: String::from("tailwind.css"), mime_type: String::from("text/css") },
	}
}

pub fn save_export_result(result: &ExportResult, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
	use std::fs;

	let file_path = if path.is_dir() { path.join(&result.filename) } else { path.to_path_buf() };

	fs::write(file_path, result.content.clone())?;
	Ok(())
}
