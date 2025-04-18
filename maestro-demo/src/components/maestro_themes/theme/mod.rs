mod context;
mod provider;
mod storage;
mod system;
mod theme_selector;
mod types;

pub mod prelude {
	pub use super::{
		context::{ThemeContext, set_document_theme, use_theme},
		provider::ThemeProvider,
		storage::{ThemeStorage, ThemeStorageError, ThemeStorageResult, get_storage},
		system::get_system_theme_detector,
		theme_selector::ThemeSelect,
		types::{ResolvedTheme, Theme},
	};
}
