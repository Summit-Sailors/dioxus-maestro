// Color selection component
use std::rc::Rc;

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
	let colors = props.colors.clone();

	let make_color_handler = |field: &'static str| {
		let on_change = props.on_change;
		let base_colors = props.colors.clone();

		Rc::new(move |value: String| {
			let mut new_colors = base_colors.clone();
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
			on_change.call(new_colors)
		})
	};

	let primary_handler = make_color_handler("primary");
	let secondary_handler = make_color_handler("secondary");
	let accent_handler = make_color_handler("accent");
	let background_handler = make_color_handler("background");
	let foreground_handler = make_color_handler("foreground");
	let card_handler = make_color_handler("card");
	let card_foreground_handler = make_color_handler("card_foreground");
	let border_handler = make_color_handler("border");
	let ring_handler = make_color_handler("ring");
	let destructive_handler = make_color_handler("destructive");
	let destructive_foreground_handler = make_color_handler("destructive_foreground");
	let muted_handler = make_color_handler("muted");
	let muted_foreground_handler = make_color_handler("muted_foreground");

	rsx! {
		div { class: "p-6 bg-[var(--card-bg)] text-[var(--card-text)] rounded-lg shadow-md",
			h3 { class: "text-lg font-medium mb-4", "Colors" }
			div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4",
				ColorInput {
					label: "Primary",
					value: colors.primary,
					on_change: move |val| primary_handler(val),
				}
				ColorInput {
					label: "Secondary",
					value: colors.secondary,
					on_change: move |val| secondary_handler(val),
				}
				ColorInput {
					label: "Accent",
					value: colors.accent,
					on_change: move |val| accent_handler(val),
				}
				ColorInput {
					label: "Background",
					value: colors.background,
					on_change: move |val| background_handler(val),
				}
				ColorInput {
					label: "Foreground",
					value: colors.foreground,
					on_change: move |val| foreground_handler(val),
				}
				ColorInput {
					label: "Card",
					value: colors.card,
					on_change: move |val| card_handler(val),
				}
				ColorInput {
					label: "Card Foreground",
					value: colors.card_foreground,
					on_change: move |val| card_foreground_handler(val),
				}
				ColorInput {
					label: "Border",
					value: colors.border,
					on_change: move |val| border_handler(val),
				}
				ColorInput {
					label: "Ring",
					value: colors.ring,
					on_change: move |val| ring_handler(val),
				}
				ColorInput {
					label: "Destructive",
					value: colors.destructive,
					on_change: move |val| destructive_handler(val),
				}
				ColorInput {
					label: "Destructive Foreground",
					value: colors.destructive_foreground,
					on_change: move |val| destructive_foreground_handler(val),
				}
				ColorInput {
					label: "Muted",
					value: colors.muted,
					on_change: move |val| muted_handler(val),
				}
				ColorInput {
					label: "Muted Foreground",
					value: colors.muted_foreground,
					on_change: move |val| muted_foreground_handler(val),
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
		div { class: "flex flex-col gap-2",
			label { class: "text-sm font-medium", "{props.label}" }
			div { class: "flex items-center gap-2",
				input {
					r#type: "color",
					value: "{props.value}",
					class: "h-8 w-8 rounded border border-[var(--border-color)] bg-[var(--input-bg)] cursor-pointer",
					oninput: handle_change,
				}
				input {
					r#type: "text",
					value: "{props.value}",
					class: "text-sm border border-[var(--border-color)] bg-[var(--input-bg)] rounded px-2 py-1 w-28 text-[var(--text-color)]",
					oninput: handle_change,
				}
			}
		}
	}
}
