// Platform specific storage for theme settings

use crate::theme::types::Theme;

#[derive(Debug)]
pub enum ThemeStorageError {
	WebStorageAccess,
	WebStorageSet,
	WebStorageGet,
	DesktopConfigDir,
	DesktopFileOpen,
	DesktopFileRead,
	DesktopFileCreate,
	DesktopFileWrite,
	ThemeSerialization(String),
	ThemeDeserialization(strum::ParseError),
}

pub type ThemeStorageResult<T> = Result<T, ThemeStorageError>;

// Storage trait for saving/loading theme preferences
pub trait ThemeStorage {
	fn set_theme(&self, theme: &Theme) -> ThemeStorageResult<()>;
	fn get_theme(&self) -> ThemeStorageResult<Option<Theme>>;
}

#[cfg(feature = "web")]
pub mod web {
	use std::str::FromStr;

	use web_sys::window;

	use crate::theme::storage::{Theme, ThemeStorage, ThemeStorageError, ThemeStorageResult};

	pub struct WebStorage;

	impl ThemeStorage for WebStorage {
		fn set_theme(&self, theme: &Theme) -> ThemeStorageResult<()> {
			window()
				.and_then(|win| win.local_storage().ok())
				.flatten()
				.ok_or(ThemeStorageError::WebStorageAccess)
				.and_then(|storage| storage.set_item("maestro-ui-theme-v1", theme.to_string().as_str()).map_err(|_| ThemeStorageError::WebStorageSet))
		}

		fn get_theme(&self) -> ThemeStorageResult<Option<Theme>> {
			window().and_then(|win| win.local_storage().ok()).flatten().ok_or(ThemeStorageError::WebStorageAccess).and_then(|storage| {
				storage
					.get_item("maestro-ui-theme-v1")
					.map_err(|_e| ThemeStorageError::WebStorageGet)?
					.map(|theme_str| Theme::from_str(&theme_str).map_err(ThemeStorageError::ThemeDeserialization))
					.transpose()
			})
		}
	}
}

#[cfg(all(feature = "desktop", not(feature = "web")))]
pub mod desktop {
	use std::{
		fs::{self, File},
		io::{Read, Write},
		path::PathBuf,
		str::FromStr,
	};

	use directories::ProjectDirs;

	use crate::theme::storage::{Theme, ThemeStorage, ThemeStorageError, ThemeStorageResult};

	pub struct DesktopStorage;

	impl DesktopStorage {
		fn get_config_path() -> ThemeStorageResult<PathBuf> {
			ProjectDirs::from("com", "maestro", "dioxus").ok_or(ThemeStorageError::DesktopConfigDir).map(|dirs| {
				let config_dir = dirs.config_dir();
				if !config_dir.exists() {
					fs::create_dir_all(config_dir).map_err(|_| ThemeStorageError::DesktopConfigDir);
				}
				config_dir.join("theme.txt")
			})
		}
	}

	impl ThemeStorage for DesktopStorage {
		fn get_theme(&self) -> ThemeStorageResult<Option<Theme>> {
			Self::get_config_path().and_then(|path| {
				File::open(path)
					.map_err(|_| ThemeStorageError::DesktopFileOpen)
					.and_then(|mut file| {
						let mut contents = String::new();
						file.read_to_string(&mut contents).map_err(|_| ThemeStorageError::DesktopFileRead)?;
						Theme::from_str(contents.trim()).map(Some).map_err(ThemeStorageError::ThemeDeserialization)
					})
					.or_else(|e| match e {
						ThemeStorageError::DesktopFileOpen => Ok(None),
						other_error => Err(other_error),
					})
			})
		}

		fn set_theme(&self, theme: &Theme) -> ThemeStorageResult<()> {
			Self::get_config_path().and_then(|path| {
				File::create(path)
					.map_err(|_| ThemeStorageError::DesktopFileCreate)
					.and_then(|mut file| file.write_all(theme.to_string().as_str().as_bytes()).map_err(|_| ThemeStorageError::DesktopFileWrite))
			})
		}
	}
}

// Default in-memory storage for platforms without specific implementations (assumes mobile for now)
#[cfg(not(any(feature = "web", feature = "desktop")))]
pub mod mobile {
	use std::sync::{Arc, Mutex};

	use crate::theme::storage::{Theme, ThemeStorage, ThemeStorageError, ThemeStorageResult};

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
				Err(_) => Err(ThemeStorageError::ThemeSerialization("Failed to acquire lock".to_string()))?,
			}
		}

		fn get_theme(&self) -> ThemeStorageResult<Option<Theme>> {
			Ok(self.theme.lock().map(|guard| guard.clone()).expect("Failed to get theme from storage"))
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
