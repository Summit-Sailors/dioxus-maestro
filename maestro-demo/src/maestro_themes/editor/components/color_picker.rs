// Color selection component
use dioxus::prelude::*;

use crate::maestro_themes::editor::state::{DesignerState, ThemedesignerAction::UpdateColor};

#[component]
pub fn ColorPicker() -> Element {
	let mut state = use_context::<Signal<DesignerState>>();
	let mut update_color = move |field: &str, value: String| {
		state.with_mut(|s| s.apply_action(UpdateColor { key: field.to_string(), value }));
	};

	rsx! {
		div { class: "p-6 bg-[var(--card-bg)] text-[var(--card-text)] rounded-lg shadow-md",
			h3 { class: "text-lg font-medium mb-4", "Colors" }
			div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4",
				ColorInput {
					label: "Primary",
					field: "primary",
					on_change: move |val| update_color("primary", val),
				}
				ColorInput {
					label: "Secondary",
					field: "secondary",
					on_change: move |val| update_color("secondary", val),
				}
				ColorInput {
					label: "Accent",
					field: "accent",
					on_change: move |val| update_color("accent", val),
				}
				ColorInput {
					label: "Background",
					field: "background",
					on_change: move |val| update_color("background", val),
				}
				ColorInput {
					label: "Foreground",
					field: "foreground",
					on_change: move |val| update_color("foreground", val),
				}
				ColorInput {
					label: "Card",
					field: "card",
					on_change: move |val| update_color("card", val),
				}
				ColorInput {
					label: "Card Foreground",
					field: "card_foreground",
					on_change: move |val| update_color("card_foreground", val),
				}
				ColorInput {
					label: "Border",
					field: "border",
					on_change: move |val| update_color("border", val),
				}
				ColorInput {
					label: "Ring",
					field: "ring",
					on_change: move |val| update_color("ring", val),
				}
				ColorInput {
					label: "Destructive",
					field: "destructive",
					on_change: move |val| update_color("destructive", val),
				}
				ColorInput {
					label: "Destructive Foreground",
					field: "destructive_foreground",
					on_change: move |val| update_color("destructive_foreground", val),
				}
				ColorInput {
					label: "Muted",
					field: "muted",
					on_change: move |val| update_color("muted", val),
				}
				ColorInput {
					label: "Muted Foreground",
					field: "muted_foreground",
					on_change: move |val| update_color("muted_foreground", val),
				}
			}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
struct ColorInputProps {
	label: &'static str,
	field: &'static str,
	on_change: EventHandler<String>,
}

#[component]
fn ColorInput(props: ColorInputProps) -> Element {
	let state = use_context::<Signal<DesignerState>>();

	let get_value = move || match props.field {
		"primary" => state().color.primary.clone(),
		"secondary" => state().color.secondary.clone(),
		"accent" => state().color.accent.clone(),
		"background" => state().color.background.clone(),
		"foreground" => state().color.foreground.clone(),
		"card" => state().color.card.clone(),
		"card_foreground" => state().color.card_foreground.clone(),
		"border" => state().color.border.clone(),
		"ring" => state().color.ring.clone(),
		"destructive" => state().color.destructive.clone(),
		"destructive_foreground" => state().color.destructive_foreground.clone(),
		"muted" => state().color.muted.clone(),
		"muted_foreground" => state().color.muted_foreground.clone(),
		_ => "#000000".to_string(),
	};

	let handle_change = move |evt: Event<FormData>| {
		let new_value = evt.value();
		props.on_change.call(new_value);
	};

	rsx! {
		div { class: "flex flex-col gap-2",
			label { class: "text-sm font-medium", "{props.label}" }
			div { class: "flex items-center gap-4",
				input {
					r#type: "color",
					value: get_value(),
					class: "h-8 w-8 rounded border border-[var(--border-color)] bg-[var(--input-bg)] cursor-pointer",
					oninput: handle_change,
				}
				input {
					r#type: "text",
					value: "{get_value()}",
					class: "text-sm border border-[var(--border-color)] bg-[var(--input-bg)] rounded px-2 py-1 w-28 text-[var(--text-color)]",
					oninput: handle_change,
				}
			}
		}
	}
}
