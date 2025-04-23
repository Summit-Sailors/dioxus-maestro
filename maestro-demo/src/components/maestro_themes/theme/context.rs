//! Theme context and hooks (Unified context provider)
use dioxus::prelude::*;

use crate::components::maestro_themes::theme::types::{ResolvedTheme, Theme};

/// Theme context to be used throughout the application

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ThemeContext {
	/// Theme preference (e.g., "light", "dark", "auto")
	pub theme: Signal<Option<Theme>>,
	/// Actual theme being applied
	pub resolved_theme: Signal<ResolvedTheme>,
	/// System preference
	pub system_prefers_dark: Signal<bool>,
	/// Function to set theme
	pub set_theme: Callback<Theme>,
}

// hooks to access theme context
pub fn use_theme() -> ThemeContext {
	use_context::<ThemeContext>()
}

// Update document class for theming
#[cfg(all(any(feature = "web", feature = "desktop"), not(feature = "server")))]
pub fn set_document_theme(theme_class: &str) {
	use web_sys::window;

	if let Some(window) = window() {
		if let Some(document) = window.document() {
			if let Some(element) = document.document_element() {
				let _ = element.set_attribute("data-theme", theme_class);
			}
		}
	}
}

// Update document class for theming (Desktop)
#[cfg(all(feature = "desktop", not(feature = "web")))]
pub fn set_document_theme(theme_class: &str) {
	// The current window instance
	if let window = dioxus_desktop::window() {
		#[cfg(target_os = "windows")]
		{
			let hwnd = window.window_handle();
			use windows::{
				Win32::{
					Foundation::*,
					System::Threading::*,
					UI::{Controls::SetWindowTheme, WindowsAndMessaging::*},
				},
				core::*,
			};

			let theme_value = if theme_class == "dark" { "DarkMode_Explorer" } else { "Explorer" };
			let wide: Vec<u16> = theme_value.encode_utf16().chain(std::iter::once(0)).collect(); // null-terminated
			let wide_ptr = PCWSTR::from_raw(wide.as_ptr());
			unsafe {
				SetWindowTheme(hwnd, wide_ptr, PCWSTR::null());
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

		let _ = document::eval(&js);

		#[cfg(target_os = "macos")]
		{
			use cocoa::appkit::{NSAppearance, NSAppearanceNameVibrantDark, NSAppearanceNameVibrantLight, NSWindow};
			use dioxus_desktop::tao::platform::macos::WindowExtMacOS;
			use objc::{msg_send, sel, sel_impl};

			let is_dark = theme_class == "dark";
			let ns_window = window.ns_window();
			unsafe {
				let appearance = if is_dark { NSAppearance(NSAppearanceNameVibrantDark) } else { NSAppearance(NSAppearanceNameVibrantLight) };
				let _: () = msg_send![ns_window, setAppearance: appearance];
			}
		}

		#[cfg(target_os = "linux")]
		{
			use std::process::Command;
			if theme_class == "dark" {
				// try setting GTK theme variant if app uses GTK
				unsafe { std::env::set_var("_GTK_THEME_VARIANT", "dark") };
				// for desktop environments that support it, we can try setting the theme
				let _ = Command::new("gsettings")
					.arg("set")
					.arg("org.gnome.desktop.interface")
					.arg("gtk-theme")
					.arg("Adwaita-dark")
					.output()
					.expect("Failed to set GTK theme");
			} else {
				unsafe { std::env::set_var("_GTK_THEME_VARIANT", "light") };
				let _ =
					Command::new("gsettings").arg("set").arg("org.gnome.desktop.interface").arg("gtk-theme").arg("Adwaita").output().expect("Failed to set GTK theme");
			}

			// Changing system theme might require elevated permissions
			// so this might not work in all environments
		}
	}
}

// fallback for other platforms
#[cfg(all(not(feature = "web"), not(feature = "desktop")))]
pub fn set_document_theme(_theme_class: &str) {
	// No-op for platforms other than web and desktop
}
