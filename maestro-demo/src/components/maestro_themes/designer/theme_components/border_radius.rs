// Border radius editor component
use dioxus::prelude::*;

use crate::components::maestro_themes::designer::state::{BorderRadiusSettings, DesignerState, ThemedesignerAction};

#[derive(Props, PartialEq, Clone)]
pub struct BorderRadiusEditorProps {
	pub state: Signal<DesignerState>,
}

#[component]
pub fn BorderRadiusEditor(props: BorderRadiusEditorProps) -> Element {
	let state = props.state;

	// CSS variable names from the stylesheet
	let radius_options = [
		("sm", "Small (sm)", &state().border_radius.sm, "--radius-sm"),
		("md", "Medium (md)", &state().border_radius.md, "--radius-md"),
		("lg", "Large (lg)", &state().border_radius.lg, "--radius-lg"),
		("xl", "Extra Large (xl)", &state().border_radius.xl, "--radius-xl"),
		("full", "Full", &state().border_radius.full, "--radius-full"),
	];

	let make_handler = |key: &'static str| {
		let key_owned = key.to_string();

		move |event: Event<FormData>| {
			let value = event.value().clone();
			state().apply_action(ThemedesignerAction::UpdateBorderRadius { key: key_owned.clone(), value });
		}
	};

	let make_reset_handler = |key: &'static str| {
		let key_owned = key.to_string();

		move |_| {
			let default_radius = BorderRadiusSettings::default();
			let default_value = match key {
				"sm" => default_radius.sm,
				"md" => default_radius.md,
				"lg" => default_radius.lg,
				"xl" => default_radius.xl,
				"full" => default_radius.full,
				_ => String::new(),
			};

			state().apply_action(ThemedesignerAction::UpdateBorderRadius { key: key_owned.clone(), value: default_value });
		}
	};

	let is_modified = |key: &str| -> bool {
		let default_radius = BorderRadiusSettings::default();
		match key {
			"sm" => state().border_radius.sm != default_radius.sm,
			"md" => state().border_radius.md != default_radius.md,
			"lg" => state().border_radius.lg != default_radius.lg,
			"xl" => state().border_radius.xl != default_radius.xl,
			"full" => state().border_radius.full != default_radius.full,
			_ => false,
		}
	};

	// Predefined size presets for quick selection
	let size_presets =
		[("None", "0"), ("XS", "0.125rem"), ("SM", "0.25rem"), ("MD", "0.5rem"), ("LG", "0.75rem"), ("XL", "1rem"), ("2XL", "1.5rem"), ("Full", "9999px")];

	rsx! {
		div { class: "p-4 bg-card text-card-foreground rounded-lg border border-border space-y-4 shadow-sm",
			h3 { class: "text-lg font-semibold", "Border Radius" }
			div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
				{
						radius_options
								.iter()
								.map(|(key, label, value, _css_var)| {
										let input_handler = make_handler(key);
										let reset_handler = make_reset_handler(key);
										let modified = is_modified(key);
										rsx! {
											div { key: "{key}",
												// label and reset button row
												div { class: "flex flex-col md:flex-row md:items-center md:justify-between gap-2 mb-1",
													label { class: "text-sm font-medium text-muted-foreground", "{label}" }
													{
															if modified {
																	rsx! {
																		button {
																			r#type: "button",
																			class: "text-xs text-muted-foreground hover:text-foreground",
																			onclick: reset_handler,
																			"Reset"
																		}
																	}
															} else {
																	rsx! {}
															}
													}
												}
												// input and preview row
												div { class: "flex flex-col sm:flex-row sm:items-center gap-2",
													div { class: "flex-grow relative",
														input {
															r#type: "text",
															class: "w-full border border-border bg-input-bg text-sm px-3 py-1.5 rounded-md focus:outline-none focus:ring-2 focus:ring-ring",
															value: "{value}",
															oninput: input_handler,
														}
													}
													div { class: "w-10 h-10 rounded-md border border-border overflow-hidden flex items-center justify-center shrink-0",
														div {
															class: "w-full h-full bg-muted",
															style: "border-radius: {value};",
														}
													}
												}
												// preset buttons
												div { class: "mt-2 flex flex-wrap gap-1",
													{
															size_presets
																	.iter()
																	.map(|(preset_name, preset_value)| {
																			let preset_handler = {
																					let key = key.to_string();
																					let preset_value = preset_value.to_string();
																					move |_| {
																							state()
																									.apply_action(ThemedesignerAction::UpdateBorderRadius {
																											key: key.clone(),
																											value: preset_value.clone(),
																									});
																					}
																			};
																			rsx! {
																				button {
																					r#type: "button",
																					class: "text-xs px-2 py-1 bg-muted hover:bg-hover-bg rounded border border-border text-[color:var(--text-color)] text-muted-foreground hover:text-foreground",
																					onclick: preset_handler,
																					"{preset_name}"
																				}
																			}
																	})
													}
												}
											}
										}
								})
				}
			}
		}
	}
}
