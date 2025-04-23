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
	maestro_headless::shared::{EAlign, ESide},
	maestro_ui::tooltip::{Tooltip, TooltipContent, TooltipRoot, TooltipTrigger},
};

mod consts;

#[component]
pub fn TooltipStyledPage() -> Element {
	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Tooltip",
			description: "A UI component that provides additional information or hints when a user hovers over or focuses on an element, typically displayed as a small overlay with text to help clarify or explain the item.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex grow items-center justify-center w-full",
					TooltipRoot { class: "mx-auto",
						Tooltip {
							TooltipTrigger { "+" }
							TooltipContent {
								side: ESide::Top,
								side_offset: 8.0,
								align: EAlign::Center,
								"Some help text"
							}
						}
					}
				}
				ExampleCodeCollapsible { code: EXAMPLE }
			}
		}

		DescriptionSection { title: "Usage and Anatomy",
			ExampleCodeAnatomy { code: EXAMPLE_ANATOMY }
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div {
					h4 { class: "font-medium text-lg text-orange-300", "TooltipRoot" }
					p { class: "mb-4", "Wrapps all parts of the tooltip component." }
					PropsTable {
						content: Vec::from([
								PropsStruct {
										prop: "delay_duration_ms".into(),
										prop_default: "700.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some(
												"The delay from when the mouse enters a tooltip trigger until the tooltip opens."
														.into(),
										),
								},
								PropsStruct {
										prop: "skip_delay_duration_ms".into(),
										prop_default: "300.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some(
												"The delay from when the mouse leaves a tooltip trigger until the tooltip closes."
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
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Tooltip" }
					p {
						"Wrapps all parts of the tooltip and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"open"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_open_change"
						}
						span { class: "font-medium", " must go in pair" }
						" if use "
						span { class: "font-medium", "controllable state" }
						". In other case may be used "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"default_open"
						}
						". Must be placed into "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"TooltipRoot"
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
										prop: "delay_duration".into(),
										prop_default: "None".into(),
										prop_type: "Option<f32>".into(),
										tooltip_text: Some("Overrides the duration given to the `Provider`.".into()),
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
					h4 { class: "font-medium text-lg text-orange-300", "TooltipTrigger" }
					p { "Opens/closes the tooltip content modal." }
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "onclick".into(),
										prop_default: "None".into(),
										prop_type: "Option<Eventhandler<Event<MouseData>>>".into(),
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
										attr: "data-state-open".into(),
										value: "delayed-open | instant-open | closed".into(),
										description: "".into(),
								},
						]),
					}
				}
				div {
					h4 { class: "font-medium text-lg text-orange-300", "TooltipContent" }
					p { class: "mb-4", "The component that appears when the enters the trigger." }
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
										attr: "--maestro-tooltip-anchor-width".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-tooltip-anchor-height".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-tooltip-content-width".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-tooltip-content-height".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
						]),
					}
				}
				div {
					h4 { class: "font-medium text-lg text-orange-300", "TooltipArrow" }
					p { class: "mb-4",
						"An arrow element to render alongside the tooltip. Must be rendered inside of "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"TooltipContent"
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
