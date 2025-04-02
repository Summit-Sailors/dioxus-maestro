use {
	crate::components::{
		description_section::DescriptionSection,
		props_table::{PropsTable, TableBody},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{
		Icon,
		icons::{
			bs_icons::{BsCheckLg, BsThreeDots},
			ld_icons::LdX,
		},
	},
	maestro_headless::{
		collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
		dialog::{Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogTitle, DialogTrigger},
		shared::EOrientation,
		tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
	},
};

#[component]
pub fn DialogPage() -> Element {
	let mut is_open = use_signal(|| false);
	rsx! {
		DescriptionSection {
			title: "Dialog",
			description: "A window overlaid on either the primary window or another dialog window, rendering the content underneath inert.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex grow items-center justify-center w-full",
					Dialog {
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
				div { class: "py-3 w-full px-6 border-t border-neutral-800 bg-neutral-950 overflow-hidden",
					Collapsible { class: "flex flex-col  max-h-[640px] h-full ",
						div { class: "flex items-center gap-3 py-3",
							CollapsibleTrigger { class: "flex items-center justify-center px-3 py-2 font-medium rounded bg-orange-600 border-2 border-transparent hover:border-orange-600 text-neutral-50 hover:bg-neutral-950 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none transition-colors ease-linear",
								"Open Code"
							}
						}
						CollapsibleContent { class: "data-[state=closed]:animate-slide-out data-[state=open]:animate-slide-in overflow-auto",
							code { class: "font-mono whitespace-pre text-xs text-neutral-300",
								pre {
									"use dioxus::prelude::*;
use maestro_headless::collapsible::{{Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogTitle, DialogTrigger}};
use dioxus_free_icons::{{
		Icon,
		icons::{{
			bs_icons::BsThreeDots,
			ld_icons::LdX,
		}},
	}};

rsx! {{
    Dialog {{
      DialogTrigger {{
        class: 'rounded-full w-10 h-10 flex items-center justify-center focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors bg-neutral-900 border border-neutral-300 text-orange-600 hover:border-neutral-100 focus-visible:ring-neutral-300 focus-visible:ring-offset-neutral-900',
        Icon {{ icon: BsThreeDots }}
      }} 
      DialogOverlay {{ class: 'w-full h-full fixed top-0 left-0 bottom-0 right-0 bg-neutral-900/20 backdrop-blur-sm z-[100] data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out' }}
			DialogContent {{ 
        class: 'w-full h-96 lg:max-w-lg md:max-w-md max-w-2xs max-h-[95vh] fixed z-[110] left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rounded bg-neutral-900 shadow border border-neutral-600 flex flex-col gap-6 px-6 py-8 overflow-y-auto data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out',
				DialogHeader {{ 
          class: 'flex justify-between gap-4',
					div {{ 
            class: 'flex flex-col gap-2',
						DialogTitle {{ 
              class: 'font-medium text-2xl text-neutral-100',
							'Uncontrolled dialog'
						}}
						DialogDescription {{ class: 'text-neutral-300', 'But it may be controlled too' }}
          }}
					DialogClose {{
						title: 'Close my popup',
						class: 'w-8 h-8 flex items-center justify-center text-neutral-300 hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-neutral-300 transition-colors',
						Icon {{ width: 16, height: 16, icon: LdX }}
					}}
				}}
				DialogBody {{ 
          class: 'flex flex-col gap-3',
					p {{ 'Here may be any type of content, for example, an image' }}
					div {{ 
            class: 'overflow-hidden rounded-lg border border-neutral-500 w-full h-40',
						img {{
							class: 'object-cover size-full',
							src: 'https://www.blueplanetaquarium.com/wp-content/uploads/2023/09/iStock-1405520633-1024x682.jpg',
							alt: 'whales',
						}}
					}}
				}}
        DialogFooter {{
          DialogClose {{
            title: 'Close my popup',
            class: 'mx-auto h-8 flex items-center justify-center text-neutral-100 bg-orange-600 px-3 py-2 rounded-md hover:bg-neutral-900 border border-transparent hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-600 transition-colors',
            'Close'
          }}
        }}
			}}
    }}
}}"
								}
							}
						}
					}
				}
			}
		}
		DescriptionSection { title: "Supports",
			ul { class: "flex flex-col gap-2 *:flex *:items-center *:gap-2",
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Controlled/uncontrolled state"
				}
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Focus is automatically trapped within dialod"
				}
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Close dialog on press "
					span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
						"ESC"
					}
					" key or click on "
					span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
						"Overlay"
					}
					"."
				}
			}
		}
		DescriptionSection {
			title: "Usage and Anatomy",
			description: "Import all parts and piece them together. Each part may be styled separately, accept own properties and additional attributes, e.g. \"data\" or \"aria\" (althought they are provided by default).",
			div { class: "grow flex flex-col rounded-md border border-neutral-800 bg-neutral-950 p-6",
				code { class: "font-mono whitespace-pre text-xs text-neutral-300",
					pre {
						"Dialog {{
      DialogTrigger {{ }} 
      DialogOverlay {{ }}
			DialogContent {{ 
				DialogHeader {{ 
						DialogTitle {{ }}
						DialogDescription {{ }}
          }}
				}}
				DialogBody {{ }}
        DialogFooter {{
          DialogClose {{ }}
        }}
			}}
    }}"
					}
				}
			}
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
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "open".into(),
											prop_default: "None".into(),
											prop_type: "Option<bool>".into(),
											tooltip_text: Some(
													"Must be used in pair with on_open_change callback".into(),
											),
									},
									TableBody {
											prop: "on_open_change".into(),
											prop_default: "None".into(),
											prop_type: "Callback<bool>".into(),
											tooltip_text: Some("Must be used in pair with 'open' prop".into()),
									},
									TableBody {
											prop: "default_open".into(),
											prop_default: "false".into(),
											prop_type: "bool".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "on_close".into(),
											prop_default: "None".into(),
											prop_type: "Callback<()>".into(),
											tooltip_text: Some("Triggers when dialog is closed".into()),
									},
									TableBody {
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
					h4 { class: "font-medium text-lg text-orange-300", "Overlay Component" }
					p {
						"A layer that covers the whole screen when dialog is opened. Prevents body from scrolling."
					}
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'div' attribules".into()),
									},
									TableBody {
											prop: "children".into(),
											prop_default: "None".into(),
											prop_type: "Element".into(),
											tooltip_text: None,
									},
							]),
						}
					}
				}
				div {
					h4 { class: "font-medium text-lg text-orange-300", "Trigger Component" }
					p { "Opens/closes the dialog." }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "disabled".into(),
											prop_default: "false".into(),
											prop_type: "bool".into(),
											tooltip_text: Some("Prevents the dialog from being opened".into()),
									},
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'button' attribules".into()),
									},
									TableBody {
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
					h4 { class: "font-medium text-lg text-orange-300", "Content Component" }
					p { class: "mb-4", "Contains a content (elements) of the dialog. Traps focus." }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'div' attribules".into()),
									},
									TableBody {
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
					h4 { class: "font-medium text-lg text-orange-300", "Header Component" }
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
						"."
					}
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'div' attribules".into()),
									},
									TableBody {
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
					h4 { class: "font-medium text-lg text-orange-300", "Title Component" }
					p { class: "mb-4", "Contains a title." }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'div' attribules".into()),
									},
									TableBody {
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
					h4 { class: "font-medium text-lg text-orange-300", "Description Component" }
					p { class: "mb-4", "Contains a description section." }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'div' attribules".into()),
									},
									TableBody {
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
					h4 { class: "font-medium text-lg text-orange-300", "Body Component" }
					p { class: "mb-4", "Section for a main content." }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'div' attribules".into()),
									},
									TableBody {
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
					h4 { class: "font-medium text-lg text-orange-300", "Footer Component" }
					p { class: "mb-4",
						"The most bottom part of the content. May contain "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"DialogClose"
						}
						"."
					}
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'div' attribules".into()),
									},
									TableBody {
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
					h4 { class: "font-medium text-lg text-orange-300", "Close Component" }
					p { class: "mb-4", "Responsible for closing the popup." }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'button' attribules".into()),
									},
									TableBody {
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
}
