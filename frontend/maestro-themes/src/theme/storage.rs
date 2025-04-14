// Platform specific storage for theme settings

use std::fmt::Display;

use crate::theme::types::Theme;

#[derive(Debug)]
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
	use web_sys::{Storage, window};

	use super::*;

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
					.map_err(|_e| ThemeStorageError::WebStorageGetError)?
					.map(|theme_str| Theme::from_str_slice(&theme_str).map_err(|e| ThemeStorageError::ThemeDeserializationError(e)))
					.transpose()
			})
		}
	}
}

#[cfg(feature = "desktop")]
pub mod desktop {
	use std::{
		fs::{self, File},
		io::{Read, Write},
		path::PathBuf,
	};

	use directories::ProjectDirs;

	use super::*;

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
						Theme::from_str_slice(contents.trim()).map(Some).map_err(|e| ThemeStorageError::ThemeDeserializationError(e))
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
					.and_then(|mut file| file.write_all(theme.as_str().as_bytes()).map_err(|_| ThemeStorageError::DesktopFileWriteError))
			})
		}
	}
}

// Default in-memory storage for platforms without specific implementations (assumes mobile for now)
#[cfg(not(any(feature = "web", feature = "desktop")))]
pub mod mobile {
	use std::sync::{Arc, Mutex};

	use super::*;

	pub struct MemoryStorage {
		theme: Arc<Mutex<Option<Theme>>>,
	}
	impl MemoryStorage {
		pub fn new() -> Self {
			Self { theme: Arc::new(Mutex::new(None::<Theme>)) }
		}
	}

	impl MemoryStorage {
		pub fn current_theme(&self) -> Option<Theme> {
			self.theme.lock().ok().and_then(|guard| guard.clone())
		}
	}

	impl ThemeStorage for MemoryStorage {
		fn set_theme(&self, theme: &Theme) -> ThemeStorageResult<()> {
			match self.theme.lock() {
				Ok(mut guard) => {
					*guard = Some(theme.clone());
					Ok(())
				},
				Err(_) => Err(ThemeStorageError::ThemeSerializationError("Failed to acquire lock".to_string()))?,
			}
		}

		fn get_theme(&self) -> ThemeStorageResult<Option<Theme>> {
			self.theme.lock().map(|guard| guard.clone()).map_err(|_| ThemeStorageError::ThemeDeserializationError("Failed to acquire lock".to_string()))
		}
	}
}

// Get appropriate storage based on the platform
pub fn get_storage() -> Box<dyn ThemeStorage> {
	#[cfg(feature = "web")]
	{
		Box::new(web::WebStorage)
	}

	#[cfg(all(feature = "desktop", not(feature = "web")))]
	{
		Box::new(desktop::DesktopStorage)
	}

	#[cfg(not(any(feature = "web", feature = "desktop")))]
	{
		Box::new(mobile::MemoryStorage::new())
	}
}
