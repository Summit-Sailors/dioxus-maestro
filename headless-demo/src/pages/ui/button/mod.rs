use {
	crate::components::{
		description_section::DescriptionSection,
		example_code::ExampleCodeCollapsible,
		features_list::Features,
		tables::{AttrsStruct, PropsStruct},
		tabs::PageTabs,
	},
	async_std::task::sleep,
	conts::EXAMPLE,
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::ld_icons::LdLoader},
	maestro_ui::button::Button,
	std::time::Duration,
};

mod conts;

#[component]
pub fn ButtonStyledPage() -> Element {
	let mut is_pending = use_signal(|| false);
	let features_list: Vec<&str> = Vec::from(["Wide list of event handlers", "Disabled/pending state", "Accepts any type of children"]);

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
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
				ExampleCodeCollapsible { code: EXAMPLE }
			}
		}
		DescriptionSection { title: "Supports",
			Features { features: features_list.clone() }
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
						" allow to prevent interaction with the button. For example, "
						span { class: "fpx-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"pending"
						}
						" is usefull for indicating some long tasks (like calling API)."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "pending".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "onclick".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<Event<MouseData>>>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "onkeydown".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<Event<KeyboardData>>>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "onkeyup".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<Event<KeyboardData>>>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "onfocus".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<Event<FocusData>>>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "onblur".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<Event<FocusData>>>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "onmousedown".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<Event<MouseData>>>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "onmouseup".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<Event<MouseData>>>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "onmouseenter".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<Event<MouseData>>>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "onmouseleave".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<Event<MouseData>>>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "onmounted".into(),
										prop_default: "None".into(),
										prop_type: "Option<EventHandler<Event<MountedData>>>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'button' attribules".into()),
								},
								PropsStruct {
										prop: "extra_attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some(
												"Helps provide attributes to the button if it is nested into other components"
														.into(),
										),
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
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if props 'disabled' or 'pending' is true".into(),
								},
								AttrsStruct {
										attr: "data-pressed".into(),
										value: "true".into(),
										description: "Indicates pressed state".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if props 'disabled' or 'pending' is true".into(),
								},
								AttrsStruct {
										attr: "data-pending".into(),
										value: "true".into(),
										description: "Indicates pending state".into(),
								},
						]),
					}
				}
			}
		}
	}
}
