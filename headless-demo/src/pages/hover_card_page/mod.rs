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
			bs_icons::{BsCheckLg, BsThreeDots},
			io_icons::IoLogoGithub,
			ld_icons::LdX,
		},
	},
	maestro_headless::{
		collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
		dialog::{Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogTitle, DialogTrigger},
		hover_card::{HoverCard, HoverCardArrow, HoverCardContent, HoverCardTrigger},
		shared::{EAlign, EOrientation, ESide},
		tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
	},
};

mod consts;

#[component]
pub fn HoverCardPage() -> Element {
	let mut is_open = use_signal(|| false);

	let features_list: Vec<&str> = Vec::from([
		"Controlled/uncontrolled state",
		"Custom side, alignment, offsets",
		"Optionally render a pointing arrow",
		"Custom open and close delays",
		"Hidden from screen readers.",
	]);

	rsx! {
		DescriptionSection {
			title: "Hover Card",
			description: "Preview some content behind a link.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex grow items-center justify-center w-full",
					HoverCard { class: "w-fit",
						HoverCardTrigger {
							class: "mx-auto w-10 h-10 flex justify-center items-center bg-neutral-900 hover:bg-neutral-700 border border-orange-600 text-neutral-300 hover:text-neutral-100 rounded-full focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors focus-visible:ring-orange-600 focus-visible:ring-offset-neutral-900",
							href: "https://github.com/Summit-Sailors/dioxus-maestro/tree/dev/frontend/maestro-headless",
							Icon { icon: IoLogoGithub, class: "w-5 h-5" }
						}
						HoverCardContent {
							side: ESide::Top,
							side_offset: 6.0,
							align: EAlign::Center,
							class: "bg-neutral-700 text-neutral-100 rounded-sm w-56 p-4 data-[state-open]:animate-fade-in data-[state=closed]:animate-fade-out z-50",
							div { class: "flex flex-col",
								h3 { class: "font-medium text-lg mb-1", "Maestro-Headless" }
								p { class: "text-neutral-300 mb-3",
									"The part of Dioxus-Maestro project"
								}
								ul { class: "list-disc list-inside",
									li { "Customizable" }
									li { "Flexible" }
									li { "Accessible" }
								}
							}
							HoverCardArrow {
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
										prop: "open_delay".into(),
										prop_default: "700.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some(
												"The delay of opening the hover card since a mouse is over the trigger"
														.into(),
										),
								},
								PropsStruct {
										prop: "	close_delay: f32,".into(),
										prop_default: "300.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some(
												"The delay of closing the hover card since a mouse leaves the trigger"
														.into(),
										),
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
					p { "Opens/closes the hover card content." }
					PageTabs {
						props_list: Vec::from([
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
										value: "modal".into(),
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
					p { class: "mb-4", "The component that appears when the hover card is open." }
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
										attr: "--maestro-hover-card-anchor-width".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-hover-card-anchor-height".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-hover-card-content-width".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-hover-card-content-height".into(),
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
							"HoverCardContent"
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
