mod tailwind;

pub use {crate::designer::state::DesignerState, tailwind::*};

#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
	TailwindConfig,
}

#[derive(Debug, Clone)]
pub struct ExportResult {
	/// Generated code or config
	pub content: String,
	pub filename: String,  // suggested file name
	pub mime_type: String, // MIME type
}

pub fn export_theme(_state: &DesignerState, format: ExportFormat) -> ExportResult {
	match format {
		ExportFormat::TailwindConfig => todo!(),
	}
}
