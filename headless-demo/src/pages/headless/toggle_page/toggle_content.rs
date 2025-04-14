use {
	crate::{
		components::{
			description_section::DescriptionSection,
			example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
			features_list::Features,
			tables::{AttrsStruct, PropsStruct},
			tabs::PageTabs,
		},
		pages::headless::toggle_page::consts::{EXAMPLE, EXAMPLE_ANATOMY},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsBrightnessAltHigh},
	maestro_headless::toggle::Toggle,
};

#[component]
pub fn ToggleContent() -> Element {
	let features_list: Vec<&str> = Vec::from(["Controlled/uncontrolled state", "Keyboard navigation"]);

	rsx! {
		p { class: "container flex flex-col gap-3 lg:py-6 py-4 text-neutral-300",
			"A UI component that allows users to switch between two states (e.g., on/off, true/false) with a simple action, often represented as a button or switch that visually indicates the current state."
		}
		div { class: "grow flex flex-col justify-center items-center overflow-hidden rounded-md border border-neutral-800 bg-neutral-950",
			div { class: "p-6 flex flex-col gap-4 items-start",
				div { class: "flex justify-center items-center gap-3",
					Toggle {
						class: "aria-[pressed=true]:bg-orange-600 bg-neutral-500 text-neutral-200 flex justify-center items-center p-3 w-10 h-10 rounded transition-colors hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 aria-[pressed=true]:hover:bg-orange-700 aria-[pressed=true]:focus-visible:ring-orange-600 aria-[pressed=false]:hover:bg-neutral-700 aria-[pressed=false]:focus-visible:ring-neutral-500",
						value: "on",
						default_pressed: false,
						Icon { icon: BsBrightnessAltHigh {} }
					}
				}
			}
			ExampleCodeCollapsible { code: EXAMPLE }
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
					h4 { class: "font-medium text-lg text-orange-300", "Toggle" }
					p {
						"The toggle itself. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"pressed"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_toggle_change"
						}
						span { class: "font-medium", " must go in pair" }
						" if use "
						span { class: "font-medium", "controllable state" }
						". In other case may be used "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"default_pressed"
						}
						"."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "pressed".into(),
										prop_default: "None".into(),
										prop_type: "Option<bool>".into(),
										tooltip_text: Some("Must go in pair with 'on_toggle_change' callback".into()),
								},
								PropsStruct {
										prop: "on_toggle_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<bool>".into(),
										tooltip_text: Some("Must go in pair with 'pressed' prop".into()),
								},
								PropsStruct {
										prop: "default_pressed".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Prevents from any interactions".into()),
								},
								PropsStruct {
										prop: "value".into(),
										prop_default: "-".into(),
										prop_type: "String".into(),
										tooltip_text: Some("Required".into()),
								},
								PropsStruct {
										prop: "extra_attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some(
												"Helps provide attributes to the toggle if it is nested into other components"
														.into(),
										),
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
										tooltip_text: None,
								},
						]),
						attrs_list: Vec::from([
								AttrsStruct {
										attr: "aria-pressed".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the toggle is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the toggle is disabled".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "on | off".into(),
										description: "".into(),
								},
						]),
					}
				}
			}
		}
	}
}
