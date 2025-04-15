use dioxus::prelude::*;

use super::state::TypographySettings;

#[derive(Props, PartialEq, Clone)]
pub struct FontSelectorProps {
	pub typography: TypographySettings,
	pub on_change: EventHandler<TypographySettings>,
}

#[component]
pub fn FontSelector(props: FontSelectorProps) -> Element {
	let mut typography = use_signal(|| props.typography.clone());

	// update typography and notify parent on change
	let mut update_typography = move |new_typography: TypographySettings| {
		typography.set(new_typography.clone());
		props.on_change.call(new_typography);
	};

	// a handler factory functions that don't capture props directly
	let mut handle_font_family = move |value: String| {
		let mut new_typography = typography.read().clone();
		new_typography.font_family = value;
		update_typography(new_typography);
	};

	let mut handle_heading_font_family = move |value: String| {
		let mut new_typography = typography.read().clone();
		new_typography.heading_font_family = value;
		update_typography(new_typography);
	};

	let mut handle_base_size = move |value: String| {
		let mut new_typography = typography.read().clone();
		new_typography.base_size = value;
		update_typography(new_typography);
	};

	let mut handle_line_height = move |value: String| {
		let mut new_typography = typography.read().clone();
		new_typography.line_height = value;
		update_typography(new_typography);
	};

	let mut handle_font_weight = move |name: String, value: u32| {
		let mut new_typography = typography.read().clone();
		new_typography.font_weights.insert(name, value);
		update_typography(new_typography);
	};

	let common_fonts = ["Inter, system-ui, sans-serif",
		"Arial, sans-serif",
		"Helvetica, sans-serif",
		"Times New Roman",
		"Georgia, serif",
		"Verdana, sans-serif",
		"Courier New, monospace",
		"Segoe UI",
		"Roboto, sans-serif",
		"Open Sans, sans-serif",
		"Lato, sans-serif"];

	rsx! {
		div { class: "typography-editor",
			h3 { class: "text-lg font-medium mb-3", "Typography" }

			div { class: "mb-4",
				label { class: "block text-sm font-medium mb-1", "Font Family" }
				select {
					class: "w-full border rounded px-2 py-1",
					value: "{typography().font_family}",
					oninput: move |event| handle_font_family(event.value()),
					{
							common_fonts
									.iter()
									.map(|font| {
											rsx! {
												option { value: "{font}", "{font}" }
											}
									})
					}
				}
				input {
					r#type: "text",
					class: "w-full border rounded px-2 py-1 mt-2",
					value: "{typography().font_family}",
					oninput: move |event| handle_font_family(event.value()),
					placeholder: "Or enter custom font stack",
				}
			}

			div { class: "mb-4",
				label { class: "block text-sm font-medium mb-1", "Heading Font Family" }
				select {
					class: "w-full border rounded px-2 py-1",
					value: "{typography().heading_font_family}",
					oninput: move |event| handle_heading_font_family(event.value()),
					{
							common_fonts
									.iter()
									.map(|font| {
											rsx! {
												option { value: "{font}", "{font}" }
											}
									})
					}
				}
				input {
					r#type: "text",
					class: "w-full border rounded px-2 py-1 mt-2",
					value: "{typography().heading_font_family}",
					oninput: move |event| handle_font_family(event.value()),
					placeholder: "Or enter custom font stack",
				}
			}
		}

		div { class: "grid grid-cols-2 gap-4 mb-4",
			div {
				label { class: "block text-sm font-medium mb-1", "Base Size" }
				input {
					r#type: "text",
					class: "w-full border rounded px-2 py-1",
					value: "{typography().base_size}",
					oninput: move |event| handle_base_size(event.value()),
				}
			}

			div {
				label { class: "block text-sm font-medium mb-1", "Line Height" }
				input {
					r#type: "text",
					class: "w-full border rounded px-2 py-1",
					value: "{typography().line_height}",
					oninput: move |event| handle_line_height(event.value()),
				}
			}
		}

		h4 { class: "text-md font-medium mb-2 mt-4", "Font Weights" }
		div { class: "grid grid-cols-2 gap-4",
			{
					let _ = typography()
							.font_weights
							.iter()
							.map(|(name, weight)| {
									let name_clone = name.clone();
									rsx! {
										div { key: "{name}",
											label { class: "block text-sm font-medium mb-1", "{name}" }
											input {
												r#type: "number",
												min: "100",
												max: "900",
												step: "100",
												class: "w-full border rounded px-2 py-1",
												value: "{weight}",
												oninput: move |event| {
														if let Ok(val) = event.value().parse::<u32>() {
																handle_font_weight(name_clone.clone(), val);
														}
												},
											}
										}
									}
							});
			}
		}
	}
}
