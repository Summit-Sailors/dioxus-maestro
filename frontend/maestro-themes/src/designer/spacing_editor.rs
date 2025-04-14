// Spacing/sizing configuration

use std::collections::HashMap;

use dioxus::prelude::*;

use crate::designer::state::SpacingScale;

#[derive(Props, PartialEq, Clone)]
pub struct SpacingEditorProps {
	pub spacing: SpacingScale,
	pub on_change: EventHandler<SpacingScale>,
}

#[component]
pub fn SpacingEditor(props: SpacingEditorProps) -> Element {
	let spacing = props.spacing.clone();

	let update_unit = move |value: String| {
		let mut new_spacing = props.spacing.clone();
		new_spacing.unit = value;
		props.on_change.call(new_spacing);
	};

	let update_spacing_value = move |key: String, value: String| {
		let mut new_spacing = props.spacing.clone();
		new_spacing.scale.insert(key, value);
		props.on_change.call(new_spacing);
	};

	let add_spacing_value = move |_| {
		let mut new_spacing = props.spacing.clone();
		// find next available number key
		let mut max_key = 0;
		for key in new_spacing.scale.keys() {
			if let Ok(num) = key.parse::<u32>() {
				if num > max_key {
					max_key = num;
				}
			}
		}
		let next_key = format!("{}", max_key + 1);
		let default_value = format!("{}rem", (max_key as f32 + 1.0) * 0.25);
		new_spacing.scale.insert(next_key, default_value);
		props.on_change.call(new_spacing);
	};

	let remove_spacing_value = move |key: String| {
		let mut new_spacing = props.spacing.clone();
		new_spacing.scale.remove(&key);
		props.on_change.call(new_spacing);
	};

	// sort keys numerically for display
	let mut keys: Vec<String> = spacing.scale.keys().cloned().collect();
	keys.sort_by(|a, b| {
		let a_num = a.parse::<u32>().unwrap_or(u32::MAX);
		let b_num = b.parse::<u32>().unwrap_or(u32::MAX);
		a_num.cmp(&b_num)
	});

	rsx! {
		div { class: "spacing-editor",
			h3 { class: "text-lg font-medium mb-3", "Spacing" }
			div { class: "mb-4",
				label { class: "block text-sm font-medium mb-1", "Unit" }
				select {
					class: "w-full border rounded px-2 py-1",
					value: "{spacing.unit}",
					oninput: move |event| update_unit(event.value().clone()),
					option { value: "px", "px" }
					option { value: "rem", "rem" }
					option { value: "em", "em" }
				}
			}
			h4 { class: "text-md font-medium mb-2 mt-4", "Spacing Scale" }
			div { class: "spacing-scale-container",
				{
						keys.iter()
								.map(|key| {
										let value = spacing.scale.get(key).unwrap_or(&String::new()).clone();
										let key_clone = key.clone();
										rsx! {
											div { key: "{key}", class: "spacing-scale-row flex items-center space-x-2 mb-2",
												label { class: "block text-sm font-medium w-12", "{key}" }
												input {
													r#type: "text",
													class: "flex-grow border rounded px-2 py-1",
													value: "{value}",
													oninput: move |event| update_spacing_value(key_clone.clone(), event.value().clone()),
												}
												button {
													r#type: "button",
													class: "text-red-500 px-2",
													onclick: move |_| remove_spacing_value(key_clone.clone()),
													"×"
												}
											}
										}
								})
				}
				button {
					r#type: "button",
					class: "mt-2 px-3 py-1 bg-blue-500 text-white rounded text-sm",
					onclick: add_spacing_value,
					"Add Spacing Value"
				}
			}
		}
	}
}
