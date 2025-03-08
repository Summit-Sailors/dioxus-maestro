use {
	crate::components::ui::{component_section::ComponentSection, features::Features},
	async_std::task::sleep,
	components::buttons_section::ButtonsSection,
	dioxus::prelude::*,
	dioxus_free_icons::{icons::ld_icons::LdX, Icon},
	dioxus_logger::tracing::info,
	maestro_headless::dialog::{Dialog, DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger},
	maestro_toast::{ctx::use_toast, toast_code::EToastCode, toast_info::ToastInfo, toast_position::EToastPosition},
	maestro_ui::{
		button::Button,
		input::{Input, InputVariant},
		label::Label,
		radio::Radio,
		range::Range,
		spinner::FreeIconSpinner,
		textarea::Textarea,
		toggle::{EToggleSwitchLabelPlacement, ToggleSwitch, ToggleSwitchLabelStatesProp},
	},
	std::time::Duration,
	tailwind_fuse::tw_merge,
};

pub mod components;

#[component]
pub fn UIDemo() -> Element {
	let selected_option = use_signal(|| "Option 1".to_string());
	let selected_options = use_signal(Vec::<String>::new);
	let toggle_state = use_signal(|| false);
	let mut text_input = use_signal(String::new);
	let mut text_area_value = use_signal(String::new);
	let mut range_value = use_signal(|| 0);

	let mut selected_value = use_signal(|| "option1".to_string());
	let mut dialog_open = use_signal(|| false);
	let mut dis = use_signal(String::new);

	// let handle_radio_change = move |value: String| {
	// 	let mut selected_value = selected_value.clone();
	// 	move |_| selected_value.set(value.clone())
	// };

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

	rsx! {
		div {
			id: "maestro-ui",
			class: "mx-auto p-4 bg-slate-900 rounded-lg shadow-lg",
			div { class: "mb-8",
				h1 { class: "text-slate-100 text-center text-3xl font-bold mb-2", "Maestro UI" }
				p { class: "text-slate-300 text-center",
					"Maestro UI is a comprehensive, type-safe, and highly customizable UI component library for Dioxus, designed to provide developers with powerful, flexible, and elegant UI building blocks."
				}
			}

			div { id: "maestro-ui-features", class: "flex space-x-2",
				Features {
					title: "Features".to_string(),
					features: vec![
							"Type Safety: Leverages Rust's type system".to_string(),
							"Reactive Design: Built for Dioxus's reactive paradigm".to_string(),
							"Flexible Styling: Tailwind CSS integration".to_string(),
							"Comprehensive Components: Wide range of UI elements".to_string(),
							"Performance: Efficient and lightweight".to_string(),
					],
				}
			}

			ButtonsSection {}

			// input fields section
			ComponentSection {
				title: "Input Fields",
				description: "Text inputs with different variants and states",
				div {
					id: "maestro-ui-inputs",
					class: "space-y-4 text-left md:w-4/5 mx-auto w-full",
					Label {
						class: "text-slate-200 block mb-2",
						text: "Default Input:".to_string(),
					}
					Input {
						class: "bg-slate-800 border border-slate-700 rounded-lg px-3 py-2 w-full text-slate-100 focus:ring focus:ring-blue-500 focus:outline-none",
						value: text_input(),
						onchange: move |event: Event<FormData>| text_input.set(event.value()),
						placeholder: "Type something...",
					}

					Label {
						class: "text-slate-200 block mb-2",
						text: "Underlined Input:".to_string(),
					}
					Input {
						class: "bg-slate-800 p-4 w-full text-slate-100 focus:ring focus:ring-blue-500 focus:outline-none",
						variant: InputVariant::Underlined,
						value: text_input(),
						onchange: move |event: Event<FormData>| text_input.set(event.value()),
						placeholder: Some("Type something..."),
					}

					Label {
						class: "text-slate-200 block mb-2",
						text: "Password Input:".to_string(),
					}
					Input {
						class: "bg-slate-800 border border-slate-700 rounded-lg px-3 py-2 w-full text-slate-100 focus:ring focus:ring-red-500 focus:outline-none",
						r#type: "password",
						value: text_input.read().to_string(),
						onchange: move |event: Event<FormData>| text_input.set(event.value()),
						placeholder: "Enter password...",
					}
				}
			}



			// select and multiselect section
			// ComponentSection {
			// 	title: "Selection Components",
			// 	description: "Single and multiple selection components",

			// 	div {
			// 		id: "maestro-ui-select",
			// 		class: "space-y-6 text-left md:w-4/5 mx-auto w-full",
			// 		Select {
			// 			options: vec![
			// 					SelectOption {
			// 							label: "Option 1".to_string(),
			// 							value: "Option 1".to_string(),
			// 					},
			// 					SelectOption {
			// 							label: "Option 2".to_string(),
			// 							value: "Option 2".to_string(),
			// 					},
			// 					SelectOption {
			// 							label: "Option 3".to_string(),
			// 							value: "Option 3".to_string(),
			// 					},
			// 			],
			// 			current_value: selected_option(),
			// 			onchange: move |value| selected_option.set(value),
			// 			label: "Single Select:",
			// 			placeholder: "Select an option",
			// 			placeholder_class: "text-slate-500",
			// 			dropdown_class: "bg-slate-900 border border-slate-700",
			// 			option_class: "hover:bg-slate-500 bg-slate-800 text-slate-100",
			// 			label_class: "text-slate-200",
			// 			button_class: "bg-slate-900 text-slate-200",
			// 		}

			// 		Select {
			// 			options: vec![
			// 					SelectOption {
			// 							label: "Option 1".to_string(),
			// 							value: "Option 1".to_string(),
			// 					},
			// 					SelectOption {
			// 							label: "Option 2".to_string(),
			// 							value: "Option 2".to_string(),
			// 					},
			// 					SelectOption {
			// 							label: "Option 3".to_string(),
			// 							value: "Option 3".to_string(),
			// 					},
			// 			],
			// 			current_value: selected_option(),
			// 			onchange: move |value| selected_option.set(value),
			// 			label: "Single Select with Search:",
			// 			placeholder: "Select an option",
			// 			placeholder_class: "text-slate-500",
			// 			dropdown_class: "bg-slate-900 border border-slate-700",
			// 			option_class: "hover:bg-slate-500 bg-slate-800 text-slate-100",
			// 			label_class: "text-slate-200",
			// 			button_class: "bg-slate-900 text-slate-200",
			// 			is_searchable: true,
			// 		}

			// 		MultiSelect {
			// 			options: vec![
			// 					SelectOption {
			// 							label: "Item 1".to_string(),
			// 							value: "Item 1".to_string(),
			// 					},
			// 					SelectOption {
			// 							label: "Item 2".to_string(),
			// 							value: "Item 2".to_string(),
			// 					},
			// 					SelectOption {
			// 							label: "Item 3".to_string(),
			// 							value: "Item 3".to_string(),
			// 					},
			// 			],
			// 			current_value: selected_options(),
			// 			onchange: move |value| selected_options.set(value),
			// 			label: "Multi Select:",
			// 			placeholder: "Select items...",
			// 			placeholder_class: "text-slate-500",
			// 			dropdown_class: "bg-slate-900 border border-slate-700",
			// 			option_class: "hover:bg-slate-500 bg-slate-800 text-slate-100",
			// 			label_class: "text-slate-200",
			// 			button_class: "bg-slate-900 text-slate-200",
			// 		}

			// 		MultiSelect {
			// 			options: vec![
			// 					SelectOption {
			// 							label: "Item 1".to_string(),
			// 							value: "Item 1".to_string(),
			// 					},
			// 					SelectOption {
			// 							label: "Item 2".to_string(),
			// 							value: "Item 2".to_string(),
			// 					},
			// 					SelectOption {
			// 							label: "Item 3".to_string(),
			// 							value: "Item 3".to_string(),
			// 					},
			// 			],
			// 			current_value: selected_options(),
			// 			onchange: move |value| selected_options.set(value),
			// 			label: "Multi Select With Search:",
			// 			placeholder: "Select items...",
			// 			placeholder_class: "text-slate-500",
			// 			dropdown_class: "bg-slate-900 border border-slate-700",
			// 			option_class: "hover:bg-slate-500 bg-slate-800 text-slate-100",
			// 			label_class: "text-slate-200",
			// 			button_class: "bg-slate-900 text-slate-200",
			// 			is_searchable: true,
			// 		}
			// 	}
			// }

			// // toggle and radio section
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
					div { class: "text-slate-300",
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
					div { class: "text-slate-300",
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
					div { class: "text-slate-50",
						Radio {
							label: "Option 3 (Disabled)",
							name: "group",
							disabled: true,
							value: "option3",
							checked: selected_value() == "option3",
							onchange: move |event: Event<FormData>| selected_value.set(event.value()),
						}
					}
					p { class: "text-sm text-slate-300", "Selected Option: {selected_value}" }
				}
			}

			// textarea and spinner section
			ComponentSection {
				title: "Textarea and Loading",
				description: "Textarea component with multiple configurations and loading spinner",

				div {
					id: "maestro-ui-textarea-spinner",
					class: "space-y-6 text-left md:w-4/5 mx-auto w-full",
					// default Textarea
					Label { class: "text-slate-200", text: "Default Textarea:",
						Textarea {
							value: text_area_value(),
							onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
							placeholder: "Enter text here...",
							class: "w-full p-4 rounded-lg border-slate-500 focus:outline-none focus:ring-2 focus:ring-primary",
						}
					}

					// disabled Textarea
					Label { class: "text-slate-200", text: "Disabled Textarea:",
						Textarea {
							value: "Disabled content".to_string(),
							disabled: true,
							placeholder: "Cannot edit this text...",
							class: "w-full p-4 border-slate-500 rounded-lg text-slate-500 cursor-not-allowed",
						}
					}

					// textarea with on_enter functionality
					Label {
						class: "text-slate-200",
						text: "Textarea with Enter Handler:",
						Textarea {
							value: text_area_value(),
							onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
							onenter: handle_textarea_onenter_click,
							placeholder: "Type and press Shift+Enter...",
							class: "w-full p-4 border-slate-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary",
						}
					}

					// textarea with custom styles
					Label {
						class: "text-slate-200",
						text: "Custom Styled Textarea:",
						Textarea {
							value: text_area_value(),
							onchange: move |event: Event<FormData>| text_area_value.set(event.value()),
							placeholder: "Styled input...",
							class: "w-full p-4 border-2 border-dashed border-primary text-primary focus:outline-none focus:ring-2 focus:ring-primary",
							style: "padding: 1rem; font-style: italic;",
						}
					}

					// loading Spinner
					div { class: "flex justify-center items-center text-slate-200 gap-4 mt-4 mb-8",
						FreeIconSpinner { size: 32 }
						span { class: "text-sm text-center text-slate-400", "Loading..." }
					}

					Range {
						value: range_value(),
						min: 0,
						max: 100,
						step: 10,
						oninput: move |event: Event<FormData>| {
								range_value.set(event.value().parse::<i32>().expect("Oh no"))
						},
						label_class: "text-slate-200",
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
						label_class: "text-slate-200",
						label: "Custom range:",
						value_class: "mt-2",
						thumb_class: "[&::-webkit-slider-thumb]:bg-blue-600 [&::-webkit-slider-thumb]:ring-blue-600 [&::-webkit-slider-thumb]:hover:ring-blue-600 [&::-webkit-slider-thumb]:hover:bg-blue-800 [&::-webkit-slider-thumb]:rounded-sm [&::-moz-range-thumb]:bg-blue-600 [&::-moz-range-thumb]:ring-blue-600 [&::-moz-range-thumb]:hover:bg-blue-800 [&::-moz-range-thumb]:rounded-sm [&::-ms-thumb]:bg-blue-600 [&::-ms-thumb]:ring-blue-600 [&::-ms-thumb]:hover:ring-blue-600 [&::-ms-thumb]:hover:bg-blue-800 [&::-ms-thumb]:rounded-sm",
						track_class: "[&::-webkit-slider-runnable-track]:bg-blue-300 [&::-moz-range-track]:bg-blue-300 [&::-ms-track]:bg-blue-300 [&::-webkit-slider-runnable-track]:hover:bg-blue-300 [&::-moz-range-track]:hover:bg-blue-300 [&::-ms-track]:hover:bg-blue-300",
					}
					Button {
						class: "flex items-center justify-center rounded-full bg-slate-300 border border-slate-300 hover:bg-slate-600 hover:text-slate-50 h-10 w-10 text-slate-900 transition-colors ease-linear focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-slate-100 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900 disabled:opacity-50 disabled:pointer-events-none",
						r#type: "button",
						onclick: move |_| dis.set("text-3xl".into()),
						"Disable"
					}
					{info!("{}", dis())}
					Dialog { open: dialog_open,
						DialogTrigger { class: tw_merge!("bg-orange-600 text-white font-medium text-lg rounded px-3 py-2", dis()),
							"Open dialog"
						}
						DialogPortal {
							DialogOverlay { class: "fixed top-0 left-0 right-0 bottom-0 bg-gray-950/20 inset-0 backdrop-blur-sm z-[100] !m-0 opacity-0 transition-all linear duration-1000 data-[state=open]:opacity-100" }
							DialogContent { class: "max-w-96 w-full rounded-md bg-white text-gray-800 shadow p-4 flex flex-col gap-5 z-[110] fixed mx-auto my-auto top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2 max-h-80 opacity-0 transition-opacity data-[state=open]:opacity-100",
								div { class: "flex items-start justify-between gap-4",
									DialogTitle { class: "text-xl font-semibold", "Dialog Title" }
									DialogClose { class: "w-4 h-4 text-gray-600 hover:text-gray-800",
										Icon { icon: LdX }
									}
								}
								DialogDescription {
									p { class: "text-base text-gray-600 text-center",
										"Some description"
									}
								}
								div { class: "w-full flex flex-col items-center justify-center",
									p {
										"this button may play role of submit and will close dialog now. But also it will simulate close after some time (for example, submit)"
									}
									Button {
										r#type: "button",
										onclick: move |_| {
												spawn(async move {
														let info = ToastInfo {
																heading: Some("Button Click Handler".to_string()),
																context: "Will close in a second".into(),
																icon: Some(EToastCode::Success),
																position: EToastPosition::TopRight,
																allow_toast_close: true,
																hide_after: 5,
														};
														toast.write().popup(info);
														sleep(Duration::from_secs(1)).await;
														dialog_open.set(false)
												});
										},
										"Close"
									}
								}
							}
						}
					}
				}
			}
		}
	}
}
