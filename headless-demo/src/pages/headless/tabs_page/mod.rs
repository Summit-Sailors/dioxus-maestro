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
		shared::EOrientation,
		tabs::{TabsContent, TabsList, TabsRoot, TabsTrigger},
	},
};
mod consts;

#[component]
pub fn TabsPage() -> Element {
	let features_list: Vec<&str> = Vec::from(["Controlled/uncontrolled state", "Keyboard navigation", "Horizontal and vertical orientation"]);

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Tabs",
			description: "A UI component that allows users to navigate between different sections of content by clicking on tab headers, displaying only the content of the selected tab while hiding the others.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start max-w-96 w-full",
					TabsRoot { default_value: "1", class: "flex flex-col gap-4",
						TabsList { class: "w-full flex items-center gap-6",
							TabsTrigger {
								value: "1",
								class: "data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 font-medium text-neutral-300 border-b border-b-transparent hover:border-b-orange-300 data-[state=active]:border-b-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[state=active]:focus-visible:border-b-neutral-100 focus-visible:text-neutral-100",
								"One"
							}
							TabsTrigger {
								value: "2",
								class: "data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 font-medium text-neutral-300 border-b border-b-transparent hover:border-b-orange-300 data-[state=active]:border-b-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[state=active]:focus-visible:border-b-neutral-100 focus-visible:text-neutral-100",
								"Two"
							}
							TabsTrigger {
								value: "3",
								class: "data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 font-medium text-neutral-300 border-b border-b-transparent hover:border-b-orange-300 data-[state=active]:border-b-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[state=active]:focus-visible:border-b-neutral-100 focus-visible:text-neutral-100",
								disabled: true,
								"Three"
							}
							TabsTrigger {
								value: "4",
								class: "data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 font-medium text-neutral-300 border-b border-b-transparent hover:border-b-orange-300 data-[state=active]:border-b-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[state=active]:focus-visible:border-b-neutral-100 focus-visible:text-neutral-100",
								"Four"
							}
						}
						TabsContent { value: "1",
							"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
						}
						TabsContent { value: "2",
							"Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
						}
						TabsContent { value: "3",
							"Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."
						}
						TabsContent { value: "4",
							"Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
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
					h4 { class: "font-medium text-lg text-orange-300", "TabsRoot" }
					p {
						"Wrapps all parts of the tabs and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"value"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_value_change"
						}
						span { class: "font-medium", " must go in pair" }
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
										tooltip_text: Some(
												"Must be used in pair with on_value_change callback".into(),
										),
								},
								PropsStruct {
										prop: "on_value_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<String>".into(),
										tooltip_text: Some("Must be used in pair with 'value' prop".into()),
								},
								PropsStruct {
										prop: "default_value".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Required if an uncontrollable state is used".into()),
								},
								PropsStruct {
										prop: "orientation".into(),
										prop_default: "EOrientation::Vertical".into(),
										prop_type: "EOrientation".into(),
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
				div {
					h4 { class: "font-medium text-lg text-orange-300", "TabsList" }
					p { class: "mb-4",
						"Contains the triggers that are aligned along the edge of the active content."
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
				div {
					h4 { class: "font-medium text-lg text-orange-300", "TabsTrigger" }
					p {
						"The button that activates its associated content. Must live in "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"TabsList"
						}
						" component."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "value".into(),
										prop_default: "-".into(),
										prop_type: "String".into(),
										tooltip_text: Some("Required. Indicates currently opened tab value".into()),
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Disables current tab".into()),
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
										attr: "data-state".into(),
										value: "active | inactive".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-expanded".into(),
										value: "true".into(),
										description: "Appears if current item is opened".into(),
								},
								AttrsStruct {
										attr: "aria-selected".into(),
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
					h4 { class: "font-medium text-lg text-orange-300", "TabsContent" }
					p { class: "mb-4", "Contains the content associated with each trigger." }
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "value".into(),
										prop_default: "-".into(),
										prop_type: "String".into(),
										tooltip_text: Some("Required. Indicates currently opened tab value".into()),
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
										attr: "active | inactive".into(),
										value: "open | closed".into(),
										description: "".into(),
								},
						]),
					}
				}
			}
		}
	}
}
