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
		icons::{bs_icons::BsThreeDots, ld_icons::LdX},
	},
	maestro_headless::{
		dialog::{DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogRoot, DialogTitle, DialogTrigger},
		switch::{SwitchIndicator, SwitchRoot},
	},
};

mod consts;

#[component]
pub fn DialogPage() -> Element {
	let mut is_open = use_signal(|| false);

	let features_list: Vec<&str> =
		Vec::from(["Controlled/uncontrolled state", "Focus is automatically trapped within dialod", "Close dialog on press ESC key or click on Overlay"]);

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Dialog",
			description: "A component that displays a modal window to present content or interactions.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "flex flex-wrap gap-3 items-center mb-4",
				div { class: "flex items-center justify-center gap-2 text-neutral-100",
					SwitchRoot {
						checked: is_open(),
						on_toggle_change: move |v| is_open.set(v),
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Default open"
				}
			}
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex grow items-center justify-center w-full",
					DialogRoot {
						open: is_open(),
						on_open_change: move |v| is_open.set(v),
						DialogTrigger { class: "rounded-full w-10 h-10 flex items-center justify-center focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors bg-neutral-900 border border-neutral-300 text-orange-600 hover:border-neutral-100 focus-visible:ring-neutral-300 focus-visible:ring-offset-neutral-900",
							Icon { icon: BsThreeDots }
						}
						DialogOverlay { class: "w-full h-full fixed top-0 left-0 bottom-0 right-0 bg-neutral-900/20 backdrop-blur-sm z-[100] data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out" }
						DialogContent { class: "w-full h-96 lg:max-w-lg md:max-w-md max-w-2xs max-h-[95vh] fixed z-[110] left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rounded bg-neutral-900 shadow border border-neutral-600 flex flex-col gap-6 px-6 py-8 overflow-y-auto data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out",
							DialogHeader { class: "flex justify-between gap-4",
								div { class: "flex flex-col gap-2",
									DialogTitle { class: "font-medium text-2xl text-neutral-100",
										"Uncontrolled dialog"
									}
									DialogDescription { class: "text-neutral-300", "But it may be controlled too" }
								}
								DialogClose {
									title: "Close my popup",
									class: "w-8 h-8 flex items-center justify-center text-neutral-300 hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-neutral-300 transition-colors",
									Icon { width: 16, height: 16, icon: LdX }
								}
							}
							DialogBody { class: "flex flex-col gap-3",
								p { "Here may be any type of content, for example, an image" }
								div { class: "overflow-hidden rounded-lg border border-neutral-500 w-full h-40",
									img {
										class: "object-cover size-full",
										src: "https://www.blueplanetaquarium.com/wp-content/uploads/2023/09/iStock-1405520633-1024x682.jpg",
										alt: "whales",
									}
								}
							}
							DialogFooter {
								DialogClose {
									title: "Close my popup",
									class: "mx-auto h-8 flex items-center justify-center text-neutral-100 bg-orange-600 px-3 py-2 rounded-md hover:bg-neutral-900 border border-transparent hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-600 transition-colors",
									"Close"
								}
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
					h4 { class: "font-medium text-lg text-orange-300", "DialogRoot" }
					p {
						"Wrapps all parts of the dialog and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"open"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_open_change"
						}
						span { class: "font-medium", " must go in pair" }
						"in case of useage "
						span { class: "font-medium", "controllable state" }
						". Otherwise may be used "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"default_open"
						}
						"."
					}
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
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
											prop: "on_close".into(),
											prop_default: "None".into(),
											prop_type: "Callback<()>".into(),
											tooltip_text: Some("Triggers when dialog is closed".into()),
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
				div {
					h4 { class: "font-medium text-lg text-orange-300", "DialogOverlay" }
					p {
						"A layer that covers the whole screen when dialog is opened. Prevents body from scrolling."
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
						]),
					}
				}
				div {
					h4 { class: "font-medium text-lg text-orange-300", "DialogTrigger" }
					p { "Opens/closes the dialog." }
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Prevents the dialog from being opened".into()),
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
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-expanded".into(),
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
					h4 { class: "font-medium text-lg text-orange-300", "DialogContent" }
					p { class: "mb-4", "Contains a content (elements) of the dialog. Traps focus." }
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
										attr: "aria-modal".into(),
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
					h4 { class: "font-medium text-lg text-orange-300", "DialogHeader" }
					p { class: "mb-4",
						"Contains a heading section. May include "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"DialogTitle"
						}
						", "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"DialogDescription"
						}
						" or "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"DialogClose"
						}
						". Must be inside of "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"DialogContent"
						}
						". Optional."
					}
					PropsTable {
						content: Vec::from([
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
				div {
					h4 { class: "font-medium text-lg text-orange-300", "DialogTitle" }
					p { class: "mb-4", "Contains a title. Optional." }
					PropsTable {
						content: Vec::from([
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
				div {
					h4 { class: "font-medium text-lg text-orange-300", "DialogDescription" }
					p { class: "mb-4", "Contains a description section. Optional." }
					PropsTable {
						content: Vec::from([
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
				div {
					h4 { class: "font-medium text-lg text-orange-300", "DialogBody" }
					p { class: "mb-4",
						"Section for a main content. Must be wrapped by "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"DialogContent"
						}
						" component. Optional."
					}
					PropsTable {
						content: Vec::from([
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
				div {
					h4 { class: "font-medium text-lg text-orange-300", "DialogFooter" }
					p { class: "mb-4",
						"The most bottom part of the content. May contain "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"DialogClose"
						}
						". Must live in "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"DialogContent"
						}
						" component. Optional."
					}
					PropsTable {
						content: Vec::from([
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
				div {
					h4 { class: "font-medium text-lg text-orange-300", "DialogClose" }
					p { class: "mb-4", "Responsible for closing the popup. Optional." }
					PropsTable {
						content: Vec::from([
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
					}
				}
			}
		}
	}
}
