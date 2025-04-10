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
	dioxus_free_icons::{Icon, icons::bs_icons::BsInfo},
	maestro_headless::{
		popover::{PopoverArrow, PopoverClose, PopoverContent, PopoverRoot, PopoverTrigger},
		range::{Range, RangeRoot, RangeThumb, RangeTrack},
		select::{OptionSelectedIndicator, SelectDropdown, SelectIcon, SelectOption, SelectRoot, SelectTrigger, SelectValue},
		shared::{EAlign, EOrientation, ESide},
		switch::{SwitchIndicator, SwitchRoot},
	},
};

mod consts;

#[component]
pub fn PopoverPage() -> Element {
	let mut is_open = use_signal(|| false);
	let mut side = use_signal(|| Vec::from([ESide::Top.to_string()]));
	let mut align = use_signal(|| Vec::from([EAlign::Center.to_string()]));
	let mut side_offset = use_signal(|| Vec::from([6.0_f32]));
	let mut align_offset = use_signal(|| Vec::from([0.0_f32]));
	let mut show_nested = use_signal(|| false);

	let features_list: Vec<&str> = Vec::from([
		"Controlled/uncontrolled state",
		"Custom side, alignment, offsets",
		"Optionally render a pointing arrow",
		"Focus is automatically trapped in the content modal",
	]);

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Popover",
			description: "A UI component that displays content, typically triggered by clicking, providing additional information or options.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "flex flex-wrap gap-5 items-center mb-4",
				SelectRoot {
					value: side(),
					on_value_change: move |value: Vec<String>| { side.set(value) },
					class: "relative w-fit",
					SelectTrigger { class: "rounded-sm border border-orange-400 bg-neutral-900 text-neutral-100 w-52 flex justify-between items-center gap-4 px-3 py-2 min-h-12 hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
						SelectValue {
							placeholder: "Chose something...",
							class: "data-[state=selected]:text-neutral-100 data-[state=placeholder]:text-neutral-500 overflow-ellipsis",
						}
						SelectIcon {}
					}
					SelectDropdown {
						side: ESide::Bottom,
						side_offset: 10.0,
						class: "rounded bg-neutral-900 text-neutral-200 border border-neutral-700 z-10 px-2 py-4 [&_*]:transition-all w-60 ",
						SelectOption {
							key: 1,
							value: ESide::Top.to_string(),
							class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto",
							"Top"
							OptionSelectedIndicator { class: "w-4 h-4" }
						}
						SelectOption {
							key: 2,
							value: ESide::Left.to_string(),
							class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto",
							"Left"
							OptionSelectedIndicator { class: "w-4 h-4" }
						}
						SelectOption {
							key: 3,
							value: ESide::Bottom.to_string(),
							class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto",
							"Bottom"
							OptionSelectedIndicator { class: "w-4 h-4" }
						}
						SelectOption {
							key: 4,
							value: ESide::Right.to_string(),
							class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto",
							"Right"
							OptionSelectedIndicator { class: "w-4 h-4" }
						}
					}
				}
				SelectRoot {
					value: align(),
					on_value_change: move |value: Vec<String>| { align.set(value) },
					class: "relative w-fit",
					SelectTrigger { class: "rounded-sm border border-orange-400 bg-neutral-900 text-neutral-100 w-52 flex justify-between items-center gap-4 px-3 py-2 min-h-12 hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
						SelectValue {
							placeholder: "Chose something...",
							class: "data-[state=selected]:text-neutral-100 data-[state=placeholder]:text-neutral-500 overflow-ellipsis",
						}
						SelectIcon {}
					}
					SelectDropdown {
						side: ESide::Bottom,
						side_offset: 10.0,
						class: "rounded bg-neutral-900 text-neutral-200 border border-neutral-700 z-10 px-2 py-4 [&_*]:transition-all w-60 ",
						SelectOption {
							key: 1,
							value: EAlign::Start.to_string(),
							class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto",
							"Start"
							OptionSelectedIndicator { class: "w-4 h-4" }
						}
						SelectOption {
							key: 2,
							value: EAlign::Center.to_string(),
							class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto",
							"Center"
							OptionSelectedIndicator { class: "w-4 h-4" }
						}
						SelectOption {
							key: 3,
							value: EAlign::End.to_string(),
							class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto",
							"End"
							OptionSelectedIndicator { class: "w-4 h-4" }
						}
					}
				}
			}
			div { class: "flex flex-wrap gap-5 items-center mb-4",
				div {
					span { class: "mb-4 inline-block", "Side offset" }
					RangeRoot {
						class: "w-52 flex items-center",
						value: side_offset(),
						on_value_change: move |v| side_offset.set(v),
						min: 0.0,
						max: 20.0,
						RangeTrack { class: "flex-1 bg-neutral-600 rounded-full h-1",
							Range { class: "flex-1 bg-orange-600 rounded-full h-1" }
						}
						RangeThumb { class: "w-6 h-6 rounded-full bg-orange-600 flex items-center justify-center text-neutral-300 text-xs cursor-pointer transition-colors hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
							"{side_offset().get(0).unwrap_or(&0.0):.0}"
						}
					}
				}
				div {
					span { class: "mb-4 inline-block", "Align offset" }
					RangeRoot {
						class: "w-52 flex items-center",
						value: align_offset(),
						on_value_change: move |v| align_offset.set(v),
						min: 0.0,
						max: 20.0,
						RangeTrack { class: "flex-1 bg-neutral-600 rounded-full h-1",
							Range { class: "flex-1 bg-orange-600 rounded-full h-1" }
						}
						RangeThumb { class: "w-6 h-6 rounded-full bg-orange-600 flex items-center justify-center text-neutral-300 text-xs cursor-pointer transition-colors hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
							"{align_offset().get(0).unwrap_or(&0.0):.0}"
						}
					}
				}
			}
			div { class: "flex items-center gap-2 text-neutral-100 mb-4 mt-5",
				SwitchRoot {
					checked: show_nested(),
					on_toggle_change: move |v| show_nested.set(v),
					class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
					SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
				}
				"Show nested popover"
			}
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex grow items-center justify-center w-full",
					PopoverRoot { class: "w-fit",
						PopoverTrigger { class: "mx-auto w-10 h-10 flex justify-center items-center bg-neutral-900 hover:bg-neutral-700 border border-orange-600 text-neutral-300 hover:text-neutral-100 rounded-full focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors focus-visible:ring-orange-600 focus-visible:ring-offset-neutral-900",
							Icon { icon: BsInfo {} }
						}
						PopoverContent {
							side: ESide::try_from(side().get(0).unwrap_or(&"top".into())).ok().unwrap_or(ESide::Top),
							side_offset: *side_offset().get(0).unwrap_or(&6.0),
							align: EAlign::try_from(align().get(0).unwrap_or(&"center".into()))
									.ok()
									.unwrap_or(EAlign::Center),
							align_offset: *align_offset().get(0).unwrap_or(&0.0),
							class: "z-10 data-[state=open]:opacity-100 data-[state=closed]:opacity-0 bg-neutral-700 text-neutral-100 text-xs text-center rounded-sm p-2 transition-opacity ease-linear",
							div { class: "flex flex-col gap-4",
								h3 { class: "font-medium text-lg", "Here maybe any content you want" }
								div { class: "overflow-hidden rounded-lg border border-neutral-500 w-full h-40",
									img {
										class: "object-cover size-full",
										src: "https://www.blueplanetaquarium.com/wp-content/uploads/2023/09/iStock-1405520633-1024x682.jpg",
										alt: "whales",
									}
								}
								if show_nested() {
									PopoverRoot { class: "w-fit mx-auto ",
										PopoverTrigger { class: "mx-auto px-3 h-10 flex justify-center items-center bg-neutral-700 hover:bg-neutral-600 border border-orange-600 text-neutral-300 hover:text-neutral-100 rounded-full focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors focus-visible:ring-orange-600 focus-visible:ring-offset-neutral-900",
											"Nested Popover"
										}
										PopoverContent {
											side: ESide::Top,
											class: "z-10 data-[state=open]:opacity-100 data-[state=closed]:opacity-0 bg-neutral-100 text-neutral-800 text-xs text-center rounded-sm p-2 transition-opacity ease-linear",
											div { class: "flex flex-col gap-3",
												h3 { class: "font-medium text-lg",
													"Nested is here!"
												}
												PopoverClose { class: "h-8 px-3 w-2/3 mx-auto rounded-md flex items-center justify-center bg-orange-600 text-neutral-100 border border-transparent transition-colors hover:bg-neutral-700 hover:border-orange-600 focus-visible:outline-none focus-visible:hover:bg-neutral-700 focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-700",
													"Close"
												}
											}
											PopoverArrow {
												width: 16.0,
												height: 8.0,
												class: "text-neutral-100",
											}
										}
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
		DescriptionSection { title: "Usage and Anatomy",
			ExampleCodeAnatomy { code: EXAMPLE_ANATOMY }
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "PopoverRoot" }
					p {
						"Wrapps all parts of the popover component and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"open"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_open_change"
						}
						span { class: "font-medium", " must go in pair " }
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
					h4 { class: "font-medium text-lg text-orange-300", "PopoverTrigger" }
					p { "Opens/closes the popover content modal." }
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
										prop_default: "None".into(),
										prop_type: "Element".into(),
										tooltip_text: None,
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
					h4 { class: "font-medium text-lg text-orange-300", "PopoverContent" }
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
					h4 { class: "font-medium text-lg text-orange-300", "PopoverArrow" }
					p { class: "mb-4",
						"An arrow element to render alongside the popover. Must be rendered inside of "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"PopoverContent"
						}
						". Optional."
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
