use {
	crate::components::ui::component_section::ComponentSection,
	dioxus::prelude::*,
	dioxus_free_icons::{icons::fa_solid_icons::FaDiamond, Icon},
	dioxus_logger::tracing::info,
	maestro_toast::{ctx::use_toast, toast_info::ToastInfo, toast_position::EToastPosition},
	maestro_ui::{
		button::{Button, ButtonSize, ButtonVariant},
		input::{Input, InputVariant},
		label::Label,
		radio::Radio,
		select::{Select, SelectOption},
		spinner::FreeIconSpinner,
		textarea::Textarea,
		toggle::{EToggleSwitchLabelPlacement, ToggleSwitch, ToggleSwitchLabelStatesProp},
	},
};

#[component]
pub fn UIDemo() -> Element {
	let mut selected_option = use_signal(|| "Option 1".to_string());
	let mut selected_options = use_signal(Vec::<String>::new);
	let toggle_state = use_signal(|| false);
	let mut text_input = use_signal(String::new);
	let mut text_area_value = use_signal(String::new);
	let entered_text = use_signal(String::new);

	let mut selected_value = use_signal(|| "option1".to_string());

	// let handle_radio_change = move |value: String| {
	// 	let mut selected_value = selected_value.clone();
	// 	move |_| selected_value.set(value.clone())
	// };

	let mut toast = use_toast();

	let mut handle_button_click = move |button_message: String| {
		let info = ToastInfo {
			heading: Some("Button Click Handler".to_string()),
			context: button_message,
			icon: None,
			position: EToastPosition::TopRight,
			allow_toast_close: true,
			hide_after: 5,
		};
		toast.write().popup(info);
	};

	let handle_textarea_onenter_click = move |_| {
		let info = ToastInfo {
			heading: Some("Shift + Enter".to_string()),
			context: text_area_value(),
			icon: None,
			position: EToastPosition::TopRight,
			allow_toast_close: true,
			hide_after: 5,
		};
		toast.write().popup(info);
	};

	rsx! {
		div { class: "max-w-4xl mx-auto py-8 px-4",
			h1 { class: "text-3xl text-gray-800 dark:text-gray-100 text-center font-bold mb-8",
				"Maestro UI Components"
			}

			// buttons section
			ComponentSection {
				title: "Buttons",
				description: "Various button styles, sizes, and types with different variants",
				div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
					Button {
						class: "px-4 py-2 rounded-lg font-medium transition-colors text-white bg-blue-500 hover:bg-blue-700",
						r#type: "button",
						onclick: move |_| handle_button_click("Default Button clicked!".to_string()),
						"Default Button"
					}
					Button {
						class: "px-4 py-2 rounded-lg border border-gray-900 text-gray-800 hover:bg-gray-100",
						r#type: "reset",
						onclick: move |_| handle_button_click("Outline Button clicked!".to_string()),
						"Outline Button"
					}
					Button {
						class: "px-2 py-1 rounded-md bg-gray-300 text-gray-900 text-sm hover:bg-gray-400",
						size: ButtonSize::Sm,
						r#type: "submit",
						onclick: move |_| handle_button_click("Small Submit Button clicked!".to_string()),
						"Small Button"
					}
					Button {
						class: "px-6 py-3 rounded-lg bg-transparent border border-gray-300 text-gray-600 hover:text-black",
						size: ButtonSize::Lg,
						r#type: "button",
						onclick: move |_| handle_button_click("Large Ghost Button clicked!".to_string()),
						"Large Button"
					}
					Button {
						class: "text-blue-500 hover:text-blue-700",
						variant: ButtonVariant::Link,
						r#type: "button",
						onclick: move |_| handle_button_click("Link Button clicked!".to_string()),
						"Link Button"
					}
					Button {
						class: "px-6 py-3 bg-gray-300 hover:bg-gray-600 text-gray-900",
						variant: ButtonVariant::Icon,
						size: ButtonSize::IconLg,
						r#type: "button",
						onclick: move |_| handle_button_click("Icon Button clicked!".to_string()),
						children: rsx! {
							Icon {
								title: "Icon Button",
								icon: FaDiamond,
								width: 24,
								height: 24,
							}
						},
					}
				}
			}

			// input fields section
			ComponentSection {
				title: "Input Fields",
				description: "Text inputs with different variants and states",
				div { class: "space-y-4",
					Label { text: "Default Input".to_string(),
						Input {
							class: "border-gray-400 rounded-lg px-3 py-2 w-full focus:ring focus:ring-blue-100",
							value: text_input(),
							onchange: move |event: Event<FormData>| text_input.set(event.value()),
							placeholder: "Type something...",
						}
					}
					Label { text: "Underlined Input".to_string(),
						Input {
							class: "border-b border-gray-400 w-full focus:ring focus:ring-blue-300",
							variant: InputVariant::Underlined,
							value: text_input(),
							onchange: move |event: Event<FormData>| text_input.set(event.value()),
							placeholder: Some("Type something..."),
						}
					}
					Label { text: "Password Input".to_string(),
						Input {
							class: "border-gray-400 rounded-lg px-3 py-2 w-full focus:ring focus:ring-red-300",
							r#type: "password",
							value: text_input.read().to_string(),
							onchange: move |event: Event<FormData>| text_input.set(event.value()),
							placeholder: "Enter password...",
						}
					}
				}
			}

			// select and multiselect section
			ComponentSection {
				title: "Selection Components",
				description: "Single and multiple selection components",

				div { class: "space-y-6",
					Select {
						options: vec![
								SelectOption {
										label: "Option 1".to_string(),
										value: "Option 1".to_string(),
								},
								SelectOption {
										label: "Option 2".to_string(),
										value: "Option 2".to_string(),
								},
								SelectOption {
										label: "Option 3".to_string(),
										value: "Option 3".to_string(),
								},
						],
						current_value: Some(selected_option.read().to_string()),
						multi: false,
						callback: move |value| selected_option.set(value),
						multi_callback: move |_| {},
						label: Some("Single Select".into()),
						placeholder: Some("Select an option".into()),
					}

					Select {
						options: vec![
								SelectOption {
										label: "Item 1".to_string(),
										value: "Item 1".to_string(),
								},
								SelectOption {
										label: "Item 2".to_string(),
										value: "Item 2".to_string(),
								},
								SelectOption {
										label: "Item 3".to_string(),
										value: "Item 3".to_string(),
								},
						],
						current_value: None,
						multi: true,
						callback: move |_| {},
						multi_callback: move |value| selected_options.set(value),
						label: Some("Multi Select".into()),
						placeholder: Some("Select items...".into()),
					}
				}
			}

			// toggle and radio section
			ComponentSection {
				title: "Toggle and Radio",
				description: "Toggle switches and radio buttons",

				div { class: "space-y-6",
					div { class: "space-y-4",
						ToggleSwitch {
							state: toggle_state,
							label_states: Some(ToggleSwitchLabelStatesProp {
									on: "Enabled",
									off: "Disabled",
							}),
							label_placement: Some(EToggleSwitchLabelPlacement::Right),
						}
					}
					div { class: "space-y-6",
						div { class: " items-center gap-2",
							Radio {
								label: "Option 1",
								name: "group",
								value: "option1",
								checked: selected_value() == "option1",
								onchange: move |event: Event<FormData>| selected_value.set(event.value()),
							}
						}
						div { class: "items-center gap-2",
							Radio {
								class: "border-blue-500 hover:border-blue-700",
								checked_class: "!bg-blue-500 !border-blue-500 hover:!border-blue-700 hover:!bg-blue-700",
								inner_class: "border-blue-500",
								label: "Option 2",
								name: "group",
								value: "option2",
								checked: selected_value() == "option2",
								onchange: move |event: Event<FormData>| selected_value.set(event.value()),
							}
						}
						div { class: "items-center gap-2",
							Radio {
								label: "Option 3 (Disabled)",
								name: "group",
								disabled: true,
								value: "option3",
								checked: selected_value() == "option3",
								onchange: move |event: Event<FormData>| selected_value.set(event.value()),
							}
						}
						p { class: "text-sm", "Selected Option: {selected_value}" }
					}
				}
			}

			// textarea and spinner section
			ComponentSection {
				title: "Textarea and Loading",
				description: "Textarea component with multiple configurations and loading spinner",

				div { class: "space-y-6",
					// default Textarea
					Label { text: Some("Default Textarea".into()),
						Textarea {
							value: text_area_value(),
							onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
							placeholder: "Enter text here...",
							class: "w-full p-4 rounded-lg border-gray-500 focus:outline-none focus:ring-2 focus:ring-primary",
						}
					}

					// disabled Textarea
					Label { text: Some("Disabled Textarea".into()),
						Textarea {
							value: "Disabled content".to_string(),
							disabled: true,
							placeholder: "Cannot edit this text...",
							class: "w-full p-4 border-gray-500 rounded-lg text-gray-500 cursor-not-allowed",
						}
					}

					// textarea with on_enter functionality
					Label { text: Some("Textarea with Enter Handler".into()),
						Textarea {
							value: text_area_value(),
							onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
							onenter: handle_textarea_onenter_click,
							placeholder: "Type and press Shift+Enter...",
							class: "w-full p-4 border-gray-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary",
						}
					}
					p { class: "text-sm text-gray-600 italic",
						"Last entered text: {entered_text.read()}"
					}

					// textarea with custom styles
					Label { text: Some("Custom Styled Textarea".into()),
						Textarea {
							value: text_area_value(),
							onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
							placeholder: "Styled input...",
							class: "w-full p-4 border-2 border-dashed border-primary text-primary focus:outline-none focus:ring-2 focus:ring-primary",
							style: "padding: 1rem; font-style: italic;",
						}
					}

					// loading Spinner
					div { class: "flex justify-center items-center gap-4 mt-4 mb-8",
						FreeIconSpinner { size: 32 }
						span { class: "text-sm text-center text-gray-600", "Loading..." }
					}
				}
			}
		}
	}
}
