use {
	crate::{
		components::{
			description_section::DescriptionSection,
			example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
			features_list::Features,
			tables::{AttrsStruct, PropsStruct},
			tabs::PageTabs,
		},
		pages::accordion_page::consts::{EXAMPLE, EXAMPLE_ANATOMY},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsCheckLg},
	maestro_headless::{
		accordion::{Accordion, AccordionContent, AccordionHeader, AccordionItem, AccordionTrigger, AccordionVariant},
		shared::EOrientation,
	},
};

mod consts;

#[component]
pub fn AccordionPage() -> Element {
	let features_list: Vec<&str> =
		Vec::from(["Controlled/uncontrolled state", "Open single or multiple items", "Keyboard navigation", "Horizontal/vertical orientation"]);

	rsx! {
		DescriptionSection {
			title: "Accordion",
			description: "A vertically stacked set of interactive headings that each reveal an associated section of content.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start w-full max-w-96",
					Accordion {
						default_value: Vec::from(["1".into()]),
						class: "relative w-full grow max-w-96 flex flex-col rounded-sm bg-neutral-900 text-neutral-100 p-0.5 transition-all ease-linear overflow-hidden",
						variant: AccordionVariant::Single,
						AccordionItem {
							value: "1",
							class: "flex flex-col data-[state=open]:gap-3 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none border-b border-b-neutral-500",
							AccordionHeader {
								AccordionTrigger { class: "px-4 py-2 h-full w-full hover:bg-neutral-800 data-[state=open]:border-b border-b-neutral-700 data-[state=open]:text-orange-600 transition-all ease-linear focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:outline-none line-clamp-1",
									"Default opened"
								}
							}
							AccordionContent { class: "flex overflow-hidden data-[state=open]:h-fit data-[state=closed]:h-0 transition-all ease-linear px-4 data-[state=open]:py-2",
								"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
							}
						}
						AccordionItem {
							value: "2",
							class: "flex flex-col data-[state=open]:gap-3 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none border-b border-b-neutral-500",
							AccordionHeader {
								AccordionTrigger { class: "px-4 py-2 h-full w-full hover:bg-neutral-800 data-[state=open]:border-b border-b-neutral-700 data-[state=open]:text-orange-600 transition-all ease-linear focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:outline-none line-clamp-1",
									"Sed ut perspiciatis unde..."
								}
							}
							AccordionContent { class: "flex overflow-hidden data-[state=open]:h-fit data-[state=closed]:h-0 transition-all ease-linear px-4 data-[state=open]:py-2",
								"Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo."
							}
						}
						AccordionItem {
							value: "3",
							disabled: true,
							class: "flex flex-col data-[state=open]:gap-3 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none border-b border-b-neutral-500",
							AccordionHeader {
								AccordionTrigger { class: "px-4 py-2 h-full w-full hover:bg-neutral-800 data-[state=open]:border-b border-b-neutral-700 data-[state=open]:text-orange-600 transition-all ease-linear focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:outline-none line-clamp-1",
									"I'm disabled :("
								}
							}
							AccordionContent { class: "flex overflow-hidden data-[state=open]:h-fit data-[state=closed]:h-0 transition-all ease-linear px-4 data-[state=open]:py-2",
								"Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem."
							}
						}
						AccordionItem {
							value: "4",
							class: "flex flex-col data-[state=open]:gap-3 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none",
							AccordionHeader {
								AccordionTrigger { class: "px-4 py-2 h-full w-full hover:bg-neutral-800 data-[state=open]:border-b border-b-neutral-700 data-[state=open]:text-orange-600 transition-all ease-linear focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:outline-none line-clamp-1",
									"Ut enim ad minima veniam"
								}
							}
							AccordionContent { class: "flex overflow-hidden data-[state=open]:h-fit data-[state=closed]:h-0 transition-all ease-linear px-4 data-[state=open]:py-2",
								"Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? "
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
						"Wrapps all parts af the accordion and manages state. Props "
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
										prop_type: "Option<Vec<String>>".into(),
										tooltip_text: Some(
												"Must be used in pair with on_value_change callback".into(),
										),
								},
								PropsStruct {
										prop: "on_value_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<Vec<String>>".into(),
										tooltip_text: Some("Must be used in pair with value prop".into()),
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
										prop_type: "EOrientation::Vertical | EOrientation::Horizontal".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "collapsible".into(),
										prop_default: "true".into(),
										prop_type: "bool".into(),
										tooltip_text: Some(
												"If false and variant 'single', at least one item will always be opened"
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
										prop_type: "AccordionVariant::Single | AccordionVariant::Multiple".into(),
										tooltip_text: None,
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
										attr: "role".into(),
										value: "accordion".into(),
										description: "".into(),
								},
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
					h4 { class: "font-medium text-lg text-orange-300", "Item Component" }
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
										attr: "role".into(),
										value: "presentation".into(),
										description: "".into(),
								},
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
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Header Component" }
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
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Trigger Component" }
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
										attr: "data-role".into(),
										value: "accordion-trigger".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Content Component" }
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
