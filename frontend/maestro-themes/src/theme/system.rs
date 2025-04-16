// Platform specific theme detection

pub trait ThemeChangeCallback: FnMut(bool) + 'static {}
impl<F: FnMut(bool) + 'static> ThemeChangeCallback for F {}

pub trait SystemThemeDetector {
	fn prefers_dark_mode(&self) -> bool;
	fn listen_for_theme_changes(&self, callback: Box<dyn ThemeChangeCallback>);
}

#[cfg(feature = "web")]
pub mod web {
	use wasm_bindgen::prelude::*;
	use web_sys::{MediaQueryListEvent, window};

	use super::*;

	pub struct WebThemeDetector;

	impl SystemThemeDetector for WebThemeDetector {
		fn prefers_dark_mode(&self) -> bool {
			if let Some(window) = window() {
				if let Ok(Some(media_query)) = window.match_media("(prefers-color-scheme: dark)") {
					return media_query.matches();
				}
			}
			false
		}

		fn listen_for_theme_changes(&self, mut callback: Box<dyn ThemeChangeCallback>) {
			if let Some(window) = window() {
				if let Ok(Some(media_query)) = window.match_media("(prefers-color-scheme: dark)") {
					let listener = Closure::wrap(Box::new(move |e: MediaQueryListEvent| {
						callback(e.matches());
					}) as Box<dyn FnMut(_)>);

					let _ = media_query.add_event_listener_with_callback("change", listener.as_ref().unchecked_ref());

					// Leak the closure so it's not dropped
					listener.forget();
				}
			}
		}
	}
}

#[cfg(all(feature = "desktop", not(feature = "web")))]
pub mod desktop {
	use std::{
		sync::{Arc, Mutex},
		thread,
		time::Duration,
	};

	use super::*;
	use crate::theme::system::ThemeChangeCallback;

	pub struct DesktopThemeDetector {
		pub dark_theme: Arc<Mutex<bool>>,
	}

