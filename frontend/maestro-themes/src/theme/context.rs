//! Theme context and hooks (Unified context provider)
use {
	crate::theme::{
		storage::{ThemeStorage, get_storage},
		system::{SytstemThemeDetector, get_system_theme_detector},
		types::{ResolvedTheme, Theme},
	},
	dioxus::prelude::*,
	std::{cell::RefCell, collections::HashMap, rc::Rc},
};

/// Theme context to be used throughout the application

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeContext {
	/// Theme preference (e.g., "light", "dark", "custom")
	pub theme: Signal<Theme>,
	/// Actual theme being applied
	pub resolved_theme: Signal<ResolvedTheme>,
	/// System preference (true means dark mode)
	pub system_prefers_dark: Signal<bool>,
	/// Function to set theme
	pub set_theme: Rc<RefCell<dyn FnMut(Theme)>>,
}

/// Storing the Theme state
pub static THEME_ATOM: Atom<Theme> = |_| Theme::System;

// hooks to access theme context
pub fn use_theme() -> ThemeContext {
	use_context::<ThemeContext>().expect("ThemeContext not found. Make sure to wrap your app with ThemeProvider.")
}

/// Current theme class for CSS
pub fn use_theme_class() -> String {
	let theme_ctx = use_theme();
	use_memo(move || theme_ctx.resolved_theme.read().deref().as_class().to_string())
}

// Update document class for theming
#[cfg(feature = "web")]
fn set_document_theme(theme_class: &str) {
	use web_sys::window;

	if let Some(window) = window() {
		if let Some(document) = window.document() {
			if let Some(element) = document.document_element() {
				let _ = element.set_attribute("data-theme", theme_class);

				// for tailwind class approach
				if theme_class == "dark" {
					let _ = element.class_list().add_1("dark");
				} else {
					let _ = element.class_list().remove_1("dark");
				}
			}
		}
	}
}

// Update document class for theming (Desktop)
#[cfg(all(feature = "desktop", not(feature = "web")))]
fn set_document_theme(theme_class: &str) {
	// The current window instance
	if let Some(window) = use_window() {
		#[cfg(target_os = "windows")]
		if let Some(hwnd) = window.hwnd() {
			use windows::{Win32::UI::Controls::DarkMode::SetWindowTheme, core::PCWSTR};

			let theme_value = if theme_class == "dark" { "DarkMode_Explorer" } else { "Explorer" };
			unsafe {
				SetWindowTheme(hwnd, PCWSTR::from_raw(theme_value.encode_utf16().collect::<Vec<_>>().as_ptr()), PCWSTR::null());
			}
		}

		// Set CSS variables in document for theming support
		let js = format!(
			r#"
    document.documentElement.style.setProperty('data-theme', '{}');
    document.body.classList.remove('light', 'dark');
    document.body.classList.add('{}');
    "#,
			theme_class, theme_class
		);

		if let Err(e) = eval(&js) {
			log::warn!("Failed to set document theme: {}", e);
		}

		#[cfg(target_os = "macos")]
		{
			use {
				cocoa::appkit::{NSAppearance, NSAppearanceNameAqua, NSAppearanceNameDarkAqua},
				objc::{msg_send, sel, sel_impl},
			};

			let is_dark = theme_class == "dark";
			if let Some(ns_window) = ns_window() {
				unsafe {
					let appearance = if is_dark { NSAppearance::appearanceNamed(NSAppearanceNameDarkAqua) } else { NSAppearance::appearanceNamed(NSAppearanceNameAqua) };
					let _: () = msg_send![ns_window, setAppearance: appearance];
				}
			}
		}

		#[cfg(target_os = "linux")]
		{
			use std::process::Command;
			if theme_class == "dark" {
				// try setting GTK theme variant if app uses GTK
				std::env::set_var("GTK_THEME_VARIANT", "dark");
				// for desktop environments that support it, we can try setting the theme
				let _ = Command::new("gsettings")
					.arg("set")
					.arg("org.gnome.desktop.interface")
					.arg("gtk-theme")
					.arg("Adwaita-dark")
					.output()
					.expect("Failed to set GTK theme");
			} else {
				std::env::set_var("GTK_THEME_VARIANT", "light");
				let _ =
					Command::new("gsettings").arg("set").arg("org.gnome.desktop.interface").arg("gtk-theme").arg("Adwaita").output().expect("Failed to set GTK theme");
			}

			// Changing system theme might require elevated permissions
			// so this might not work in all environments
		}

		// Emit a theme change event that can be caught by other components
		emit("theme_change", theme_class);
	}
}

// fallback for other platforms
#[cfg(all(not(feature = "web"), not(feature = "desktop")))]
fn set_document_theme(_theme_class: &str) {
	// No-op for platforms other than web and desktop
}
