use {
	async_std::task::sleep,
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::fa_solid_icons::{FaCopy, FaMaximize, FaMinimize},
		Icon,
	},
	maestro_hooks::clipboard::use_clipboard,
	maestro_ui::select::{Select, SelectOption},
	std::collections::HashMap,
	syntect::{
		easy::HighlightLines,
		highlighting::ThemeSet,
		html::{styled_line_to_highlighted_html, IncludeBackground},
		parsing::SyntaxSet,
	},
	tailwind_fuse::tw_join,
};

#[derive(Props, PartialEq, Clone)]
pub struct CodeEditorProps {
	#[props(default = "rust".to_string())]
	language: String,
	#[props(into, default = String::from("Example Code"))]
	title: String,
	#[props(into)]
	demo: Element,
	#[props(into)]
	menu_toggle: Element,
	#[props(into)]
	backdrop: Element,
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
		let highlighted_html = code
			.lines()
			.map(|line| {
				let regions = highlighter.highlight_line(line, &syntax_set).unwrap();
				styled_line_to_highlighted_html(&regions[..], IncludeBackground::Yes).unwrap_or_else(|_| line.to_string())
			})
			.collect::<Vec<String>>()
			.join("\n");

		highlighted_html
	};

	let current_code = code().get(&selected_file()).unwrap_or(&String::new()).clone();
	let highlighted_code = highlight_code(&current_code, &props.language);

	let action_buttons = rsx! {
		// header section
		div { class: "text-white z-10 bg-gray-900 left-0 rounded-md p-2",
			div { class: "flex flex-col space-y-2",

				div { {props.menu_toggle} }

				div {
					button {
						class: "p-2 rounded-full hover:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 relative",
						disabled: "{is_copying()}",
						onclick: handle_copy,
						title: "Copy Code",
						Icon { icon: FaCopy, width: 20, height: 20 }
						div {
							class: tw_join!(
									"absolute -bottom-8 left-1/2 transform -translate-x-1/2 bg-gray-900 text-white text-xs py-1 px-2 rounded transition-opacity duration-300 {}",
									if copy_status().is_empty() { "opacity-0" } else { "opacity-100" }
							),
							"{copy_status}"
						}
					}
				}

				div {
					button {
						class: "p-2 rounded-full hover:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500",
						onclick: toggle_expanded,
						title: if is_expanded() { "Collapse Code" } else { "View Code" },
						{
								if is_expanded() {
										rsx! {
											Icon { icon: FaMaximize, width: 20, height: 20 }
										}
								} else {
										rsx! {
											Icon { icon: FaMinimize, width: 20, height: 20 }
										}
								}
						}
					}
				}
			}
		}
	};

	rsx! {

		div { class: tw_join!("flex min-h-screen",), {action_buttons.clone()} }

		div { class: "dark p-2 bg-gray-900 border border-gray-700 rounded-lg h-full w-full flex flex-col",

			{props.backdrop}

			// scrollable container for demo and code
			div {
				class: tw_join!(
						"grid grid-cols-1 transition-all duration-500 ease-in-out lg:overflow-hidden", if
						is_expanded() { "lg:grid-cols-2" } else { "grid-cols-1" }
				),

				// demo component section
				div { class: "bg-gray-300 overflow-hidden dark:bg-gray-700 rounded-lg shadow-md border border-gray-800 dark:border-gray-800 mt-4 flex-1",
					div { class: "w-full h-full overflow-auto", {props.demo} }
				}

				// code section
				if is_expanded() {
					div { class: "overflow-hidden rounded-lg bg-gray-900 px-6 flex-1 mt-4 flex flex-col",
						h2 { class: "text-xl font-semibold text-center text-white mt-4",
							"Source Code"
						}

						div { class: "flex justify-center my-3",

							Select {
								options: file_keys,
								current_value: Some(selected_file.read().to_string()),
								onchange: move |value| selected_file.set(value),
								label: "Single Select",
								placeholder: "Select an option",
								button_class: "text-gray-200 bg-gray-800",
								dropdown_class: "text-gray-200 !bg-gray-800 border-gray-700",
								option_class: "hover:bg-gray-700 text-gray-200",
							}
						}

						div { class: "relative flex-1 bg-gray-900 rounded-lg shadow-md border border-gray-700 p-4 h-full overflow-auto",

							div { class: "flex justify-between items-center bg-gray-800 text-gray-300 text-xs px-4 py-2 rounded-t-md",
								span { class: "font-mono", "{selected_file()}" }
								div { class: "flex gap-1",
									span { class: "w-3 h-3 bg-red-500 rounded-full" }
									span { class: "w-3 h-3 bg-yellow-500 rounded-full" }
									span { class: "w-3 h-3 bg-green-500 rounded-full" }
								}
							}

							div {
								class: "font-mono text-sm whitespace-pre p-4",
								// dangerous_inner_html to render the highlighted code
								dangerous_inner_html: "{highlighted_code}",
							}
						}
					}
				}
			}
		}
	}
}
