use {
	crate::{
		components::{
			description_section::DescriptionSection,
			example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
			tables::{AttrsStruct, PropsStruct},
			tabs::PageTabs,
		},
		pages::ui::checkbox_page::consts::{EXAMPLE, EXAMPLE_ANATOMY},
	},
	dioxus::prelude::*,
	maestro_headless::{
		checkbox::{CheckboxIndicator, CheckboxRoot},
		select::{OptionSelectedIndicator, SelectDropdown, SelectIcon, SelectOption, SelectRoot, SelectTrigger, SelectValue},
		switch::{SwitchIndicator, SwitchRoot},
	},
	maestro_ui::{
		checkbox::{Checkbox, CheckboxIndicatorVariant, CheckboxRound, CheckboxSize},
		shared::ESide,
	},
	strum::IntoEnumIterator,
};

#[component]
pub fn CheckboxContent() -> Element {
	let mut size = use_signal(|| Vec::from([CheckboxSize::Md.to_string()]));
	let mut round = use_signal(|| Vec::from([CheckboxRound::Md.to_string()]));
	let mut variant = use_signal(|| Vec::from([CheckboxIndicatorVariant::Tick.to_string()]));
	let mut disabled = use_signal(|| false);

	rsx! {
		p { class: "container flex flex-col gap-3 lg:py-6 py-4 text-neutral-300",
			"A control that allows the user to toggle between checked and not checked."
		}
		div { class: "flex flex-wrap gap-5 items-center mb-4",
			div { class: "flex flex-col gap-2 text-neutral-300 font-medium",
				"Indicator Variant"
				SelectRoot {
					value: variant(),
					on_value_change: move |value: Vec<String>| { variant.set(value) },
					class: "relative w-fit",
					SelectTrigger { class: "rounded-sm border border-orange-400 bg-neutral-900 text-neutral-100 w-52 flex justify-between items-center gap-4 px-3 py-2 min-h-12 hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
						SelectValue {
							placeholder: "Chose something...",
							class: "data-[state=selected]:text-neutral-100 data-[state=placeholder]:text-neutral-500 overflow-ellipsis",
						}
						SelectIcon {}
					}
					SelectDropdown {
						side: ESide::Bottom,
						side_offset: 10.0,
						class: "rounded bg-neutral-900 text-neutral-200 border border-neutral-700 z-10 px-2 py-4 [&_*]:transition-all w-60 ",
						for item in CheckboxIndicatorVariant::iter() {
							SelectOption {
								key: item.to_string(),
								value: item.to_string(),
								class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto",
								"{item.to_string()}"
								OptionSelectedIndicator { class: "w-4 h-4" }
							}
						}
					}
				}
			}
			div { class: "flex flex-col gap-2 text-neutral-300 font-medium",
				"Size"
				SelectRoot {
					value: size(),
					on_value_change: move |value: Vec<String>| { size.set(value) },
					class: "relative w-fit",
					SelectTrigger { class: "rounded-sm border border-orange-400 bg-neutral-900 text-neutral-100 w-52 flex justify-between items-center gap-4 px-3 py-2 min-h-12 hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
						SelectValue {
							placeholder: "Chose something...",
							class: "data-[state=selected]:text-neutral-100 data-[state=placeholder]:text-neutral-500 overflow-ellipsis",
						}
						SelectIcon {}
					}
					SelectDropdown {
						side: ESide::Bottom,
						side_offset: 10.0,
						class: "rounded bg-neutral-900 text-neutral-200 border border-neutral-700 z-10 px-2 py-4 [&_*]:transition-all w-60 ",
						for item in CheckboxSize::iter() {
							SelectOption {
								key: item.to_string(),
								value: item.to_string(),
								class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto",
								"{item.to_string()}"
								OptionSelectedIndicator { class: "w-4 h-4" }
							}
						}
					}
				}
			}
			div { class: "flex flex-col gap-2 text-neutral-300 font-medium",
				"Border radius"
				SelectRoot {
					value: round(),
					on_value_change: move |value: Vec<String>| { round.set(value) },
					class: "relative w-fit",
					SelectTrigger { class: "rounded-sm border border-orange-400 bg-neutral-900 text-neutral-100 w-52 flex justify-between items-center gap-4 px-3 py-2 min-h-12 hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
						SelectValue {
							placeholder: "Chose something...",
							class: "data-[state=selected]:text-neutral-100 data-[state=placeholder]:text-neutral-500 overflow-ellipsis",
						}
						SelectIcon {}
					}
					SelectDropdown {
						side: ESide::Bottom,
						side_offset: 10.0,
						class: "rounded bg-neutral-900 text-neutral-200 border border-neutral-700 z-10 px-2 py-4 [&_*]:transition-all w-60 ",
						for item in CheckboxRound::iter() {
							SelectOption {
								key: item.to_string(),
								value: item.to_string(),
								class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto",
								"{item.to_string()}"
								OptionSelectedIndicator { class: "w-4 h-4" }
							}
						}
					}
				}
			}
		}
		div { class: "flex flex-wrap gap-5 items-center mb-4",
			div { class: "flex flex-wrap items-center gap-2 text-neutral-100 mb-4 mt-5",
				SwitchRoot {
					checked: disabled(),
					on_toggle_change: move |v| disabled.set(v),
					class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
					SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
				}
				"Disable"
			}
		}
		div { class: "grow flex flex-col justify-center items-center overflow-hidden rounded-md border border-neutral-800 bg-neutral-950",
			div { class: "p-6 flex flex-col gap-4 items-start",
				div { class: "flex justify-center items-center gap-3",
					Checkbox {
						id: "maestro-box",
						value: "some",
						disabled: disabled(),
						size: CheckboxSize::try_from(size().get(0).unwrap_or(&CheckboxSize::default().to_string()))
								.ok()
								.unwrap_or(CheckboxSize::default()),
						indicator_variant: CheckboxIndicatorVariant::try_from(
										variant().get(0).unwrap_or(&CheckboxIndicatorVariant::default().to_string()),
								)
								.ok()
								.unwrap_or(CheckboxIndicatorVariant::default()),
						round: CheckboxRound::try_from(
										round().get(0).unwrap_or(&CheckboxRound::default().to_string()),
								)
								.ok()
								.unwrap_or(CheckboxRound::default()),
					}
					label { class: "text-slate-100", r#for: "maestro-box", "Check Me" }
				}
			}
			ExampleCodeCollapsible { code: EXAMPLE }
		}

		DescriptionSection { title: "Usage and Anatomy",
			ExampleCodeAnatomy { code: EXAMPLE_ANATOMY }
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "CheckboxRoot" }
					p {
						"Indicates a checkbox input. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"checked"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_change"
						}
						span { class: "font-medium", " must go in pair" }
						"in case of usage "
						span { class: "font-medium", "controllable state" }
						". In other case may be used "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"default_checked"
						}
						"."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "checked".into(),
										prop_default: "None".into(),
										prop_type: "Option<bool>".into(),
										tooltip_text: Some("Must go in pair with 'on_value_change' callback".into()),
								},
								PropsStruct {
										prop: "on_value_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<bool>".into(),
										tooltip_text: Some("Must go in pair with 'checked' prop".into()),
								},
								PropsStruct {
										prop: "default_checked".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Prevents checkbox from selecting/deselecting".into()),
								},
								PropsStruct {
										prop: "required".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "value".into(),
										prop_default: "-".into(),
										prop_type: "String".into(),
										tooltip_text: Some("Required".into()),
								},
								PropsStruct {
										prop: "extra_attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some(
												"Helps provide attributes to the checkbox if it is nested into other components"
														.into(),
										),
								},
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'button' attribules".into()),
								},
								PropsStruct {
										prop: "children".into(),
										prop_default: "-".into(),
										prop_type: "Element".into(),
										tooltip_text: Some("Required".into()),
								},
						]),
						attrs_list: Vec::from([
								AttrsStruct {
										attr: "aria-checked".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the checkbox is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-required".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the checkbox is disabled".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "checked | unchecked".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "CheckboxIndicator" }
					p {
						"Appears when the checkbox is checked. May accept checked component. Must be wrapped by "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"CheckboxRoot"
						}
						"."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'span' attribules".into()),
								},
								PropsStruct {
										prop: "children".into(),
										prop_default: "None".into(),
										prop_type: "Option<Element>".into(),
										tooltip_text: None,
								},
						]),
						attrs_list: Vec::from([
								AttrsStruct {
										attr: "aria-checked".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the checkbox is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the checkbox is disabled".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "checked | unchecked".into(),
										description: "".into(),
								},
						]),
					}
				}
			}
		}
	}
}
