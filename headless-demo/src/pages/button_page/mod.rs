use {
	crate::components::{
		description_section::DescriptionSection,
		props_table::{PropsTable, TableBody},
	},
	async_std::task::sleep,
	dioxus::prelude::*,
	dioxus_free_icons::{
		Icon,
		icons::{bs_icons::BsCheckLg, ld_icons::LdLoader},
	},
	maestro_headless::{
		button::Button,
		collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
		shared::EOrientation,
	},
	std::time::Duration,
};

#[component]
pub fn ButtonPage() -> Element {
	let mut is_pending = use_signal(|| false);

	rsx! {
		DescriptionSection {
			title: "Button",
			description: "Button component with different states, such as pending and disabled.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start",
					Button {
						pending: is_pending,
						class: "h-10 px-3 py-2 rounded-md flex items-center justify-center gap-3 transition-all ease-linear bg-neutral-900 text-neutral-100 border border-neutral-100 hover:border-orange-600 focus-visible:border-orange-600 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50 data-[pending=true]:opacity-50",
						onclick: move |_| {
								is_pending.set(true);
								spawn(async move {
										sleep(Duration::from_millis(1000)).await;
										is_pending.set(false);
								});
						},
						if is_pending() {
							{
									rsx! {
										"Pending"
										Icon { icon: LdLoader, class: "animate-spin ease-linear" }
									}
							}
						} else {
							"Active"
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
use maestro_headless::button::Button;

let mut is_pending = use_signal(|| false);

rsx! {{
  Button {{
    pending: is_pending,,
    class: 'h-10 px-3 py-2 rounded-md flex items-center justify-center gap-3 transition-all ease-linear bg-neutral-900 text-neutral-100 border border-neutral-100 hover:border-orange-600 focus-visible:border-orange-600 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50 data-[pending=true]:opacity-50',
    onclick: move |_| {{
      is_pending.set(true);
      spawn(async move {{
        sleep(Duration::from_millis(1000)).await;
        is_pending.set(false);
      }});
    }},
    if is_pending() {{
      {{
        rsx! {{
          'Pending'
          Icon {{ icon: LdLoader, class: 'animate-spin ease-linear' }}			
          }}
        }} else {{'Active'}}
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
					"Wide list of event handlers"
				}
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Disabled/pending state"
				}
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Accepts any type of children"
				}
			}
		}
		DescriptionSection { title: "Usage and Anatomy",
			div { class: "grow flex flex-col rounded-md border border-neutral-800 bg-neutral-950 p-6",
				code { class: "font-mono whitespace-pre text-xs text-neutral-300",
					pre { "Button {{ }}" }
				}
			}
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Root Component" }
					p {
						"Simple html button . Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"disabled"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"pending"
						}
						span { class: "font-medium", "must go in pair" }
						"allows to prevent interaction with button. For example, "
						span { class: "font-medium", "pending" }
						" is usefull for indicating long tasks."
					}
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "pending".into(),
											prop_default: "false".into(),
											prop_type: "bool".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "disabled".into(),
											prop_default: "false".into(),
											prop_type: "bool".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "onclick".into(),
											prop_default: "None".into(),
											prop_type: "Option<EventHandler<Event<MouseData>>>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "onkeydown".into(),
											prop_default: "None".into(),
											prop_type: "Option<EventHandler<Event<KeyboardData>>>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "onkeyup".into(),
											prop_default: "None".into(),
											prop_type: "Option<EventHandler<Event<KeyboardData>>>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "onfocus".into(),
											prop_default: "None".into(),
											prop_type: "Option<EventHandler<Event<FocusData>>>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "onblur".into(),
											prop_default: "None".into(),
											prop_type: "Option<EventHandler<Event<FocusData>>>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "onmousedown".into(),
											prop_default: "None".into(),
											prop_type: "Option<EventHandler<Event<MouseData>>>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "onmouseup".into(),
											prop_default: "None".into(),
											prop_type: "Option<EventHandler<Event<MouseData>>>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "onmouseenter".into(),
											prop_default: "None".into(),
											prop_type: "Option<EventHandler<Event<MouseData>>>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "onmouseleave".into(),
											prop_default: "None".into(),
											prop_type: "Option<EventHandler<Event<MouseData>>>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "onmounted".into(),
											prop_default: "None".into(),
											prop_type: "Option<EventHandler<Event<MountedData>>>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'button' attribules".into()),
									},
									TableBody {
											prop: "extra_attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some(
													"Helps provide attributes to the button if it is nested into other components"
															.into(),
											),
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
			}
		}
	}
}
