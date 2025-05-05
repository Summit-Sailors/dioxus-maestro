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

use crate::maestro_themes::{
	editor::state::DesignerState,
	exporter::{ThemeOptions, export_theme},
};

#[derive(Props, PartialEq, Clone)]
pub struct ThemeViewerProps {
	pub theme_options: ThemeOptions,
	pub show_generated_themes: Signal<bool>,
}

#[component]
pub fn ThemeViewer(mut props: ThemeViewerProps) -> Element {
	let state = use_context::<Signal<DesignerState>>();
	let theme_viewer_props = props.clone();
	let mut is_generating = use_signal(|| false);

	// the raw theme content separately for copying
	let mut raw_stylesheet = use_signal(String::new);
	// the highlighted HTML for display
	let mut highlighted_stylesheet = use_signal(String::new);

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

		let theme = &theme_set.themes["base16-ocean.dark"];

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
		// Use the raw content for copying, not the highlighted HTML
		let content = raw_stylesheet();
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
		let raw_content = export_theme(&state(), &props.theme_options);
		raw_stylesheet.set(raw_content.clone());
		highlighted_stylesheet.set(highlight_code(raw_content.as_str(), props.theme_options.format.language()));
		is_generating.set(false);
	});

	rsx! {
		div { class: "p-4 w-full max-w-lg mx-auto z-[9999] overflow-y-auto bg-[var(--card-bg)] text-[var(--card-text)] rounded-xl shadow-lg",
			h1 { class: "text-2xl font-bold mb-4 text-[var(--text-color)]", "Stylesheet Preview" }
			if is_generating() {
				div { class: "p-4 mb-4 text-[var(--text-color)] bg-[var(--highlight-color)] rounded border border-[var(--border-color)]",
					"Generating stylesheet..."
				}
			}
			div { class: "relative flex items-center justify-between gap-2 mb-2",
				button {
					r#type: "button",
					class: "text-[var(--muted-text)] hover:text-[var(--text-color)] transition-colors",
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
							"absolute -bottom-8 left-1/2 transform -translate-x-1/2 text-xs py-1 px-2 rounded transition-opacity duration-300 bg-[var(--accent-bg)] text-[var(--accent-text)]",
							if copy_status().is_empty() { "opacity-0" } else { "opacity-100" }
					),
					"{copy_status}"
				}
				button {
					class: "px-4 py-2 border border-[color:var(--border-color)] rounded hover:bg-[var(--hover-bg)] text-[color:var(--text-color)]",
					onclick: move |_| props.show_generated_themes.set(false),
					Icon {
						icon: IoCloseCircle,
						width: 20,
						height: 20,
						class: "fill-none",
					}
				}
			}
			if !highlighted_stylesheet().is_empty() {
				div { class: "relative flex-1 w-full rounded-lg flex flex-col bg-[color:var(--bg-color)] lg:px-16 sm:px-6 px-2 py-8 h-full overflow-x-auto mt-8",

					div { class: "flex w-full justify-between items-center bg-[color:var(--secondary-bg)] text-[color:var(--secondary-text)] text-xs px-4 py-2 rounded-t-md",
						span { class: "font-mono", "Generated Stylesheet" }
						div { class: "flex gap-1",
							span { class: "w-3 h-3 bg-red-500 rounded-full" }
							span { class: "w-3 h-3 bg-yellow-500 rounded-full" }
							span { class: "w-3 h-3 bg-green-500 rounded-full" }
						}
					}

					div {
						class: "font-mono text-sm whitespace-pre p-4 text-[color:var(--text-color)]",
						// dangerous_inner_html to render the highlighted code
						dangerous_inner_html: "{highlighted_stylesheet()}",
					}
				}
			} else if !is_generating() && highlighted_stylesheet().is_empty() {
				div { class: "p-4 mt-4 text-[var(--muted-text)] bg-[var(--input-bg)] rounded border border-[var(--border-color)] text-center",
					"There was a problem generating the stylesheet."
				}
			}
		}
	}
}
