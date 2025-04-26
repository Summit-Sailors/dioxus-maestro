// Spacing editor component
use std::collections::HashMap;

use dioxus::prelude::*;

use crate::components::maestro_themes::designer::state::{
	DesignerState,
	ThemedesignerAction::{RemoveSpacingValue, UpdateSpacingUnit, UpdateSpacingValue},
};

const PX_TO_REM: f32 = 1.0 / 16.0; // 1rem = 16px
const REM_TO_PX: f32 = 16.0; // 16px = 1rem
const PX_TO_EM: f32 = 1.0 / 16.0; // 1em = 16px (assuming base font size)
const EM_TO_PX: f32 = 16.0; // 16px = 1em (assuming base font size)

#[component]
pub fn SpacingEditor() -> Element {
	let mut state = use_context::<Signal<DesignerState>>();

	let mut handle_unit_change = move |value: String| {
		let old_unit = state().spacing.unit.clone();
		let mut converted_scale = HashMap::new();
		for (key, val) in state().spacing.scale.iter() {
			if let Some(converted_value) = convert_unit_value(val, &old_unit, &value) {
				converted_scale.insert(key.clone(), converted_value);
			} else {
				// If conversion fails, keep the original value
				converted_scale.insert(key.clone(), val.clone());
			}
		}
		state.with_mut(|s| s.apply_action(UpdateSpacingUnit { value }));
		for (key, val) in converted_scale {
			state.with_mut(|s| s.apply_action(UpdateSpacingValue { key, value: val }));
		}
	};

	let mut handle_value_change = move |key: String, event: Event<FormData>| {
		let value = event.value().to_string();
		state.with_mut(|s| s.apply_action(UpdateSpacingValue { key, value }));
	};

	let mut handle_add_value = move |_: ()| {
		let mut max_key = 0;
		for key in state().spacing.scale.keys() {
			if let Ok(num) = key.parse::<u32>() {
				if num > max_key {
					max_key = num;
				}
			}
		}
		let next_key = format!("{}", max_key + 1);

		let base_px_value = (max_key as f32 + 1.0) * 4.0;

		let numeric_value = match state().spacing.unit.as_str() {
			"px" => base_px_value,
			"rem" => base_px_value * PX_TO_REM,
			"em" => base_px_value * PX_TO_EM,
			_ => base_px_value,
		};

		// format the value and remove trailing zeros
		let formatted_value = if numeric_value.fract() < 0.01 { format!("{}", numeric_value.round()) } else { format!("{:.2}", numeric_value) };

		let formatted_value = if formatted_value.contains('.') { formatted_value.trim_end_matches('0').trim_end_matches('.').to_string() } else { formatted_value };

		let default_value = format!("{}{}", formatted_value, state().spacing.unit);

		state.with_mut(|s| s.apply_action(UpdateSpacingValue { key: next_key, value: default_value }));
	};

	let mut handle_remove_value = move |key: String| {
		state.with_mut(|s| s.apply_action(RemoveSpacingValue { key }));
	};

	// a memo for the sorted keys to avoid recalculating on each render
	let sorted_keys = use_memo(move || {
		let spacing_scale = &state().spacing.scale;
		let mut keys: Vec<String> = spacing_scale.keys().cloned().collect();
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
          value: "{state().spacing.unit}",
          oninput: move |event| handle_unit_change(event.value()),
          option { value: "px", "px" }
          option { value: "rem", "rem" }
          option { value: "em", "em" }
        }
      }
      h4 { class: "text-md font-medium mb-2 mt-4", "Spacing Scale" }
      div { class: "spacing-scale-container",
        {
            let sorted_keys_clone = sorted_keys().clone();
            sorted_keys_clone
                .into_iter()
                .map(|key| {
                    let value = state
                        .read()
                        .spacing
                        .scale
                        .get(&*key)
                        .unwrap_or(&String::new())
                        .clone();
                    let key_for_value = key.clone();
                    let key_for_remove = key.clone();
                    rsx! {
                      div { key: "{key}", class: "spacing-scale-row flex items-center space-x-2 mb-2",
                        label { class: "block text-sm font-medium w-12", "{key}" }
                        input {
                          r#type: "text",
                          class: "flex-grow border rounded px-2 py-1",
                          value: "{value}",
                          oninput: move |event| handle_value_change(key_for_value.clone(), event),
                        }
                        button {
                          r#type: "button",
                          class: "text-[color:var(--destructive)] px-2",
                          onclick: move |_| handle_remove_value(key_for_remove.clone()),
                          "×"
                        }
                      }
                    }
                })
        }
        button {
          r#type: "button",
          class: "mt-2 px-3 py-1 bg-[color:var(--primary)] text-white rounded text-sm",
          onclick: move |_| handle_add_value(()),
          "Add Spacing Value"
        }
      }
    }
  }
}

fn parse_value_and_unit(value: &str) -> Option<(f32, String)> {
	// the position where the numeric part ends
	let numeric_end = value.chars().position(|c| !c.is_ascii_digit() && c != '.' && c != '-')?;

	let numeric_str = &value[0..numeric_end];
	let numeric_value = numeric_str.parse::<f32>().ok()?;

	let unit = value[numeric_end..].trim().to_string();

	Some((numeric_value, unit))
}

fn convert_unit_value(value: &str, from_unit: &str, to_unit: &str) -> Option<String> {
	if from_unit == to_unit {
		return Some(value.to_string());
	}

	let (numeric_value, value_unit) = parse_value_and_unit(value)?;

	// if the value's unit doesn't match the expected from_unit, we can't convert
	if value_unit != from_unit {
		// if the value already has the target unit
		if value_unit == to_unit {
			return Some(value.to_string());
		}
		// otherwise, assume the value is in the specified from_unit
	}

	// to px as an intermediate step if necessary
	let px_value = match from_unit {
		"px" => numeric_value,
		"rem" => numeric_value * REM_TO_PX,
		"em" => numeric_value * EM_TO_PX,
		_ => return None,
	};

	// from px to target unit
	let converted_value = match to_unit {
		"px" => px_value,
		"rem" => px_value * PX_TO_REM,
		"em" => px_value * PX_TO_EM,
		_ => return None,
	};

	let formatted_value = if converted_value.fract() < 0.01 { format!("{}", converted_value.round()) } else { format!("{:.2}", converted_value) };

	// remove trailing zeros after decimal point
	let formatted_value = if formatted_value.contains('.') { formatted_value.trim_end_matches('0').trim_end_matches('.').to_string() } else { formatted_value };

	Some(format!("{}{}", formatted_value, to_unit))
}
