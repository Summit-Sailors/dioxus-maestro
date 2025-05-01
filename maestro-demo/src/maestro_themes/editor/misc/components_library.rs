use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaCopy};
use maestro_toast::{ctx::use_toast, toast_info::ToastInfo, toast_position::EToastPosition};
use maestro_ui::{
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
};

use crate::components::ui::component_section::ComponentSection;

// TODO: Apply CSS variables in a way that would correctly match the applied varibales during theme editing
pub fn get_components_section(components_section_id: &str) -> Result<VNode, RenderError> {
	let mut selected_option = use_signal(|| "Option 1".to_string());
	let mut selected_options = use_signal(Vec::<String>::new);

	let toggle_state = use_signal(|| false);
	let mut text_input = use_signal(String::new);
	let mut text_area_value = use_signal(String::new);
	let mut range_value = use_signal(|| 0);

	let mut selected_value = use_signal(|| "option1".to_string());

	let mut toast = use_toast();

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

	match components_section_id {
		"buttons" => {
			rsx! {
        div {
          ComponentSection {
            title: "Buttons",
            description: "Various button styles, sizes, and types with different variants",
            div {
              id: "maestro-ui-buttons",
              class: "grid grid-cols-1 md:grid-cols-3 gap-6 md:w-4/5 mx-auto w-full",
              Button {
                class: "bg-[color:var(--primary)] text-[color:var(--primary-foreground)] hover:bg-[color:oklch(0.52_0.19_263.83)]",
                r#type: "button",
                "Default"
              }
              Button {
                class: "rounded-lg border border-[color:var(--primary)] text-[color:var(--primary)] hover:bg-[color:var(--primary)] hover:text-[color:var(--primary-foreground)]",
                variant: ButtonVariant::Outline,
                r#type: "button",
                "Outline"
              }
              Button {
                class: "text-[color:var(--muted-foreground)] hover:underline hover:bg-[color:oklch(0.9_0_0)",
                variant: ButtonVariant::Ghost,
                r#type: "reset",
                "Ghost"
              }
              Button {
                class: "px-2 py-1 bg-[color:var(--primary)] text-[color:var(--foreground)] text-sm hover:bg-[color:oklch(0.9_0_0)]",
                size: ButtonSize::Sm,
                r#type: "button",
                "Small"
              }
              Button {
                class: "rounded-lg text-[color:var(--foreground)] bg-[color:var(--primary)] hover:bg-[color:oklch(0.9_0_0)]",
                size: ButtonSize::Xl,
                r#type: "button",
                "Large"
              }
              Button {
                class: "text-[color:var(--accent)] hover:text-[color:oklch(0.75_0.18_75.59)] hover:bg-[color:oklch(0.9_0_0)]",
                variant: ButtonVariant::Link,
                r#type: "button",
                "Link"
              }
              Button {
                class: "px-6 py-3 bg-[color:var(--primary)] hover:bg-[color:oklch(0.9_0_0)] text-[color:var(--foreground)]",
                variant: ButtonVariant::Icon,
                size: ButtonSize::IconLg,
                r#type: "button",
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
        }
      }
		},
		"input-and-labels" => {
			rsx! {
        div {
          // input fields section
          ComponentSection {
            title: "Input Fields",
            description: "Text inputs with different variants and states",
            div {
              id: "maestro-ui-inputs",
              class: "space-y-4 text-left md:w-4/5 mx-auto w-full",
              Label {
                class: "text-[color:var(--text-color)] block mb-2",
                text: "Default Input:".to_string(),
              }
              Input {
                class: "bg-[color:var(--input-bg)] border border-[color:var(--border-color)] rounded-lg px-3 py-2 w-full text-[color:var(--text-color)] focus:ring focus:ring-[color:var(--focus-ring)] focus:outline-none",
                value: text_input(),
                onchange: move |event: Event<FormData>| text_input.set(event.value()),
                placeholder: "Type something...",
              }

              Label {
                class: "text-[color:var(--text-color)] block mb-2",
                text: "Underlined Input:".to_string(),
              }
              Input {
                class: "bg-[color:var(--input-bg)] p-4 w-full text-[color:var(--text-color)] focus:ring focus:ring-[color:var(--focus-ring)] focus:outline-none",
                variant: InputVariant::Underlined,
                value: text_input(),
                onchange: move |event: Event<FormData>| text_input.set(event.value()),
                placeholder: Some("Type something..."),
              }

              Label {
                class: "text-[color:var(--text-color)] block mb-2",
                text: "Password Input:".to_string(),
              }
              Input {
                class: "bg-[color:var(--input-bg)] border border-[color:var(--border-color)] rounded-lg px-3 py-2 w-full text-[color:var(--text-color)] focus:ring focus:ring-[color:var(--destructive)] focus:outline-none",
                r#type: "password",
                value: text_input.read().to_string(),
                onchange: move |event: Event<FormData>| text_input.set(event.value()),
                placeholder: "Enter password...",
              }
            }
          }
        }
      }
		},
		"select-and-multiselect" => {
			rsx! {
        div {
          // select and multiselect section
          ComponentSection {
            title: "Selection Components",
            description: "Single and multiple selection components",
            div {
              id: "maestro-ui-select",
              class: "space-y-6 text-left md:w-4/5 mx-auto w-full",
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
                placeholder_class: "text-[color:var(--muted-text)]",
                dropdown_class: "bg-[color:var(--bg-color)] border border-[color:var(--border-color)]",
                option_class: "hover:bg-[color:var(--hover-bg)] bg-[color:var(--input-bg)] text-[color:var(--text-color)]",
                label_class: "text-[color:var(--text-color)]",
                button_class: "bg-[color:var(--bg-color)] text-[color:var(--text-color)]",
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
                placeholder_class: "text-[color:var(--muted-text)]",
                dropdown_class: "bg-[color:var(--bg-color)] border border-[color:var(--border-color)]",
                option_class: "hover:bg-[color:var(--hover-bg)] bg-[color:var(--input-bg)] text-[color:var(--text-color)]",
                label_class: "text-[color:var(--text-color)]",
                button_class: "bg-[color:var(--bg-color)] text-[color:var(--text-color)]",
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
                placeholder_class: "text-[color:var(--muted-text)]",
                dropdown_class: "bg-[color:var(--bg-color)] border border-[color:var(--border-color)]",
                option_class: "hover:bg-[color:var(--hover-bg)] bg-[color:var(--input-bg)] text-[color:var(--text-color)]",
                label_class: "text-[color:var(--text-color)]",
                button_class: "bg-[color:var(--bg-color)] text-[color:var(--text-color)]",
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
                placeholder_class: "text-[color:var(--muted-text)]",
                dropdown_class: "bg-[color:var(--bg-color)] border border-[color:var(--border-color)]",
                option_class: "hover:bg-[color:var(--hover-bg)] bg-[color:var(--input-bg)] text-[color:var(--text-color)]",
                label_class: "text-[color:var(--text-color)]",
                button_class: "bg-[color:var(--bg-color)] text-[color:var(--text-color)]",
                is_searchable: true,
              }
            }
          }
        }
      }
		},
		"toggle-and-radio" => {
			rsx! {
        div {
          // toggle and radio section
          ComponentSection {
            title: "Toggle and Radio",
            description: "Toggle switches and radio buttons",
            div {
              id: "maestro-ui-toggle-radio",
              class: "space-y-6 md:w-4/5 mx-auto w-full flex flex-col items-center",
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
              div { class: "text-[color:var(--text-color)]",
                Radio {
                  class: "border-[color:var(--primary)] hover:border-[color:var(--primary)]",
                  checked_class: "!bg-[color:var(--primary)] !border-[color:var(--primary)] hover:!border-[color:var(--primary)] hover:!bg-[color:var(--primary)]",
                  inner_class: "border-[color:var(--primary)]",
                  label: "Option 1",
                  name: "group",
                  value: "option1",
                  checked: selected_value() == "option1",
                  onchange: move |event: Event<FormData>| selected_value.set(event.value()),
                }
              }
              div { class: "text-[color:var(--text-color)]",
                Radio {
                  class: "border-[color:var(--primary)] hover:border-[color:var(--primary)]",
                  checked_class: "bg-[color:var(--primary)] border-[color:var(--primary)] group-hover:border-[color:var(--primary)] group-hover:bg-[color:var(--primary)]",
                  inner_class: "border-[color:var(--primary)]",
                  label: "Option 2",
                  name: "group",
                  value: "option2",
                  checked: selected_value() == "option2",
                  onchange: move |event: Event<FormData>| selected_value.set(event.value()),
                }
              }
              div { class: "text-[color:var(--muted-text)]",
                Radio {
                  label: "Option 3 (Disabled)",
                  name: "group",
                  disabled: true,
                  value: "option3",
                  checked: selected_value() == "option3",
                  onchange: move |event: Event<FormData>| selected_value.set(event.value()),
                }
              }
              p { class: "text-sm text-[color:var(--text-color)]",
                "Selected Option: {selected_value}"
              }
            }
          }
        }
      }
		},
		"textarea-spinner-range" => {
			rsx! {
        div {
          // textarea and spinner section
          ComponentSection {
            title: "Textarea and Loading",
            description: "Textarea component with multiple configurations and loading spinner",
            div {
              id: "maestro-ui-textarea-spinner",
              class: "space-y-6 text-left md:w-4/5 mx-auto w-full",
              // default Textarea
              Label {
                class: "text-[color:var(--text-color)]",
                text: Some("Default Textarea:".into()),
                Textarea {
                  value: text_area_value(),
                  onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
                  placeholder: "Enter text here...",
                  class: "w-full p-4 rounded-lg border-[color:var(--border-color)] bg-[color:var(--input-bg)] text-[color:var(--text-color)] focus:outline-none focus:ring-2 focus:ring-[color:var(--primary)]",
                }
              }

              // disabled Textarea
              Label {
                class: "text-[color:var(--text-color)]",
                text: Some("Disabled Textarea:".into()),
                Textarea {
                  value: "Disabled content".to_string(),
                  disabled: true,
                  placeholder: "Cannot edit this text...",
                  class: "w-full p-4 border-[color:var(--border-color)] bg-[color:var(--input-bg)] rounded-lg text-[color:var(--muted-text)] cursor-not-allowed",
                }
              }

              // textarea with on_enter functionality
              Label {
                class: "text-[color:var(--text-color)]",
                text: Some("Textarea with Enter Handler:".into()),
                Textarea {
                  value: text_area_value(),
                  onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
                  onenter: handle_textarea_onenter_click,
                  placeholder: "Type and press Shift+Enter...",
                  class: "w-full p-4 border-[color:var(--border-color)] bg-[color:var(--input-bg)] text-[color:var(--text-color)] rounded-lg focus:outline-none focus:ring-2 focus:ring-[color:var(--primary)]",
                }
              }

              // textarea with custom styles
              Label {
                class: "text-[color:var(--text-color)]",
                text: Some("Custom Styled Textarea:".into()),
                Textarea {
                  value: text_area_value(),
                  onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
                  placeholder: "Styled input...",
                  class: "w-full p-4 border-2 border-dashed border-[color:var(--primary)] text-[color:var(--primary)] bg-[color:var(--input-bg)] focus:outline-none focus:ring-2 focus:ring-[color:var(--primary)]",
                  style: "padding: 1rem; font-style: italic;",
                }
              }

              // loading Spinner
              div { class: "flex justify-center items-center text-[color:var(--text-color)] gap-4 mt-4 mb-8",
                FreeIconSpinner { size: 32 }
                span { class: "text-sm text-center text-[color:var(--muted-text)]",
                  "Loading..."
                }
              }

              Range {
                value: range_value(),
                min: 0,
                max: 100,
                step: 10,
                oninput: move |event: Event<FormData>| {
                    range_value.set(event.value().parse::<i32>().expect("Oh no"))
                },
                label_class: "text-[color:var(--text-color)]",
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
                label_class: "text-[color:var(--text-color)]",
                label: "Custom range:",
                value_class: "mt-2",
                thumb_class: "[&::-webkit-slider-thumb]:bg-[color:var(--range-thumb-bg)] [&::-webkit-slider-thumb]:ring-[color:var(--range-thumb-ring)] [&::-webkit-slider-thumb]:hover:ring-[color:var(--range-thumb-ring)] [&::-webkit-slider-thumb]:hover:bg-[color:var(--hover-bg)] [&::-webkit-slider-thumb]:rounded-sm [&::-moz-range-thumb]:bg-[color:var(--range-thumb-bg)] [&::-moz-range-thumb]:ring-[color:var(--range-thumb-ring)] [&::-moz-range-thumb]:hover:bg-[color:var(--hover-bg)] [&::-moz-range-thumb]:rounded-sm [&::-ms-thumb]:bg-[color:var(--range-thumb-bg)] [&::-ms-thumb]:ring-[color:var(--range-thumb-ring)] [&::-ms-thumb]:hover:ring-[color:var(--range-thumb-ring)] [&::-ms-thumb]:hover:bg-[color:var(--hover-bg)] [&::-ms-thumb]:rounded-sm",
                track_class: "[&::-webkit-slider-runnable-track]:bg-[color:var(--range-track-bg)] [&::-moz-range-track]:bg-[color:var(--range-track-bg)] [&::-ms-track]:bg-[color:var(--range-track-bg)] [&::-webkit-slider-runnable-track]:hover:bg-[color:var(--range-track-hover)] [&::-moz-range-track]:hover:bg-[color:var(--range-track-hover)] [&::-ms-track]:hover:bg-[color:var(--range-track-hover)]",
              }
            }
          }
        }
      }
		},
		_ => rsx! {
      div { "The components requested aren't available" }
    },
	}
}
