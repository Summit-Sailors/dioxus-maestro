use {
	crate::components::ui::{component_section::ComponentSection, features::Features},
	dioxus::prelude::*,
	dioxus_free_icons::{icons::fa_solid_icons::FaCopy, Icon},
	maestro_toast::{ctx::use_toast, toast_info::ToastInfo, toast_position::EToastPosition},
	maestro_ui::{
		button::{Button, ButtonSize, ButtonVariant},
		input::{Input, InputVariant},
		label::Label,
		multi_select::MultiSelect,
		radio::Radio,
		range::Range,
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
	let mut range_value = use_signal(|| 0);

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
		div { class: "mx-auto p-4 bg-gray-900 rounded-lg shadow-lg",
    div { class: "mb-8",
      h1 { class: "text-gray-100 text-center text-3xl font-bold mb-2", "Maestro UI" }
      p { class: "text-gray-300 text-center",
        "Maestro UI is a comprehensive, type-safe, and highly customizable UI component library for Dioxus, designed to provide developers with powerful, flexible, and elegant UI building blocks."
      }
    }

    div {
      class: "flex space-x-2",  
      Features {
        title: "UI".to_string(),
        features: vec![
          "Type Safety: Leverages Rust's type system".to_string(),
          "Reactive Design: Built for Dioxus's reactive paradigm".to_string(),
          "Flexible Styling: Tailwind CSS integration".to_string(),
          "Comprehensive Components: Wide range of UI elements".to_string(),
          "Performance: Efficient and lightweight".to_string()
        ]
      }
    }

			// buttons section
			ComponentSection {
				title: "Buttons",
				description: "Various button styles, sizes, and types with different variants",
				div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
					Button {
						r#type: "button",
						onclick: move |_| handle_button_click("Default Button clicked!".to_string()),
						"Default"
					}
					Button {
						class: "rounded-lg text-gray-200 hover:bg-gray-800",
						variant: ButtonVariant::Outline,
						r#type: "button",
						onclick: move |_| handle_button_click("Outline Button clicked!".to_string()),
						"Outline"
					}
					Button {
						class: "text-gray-700",
						variant: ButtonVariant::Ghost,
						r#type: "reset",
						onclick: move |_| handle_button_click("Ghost Button clicked!".to_string()),
						"Ghost"
					}
					Button {
						class: "px-2 py-1 bg-gray-300 text-gray-900 text-sm hover:bg-gray-400",
						size: ButtonSize::Sm,
						r#type: "button",
						onclick: move |_| handle_button_click("Small Submit Button clicked!".to_string()),
						"Small"
					}
					Button {
						class: "rounded-lg",
						size: ButtonSize::Xl,
						r#type: "button",
						onclick: move |_| handle_button_click("Large Ghost Button clicked!".to_string()),
						"Large"
					}
					Button {
						class: "text-blue-500 hover:text-blue-700",
						variant: ButtonVariant::Link,
						r#type: "button",
						onclick: move |_| handle_button_click("Link Button clicked!".to_string()),
						"Link"
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
								icon: FaCopy,
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
        div { 
          class: "space-y-4 text-left",
          Label {
            class: "text-gray-200 block mb-2",
            text: "Default Input:".to_string(),
          }
          Input {
            class: "bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 w-full text-gray-100 focus:ring focus:ring-blue-500 focus:outline-none",
            value: text_input(),
            onchange: move |event: Event<FormData>| text_input.set(event.value()),
            placeholder: "Type something...",
          }
  
          Label {
            class: "text-gray-200 block mb-2",
            text: "Underlined Input:".to_string(),
          }
          Input {
            class: "bg-gray-800 p-4 w-full text-gray-100 focus:ring focus:ring-blue-500 focus:outline-none",
            variant: InputVariant::Underlined,
            value: text_input(),
            onchange: move |event: Event<FormData>| text_input.set(event.value()),
            placeholder: Some("Type something..."),
          }
  
          Label {
            class: "text-gray-200 block mb-2",
            text: "Password Input:".to_string(),
          }
          Input {
            class: "bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 w-full text-gray-100 focus:ring focus:ring-red-500 focus:outline-none",
            r#type: "password",
            value: text_input.read().to_string(),
            onchange: move |event: Event<FormData>| text_input.set(event.value()),
            placeholder: "Enter password...",
          }
        }
      }
    
    

			// select and multiselect section
			ComponentSection {
				title: "Selection Components",
				description: "Single and multiple selection components",

				div { 
          class: "space-y-6 text-left",
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
						current_value: selected_option(),
						onchange: move |value| selected_option.set(value),
						label: "Single Select:",
						placeholder: "Select an option",
            placeholder_class: "text-gray-500",
						dropdown_class: "bg-gray-900 border border-gray-700",
						option_class: "hover:bg-gray-500 bg-gray-800 text-gray-100",
						label_class: "text-gray-200",
						button_class: "bg-gray-900 text-gray-200"
					}

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
						current_value: selected_option(),
						onchange: move |value| selected_option.set(value),
						label: "Single Select with Search:",
						placeholder: "Select an option",
            placeholder_class: "text-gray-500",
						dropdown_class: "bg-gray-900 border border-gray-700",
						option_class: "hover:bg-gray-500 bg-gray-800 text-gray-100",
						label_class: "text-gray-200",
						button_class: "bg-gray-900 text-gray-200",
						is_searchable: true,
					}

					MultiSelect {
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
						current_value: selected_options(),
						onchange: move |value| selected_options.set(value),
						label: "Multi Select:",
						placeholder: "Select items...",
            placeholder_class: "text-gray-500",
            dropdown_class: "bg-gray-900 border border-gray-700",
						option_class: "hover:bg-gray-500 bg-gray-800 text-gray-100",
						label_class: "text-gray-200",
						button_class: "bg-gray-900 text-gray-200",
					}

					MultiSelect {
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
						current_value: selected_options(),
						onchange: move |value| selected_options.set(value),
						label: "Multi Select With Search:",
						placeholder: "Select items...",
            placeholder_class: "text-gray-500",
            dropdown_class: "bg-gray-900 border border-gray-700",
						option_class: "hover:bg-gray-500 bg-gray-800 text-gray-100",
						label_class: "text-gray-200",
						button_class: "bg-gray-900 text-gray-200",
						is_searchable: true,
					}
				}
			}

			// toggle and radio section
			ComponentSection {
				title: "Toggle and Radio",
				description: "Toggle switches and radio buttons",

				div { 
          class: "space-y-6",
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
					div { 
            class: "flex justify-center grid space-y-6",
						div { 
              class: "items-center gap-2 text-gray-300",
							Radio {
                class: "border-blue-500 hover:border-blue-700",
								checked_class: "!bg-blue-500 !border-blue-500 hover:!border-blue-700 hover:!bg-blue-700",
								inner_class: "border-blue-500",
								label: "Option 1",
								name: "group",
								value: "option1",
								checked: selected_value() == "option1",
								onchange: move |event: Event<FormData>| selected_value.set(event.value()),
							}
						}
						div { 
              class: "items-center gap-2 text-gray-300",
							Radio {
								class: "border-blue-500 hover:border-blue-700",
								checked_class: "bg-blue-500 border-blue-500 group-hover:border-blue-700 group-hover:bg-blue-700",
								inner_class: "border-blue-500",
								label: "Option 2",
								name: "group",
								value: "option2",
								checked: selected_value() == "option2",
								onchange: move |event: Event<FormData>| selected_value.set(event.value()),
							}
						}
						div { 
              class: "items-center gap-2 text-gray-300",
							Radio {
								label: "Option 3 (Disabled)",
								name: "group",
								disabled: true,
								value: "option3",
								checked: selected_value() == "option3",
								onchange: move |event: Event<FormData>| selected_value.set(event.value()),
							}
						}
						p { class: "text-sm text-gray-300", "Selected Option: {selected_value}" }
					}
				}
			}

			// textarea and spinner section
			ComponentSection {
				title: "Textarea and Loading",
				description: "Textarea component with multiple configurations and loading spinner",

				div { class: "space-y-6 text-left",
					// default Textarea
					Label {
            class:"text-gray-200",
            text: Some("Default Textarea:".into()),
						Textarea {
							value: text_area_value(),
							onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
							placeholder: "Enter text here...",
							class: "w-full p-4 rounded-lg border-gray-500 focus:outline-none focus:ring-2 focus:ring-primary",
						}
					}

					// disabled Textarea
					Label { 
            class:"text-gray-200",
            text: Some("Disabled Textarea:".into()),
						Textarea {
							value: "Disabled content".to_string(),
							disabled: true,
							placeholder: "Cannot edit this text...",
							class: "w-full p-4 border-gray-500 rounded-lg text-gray-500 cursor-not-allowed",
						}
					}

					// textarea with on_enter functionality
					Label {
            class:"text-gray-200",
            text: Some("Textarea with Enter Handler:".into()),
						Textarea {
							value: text_area_value(),
							onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
							onenter: handle_textarea_onenter_click,
							placeholder: "Type and press Shift+Enter...",
							class: "w-full p-4 border-gray-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary",
						}
					}

					// textarea with custom styles
					Label {
            class:"text-gray-200",
            text: Some("Custom Styled Textarea:".into()),
						Textarea {
							value: text_area_value(),
							onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
							placeholder: "Styled input...",
							class: "w-full p-4 border-2 border-dashed border-primary text-primary focus:outline-none focus:ring-2 focus:ring-primary",
							style: "padding: 1rem; font-style: italic;",
						}
					}

					// loading Spinner
					div { 
            class: "flex justify-center items-center text-gray-200 gap-4 mt-4 mb-8",
						FreeIconSpinner { size: 32 }
						span { class: "text-sm text-center text-gray-400", "Loading..." }
					}

					Range {
						value: range_value(),
						min: 0,
						max: 100,
						step: 10,
						oninput: move |event: Event<FormData>| {
							range_value.set(event.value().parse::<i32>().expect("Oh no"))
						},
            label_class: "text-gray-200",
						label: "Default range:",
					}

					Range {
						value: range_value(),
						min: 0,
						max: 100,
						step: 10,
						oninput: move |event: Event<FormData>| {
							range_value.set(event.value().parse::<i32>().expect("Oh no"))
						},
            label_class: "text-gray-200",
						label: "Custom range:",
						value_class: "mt-2",
						thumb_class: "[&::-webkit-slider-thumb]:bg-blue-600 [&::-webkit-slider-thumb]:ring-blue-600 [&::-webkit-slider-thumb]:hover:ring-blue-600 [&::-webkit-slider-thumb]:hover:bg-blue-800 [&::-webkit-slider-thumb]:rounded-sm [&::-moz-range-thumb]:bg-blue-600 [&::-moz-range-thumb]:ring-blue-600 [&::-moz-range-thumb]:hover:bg-blue-800 [&::-moz-range-thumb]:rounded-sm [&::-ms-thumb]:bg-blue-600 [&::-ms-thumb]:ring-blue-600 [&::-ms-thumb]:hover:ring-blue-600 [&::-ms-thumb]:hover:bg-blue-800 [&::-ms-thumb]:rounded-sm",
						track_class: "[&::-webkit-slider-runnable-track]:bg-blue-300 [&::-moz-range-track]:bg-blue-300 [&::-ms-track]:bg-blue-300 [&::-webkit-slider-runnable-track]:hover:bg-blue-300 [&::-moz-range-track]:hover:bg-blue-300 [&::-ms-track]:hover:bg-blue-300",
					}
				}
			}
		}
	}
}
