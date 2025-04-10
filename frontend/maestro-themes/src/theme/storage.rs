// Platform specific storage for theme settings

use {crate::theme::types::Theme, std::fmt::Display};

#[derive(Debug, Display)]
pub enum ThemeStorageError {
	WebStorageAccessError,
	WebStorageSetError,
	WebStorageGetError,
	DesktopConfigDirError,
	DesktopFileOpenError,
	DesktopFileReadError,
	DesktopFileCreateError,
	DesktopFileWriteError,
	ThemeSerializationError(String),
	ThemeDeserializationError(String),
}

pub type ThemeStorageResult<T> = Result<T, ThemeStorageError>;

// Storage trait for saving/loading theme preferences
pub trait ThemeStorage {
	fn set_theme(&self, theme: &Theme) -> ThemeStorageResult<()>;
	fn get_theme(&self) -> ThemeStorageResult<Option<Theme>>;
}

#[cfg(feature = "web")]
pub mod web {
	use {
		super::*,
		web_sys::{Storage, window},
	};

	pub struct WebStorage;

	impl ThemeStorage for WebStorage {
		fn set_theme(&self, theme: &Theme) -> ThemeStorageResult<()> {
			window()
				.and_then(|win| win.local_storage().ok())
				.flatten()
				.ok_or(ThemeStorageError::WebStorageAccessError)
				.and_then(|storage| storage.set_item("maestro-ui-theme-v1", theme.as_str()).map_err(|_| ThemeStorageError::WebStorageSetError))
		}

		fn get_theme(&self) -> ThemeStorageResult<Option<Theme>> {
			window().and_then(|win| win.local_storage().ok()).flatten().ok_or(ThemeStorageError::WebStorageAccessError).and_then(|storage| {
				storage
					.get_item("maestro-ui-theme-v1")
					.ok()
					.flatten()
					.map(|theme_str| Theme::from_str_slice(&theme_str).map_err(|e| ThemeStorageError::ThemeDeserializationError(format!("{}", e))))
					.transpose() // Result<Option<Result<Theme, Error>>, Error> -> Result<Option<Theme>, Error>
			})
		}
	}
}

#[cfg(feature = "desktop")]
pub mod desktop {
	use {
		super::*,
		directories::ProjectDirs,
		std::{
			fs::{self, File},
			io::{Read, Write},
			path::PathBuf,
		},
	};

	pub struct DesktopStorage;

	impl DesktopStorage {
		fn get_config_path() -> ThemeStorageResult<PathBuf> {
			ProjectDirs::from("com", "maestro", "dioxus").ok_or(ThemeStorageError::DesktopConfigDirError).map(|dirs| {
				let config_dir = dirs.config_dir();
				if !config_dir.exists() {
					fs::create_dir_all(config_dir).map_err(|_| ThemeStorageError::DesktopConfigDirError)?;
				}
				config_dir.join("theme.txt")
			})
		}
	}

	impl ThemeStorage for DesktopStorage {
		fn get_theme(&self) -> ThemeStorageResult<Option<Theme>> {
			Self::get_config_path().and_then(|path| {
				File::open(path)
					.map_err(|_| ThemeStorageError::DesktopFileOpenError)
					.and_then(|mut file| {
						let mut contents = String::new();
						file.read_to_string(&mut contents).map_err(|_| ThemeStorageError::DesktopFileReadError)?;
						Ok(Some(Theme::from_str_slice(contents.trim()).map_err(|e| ThemeStorageError::ThemeDeserializationError(format!("{}", e)))?))
					})
					.or_else(|e| match e {
						ThemeStorageError::DesktopFileOpenError => Ok(None),
						other_error => Err(other_error),
					})
			})
		}

		fn set_theme(&self, theme: &Theme) -> ThemeStorageResult<()> {
			Self::get_config_path().and_then(|path| {
				File::create(path)
					.map_err(|_| ThemeStorageError::DesktopFileCreateError)
					.and_then(|mut file| file.write_all(theme.as_str()).map_err(|_| ThemeStorageError::DesktopFileWriteError))
			})
		}
	}
}

use std::cell::RefCell;

// Default in-memory storage for platforms without specific implementations
pub struct MemoryStorage {
	theme: RefCell<Option<Theme>>,
}
impl MemoryStorage {
	pub fn new() -> Self {
		Self { theme: RefCell::new(None) }
	}
}

impl MemoryStorage {
	pub fn current_theme(&self) -> Option<Theme> {
		self.theme.borrow().clone()
	}
}

impl ThemeStorage for MemoryStorage {
	fn set_theme(&self, theme: &Theme) -> ThemeStorageResult<()> {
		*self.theme.borrow_mut() = Some(theme.clone());
		Ok(())
	}

	fn get_theme(&self) -> ThemeStorageResult<Option<Theme>> {
		Ok(self.theme.borrow().clone())
	}
}

// Get appropriate storage based on the platform
pub fn get_storage() -> Box<dyn ThemeStorage> {
	#[cfg(feature = "web")]
	{
		Box::new(web::WebStorage);
	}

	#[cfg(all(feature = "desktop", not(feature = "web")))]
	{
		Box::new(desktop::DesktopStorage);
	}

	#[cfg(not(any(feature = "web", feature = "desktop")))]
	{
		Box::new(MemoryStorage::new());
	}
}
