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
		shared::{EAlign, EOrientation, ESide},
		tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
	},
};

mod consts;

#[component]
pub fn PopoverPage() -> Element {
	let features_list: Vec<&str> = Vec::from([
		"Controlled/uncontrolled state",
		"Custom side, alignment, offsets",
		"Optionally render a pointing arrow",
		"Focus is automatically trapped in the content modal",
	]);

	rsx! {
		DescriptionSection { title: "Popover", description: "Renders content in a popup" }
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex grow items-center justify-center w-full",
					Popover { class: "w-fit",
						PopoverTrigger { class: "mx-auto w-10 h-10 flex justify-center items-center bg-neutral-900 hover:bg-neutral-700 border border-orange-600 text-neutral-300 hover:text-neutral-100 rounded-full focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors focus-visible:ring-orange-600 focus-visible:ring-offset-neutral-900",
							Icon { icon: BsInfo {} }
						}
						PopoverContent {
							side_offset: 4.0,
							align: EAlign::Center,
							class: "z-10 data-[state=open]:opacity-100 data-[state=closed]:opacity-0 bg-neutral-700 text-neutral-100 text-xs text-center rounded-sm p-2 transition-opacity ease-linear",
							div { class: "flex flex-col gap-3",
								h3 { class: "font-medium text-lg", "Here maybe any content you want" }
								div { class: "overflow-hidden rounded-lg border border-neutral-500 w-full h-40",
									img {
										class: "object-cover size-full",
										src: "https://www.blueplanetaquarium.com/wp-content/uploads/2023/09/iStock-1405520633-1024x682.jpg",
										alt: "whales",
									}
								}
								PopoverClose { class: "h-8 px-3 w-2/3 mx-auto rounded-md flex items-center justify-center bg-orange-600 text-neutral-100 border border-transparent transition-colors hover:bg-neutral-700 hover:border-orange-600 focus-visible:outline-none focus-visible:hover:bg-neutral-700 focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-700",
									"Close me"
								}
							}
							PopoverArrow {
								width: 16.0,
								height: 8.0,
								class: "text-neutral-700",
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
						"Wrapps all parts af the hover card component and manages state. Props "
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
				div {
					h4 { class: "font-medium text-lg text-orange-300", "Trigger Component" }
					p { "Opens/closes the popover content modal." }
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'a' attribules".into()),
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
										attr: "aria-haspopup".into(),
										value: "dialog".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-expanded".into(),
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
					h4 { class: "font-medium text-lg text-orange-300", "Content Component" }
					p { class: "mb-4", "The component that appears when the trigger pressed." }
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "side".into(),
										prop_default: "ESide::Bottom".into(),
										prop_type: "ESide".into(),
										tooltip_text: Some(
												"The preferred side of the trigger to render against when open".into(),
										),
								},
								PropsStruct {
										prop: "side_offset".into(),
										prop_default: "0.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some("The distance from the trigger (px)".into()),
								},
								PropsStruct {
										prop: "align".into(),
										prop_default: "EAlign::Center".into(),
										prop_type: "EAlign".into(),
										tooltip_text: Some("The preferred alignment against the trigger".into()),
								},
								PropsStruct {
										prop: "align_offset".into(),
										prop_default: "0.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some(
												"The offset from the 'start' or 'end' alignment options (px)".into(),
										),
								},
								PropsStruct {
										prop: "avoid_collisions".into(),
										prop_default: "true".into(),
										prop_type: "bool".into(),
										tooltip_text: Some(
												"Overrides the side preferences to prevent collisions with boundary edges."
														.into(),
										),
								},
								PropsStruct {
										prop: "collision_padding".into(),
										prop_default: "0.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some(
												"The distance in pixels from the boundary edges where collision detection should occur"
														.into(),
										),
								},
								PropsStruct {
										prop: "onmouseenter".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<MouseEvent>>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "onmouseleave".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<MouseEvent>>".into(),
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
										attr: "aria-modal".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-hidden".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "open | closed".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-side".into(),
										value: "left | right | bottom | top".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-align".into(),
										value: "start | end | center".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "--maestro-popover-anchor-width".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-popover-anchor-height".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-popover-content-width".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-popover-content-height".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
						]),
					}
				}
				div {
					h4 { class: "font-medium text-lg text-orange-300", "Arrow Component" }
					p { class: "mb-4",
						"An arrow element to render alongside the hover card. Must be rendered inside of "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"PopoverContent"
						}
						"."
					}
					PropsTable {
						content: Vec::from([
								PropsStruct {
										prop: "width".into(),
										prop_default: "10.0".into(),
										prop_type: "f32".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "height".into(),
										prop_default: "5.0".into(),
										prop_type: "f32".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'svg' attribules".into()),
								},
								PropsStruct {
										prop: "children".into(),
										prop_default: "-".into(),
										prop_type: "Element".into(),
										tooltip_text: Some("Required".into()),
								},
						]),
					}
				}
			}
		}
	}
}
