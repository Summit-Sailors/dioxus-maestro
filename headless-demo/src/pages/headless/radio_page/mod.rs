use {
	crate::components::{
		description_section::DescriptionSection,
		example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
		features_list::Features,
		tables::{AttrsStruct, PropsStruct, PropsTable},
		tabs::PageTabs,
	},
	consts::{EXAMPLE, EXAMPLE_ANATOMY},
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::ld_icons::LdSmile},
	maestro_headless::{
		radio_group::{RadioGroupIndicator, RadioGroupItem, RadioGroupRoot},
		shared::EOrientation,
		switch::{SwitchIndicator, SwitchRoot},
	},
	tailwind_fuse::tw_merge,
};

mod consts;

#[component]
pub fn RadioPage() -> Element {
	let features_list: Vec<&str> = Vec::from([
		"Controlled/uncontrolled state",
		"Keyboard navigation",
		"Group manages checked state of each radio button",
		"Supports vertical / horizontal orientation",
	]);

	let mut disabled = use_signal(|| false);
	let mut switch = use_signal(|| false);
	let mut use_custom_indicator = use_signal(|| false);
	let orientation = use_memo(move || if switch() { EOrientation::Vertical } else { EOrientation::Horizontal });

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Radio Group",
			description: "A group of radio buttons where only one item can be checked at a time.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "flex flex-wrap gap-5 items-center mb-4",
				div { class: "flex items-center gap-2 text-neutral-100 mb-4 mt-5",
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
				div { class: "flex items-center gap-2 text-neutral-100 mb-4 mt-5",
					SwitchRoot {
						checked: use_custom_indicator(),
						on_toggle_change: move |v| use_custom_indicator.set(v),
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Custom Indicator"
				}
			}
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex grow items-center justify-center w-full",
					RadioGroupRoot {
						default_value: "coffee",
						orientation: orientation(),
						class: "flex data-[orientation=horizontal]:items-center data-[orientation=vertical]:justify-center data-[orientation=vertical]:flex-col gap-4",
						disabled: disabled(),
						div { class: "flex items-center gap-2",
							RadioGroupItem {
								value: "coffee",
								id: "maestro-radio-1",
								class: "w-6 h-6 rounded-full flex items-center justify-center transition-colors border border-orange-400 hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-400 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50",
								RadioGroupIndicator {
									class: tw_merge!(
											"w-3 h-3 rounded-full", (! use_custom_indicator()).then_some("bg-orange-600")
									),
									if use_custom_indicator() {
										Icon {
											icon: LdSmile,
											class: "w-5 h-5 text-orange-400",
										}
									}
								}
							}
							label { r#for: "maestro-radio-1", "Coffee" }
						}
						div { class: "flex items-center gap-2",
							RadioGroupItem {
								value: "water",
								id: "maestro-radio-2",
								class: "w-6 h-6 rounded-full flex items-center justify-center transition-colors border border-orange-400 hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-400 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50",
								disabled: true,
								RadioGroupIndicator {
									class: tw_merge!(
											"w-3 h-3 rounded-full", (! use_custom_indicator()).then_some("bg-orange-600")
									),
									if use_custom_indicator() {
										Icon {
											icon: LdSmile,
											class: "w-5 h-5 text-orange-400",
										}
									}
								}
							}
							label { r#for: "maestro-radio-2", "Water" }
						}
						div { class: "flex items-center gap-2",
							RadioGroupItem {
								value: "juice",
								id: "maestro-radio-3",
								class: "w-6 h-6 rounded-full flex items-center justify-center transition-colors border border-orange-400 hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-400 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50",
								RadioGroupIndicator {
									class: tw_merge!(
											"w-3 h-3 rounded-full", (! use_custom_indicator()).then_some("bg-orange-600")
									),
									if use_custom_indicator() {
										Icon {
											icon: LdSmile,
											class: "w-5 h-5 text-orange-400",
										}
									}
								}
							}
							label { r#for: "maestro-radio-3", "Juice" }
						}
					}
				}
				ExampleCodeCollapsible { code: EXAMPLE }
			}
		}
		DescriptionSection { title: "Supports",
			Features { features: features_list.clone() }
		}
		DescriptionSection { title: "Usage and Anatomy",
			ExampleCodeAnatomy { code: EXAMPLE_ANATOMY }
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "RadioGroupRoot" }
					p {
						"Wrapps all radio buttons of the group and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"value"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_value_change"
						}
						" in case of usage "
						span { class: "font-medium", "controllable state must go in pair" }
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
										prop: "required".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
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
										description: "Appears only if the group is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-required".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the group is disabled".into(),
								},
								AttrsStruct {
										attr: "data-required".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "checked | unchecked".into(),
										description: "".into(),
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
					h4 { class: "font-medium text-lg text-orange-300", "RadioGroupItem" }
					p { "Indicates radio input." }
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
												"Prevents checkbox from selecting/deselecting current item".into(),
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
										description: "Appears only if group or current item is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if group or current item is disabled".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "checked | unchecked".into(),
										description: "".into(),
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
					h4 { class: "font-medium text-lg text-orange-300", "RadioGroupIndicator" }
					p {
						"Appears when the radio is checked. May accept checked component and must be wrapped by ."
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"RadioGroupItem"
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
										description: "Appears only if the group is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the group is disabled".into(),
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
