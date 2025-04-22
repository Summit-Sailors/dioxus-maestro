// Border radius editor component
use dioxus::prelude::*;

use crate::components::maestro_themes::designer::state::BorderRadiusSettings;

#[derive(Props, PartialEq, Clone)]
pub struct BorderRadiusEditorProps {
	pub border_radius: BorderRadiusSettings,
	pub on_change: EventHandler<BorderRadiusSettings>,
}

#[component]
pub fn BorderRadiusEditor(props: BorderRadiusEditorProps) -> Element {
	let border_radius = props.border_radius.clone();

	// CSS variable names from the stylesheet
	let radius_options = [
		("sm", "Small (sm)", &border_radius.sm, "--radius-sm"),
		("md", "Medium (md)", &border_radius.md, "--radius-md"),
		("lg", "Large (lg)", &border_radius.lg, "--radius-lg"),
		("xl", "Extra Large (xl)", &border_radius.xl, "--radius-xl"),
		("full", "Full", &border_radius.full, "--radius-full"),
	];

	let make_handler = |key: &'static str, on_change: &EventHandler<BorderRadiusSettings>, base_radius: &BorderRadiusSettings| {
		let on_change = *on_change;
		let base_radius = base_radius.clone();

		move |event: Event<FormData>| {
			let mut new_radius = base_radius.clone();
			let value = event.value().clone();

			match key {
				"sm" => new_radius.sm = value,
				"md" => new_radius.md = value,
				"lg" => new_radius.lg = value,
				"xl" => new_radius.xl = value,
				"full" => new_radius.full = value,
				_ => {},
			}

			on_change.call(new_radius);
		}
	};

	let make_reset_handler = |key: &'static str, on_change: &EventHandler<BorderRadiusSettings>, base_radius: &BorderRadiusSettings| {
		let on_change = *on_change;
		let base_radius = base_radius.clone();

		move |_| {
			let mut new_radius = base_radius.clone();
			let default_radius = BorderRadiusSettings::default();

			match key {
				"sm" => new_radius.sm = default_radius.sm,
				"md" => new_radius.md = default_radius.md,
				"lg" => new_radius.lg = default_radius.lg,
				"xl" => new_radius.xl = default_radius.xl,
				"full" => new_radius.full = default_radius.full,
				_ => {},
			}

			on_change.call(new_radius);
		}
	};

	let is_modified = |key: &str| -> bool {
		let default_radius = BorderRadiusSettings::default();
		match key {
			"sm" => border_radius.sm != default_radius.sm,
			"md" => border_radius.md != default_radius.md,
			"lg" => border_radius.lg != default_radius.lg,
			"xl" => border_radius.xl != default_radius.xl,
			"full" => border_radius.full != default_radius.full,
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
										let input_handler = make_handler(
												key,
												&props.on_change,
												&props.border_radius,
										);
										let reset_handler = make_reset_handler(
												key,
												&props.on_change,
												&props.border_radius,
										);
										let modified = is_modified(key);
										rsx! {
											div { key: "{key}",
												div { class: "flex justify-between items-center mb-1",

													label { class: "text-sm font-medium text-muted-foreground", "{label}" }
													// Only show reset button if value is modified
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

													div { class: "flex items-center space-x-2",
														// Main input
														div { class: "flex-grow relative",
															input {
																r#type: "text",
																class: "w-full border border-border bg-input-bg text-sm px-3 py-1.5 rounded-md focus:outline-none focus:ring-2 focus:ring-ring",
																value: "{value}",
																oninput: input_handler,
															}
														}
														// Preview box
														div { class: "w-10 h-10 rounded-md border border-border overflow-hidden flex items-center justify-center",
															div {
																class: "w-full h-full bg-muted",
																style: "border-radius: {value};",
															}
														}
													}

													// Quick preset buttons
													div { class: "mt-2 flex flex-wrap gap-1",
														{
																size_presets
																		.iter()
																		.map(|(preset_name, preset_value)| {
																				let preset_handler = {
																						let key = key.to_string();
																						let preset_value = preset_value.to_string();
																						let on_change = props.on_change;
																						let base_radius = props.border_radius.clone();
																						move |_| {
																								let mut new_radius = base_radius.clone();
																								match key.as_str() {
																										"sm" => new_radius.sm = preset_value.clone(),
																										"md" => new_radius.md = preset_value.clone(),
																										"lg" => new_radius.lg = preset_value.clone(),
																										"xl" => new_radius.xl = preset_value.clone(),
																										"full" => new_radius.full = preset_value.clone(),
																										_ => {}
																								}
																								on_change.call(new_radius);
																						}
																				};
																				rsx! {
																					button {
																						r#type: "button",
																						class: "text-xs px-2 py-1 bg-muted hover:bg-hover-bg rounded border border-border text-muted-foreground hover:text-foreground",
																						onclick: preset_handler,
																						"{preset_name}"
																					}
																				}
																		})
														}
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
