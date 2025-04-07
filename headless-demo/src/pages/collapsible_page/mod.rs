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
	dioxus_free_icons::{Icon, icons::bs_icons::BsCheckLg},
	maestro_headless::{
		collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
		shared::EOrientation,
		tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
	},
};
mod consts;

#[component]
pub fn CollapsiblePage() -> Element {
	let mut is_open = use_signal(|| false);

	let features_list: Vec<&str> = Vec::from(["Controlled/uncontrolled state", "Keyboard navigation"]);

	rsx! {
		DescriptionSection {
			title: "Collapsible",
			description: "An interactive component which expands/collapses a panel.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start max-w-96 w-full",
					Collapsible {
						class: "flex flex-col space-y-3 w-full",
						open: is_open(),
						on_open_change: move |value: bool| is_open.set(value),
						div { class: "flex justify-between items-center gap-3",
							span { class: "text-neutral-100", "Collapsible Component" }
							CollapsibleTrigger { class: "h-8 rounded-md flex items-center justify-center px-3 py-2 border border-neutral-300 text-neutral-300 hover:text-neutral-100 hover:border-neutral-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-neutral-300 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
								if is_open() {
									"Collapse"
								} else {
									"Expand"
								}
							}
						}
						CollapsibleContent { class: "data-[state=closed]:animate-slide-out data-[state=open]:animate-slide-in",
							span { class: "text-neutral-100",
								"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
							}
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
						"Wrapps all parts af the collapsible and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"open"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_open_change"
						}
						span { class: "font-medium", "must go in pair" }
						"if use "
						span { class: "font-medium", "controllable state" }
						". In other case may be used "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"default_open"
						}
						"."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "open".into(),
										prop_default: "None".into(),
										prop_type: "Option<bool>".into(),
										tooltip_text: Some(
												"Must be used in pair with on_open_change callback".into(),
										),
								},
								PropsStruct {
										prop: "on_open_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<bool>".into(),
										tooltip_text: Some("Must be used in pair with 'open' prop".into()),
								},
								PropsStruct {
										prop: "default_open".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Prevents toggling all items".into()),
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
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "open | closed".into(),
										description: "".into(),
								},
						]),
					}
				}
				div {
					h4 { class: "font-medium text-lg text-orange-300", "Trigger Component" }
					p { "Expands/collapces content." }
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
										attr: "aria-expanded".into(),
										value: "true".into(),
										description: "Appears if current item is opened".into(),
								},
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "".into(),
								},
						]),
					}
				}
				div {
					h4 { class: "font-medium text-lg text-orange-300", "Content Component" }
					p { class: "mb-4", "Contains the collapsible content." }
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
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-expanded".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "--maestro-headless-collapsible-width".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-headless-collapsible-height".into(),
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
