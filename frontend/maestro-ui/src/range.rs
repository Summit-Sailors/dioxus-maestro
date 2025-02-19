use dioxus::prelude::*;

#[component]
pub fn Range(current_value: i32, callback: EventHandler<i32>, label: Option<String>, min_value: i32, max_value: i32, step: i32) -> Element {
	rsx! {
		div { class: "flex flex-col gap-5 w-full relative maestro-range__wrapper",
			if let Some(label) = label {
				span { class: "text-gray-400 maestro-range__label", {label} }
			}
			input {
				r#type: "range",
				value: current_value,
				step,
				min: min_value,
				max: max_value,
				oninput: move |e| callback.call(e.value().parse::<i32>().expect("Oh no"))
			}
			div { class: "flex item justify-between maestro-range__values",
				span { class: "text-sm text-gray-400 maestro-range__values_min", "{min_value}" }
				span { class: "text-sm text-gray-200 maestro-range__values_current", {format!("Current: {}", current_value)} }
				span { class: "text-sm text-gray-400 maestro-range__values_max", "{max_value}" }
			}
		}
	}
}
