use {
	crate::{
		components::{
			description_section::DescriptionSection,
			example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
			tables::{AttrsStruct, PropsStruct},
			tabs::PageTabs,
		},
		pages::ui::toggle_page::consts::{EXAMPLE_GROUP, EXAMPLE_GROUP_ANATOMY},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{
		Icon,
		icons::ld_icons::{LdAlignCenter, LdAlignLeft, LdAlignRight},
	},
	maestro_headless::{
		shared::EOrientation,
		switch::{SwitchIndicator, SwitchRoot},
	},
	maestro_ui::toggle_group::{ToggleGroup, ToggleGroupItem},
};

#[component]
pub fn ToggleGroupContent() -> Element {
	let mut value = use_signal(|| "1".to_string());
	let mut switch = use_signal(|| false);
	let orientation = use_memo(move || if switch() { EOrientation::Vertical } else { EOrientation::Horizontal });
	let mut disabled = use_signal(|| false);

	rsx! {
		p { class: "container flex flex-col gap-3 lg:py-6 py-4 text-neutral-300",
			"A UI component that consists of multiple toggle buttons, where users can switch between different options, with only one option being active at a time, similar to a radio button group but with a toggle interface."
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
			div { class: "flex items-center gap-2 text-neutral-100 mb-4 mt-5",
				SwitchRoot {
					checked: switch(),
					on_toggle_change: move |v| switch.set(v),
					class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
					SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
				}
				"Vertical"
			}
		}
		div { class: "grow flex flex-col justify-center items-center overflow-hidden rounded-md border border-neutral-800 bg-neutral-950",
			div { class: "p-6 flex flex-col gap-4 items-start",
				ToggleGroup {
					value: value(),
					on_value_chenge: move |v: String| value.set(v),
					disabled: disabled(),
					orientation: orientation(),
					ToggleGroupItem { value: "1",
						Icon { icon: LdAlignRight }
					}
					ToggleGroupItem { value: "2",
						Icon { icon: LdAlignCenter }
					}
					ToggleGroupItem { value: "3",
						Icon { icon: LdAlignLeft }
					}
				}
			}
			ExampleCodeCollapsible { code: EXAMPLE_GROUP }
		}
		DescriptionSection { title: "Usage and Anatomy",
			ExampleCodeAnatomy { code: EXAMPLE_GROUP_ANATOMY }
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "ToggleGroupRoot" }
					p {
						"Wrapps all checkboxes of the toggle group and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"value"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_value_change"
						}
						span { class: "font-medium", " must go in pair" }
						" if use "
						span { class: "font-medium", "controllable state" }
						". In other case may be used "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"default_value"
						}
						"."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "value".into(),
										prop_default: "None".into(),
										prop_type: "Option<String>".into(),
										tooltip_text: Some("Must go in pair with 'on_value_change' callback".into()),
								},
								PropsStruct {
										prop: "on_value_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<String>".into(),
										tooltip_text: Some("Must go in pair with 'value' prop".into()),
								},
								PropsStruct {
										prop: "default_value".into(),
										prop_default: "''".into(),
										prop_type: "String".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "orientation".into(),
										prop_default: "EOrientation::Vertical".into(),
										prop_type: "EOrientation".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Prevents from toggling any item in the group".into()),
								},
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'div' attribules".into()),
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
										description: "Appears only if the whole group is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the whole group is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-orientation".into(),
										value: "horizontal | vartical".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-orientation".into(),
										value: "horizontal | vartical".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "ToggleGroupItem" }
					p { "Indicates toggle button." }
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "value".into(),
										prop_default: "-".into(),
										prop_type: "String".into(),
										tooltip_text: Some("Required".into()),
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some(
												"Prevents selecting/deselecting of the current item".into(),
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
										prop_default: "None".into(),
										prop_type: "Element".into(),
										tooltip_text: Some("Custom checked indicator if specified".into()),
								},
						]),
						attrs_list: Vec::from([
								AttrsStruct {
										attr: "aria-pressed".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the group or current item is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the group or current item is disabled".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "on | off".into(),
										description: "".into(),
								},
						]),
					}
				}
			}
		}
	}
}
