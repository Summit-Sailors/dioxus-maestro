use dioxus::prelude::*;

use crate::components::maestro_themes::designer::state::{
	DesignerState,
	ThemedesignerAction::{UpdateFontFamiliy, UpdateFontSize, UpdateFontWight, UpdateHeadingFontFamily, UpdateLineHeight},
};

#[component]
pub fn FontSelector() -> Element {
	let mut state = use_context::<Signal<DesignerState>>();

	let mut handle_font_family = move |value: String| {
		state.with_mut(|s| s.apply_action(UpdateFontFamiliy { value }));
	};

	let mut handle_heading_font_family = move |value: String| {
		state.with_mut(|s| s.apply_action(UpdateHeadingFontFamily { value }));
	};

	let mut handle_base_size = move |value: String| {
		state.with_mut(|s| s.apply_action(UpdateFontSize { value }));
	};

	let mut handle_line_height = move |value: String| {
		state.with_mut(|s| s.apply_action(UpdateLineHeight { value }));
	};

	let mut handle_font_weight = move |name: String, value: u32| {
		state.with_mut(|s| s.apply_action(UpdateFontWight { name, value }));
	};

	let common_fonts = [
		"Inter, system-ui, sans-serif",
		"Arial, sans-serif",
		"Helvetica, sans-serif",
		"Times New Roman",
		"Georgia, serif",
		"Verdana, sans-serif",
		"Courier New, monospace",
		"Segoe UI",
		"Roboto, sans-serif",
		"Open Sans, sans-serif",
		"Lato, sans-serif",
	];

	rsx! {
		div { class: "p-4 bg-[var(--card-bg)] text-[var(--card-text)] rounded-xl shadow-md",
			h3 { class: "text-lg font-semibold mb-4", "Typography" }

			div { class: "mb-6",
				label { class: "block text-sm font-medium mb-1", "Font Family" }
				select {
					class: "w-full bg-[var(--input-bg)] text-[var(--text-color)] border border-[var(--border-color)] rounded-md px-3 py-2",
					value: "{state().typography.font_family}",
					oninput: move |event| handle_font_family(event.value()),
					{common_fonts.iter().map(|font| rsx! {
						option { value: "{font}", "{font}" }
					})}
				}
				input {
					r#type: "text",
					class: "w-full mt-2 bg-[var(--input-bg)] text-[var(--text-color)] border border-[var(--border-color)] rounded-md px-3 py-2",
					value: "{state().typography.font_family}",
					oninput: move |event| handle_font_family(event.value()),
					placeholder: "Or enter custom font stack",
				}
			}

			div { class: "mb-6",
				label { class: "block text-sm font-medium mb-1", "Heading Font Family" }
				select {
					class: "w-full bg-[var(--input-bg)] text-[var(--text-color)] border border-[var(--border-color)] rounded-md px-3 py-2",
					value: "{state().typography.heading_font_family}",
					oninput: move |event| handle_heading_font_family(event.value()),
					{common_fonts.iter().map(|font| rsx! {
						option { value: "{font}", "{font}" }
					})}
				}
				input {
					r#type: "text",
					class: "w-full mt-2 bg-[var(--input-bg)] text-[var(--text-color)] border border-[var(--border-color)] rounded-md px-3 py-2",
					value: "{state().typography.heading_font_family}",
					oninput: move |event| handle_heading_font_family(event.value()),
					placeholder: "Or enter custom font stack",
				}
			}

			div { class: "grid grid-cols-2 gap-4 mb-6",
				div {
					label { class: "block text-sm font-medium mb-1", "Base Size" }
					input {
						r#type: "text",
						class: "w-full bg-[var(--input-bg)] text-[var(--text-color)] border border-[var(--border-color)] rounded-md px-3 py-2",
						value: "{state().typography.base_size}",
						oninput: move |event| handle_base_size(event.value()),
					}
				}

				div {
					label { class: "block text-sm font-medium mb-1", "Line Height" }
					input {
						r#type: "text",
						class: "w-full bg-[var(--input-bg)] text-[var(--text-color)] border border-[var(--border-color)] rounded-md px-3 py-2",
						value: "{state().typography.line_height}",
						oninput: move |event| handle_line_height(event.value()),
					}
				}
			}

			h4 { class: "text-md font-semibold mb-3", "Font Weights" }
			div { class: "grid grid-cols-2 gap-4",
				{
						let font_weights = state().typography.font_weights.clone();
						font_weights
								.into_iter()
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
													class: "w-full bg-[var(--input-bg)] text-[var(--text-color)] border border-[var(--border-color)] rounded-md px-3 py-2",
													value: "{weight}",
													oninput: move |event| {
															if let Ok(val) = event.value().parse::<u32>() {
																	handle_font_weight(name_clone.clone(), val);
															}
													},
												}
											}
										}
								})
				}
			}
		}
	}
}
