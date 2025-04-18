// Shadow settings editor component
use dioxus::prelude::*;

use crate::components::maestro_themes::designer::ShadowSettings;

#[derive(Props, PartialEq, Clone)]
pub struct ShadowEditorProps {
	pub shadow: ShadowSettings,
	pub on_change: EventHandler<ShadowSettings>,
}

#[derive(Props, PartialEq, Clone)]
pub struct ShadowInputProps {
	pub label: String,
	pub value: String,
	pub key_name: &'static str,
	pub on_change: EventHandler<(&'static str, String)>,
}

#[component]
fn ShadowInput(props: ShadowInputProps) -> Element {
	rsx! {
		div {
			label { class: "block text-sm font-medium mb-1", "{props.label}" }
			div { class: "flex items-center gap-4",
				div {
					class: "shadow-preview w-16 h-16 bg-white rounded",
					style: "box-shadow: {props.value};",
				}
				textarea {
					class: "flex-grow border rounded px-2 py-1 text-sm",
					rows: "2",
					value: "{props.value}",
					oninput: move |event| props.on_change.call((props.key_name, event.value().clone())),
				}
			}
		}
	}
}

#[component]
pub fn ShadowEditor(props: ShadowEditorProps) -> Element {
	let shadow = props.shadow.clone();
	let props_state = use_signal(|| props.clone());

	let update_shadow = use_callback(move |(key, value): (&'static str, String)| {
		let mut new_shadow = props_state().shadow.clone();
		match key {
			"sm" => new_shadow.sm = value,
			"md" => new_shadow.md = value,
			"lg" => new_shadow.lg = value,
			"xl" => new_shadow.xl = value,
			"xxl" => new_shadow.xxl = value,
			_ => {},
		}
		props_state().on_change.call(new_shadow);
	});

	rsx! {
		div { class: "shadow-editor",
			h3 { class: "text-lg font-medium mb-3", "Shadows" }
			div { class: "shadow-preview-grid space-y-4",
				ShadowInput {
					label: "Small (sm)".to_string(),
					value: shadow.sm.clone(),
					key_name: "sm",
					on_change: update_shadow,
				}
				ShadowInput {
					label: "Medium (md)".to_string(),
					value: shadow.md.clone(),
					key_name: "md",
					on_change: update_shadow,
				}
				ShadowInput {
					label: "Large (lg)".to_string(),
					value: shadow.lg.clone(),
					key_name: "lg",
					on_change: update_shadow,
				}
				ShadowInput {
					label: "Extra Large (xl)".to_string(),
					value: shadow.xl.clone(),
					key_name: "xl",
					on_change: update_shadow,
				}
				ShadowInput {
					label: "Extra Extra Large (xxl)".to_string(),
					value: shadow.xxl.clone(),
					key_name: "xxl",
					on_change: update_shadow,
				}
			}
		}
	}
}
