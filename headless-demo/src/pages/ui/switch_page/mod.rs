use {
	crate::components::{
		description_section::DescriptionSection,
		example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
		features_list::Features,
		tables::{AttrsStruct, PropsStruct},
		tabs::PageTabs,
	},
	consts::{EXAMPLE, EXAMPLE_ANATOMY},
	dioxus::prelude::*,
	maestro_ui::{
		select::{Select, SelectDropdown, SelectOption, SelectTrigger, SelectValue},
		switch::{Switch, SwitchRound, SwitchSize},
	},
	std::str::FromStr,
	strum::IntoEnumIterator,
};

mod consts;

#[component]
pub fn SwitchStyledPage() -> Element {
	let mut size = use_signal(|| Vec::from([SwitchSize::Sm.to_string()]));
	let mut round = use_signal(|| Vec::from([SwitchRound::Full.to_string()]));

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Switch",
			description: "A UI component that allows to toggle between two states (e.g., on/off, enabled/disabled) with a simple, interactive control, typically represented as a slider or button.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "flex flex-wrap gap-5 items-center mb-4",
				div { class: "flex flex-col gap-2 text-neutral-300 font-medium",
					"Size"
					Select {
						value: size(),
						on_value_change: move |value: Vec<String>| { size.set(value) },
						SelectTrigger {
							SelectValue { placeholder: "Chose something..." }
						}
						SelectDropdown {
							// side: ESide::Bottom,
							side_offset: 10.0,
							class: "w-60",
							for item in SwitchSize::iter() {
								SelectOption {
									key: item.to_string(),
									value: item.to_string(),
									"{item.to_string()}"
								}
							}
						}
					}
				}
				div { class: "flex flex-col gap-2 text-neutral-300 font-medium",
					"Round"
					Select {
						value: round(),
						on_value_change: move |value: Vec<String>| { round.set(value) },
						SelectTrigger {
							SelectValue { placeholder: "Chose something..." }
						}
						SelectDropdown {
							// side: ESide::Bottom,
							side_offset: 10.0,
							class: "w-60",
							for item in SwitchRound::iter() {
								SelectOption {
									key: item.to_string(),
									value: item.to_string(),
									"{item.to_string()}"
								}
							}
						}
					}
				}
			}
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex grow items-center justify-center w-full gap-2 text-neutral-100",
					Switch {
						size: SwitchSize::from_str(size().get(0).unwrap_or(&SwitchSize::default().to_string()))
								.ok()
								.unwrap_or(SwitchSize::default()),
						round: SwitchRound::from_str(round().get(0).unwrap_or(&SwitchRound::default().to_string()))
								.ok()
								.unwrap_or(SwitchRound::default()),
					}
					"Toggle me!"
				}
				ExampleCodeCollapsible { code: EXAMPLE }
			}
		}

		DescriptionSection { title: "Usage and Anatomy",
			ExampleCodeAnatomy { code: EXAMPLE_ANATOMY }
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "SwitchRoot" }
					p {
						"Wrapps switch component and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"checked"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_toggle_change"
						}
						span { class: "font-medium", " must go in pair" }
						" if case of usage "
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
										prop: "value".into(),
										prop_default: "None".into(),
										prop_type: "Option<String>".into(),
										tooltip_text: Some("Value for switch. Default 'on'/'off'".into()),
								},
								PropsStruct {
										prop: "checked".into(),
										prop_default: "None".into(),
										prop_type: "Option<bool>".into(),
										tooltip_text: Some("Must go in pair with 'on_toggle_change' callback".into()),
								},
								PropsStruct {
										prop: "on_toggle_change".into(),
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
										tooltip_text: Some("Prevents from toggling".into()),
								},
								PropsStruct {
										prop: "required".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
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
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the switch is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-required".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the switch is disabled".into(),
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
					h4 { class: "font-medium text-lg text-orange-300", "SwitchIndicator" }
					p {
						"Visually indicates whether the switch is on or off. Must live in "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"SwitchRoot"
						}
						" component."
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
										prop_type: "Element".into(),
										tooltip_text: Some("Custom checked indicator if specified".into()),
								},
						]),
						attrs_list: Vec::from([
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
