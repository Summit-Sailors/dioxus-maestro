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
	dioxus_free_icons::{
		Icon,
		icons::{
			bs_icons::{BsCheckLg, BsInfo, BsThreeDots},
			io_icons::IoLogoGithub,
			ld_icons::LdX,
		},
	},
	maestro_headless::{
		collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
		dialog::{Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogTitle, DialogTrigger},
		popover::{Popover, PopoverArrow, PopoverClose, PopoverContent, PopoverTrigger},
		radio_group::{RadioGroup, RadioGroupIndicator, RadioGroupItem},
		shared::{EAlign, EOrientation, ESide},
		tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
	},
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

	rsx! {
		DescriptionSection {
			title: "Radio Group",
			description: "A group of radio buttons where only one item can be checked at a time.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex grow items-center justify-center w-full",
					RadioGroup { name: "radio", class: "flex items-center gap-4",
						div { class: "flex items-center gap-2",
							RadioGroupItem {
								value: "coffee",
								id: "maestro-radio-1",
								class: "w-6 h-6 rounded-full flex items-center justify-center transition-colors border border-orange-400 hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-400 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50",
								RadioGroupIndicator { class: "w-3 h-3 rounded-full bg-orange-600" }
							}
							label { r#for: "maestro-radio-1", "Coffee" }
						}
						div { class: "flex items-center gap-2",
							RadioGroupItem {
								value: "water",
								id: "maestro-radio-2",
								class: "w-6 h-6 rounded-full flex items-center justify-center transition-colors border border-orange-400 hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-400 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50",
								disabled: true,
								RadioGroupIndicator { class: "w-3 h-3 rounded-full bg-orange-600" }
							}
							label { r#for: "maestro-radio-2", "Water" }
						}
						div { class: "flex items-center gap-2",
							RadioGroupItem {
								value: "juice",
								id: "maestro-radio-3",
								class: "w-6 h-6 rounded-full flex items-center justify-center transition-colors border border-orange-400 hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-400 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50",
								RadioGroupIndicator { class: "w-3 h-3 rounded-full bg-orange-600" }
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
		DescriptionSection {
			title: "Usage and Anatomy",
			description: "Import all parts and piece them together. Each part may be styled separately, accept own properties and additional attributes, e.g. \"data\" or \"aria\" (althought they are provided by default).",
			ExampleCodeAnatomy { code: EXAMPLE_ANATOMY }
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Root Component" }
					p {
						"Wrapps all radio buttons of the group and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"value"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_value_change"
						}
						span { class: "font-medium", "must go in pair" }
						"if use "
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
										prop_type: "EOrientation::Vertical | EOrientation::Horizontal".into(),
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
										description: "Appears only if the whole accordion is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-required".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the whole accordion is disabled".into(),
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
					h4 { class: "font-medium text-lg text-orange-300", "Item Component" }
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
										prop_default: "None".into(),
										prop_type: "Element".into(),
										tooltip_text: Some("Custom checked indicator if specified".into()),
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
										description: "Appears only if the whole accordion is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the whole accordion is disabled".into(),
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
					h4 { class: "font-medium text-lg text-orange-300", "Indicator Component" }
					p { "Appears when the radio is checked. May accept checked component." }
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
										description: "Appears only if the whole accordion is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the whole accordion is disabled".into(),
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
