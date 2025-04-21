use dioxus::prelude::*;

use crate::components::maestro_themes::designer::state::SpacingScale;

#[derive(Props, PartialEq, Clone)]
pub struct SpacingEditorProps {
	pub spacing: SpacingScale,
	pub on_change: EventHandler<SpacingScale>,
}

#[component]
pub fn SpacingEditor(props: SpacingEditorProps) -> Element {
	let mut spacing = use_signal(|| props.spacing.clone());

	// callbacks for each operation
	let on_change = props.on_change;

	let handle_unit_change = use_callback(move |value: String| {
		let mut new_spacing = spacing.read().clone();
		new_spacing.unit = value;
		spacing.set(new_spacing.clone());
		on_change.call(new_spacing);
	});

	let handle_value_change = use_callback(move |(key, value): (String, String)| {
		let mut new_spacing = spacing.read().clone();
		new_spacing.scale.insert(key, value);
		spacing.set(new_spacing.clone());
		on_change.call(new_spacing);
	});

	let handle_add_value = use_callback(move |_: ()| {
		let mut new_spacing = spacing.read().clone();
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
		spacing.set(new_spacing.clone());
		on_change.call(new_spacing);
	});

	let handle_remove_value = use_callback(move |key: String| {
		let mut new_spacing = spacing.read().clone();
		new_spacing.scale.remove(&key);
		spacing.set(new_spacing.clone());
		on_change.call(new_spacing);
	});

	// a memo for the sorted keys to avoid recalculating on each render
	let sorted_keys = use_memo(move || {
		let mut keys: Vec<String> = spacing.read().scale.keys().cloned().collect();
		keys.sort_by(|a, b| {
			let a_num = a.parse::<u32>().unwrap_or(u32::MAX);
			let b_num = b.parse::<u32>().unwrap_or(u32::MAX);
			a_num.cmp(&b_num)
		});
		keys
	});

	rsx! {
		div { class: "spacing-editor",
			h3 { class: "text-lg font-medium mb-3", "Spacing" }
			div { class: "mb-4",
				label { class: "block text-sm font-medium mb-1", "Unit" }
				select {
					class: "w-full border rounded px-2 py-1",
					value: "{spacing.read().unit}",
					oninput: move |event| handle_unit_change(event.value().clone()),
					option { value: "px", "px" }
					option { value: "rem", "rem" }
					option { value: "em", "em" }
				}
			}
			h4 { class: "text-md font-medium mb-2 mt-4", "Spacing Scale" }
			div { class: "spacing-scale-container",
				{
						let _ = sorted_keys
								.iter()
								.map(|key| {
										let value = spacing().scale.get(&*key).unwrap_or(&String::new()).clone();
										let key_for_value = key.clone();
										let key_for_remove = key.clone();
										rsx! {
											div { key: "{key}", class: "spacing-scale-row flex items-center space-x-2 mb-2",
												label { class: "block text-sm font-medium w-12", "{key}" }
												input {
													r#type: "text",
													class: "flex-grow border rounded px-2 py-1",
													value: "{value}",
													oninput: move |event| handle_value_change((key_for_value.clone(), event.value().clone())),
												}
												button {
													r#type: "button",
													class: "text-red-500 px-2",
													onclick: move |_| handle_remove_value(key_for_remove.clone()),
													"×"
												}
											}
										}
								});
				}
				button {
					r#type: "button",
					class: "mt-2 px-3 py-1 bg-blue-500 text-white rounded text-sm",
					onclick: move |_| handle_add_value(()),
					"Add Spacing Value"
				}
			}
		}
	}
}