	// for mac
	struct ThemeChangeContext<F: FnMut(bool) + 'static + ?Sized> {
		dark_theme: Arc<Mutex<bool>>,
		callback: Mutex<F>,
	}

	impl DesktopThemeDetector {
		pub fn new() -> Self {
			#[cfg(target_os = "macos")]
			{
				let dark_mode = macos_dark_mode();
				Self { dark_theme: Arc::new(Mutex::new(dark_mode)) }
			}
			#[cfg(target_os = "windows")]
			{
				Self { dark_theme: Arc::new(Mutex::new(windows_dark_mode())) }
			}
			#[cfg(target_os = "linux")]
			{
				Self { dark_theme: Arc::new(Mutex::new(linux_dark_mode())) }
			}
			#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
			Self { dark_theme: Arc::new(Mutex::new(true)) }
		}
	}

	impl SystemThemeDetector for DesktopThemeDetector {
		fn prefers_dark_mode(&self) -> bool {
			*self.dark_theme.lock().unwrap()
		}

		fn listen_for_theme_changes(&self, callback: Box<dyn ThemeChangeCallback>) {
			#[cfg(target_os = "macos")]
			{
				use std::os::raw::{c_int, c_void};

				#[cfg(target_os = "macos")]
				use core_foundation::{
					base::{CFAllocatorRef, CFRelease, CFTypeRef, kCFAllocatorDefault},
					dictionary::{CFDictionaryGetValue, CFDictionaryRef},
					string::{CFStringCreateWithCString, CFStringRef, kCFStringEncodingUTF8},
				};

				unsafe extern "C" {
					fn CFNotificationCenterAddObserver(
						center: CFTypeRef,
						observer: *const c_void,
						callback: extern "C" fn(*mut c_void, CFTypeRef, CFStringRef, *const c_void, CFDictionaryRef),
						name: CFStringRef,
						object: *const c_void,
						suspensionBehavior: c_int,
					);

					fn CFNotificationCenterGetDistributedCenter() -> CFTypeRef;

					fn CFStringCreateWithCString(allocator: CFAllocatorRef, cstr: *const i8, encoding: u32) -> CFStringRef;
				}

				let dark_theme = self.dark_theme.clone();
				let context = Box::new(ThemeChangeContext { dark_theme, callback: Mutex::new(callback) });
				let context_ptr = Box::into_raw(context);

				extern "C" fn theme_change_callback(
					info: *mut ThemeChangeContext<Box<dyn ThemeChangeCallback>>,
					_observer: CFTypeRef,
					_name: CFStringRef,
					_object: *const c_void,
					_user_info: CFDictionaryRef,
				) {
					unsafe {
						let context = &mut *info;
						let context = &mut *context;
						let is_dark = macos_dark_mode();
						// Update stored theme value
						if let Ok(mut current) = context.dark_theme.lock() {
							*current = is_dark;
						}

						if let Ok(mut callback) = context.callback.lock() {
							callback(is_dark);
						}
					}
				}

				unsafe {
					let notification_center = CFNotificationCenterGetDistributedCenter();
					let notification_name = CFStringCreateWithCString(
						kCFAllocatorDefault,
						std::ffi::CStr::from_bytes_with_nul_unchecked(b"AppleInterfaceThemeChangedNotification\0").as_ptr() as *const i8,
						kCFStringEncodingUTF8,
					);

					CFNotificationCenterAddObserver(
						notification_center,
						context_ptr as *const _,
						theme_change_callback,
						notification_name,
						std::ptr::null(),
						1, // deliver asynchronously
					);

					// release the string
					CFRelease(notification_name as CFTypeRef);
				}
			}
			#[cfg(target_os = "windows")]
			{
				use std::sync::{
					Arc,
					atomic::{AtomicBool, Ordering},
				};

				let dark_theme = self.dark_theme.clone();
				let running = Arc::new(AtomicBool::new(true));
				let running_clone = running.clone();

				std::thread::spawn(move || {
					use winreg::{RegKey, enums::*};

					let hkcu = RegKey::predef(HKEY_CURRENT_USER);
					let path = r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
					let mut last_value = windows_dark_mode();

					while running_clone.load(Ordering::Relaxed) {
						std::thread::sleep(Duration::from_secs(1));

						let current_value = windows_dark_mode();
						if current_value != last_value {
							last_value = current_value;
							if let Ok(mut current) = dark_theme.lock() {
								*current = current_value;
							}
							callback(current_value);
						}
					}
				});
			}

			#[cfg(target_os = "linux")]
			{
				let dark_theme = self.dark_theme.clone();
				std::thread::spawn(move || {
					use std::process::Command;
					let mut last_value = linux_dark_mode();

					loop {
						std::thread::sleep(Duration::from_secs(1));

						let current_value = linux_dark_mode();
						if current_value != last_value {
							last_value = current_value;
							if let Ok(mut current) = dark_theme.lock() {
								*current = current_value;
							}
							callback(current_value);
						}
					}
				});
			}

			#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
			{
				// Fallback for unsupported platforms
				let dark_theme = self.dark_theme.clone();
				std::thread::spawn(move || {
					loop {
						std::thread::sleep(Duration::from_secs(1));
						let is_dark = true; // Default to dark mode
						if let Ok(mut current) = dark_theme.lock() {
							*current = is_dark;
						}
						callback(is_dark);
					}
				});
			}
		}
	}

	#[cfg(target_os = "macos")]
	fn macos_dark_mode() -> bool {
		use std::{ffi::CStr, os::raw::c_void};

		use core_foundation::{
			base::{CFRelease, CFTypeRef, TCFType},
			dictionary::{CFDictionaryGetValue, CFDictionaryRef},
			number::{CFNumberGetValue, CFNumberRef},
			string::{CFStringGetCStringPtr, CFStringRef, kCFStringEncodingUTF8},
		};

		unsafe extern "C" {
			fn CFPreferencesCopyAppValue(key: CFStringRef, app_id: CFStringRef) -> CFTypeRef;
			fn CFStringCreateWithCString(allocator: *const c_void, cstr: *const i8, encoding: u32) -> CFStringRef;
			fn CFPreferencesSynchronize(app_id: CFStringRef, user: CFStringRef, host: CFStringRef) -> bool;
			fn CFCopyTypeIDDescription(type_id: usize) -> CFStringRef;
			fn CFGetTypeID(cf: CFTypeRef) -> usize;
			static mut kCFPreferencesCurrentApplication: CFStringRef;
			static mut kCFPreferencesCurrentUser: CFStringRef;
			static mut kCFPreferencesAnyHost: CFStringRef;
		}

		unsafe {
			CFPreferencesSynchronize(kCFPreferencesCurrentApplication, kCFPreferencesCurrentUser, kCFPreferencesAnyHost);
			let key =
				CFStringCreateWithCString(std::ptr::null(), CStr::from_bytes_with_nul_unchecked(b"AppleInterfaceStyle\0").as_ptr() as *const i8, kCFStringEncodingUTF8);

			let value = CFPreferencesCopyAppValue(key, kCFPreferencesCurrentApplication);
			CFRelease(key as CFTypeRef);

			if !value.is_null() {
				let is_dark = {
					let ptr = CFStringGetCStringPtr(value as CFStringRef, kCFStringEncodingUTF8);
					if !ptr.is_null() {
						let cstr = CStr::from_ptr(ptr);
						if let Ok(s) = cstr.to_str() { s == "Dark" } else { false }
					} else {
						false
					}
				};

				CFRelease(value as CFTypeRef);
				is_dark
			} else {
				true // dark theme
			}
		}
	}

	#[cfg(target_os = "windows")]
	fn windows_dark_mode() -> bool {
		use winreg::{RegKey, enums::*};

		let hkcu = RegKey::predef(HKEY_CURRENT_USER);
		if let Ok(personalize) = hkcu.open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize") {
			if let Ok(value) = personalize.get_value::<u32, _>("AppsUseLightTheme") {
				return value == 0;
			}
		}
		false
	}

	#[cfg(target_os = "linux")]
	fn linux_dark_mode() -> bool {
		use std::process::Command;

		// try with GNOME first
		if let Ok(output) = Command::new("gsettings").arg("get").arg("org.gnome.desktop.interface").arg("color-scheme").output() {
			if let Ok(stdout) = String::from_utf8(output.stdout) {
				if stdout.contains("dark") {
					return true;
				}
			}
		}

		// try alternative gtk setting
		if let Ok(output) = Command::new("gsettings").arg("get").arg("org.gtk.Settings").arg("gtk-theme").output() {
			if let Ok(stdout) = String::from_utf8(output.stdout) {
				if stdout.to_lowercase().contains("dark") {
					return true;
				}
			}
		}

		// try kde plasma
		if let Ok(output) = Command::new("kreadconfigs").arg(&["--group", "General", "--key", "ColorScheme", "--file", "kdeglobals"]).output() {
			if let Ok(stdout) = String::from_utf8(output.stdout) {
				if stdout.to_lowercase().contains("dark") {
					return true;
				}
			}
		}

		// try xfce
		if let Ok(output) = Command::new("xfconf-query").arg(&["-c", "xsettings", "-p", "/Net/ThemeName"]).output() {
			if let Ok(stdout) = String::from_utf8(output.stdout) {
				if stdout.to_lowercase().contains("dark") {
					return true;
				}
			}
		}

		false
	}
}

#[cfg(not(any(feature = "web", feature = "desktop")))]
pub mod mobile {
	use crate::system::{SystemThemeDetector, ThemeChangeCallback};
	// Default implementation for unsupported platforms
	pub struct DefaultThemeDetector;

	impl SystemThemeDetector for DefaultThemeDetector {
		fn prefers_dark_mode(&self) -> bool {
			true
		}

		fn listen_for_theme_changes(&self, _callback: Box<dyn ThemeChangeCallback>) {
			// No-op for unsupported platforms
		}
	}
}

// Get appropriate detector for the current platform
pub fn get_system_theme_detector() -> Box<dyn SystemThemeDetector> {
	#[cfg(feature = "web")]
	{
		Box::new(web::WebThemeDetector) as Box<dyn SystemThemeDetector>
	}

	#[cfg(all(feature = "desktop", not(feature = "web")))]
	{
		Box::new(desktop::DesktopThemeDetector::new()) as Box<dyn SystemThemeDetector>
	}

	#[cfg(not(any(feature = "web", feature = "desktop")))]
	{
		Box::new(mobile::DefaultThemeDetector) as Box<dyn SystemThemeDetector>
	}
}
