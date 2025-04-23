use std::collections::HashMap;

use async_std::task::sleep;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::io_icons::IoCopyOutline};
use maestro_hooks::clipboard::use_clipboard;
use maestro_ui::{
	button::{Button, ButtonSize, ButtonVariant},
	select::{Select, SelectOption},
};
use syntect::{
	easy::HighlightLines,
	highlighting::ThemeSet,
	html::{IncludeBackground, styled_line_to_highlighted_html},
	parsing::SyntaxSet,
};
use tailwind_fuse::tw_join;

use crate::components::icons::{collapse::Collapse, expand::Expand};

#[derive(Clone, PartialEq, Props)]
pub struct CodeEditorProps {
	#[props(default = "rust".to_string())]
	language: String,
	#[props(into, default = String::from("Example Code"))]
	title: String,
	#[props(into)]
	demo: Element,
	code_map: HashMap<String, String>,
}

#[component]
pub fn CodeEditor(props: CodeEditorProps) -> Element {
	let code = use_signal(|| props.code_map.clone());
	let mut is_expanded = use_signal(|| false);
	let clipboard = use_clipboard();
	let mut copy_status = use_signal(String::new);
	let mut is_copying = use_signal(|| false);

	// the currently selected code
	let file_keys: Vec<SelectOption<String>> = props.code_map.keys().cloned().map(|value| SelectOption { label: value.clone(), value: value.clone() }).collect();
	let mut selected_file = use_signal(|| file_keys.first().map(|value| value.value.clone()).unwrap_or("".to_string()).clone());

	let handle_copy = move |_| {
		let content = code().get(&selected_file()).unwrap_or(&"".to_string()).clone();
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

	let toggle_expanded = move |_| {
		is_expanded.toggle();
	};

	// syntax highlighting function
	let highlight_code = move |code: &str, lang: &str| -> String {
		// loading syntax set and theme set
		let syntax_set = SyntaxSet::load_defaults_newlines();
		let theme_set = ThemeSet::load_defaults();

		// determining the syntax to use
		let syntax = syntax_set.find_syntax_by_token(lang).unwrap_or_else(|| syntax_set.find_syntax_by_extension("rs").unwrap());

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

	let current_code = code().get(&selected_file()).unwrap_or(&String::new()).clone();
	let highlighted_code = highlight_code(&current_code, &props.language);

	let action_buttons = rsx! {
		div { class: "text-[color:var(--text-color)] z-10 bg-[color:var(--bg-color)] sticky top-0 left-0 px-5 py-6",
			div { class: "flex flex-col sm:flex-row gap-4 items-center sm:items-start sm:justify-center",
				div { class: "relative",
					Button {
						variant: ButtonVariant::Icon,
						size: ButtonSize::IconMd,
						r#type: "button",
						class: "text-[color:var(--muted-foreground)] hover:text-[color:var(--text-color)] transition-colors",
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
								"absolute -bottom-8 left-1/2 transform -translate-x-1/2 bg-[color:var(--bg-color)] text-[color:var(--text-color)] text-xs py-1 px-2 rounded transition-opacity duration-300 {}",
								if copy_status().is_empty() { "opacity-0" } else { "opacity-100" }
						),
						"{copy_status}"
					}
				}

				Button {
					variant: ButtonVariant::Icon,
					size: ButtonSize::IconMd,
					r#type: "button",
					class: "text-slate-300 hover:text-slate-100 transition-colors",
					onclick: toggle_expanded,
					title: if is_expanded() { "Collapse Code" } else { "View Code" },
					if is_expanded() {
						Collapse { class: "w-5 h-5" }
					} else {
						Expand { class: "w-5 h-5" }
					}
				}
			}
		}
	};

	rsx! {
		div { class: "grid sm:grid-cols-[108px_1fr] grid-cols-[42px_1fr] overflow-y-auto",
			div { class: "border-r border-[color:var(--border-color)] relative",
				{action_buttons.clone()}
			}

			div { class: "lg:px-16 lg:py-16 sm:py-8 sm:px-6 py-6 px-4 bg-[color:var(--bg-color)] h-full w-full flex flex-col overflow-x-hidden",
				// code section
				if is_expanded() {
					div { class: "bg-[color:var(--bg-color)] flex-1 flex flex-col",
						h2 { class: "text-[color:var(--text-color)] text-center text-2xl sm:text-3xl lg:text-4xl 2xl:text-5xl font-semibold mb-3",
							"Source Code"
						}

						div { class: "flex justify-center",

							Select {
								options: file_keys,
								current_value: Some(selected_file.read().to_string()),
								onchange: move |value| selected_file.set(value),
								label: "Single Select",
								placeholder: "Select an option",
								button_class: "text-[color:var(--text-color)] bg-[color:var(--input-bg)]",
								dropdown_class: "text-[color:var(--text-color)] bg-[color:var(--input-bg)] border-[color:var(--border-color)]",
								option_class: "hover:bg-[color:var(--muted)] text-[color:var(--text-color)]",
							}
						}

						div { class: "relative flex-1 rounded-lg flex flex-col bg-[color:var(--bg-color)] lg:px-16 sm:px-6 px-2 py-8 h-full overflow-x-auto mt-8",

							div { class: "flex justify-between items-center bg-[color:var(--secondary-bg)] text-[color:var(--secondary-text)] text-xs px-4 py-2 rounded-t-md",
								span { class: "font-mono", "{selected_file()}" }
								div { class: "flex gap-1",
									span { class: "w-3 h-3 bg-red-500 rounded-full" }
									span { class: "w-3 h-3 bg-yellow-500 rounded-full" }
									span { class: "w-3 h-3 bg-green-500 rounded-full" }
								}
							}

							div {
								class: "font-mono text-sm whitespace-pre p-4 text-[color:var(--text-color)]",
								// dangerous_inner_html to render the highlighted code
								dangerous_inner_html: "{highlighted_code}",
							}
						}
					}
				} else {
					div { class: "w-full h-full", {props.demo} }
				}
			}
		}
	}
}
