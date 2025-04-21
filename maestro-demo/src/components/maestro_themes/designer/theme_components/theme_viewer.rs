// The component that displays the generated theme to the user (for copy)

use async_std::task::sleep;
use dioxus::prelude::*;
use dioxus_free_icons::{
	Icon,
	icons::io_icons::{IoCloseCircle, IoCopyOutline},
};
use maestro_hooks::clipboard::use_clipboard;
use syntect::{
	easy::HighlightLines,
	highlighting::ThemeSet,
	html::{IncludeBackground, styled_line_to_highlighted_html},
	parsing::SyntaxSet,
};
use tailwind_fuse::tw_join;

use crate::components::maestro_themes::{
	designer::state::DesignerState,
	exporter::{ThemeOptions, export_theme},
};

#[derive(Props, PartialEq, Clone)]
pub struct ThemeViewerProps {
	pub state: DesignerState,
	pub theme_options: ThemeOptions,
	pub show_generated_themes: Signal<bool>,
}

#[component]
pub fn ThemeViewer(mut props: ThemeViewerProps) -> Element {
	let theme_viewer_props = props.clone();
	let mut is_generating = use_signal(|| false);
	let mut stylesheet = use_signal(String::new);

	let clipboard = use_clipboard();
	let mut copy_status = use_signal(String::new);
	let mut is_copying = use_signal(|| false);

	// syntax highlighting function
	let highlight_code = move |code: &str, lang: &str| -> String {
		// loading syntax set and theme set
		let syntax_set = SyntaxSet::load_defaults_newlines();
		let theme_set = ThemeSet::load_defaults();

		// determining the syntax to use
		let syntax = syntax_set
			.find_syntax_by_token(lang)
			.unwrap_or_else(|| syntax_set.find_syntax_by_extension(theme_viewer_props.theme_options.format.extension()).unwrap());

		let theme = &theme_set.themes["GitHub"];

		// highlight code
		let mut highlighter = HighlightLines::new(syntax, theme);

		code
			.lines()
			.map(|line| {
				let regions = highlighter.highlight_line(line, &syntax_set).unwrap();
				styled_line_to_highlighted_html(&regions[..], IncludeBackground::Yes).unwrap_or_else(|_| line.to_string())
			})
			.collect::<Vec<String>>()
			.join("\n")
	};

	let handle_copy = move |_| {
		let content = stylesheet();
		let mut clipboard = clipboard.clone();
		is_copying.set(true);
		spawn(async move {
			match clipboard.set(content).await {
				Ok(_) => copy_status.set("Copied!".to_string()),
				Err(_) => copy_status.set("Failed to copy".to_string()),
			}
			is_copying.set(false);
			spawn(async move {
				sleep(std::time::Duration::from_secs(2)).await;
				copy_status.set(String::new());
			});
		});
	};

	use_effect(move || {
		is_generating.set(true);
		stylesheet.set(highlight_code(export_theme(&theme_viewer_props.state, &props.theme_options).as_str(), props.theme_options.format.language()));
		is_generating.set(false);
	});

	rsx! {
		div { class: "p-4 w-full max-w-lg mx-auto z-50",
			h1 { class: "text-2xl font-bold mb-4", "Stylesheet Preview" }
			if is_generating() {
				div { class: "p-4 mb-4 text-blue-700 bg-blue-100 rounded border border-blue-400",
					"Generating stylesheet..."
				}
			}
			div { class: "relative",
				button {
					r#type: "button",
					class: "text-slate-300 hover:text-slate-100 transition-colors",
					disabled: "{is_copying()}",
					onclick: handle_copy,
					title: "Copy Code",
					Icon {
						icon: IoCopyOutline,
						width: 20,
						height: 20,
						class: "fill-none",
					}
				}
				div {
					class: tw_join!(
							"absolute -bottom-8 left-1/2 transform -translate-x-1/2 bg-slate-900 text-white text-xs py-1 px-2 rounded transition-opacity duration-300 {}",
							if copy_status().is_empty() { "opacity-0" } else { "opacity-100" }
					),
					"{copy_status}"
				}

				button {
					class: "px-4 py-2 border border-gray-300 rounded hover:bg-gray-100",
					onclick: move |_| props.show_generated_themes.set(false),
					Icon {
						icon: IoCloseCircle,
						width: 20,
						height: 20,
						class: "fill-none",
					}
				}
			}
			if !stylesheet().is_empty() {
				div {
					class: "font-mono text-sm whitespace-pre p-4 mt-4",
					// dangerous_inner_html to render the highlighted code
					dangerous_inner_html: "{stylesheet()}",
					label { class: "block mb-2 font-medium", "Generated Sheet:" }
				}
			} else if !is_generating() && stylesheet().is_empty() {
				div { class: "p-4 mt-4 text-gray-100 bg-gray-100 rounded border border-gray-300 text-center",
					"There was a problem generating the stylesheet."
				}
			}
		}
	}
}
