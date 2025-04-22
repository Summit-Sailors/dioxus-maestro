use dioxus::prelude::*;
use maestro_toast::{ctx::use_toast, toast_code::EToastCode, toast_info::ToastInfo, toast_position::EToastPosition};

use crate::components::ui::features::Features;

#[component]
pub fn ToastDemo() -> Element {
	let mut toast = use_toast();

	let show_success = move |_| {
		let info = ToastInfo {
			heading: Some("Success!".to_string()),
			context: "Operation completed successfully".to_string(),
			icon: Some(EToastCode::Success),
			position: EToastPosition::TopRight,
			allow_toast_close: true,
			hide_after: 5,
		};
		toast.write().popup(info);
	};

	let show_error = move |_| {
		let info = ToastInfo {
			heading: Some("Error".to_string()),
			context: "Something went wrong".to_string(),
			icon: Some(EToastCode::Error),
			position: EToastPosition::BottomRight,
			allow_toast_close: true,
			hide_after: 8,
		};
		toast.write().popup(info);
	};

	let show_warning = move |_| {
		let info = ToastInfo {
			heading: None,
			context: "This is a warning message".to_string(),
			icon: Some(EToastCode::Warning),
			position: EToastPosition::BottomLeft,
			allow_toast_close: false,
			hide_after: 6,
		};
		toast.write().popup(info);
	};

	let show_info = move |_| {
		let info = ToastInfo {
			heading: Some("Info".to_string()),
			context: "Here's some information".to_string(),
			icon: Some(EToastCode::Info),
			position: EToastPosition::TopLeft,
			allow_toast_close: true,
			hide_after: 7,
		};
		toast.write().popup(info);
	};

	let show_custom = move |_| {
		let info = ToastInfo {
			heading: Some("Custom Toast".to_string()),
			context: "This is a custom toast without an icon".to_string(),
			icon: None,
			position: EToastPosition::TopRight,
			allow_toast_close: true,
			hide_after: 5,
		};
		toast.write().popup(info);
	};

	let clear_all = move |_| {
		toast.write().clear();
	};

	rsx! {
		div {
			id: "maestro-toast",
			class: "bg-[color:var(--bg-color)] rounded-lg shadow-md p-4 w-full",

			div { id: "maestro-toast-header", class: "mb-8",
				h1 { class: "text-[color:var(--text-color)] text-center text-3xl font-bold mb-2",
					"Maestro Toast"
				}
				p { class: "text-[color:var(--muted-text)] text-center",
					"A powerful, flexible, and intuitive toast notification system designed specifically for Dioxus applications. While Dioxus provides basic UI capabilities,
					our toast component takes notification management to the next level with its robust feature set and elegant design."
				}
			}

			div { id: "maestro-toast-features", class: "flex space-x-2",
				Features {
					title: "Features".to_string(),
					features: vec![
							"No Manual State Management: Automatic toast tracking and removal".to_string(),
							"Flexible Configuration: Granular control over toast behavior".to_string(),
							"Built-in Best Practices: Sensible defaults with full customization".to_string(),
							"Different positions (check each toast's position)".to_string(),
							"Various toast types with matching icons and colors".to_string(),
							"Configurable auto-close timers (5-8 seconds)".to_string(),
							"Optional close buttons".to_string(),
							"Optional headers".to_string(),
							"Custom toast without icon".to_string(),
					],
				}
			}

			div {
				id: "maestro-toast-controls",
				class: "grid grid-cols-1 justify-center md:grid-cols-3 gap-4 mb-6 p-6",
				button {
					class: "px-2 py-2 bg-green-500 text-white text-center rounded hover:bg-green-600",
					onclick: show_success,
					"Success"
				}
				button {
					class: "px-2 py-2 bg-red-500 text-white rounded hover:bg-red-600",
					onclick: show_error,
					"Error"
				}
				button {
					class: "px-2 py-2 bg-yellow-500 text-white rounded hover:bg-yellow-600",
					onclick: show_warning,
					"Warning"
				}
				button {
					class: "px-2 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
					onclick: show_info,
					"Info"
				}
				button {
					class: "px-2 py-2 bg-slate-500 text-white rounded hover:bg-slate-600",
					onclick: show_custom,
					"Custom"
				}
				button {
					class: "px-2 py-2 bg-slate-700 text-white rounded hover:bg-slate-800",
					onclick: clear_all,
					"Clear All"
				}
			}
		}
	}
}
