// Color selection component
use dioxus::prelude::*;

use crate::components::maestro_themes::designer::state::ColorPalette;

#[derive(Props, PartialEq, Clone)]
pub struct ColorPickerProps {
	/// Current color Palette
	colors: ColorPalette,
	/// Callback when color changes
	on_change: EventHandler<ColorPalette>,
}

#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
	let colors = use_signal(|| props.colors.clone());

	let update_color = move |field: &str, value: String| {
		match field {
			"primary" => colors().primary = value,
			"secondary" => colors().secondary = value,
			"accent" => colors().accent = value,
			"background" => colors().background = value,
			"foreground" => colors().foreground = value,
			"card" => colors().card = value,
			"card_foreground" => colors().card_foreground = value,
			"border" => colors().border = value,
			"ring" => colors().ring = value,
			"destructive" => colors().destructive = value,
			"destructive_foreground" => colors().destructive_foreground = value,
			"muted" => colors().muted = value,
			"muted_foreground" => colors().muted_foreground = value,
			_ => {},
		}
		props.on_change.call(colors());
	};

	rsx! {
		div { class: "p-6 bg-[var(--card-bg)] text-[var(--card-text)] rounded-lg shadow-md",
			h3 { class: "text-lg font-medium mb-4", "Colors" }
			div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4",
				ColorInput {
					label: "Primary",
					value: colors().primary,
					on_change: move |val| update_color("primary", val),
				}
				ColorInput {
					label: "Secondary",
					value: colors().secondary,
					on_change: move |val| update_color("secondary", val),
				}
				ColorInput {
					label: "Accent",
					value: colors().accent,
					on_change: move |val| update_color("accent", val),
				}
				ColorInput {
					label: "Background",
					value: colors().background,
					on_change: move |val| update_color("background", val),
				}
				ColorInput {
					label: "Foreground",
					value: colors().foreground,
					on_change: move |val| update_color("foreground", val),
				}
				ColorInput {
					label: "Card",
					value: colors().card,
					on_change: move |val| update_color("card", val),
				}
				ColorInput {
					label: "Card Foreground",
					value: colors().card_foreground,
					on_change: move |val| update_color("card_foreground", val),
				}
				ColorInput {
					label: "Border",
					value: colors().border,
					on_change: move |val| update_color("border", val),
				}
				ColorInput {
					label: "Ring",
					value: colors().ring,
					on_change: move |val| update_color("ring", val),
				}
				ColorInput {
					label: "Destructive",
					value: colors().destructive,
					on_change: move |val| update_color("destructive", val),
				}
				ColorInput {
					label: "Destructive Foreground",
					value: colors().destructive_foreground,
					on_change: move |val| update_color("destructive_foreground", val),
				}
				ColorInput {
					label: "Muted",
					value: colors().muted,
					on_change: move |val| update_color("muted", val),
				}
				ColorInput {
					label: "Muted Foreground",
					value: colors().muted_foreground,
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
	let mut color_value = use_signal(|| props.value);

	let handle_change = move |evt: Event<FormData>| {
		let new_value = evt.value();
		if new_value.starts_with('#') {
			color_value.set(new_value.clone());
			props.on_change.call(new_value);
		} else {
			log::warn!("Invalid color format: {}", new_value);
		}
	};

	rsx! {
		div { class: "flex flex-col gap-2",
			label { class: "text-sm font-medium", "{props.label}" }
			div { class: "flex items-center gap-2",
				input {
					r#type: "color",
					value: "{color_value}",
					class: "h-8 w-8 rounded border border-[var(--border-color)] bg-[var(--input-bg)] cursor-pointer",
					oninput: handle_change,
				}
				input {
					r#type: "text",
					value: "{color_value}",
					class: "text-sm border border-[var(--border-color)] bg-[var(--input-bg)] rounded px-2 py-1 w-28 text-[var(--text-color)]",
					oninput: handle_change,
				}
			}
		}
	}
}
