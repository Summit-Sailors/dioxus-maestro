// Border radius editor component
use dioxus::prelude::*;

use crate::components::maestro_themes::designer::state::{BorderRadiusSettings, DesignerState, ThemedesignerAction};

#[component]
pub fn BorderRadiusEditor() -> Element {
	let mut state = use_context::<Signal<DesignerState>>();

	// CSS variable names from the stylesheet
	let radius_options = [
		(String::from("sm"), String::from("Small (sm)"), state().border_radius.sm.clone(), String::from("--radius-sm")),
		(String::from("md"), String::from("Medium (md)"), state().border_radius.md.clone(), String::from("--radius-md")),
		(String::from("lg"), String::from("Large (lg)"), state().border_radius.lg.clone(), String::from("--radius-lg")),
		(String::from("xl"), String::from("Extra Large (xl)"), state().border_radius.xl.clone(), String::from("--radius-xl")),
		(String::from("full"), String::from("Full"), state().border_radius.full.clone(), String::from("--radius-full")),
	];

	let mut input_handler = move |key: String, event: Event<FormData>| {
		let value = event.value().clone();
		state.with_mut(|s| s.apply_action(ThemedesignerAction::UpdateBorderRadius { key, value }));
	};

	let mut reset_handler = move |key: String| {
		let default_radius = BorderRadiusSettings::default();
		state.with_mut(|s| match key.as_str() {
			"sm" => s.border_radius.sm = default_radius.sm,
			"md" => s.border_radius.md = default_radius.md,
			"lg" => s.border_radius.lg = default_radius.lg,
			"xl" => s.border_radius.xl = default_radius.xl,
			"full" => s.border_radius.full = default_radius.full,
			_ => {},
		});
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
										let modified = is_modified(key);
										let key_for_reset = key.clone();
										let key_for_input = key.clone();
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
																			onclick: move |_| reset_handler(key_for_reset.clone()),
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
															oninput: move |event| input_handler(key_for_input.clone(), event),
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
																			let key_for_preset = key.clone();
																			let preset_value = preset_value.to_string();
																			let preset_handler = move |_| {
																					state
																							.with_mut(|s| {
																									match key_for_preset.as_str() {
																											"sm" => s.border_radius.sm = preset_value.clone(),
																											"md" => s.border_radius.md = preset_value.clone(),
																											"lg" => s.border_radius.lg = preset_value.clone(),
																											"xl" => s.border_radius.xl = preset_value.clone(),
																											"full" => s.border_radius.full = preset_value.clone(),
																											_ => {}
																									}
																							});
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
