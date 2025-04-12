// Color selection component

use {crate::designer::state::ColorPalette, dioxus::prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct ColorPickerProps {
	/// Current color Palette
	colors: ColorPalette,
	/// Callback wneh color changes
	on_change: EventHandler<ColorPalette>,
}

#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
	let colors = props.colors.clone();

	let update_color = move |field: &str, value: String| {
		let mut new_colors = props.colors.clone();
		match field {
			"primary" => new_colors.primary = value,
			"secondary" => new_colors.secondary = value,
			"accent" => new_colors.accent = value,
			"background" => new_colors.background = value,
			"foreground" => new_colors.foreground = value,
			"card" => new_colors.card = value,
			"card_foreground" => new_colors.card_foreground = value,
			"border" => new_colors.border = value,
			"ring" => new_colors.ring = value,
			"destructive" => new_colors.destructive = value,
			"destructive_foreground" => new_colors.destructive_foreground = value,
			"muted" => new_colors.muted = value,
			"muted_foreground" => new_colors.muted_foreground = value,
			_ => {},
		}
		props.on_change.call(new_colors)
	};

	rsx! {
		div { class: "color-picker-container",
			h3 { class: "text-lg font-medium mb-3", "Colors" }
			div { class: "color-picker-grid",
				ColorInput {
					label: "Primary",
					value: colors.primary,
					on_change: move |val| update_color("primary", val),
				}
				ColorInput {
					label: "Secondary",
					value: colors.secondary,
					on_change: move |val| update_color("secondary", val),
				}
				ColorInput {
					label: "Accent",
					value: colors.accent,
					on_change: move |val| update_color("accent", val),
				}
				ColorInput {
					label: "Background",
					value: colors.background,
					on_change: move |val| update_color("background", val),
				}
				ColorInput {
					label: "Foreground",
					value: colors.foreground,
					on_change: move |val| update_color("foreground", val),
				}
				ColorInput {
					label: "Card",
					value: colors.card,
					on_change: move |val| update_color("card", val),
				}
				ColorInput {
					label: "Card Foreground",
					value: colors.card_foreground,
					on_change: move |val| update_color("card_foreground", val),
				}
				ColorInput {
					label: "Border",
					value: colors.border,
					on_change: move |val| update_color("border", val),
				}
				ColorInput {
					label: "Ring",
					value: colors.ring,
					on_change: move |val| update_color("ring", val),
				}
				ColorInput {
					label: "Destructive",
					value: colors.destructive,
					on_change: move |val| update_color("destructive", val),
				}
				ColorInput {
					label: "Destructive Foreground",
					value: colors.destructive_foreground,
					on_change: move |val| update_color("destructive_foreground", val),
				}
				ColorInput {
					label: "Muted",
					value: colors.muted,
					on_change: move |val| update_color("muted", val),
				}
				ColorInput {
					label: "Muted Foreground",
					value: colors.muted_foreground,
					on_change: move |val| update_color("muted_foreground", val),
				}
			}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
struct ColorInputProps {
	label: &'static str,
	value: String,
	on_change: EventHandler<String>,
}

#[component]
fn ColorInput(props: ColorInputProps) -> Element {
	let handle_change = move |evt: Event<FormData>| props.on_change.call(evt.value());

	rsx! {
		div { class: "color-input-group",
			label { class: "block text-sm font-medium mb-2", "{props.label}" }
			div { class: "color-input-wrapper flex items-center space-x-2",
				input {
					r#type: "color",
					value: "{props.value}",
					class: "color-picker h-8 w-8 rounded",
					oninput: handle_change,
				}
				input {
					r#type: "text",
					value: "{props.value}",
					class: "color-text-input text-sm border rounded px-2 py-1 w-28",
					oninput: handle_change,
				}
			}
		}
	}
}
