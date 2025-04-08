use {
	crate::components::{
		description_section::DescriptionSection,
		example_code::ExampleCodeCollapsible,
		features_list::Features,
		tables::{AttrsStruct, PropsStruct},
		tabs::PageTabs,
	},
	consts::EXAMPLE,
	dioxus::prelude::*,
	maestro_headless::{separator::Separator, shared::EOrientation},
};

mod consts;

#[component]
pub fn SeparatorPage() -> Element {
	let features_list: Vec<&str> = Vec::from(["Horizontal and vertical orientations"]);

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Separator",
			description: "Visually or semantically separates content.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start",
					div { class: "flex items-center flex-col text-neutral-100 gap-5",
						h3 { "Hello, this is Maestro Headless lib!" }
						Separator { class: "mx-4 bg-neutral-700 data-[orientation=horizontal]:h-px data-[orientation=vertical]:h-full data-[orientation=horizontal]:w-full data-[orientation=vertical]:w-px" }
						div { class: "flex gap-4 h-5",
							span { "Customizable" }
							Separator {
								orientation: EOrientation::Vertical,
								class: "bg-neutral-700 data-[orientation=horizontal]:h-px data-[orientation=vertical]:h-full data-[orientation=horizontal]:w-full data-[orientation=vertical]:w-px",
							}
							span { "Easy to use" }
							Separator {
								orientation: EOrientation::Vertical,
								class: "bg-neutral-700 data-[orientation=horizontal]:h-px data-[orientation=vertical]:h-full data-[orientation=horizontal]:w-full data-[orientation=vertical]:w-px",
							}
							span { "Flexible" }
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
					pre { "Separator {{ }}" }
				}
			}
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Root Component" }
					p { "Displays separator" }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PageTabs {
							props_list: Vec::from([
									PropsStruct {
											prop: "orientation".into(),
											prop_default: "EOrientation::Horizontal".into(),
											prop_type: "EOrientation".into(),
											tooltip_text: None,
									},
									PropsStruct {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'div' attribules".into()),
									},
							]),
							attrs_list: Vec::from([
									AttrsStruct {
											attr: "aria-orientation".into(),
											value: "horizontal | vertical".into(),
											description: "".into(),
									},
									AttrsStruct {
											attr: "data-orientation".into(),
											value: "horizontal | vertical".into(),
											description: "".into(),
									},
							]),
						}
					}
				}
			}
		}
	}
}
