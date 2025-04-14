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
	maestro_headless::{
		accordion::AccordionVariant,
		shared::EOrientation,
		switch::{SwitchIndicator, SwitchRoot},
	},
	styled_accordion::StyledAccordion,
};

mod consts;
mod styled_accordion;

#[component]
pub fn AccordionPage() -> Element {
	let mut variant = use_signal(|| AccordionVariant::Single);
	let mut orientation = use_signal(|| EOrientation::Vertical);
	let mut disabled = use_signal(|| false);
	let mut collapsible = use_signal(|| true);
	let mut value = use_signal(|| Vec::from(["1".into()]));

	let features_list: Vec<&str> =
		Vec::from(["Controlled/uncontrolled state", "Open single or multiple items", "Keyboard navigation", "Horizontal/vertical orientation"]);

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Accordion",
			description: "UI component that allows to toggle the visibility of content within sections, one or multiple sections at a time.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4",
			div { class: "flex flex-wrap gap-3 items-center mb-4",
				div { class: "flex items-center justify-center gap-2 text-neutral-100",
					SwitchRoot {
						checked: disabled(),
						on_toggle_change: move |v| disabled.set(v),
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Disable"
				}
				div { class: "flex items-center justify-center gap-2 text-neutral-100",
					SwitchRoot {
						checked: variant() == AccordionVariant::Multiple,
						on_toggle_change: move |v| {
								if v {
										variant.set(AccordionVariant::Multiple);
								} else {
										variant.set(AccordionVariant::Single);
								}
								value.set(Vec::from(["1".to_string()]));
						},
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Multiple"
				}
				div { class: "flex items-center justify-center gap-2 text-neutral-100",
					SwitchRoot {
						checked: orientation() == EOrientation::Horizontal,
						on_toggle_change: move |v| {
								if v {
										orientation.set(EOrientation::Horizontal);
								} else {
										orientation.set(EOrientation::Vertical);
								}
						},
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Horizontal"
				}
				div { class: "flex items-center justify-center gap-2 text-neutral-100",
					SwitchRoot {
						checked: collapsible(),
						on_toggle_change: move |v| collapsible.set(v),
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Collapsible"
				}
			}
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-6 items-center justify-center w-full overflow-auto",
					StyledAccordion {
						value: Some(value()),
						on_value_change: move |v| value.set(v),
						orientation: orientation(),
						collapsible: collapsible(),
						variant: variant(),
						disabled: disabled(),
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
					h4 { class: "font-medium text-lg text-orange-300", "AccordionRoot" }
					p {
						"Wrapps all parts of the accordion and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"value"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_value_change"
						}
						" in case of "
						span { class: "font-medium", "controllable state" }
						" must go in pair. In other case may be used "
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
										prop_type: "Option<Vec<String>>".into(),
										tooltip_text: Some(
												"Must be used in pair with `on_value_change` callback".into(),
										),
								},
								PropsStruct {
										prop: "on_value_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<Vec<String>>".into(),
										tooltip_text: Some("Must be used in pair with `value` prop".into()),
								},
								PropsStruct {
										prop: "default_value".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<String>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "orientation".into(),
										prop_default: "EOrientation::Vertical".into(),
										prop_type: "EOrientation".into(),
										tooltip_text: Some(
												"Accepts `EOrientation::Vertical` or `EOrientation::Horizontal`".into(),
										),
								},
								PropsStruct {
										prop: "collapsible".into(),
										prop_default: "true".into(),
										prop_type: "bool".into(),
										tooltip_text: Some(
												"If false and variant `AccordionVariant::Single`, at least one item will always be opened"
														.into(),
										),
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Prevents toggling all items".into()),
								},
								PropsStruct {
										prop: "variant".into(),
										prop_default: "AccordionVariant::Single".into(),
										prop_type: "AccordionVariant".into(),
										tooltip_text: Some(
												"Accepts `AccordionVariant::Single` or `AccordionVariant::Multiple`"
														.into(),
										),
								},
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'ul' attribules".into()),
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
										description: "Appears only if the whole accordion is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-orientation".into(),
										value: "vertical | horizontal".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the whole accordion is disabled".into(),
								},
								AttrsStruct {
										attr: "data-orientation".into(),
										value: "vertical | horizontal".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-role".into(),
										value: "accordion".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "AccordionItem" }
					p {
						"Wraps all the parts of a collapsible section. Prop "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"value"
						}
						" is used for determining currently opened item(s)."
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"disabled"
						}
						" allows to disable "
						span { class: "font-medium", "current" }
						" item and prevent it from expanding and other interactions."
					}
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
										tooltip_text: Some("Prevents toggling current item".into()),
								},
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'li' attribules".into()),
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
										description: "Appears only if the current item is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the current item is disabled".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "open | closed".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-role".into(),
										value: "accordion-item".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-orientation".into(),
										value: "vertical | horizontal".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-orientation".into(),
										value: "vertical | horizontal".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "AccordionHeader" }
					p { class: "flex gap-1 mb-4",
						"Wrapps an "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"AccordionTrigger"
						}
						"."
					}
					PageTabs {
						props_list: Vec::from([
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
										attr: "data-state".into(),
										value: "open | closed".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-orientation".into(),
										value: "vertical | horizontal".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-orientation".into(),
										value: "vertical | horizontal".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "AccordionTrigger" }
					p {
						"Opens/closes current accordion item. Must be nested inside of the "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"AccordionHeader"
						}
						"."
					}
					PageTabs {
						props_list: Vec::from([
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
										tooltip_text: None,
								},
						]),
						attrs_list: Vec::from([
								AttrsStruct {
										attr: "data-state".into(),
										value: "open | closed".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-expanded".into(),
										value: "true".into(),
										description: "Appears if current item is opened".into(),
								},
								AttrsStruct {
										attr: "data-role".into(),
										value: "accordion-trigger".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "AccordionContent" }
					p { class: "mb-4", "Contains the collapsible content for an item." }
					PageTabs {
						props_list: Vec::from([
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
										attr: "data-state".into(),
										value: "open | closed".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-role".into(),
										value: "accordion-content".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-expanded".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-orientation".into(),
										value: "vertical | horizontal".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-orientation".into(),
										value: "vertical | horizontal".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "--maestro-headless-accordion-height".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-headless-accordion-height".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
						]),
					}
				}
			}
		}
	}
}
