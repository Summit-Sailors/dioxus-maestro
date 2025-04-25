// Color selection component
use dioxus::prelude::*;

use crate::components::maestro_themes::designer::state::{DesignerState, ThemedesignerAction::UpdateColor};

#[derive(Props, PartialEq, Clone)]
pub struct ColorPickerProps {
	state: Signal<DesignerState>,
}

#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
	let state = props.state;
	let update_color = move |field: &str, value: String| {
		state().apply_action(UpdateColor { key: field.to_string(), value });
	};

	rsx! {
		div { class: "p-6 bg-[var(--card-bg)] text-[var(--card-text)] rounded-lg shadow-md",
			h3 { class: "text-lg font-medium mb-4", "Colors" }
			div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4",
				ColorInput {
					label: "Primary",
					value: state().color.primary,
					on_change: move |val| update_color("primary", val),
				}
				ColorInput {
					label: "Secondary",
					value: state().color.secondary,
					on_change: move |val| update_color("secondary", val),
				}
				ColorInput {
					label: "Accent",
					value: state().color.accent,
					on_change: move |val| update_color("accent", val),
				}
				ColorInput {
					label: "Background",
					value: state().color.background,
					on_change: move |val| update_color("background", val),
				}
				ColorInput {
					label: "Foreground",
					value: state().color.foreground,
					on_change: move |val| update_color("foreground", val),
				}
				ColorInput {
					label: "Card",
					value: state().color.card,
					on_change: move |val| update_color("card", val),
				}
				ColorInput {
					label: "Card Foreground",
					value: state().color.card_foreground,
					on_change: move |val| update_color("card_foreground", val),
				}
				ColorInput {
					label: "Border",
					value: state().color.border,
					on_change: move |val| update_color("border", val),
				}
				ColorInput {
					label: "Ring",
					value: state().color.ring,
					on_change: move |val| update_color("ring", val),
				}
				ColorInput {
					label: "Destructive",
					value: state().color.destructive,
					on_change: move |val| update_color("destructive", val),
				}
				ColorInput {
					label: "Destructive Foreground",
					value: state().color.destructive_foreground,
					on_change: move |val| update_color("destructive_foreground", val),
				}
				ColorInput {
					label: "Muted",
					value: state().color.muted,
					on_change: move |val| update_color("muted", val),
				}
				ColorInput {
					label: "Muted Foreground",
					value: state().color.muted_foreground,
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
		color_value.set(new_value.clone());
		props.on_change.call(new_value);
	};

	rsx! {
		div { class: "flex flex-col gap-2",
			label { class: "text-sm font-medium", "{props.label}" }
			div { class: "flex items-center gap-4",
				input {
					r#type: "color",
					value: "{color_value()}",
					class: "h-8 w-8 rounded border border-[var(--border-color)] bg-[var(--input-bg)] cursor-pointer",
					oninput: handle_change,
				}
				input {
					r#type: "text",
					value: "{color_value()}",
					class: "text-sm border border-[var(--border-color)] bg-[var(--input-bg)] rounded px-2 py-1 w-28 text-[var(--text-color)]",
					oninput: handle_change,
				}
			}
		}
	}
}
