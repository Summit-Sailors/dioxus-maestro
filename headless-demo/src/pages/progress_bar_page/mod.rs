use {
	crate::components::{
		description_section::DescriptionSection,
		example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
		features_list::Features,
		tables::{AttrsStruct, PropsStruct, PropsTable},
		tabs::PageTabs,
	},
	async_std::task::sleep,
	consts::{EXAMPLE, EXAMPLE_ANATOMY},
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsArrowCounterclockwise},
	maestro_headless::{
		button::Button,
		progress::{ProgressIndicator, ProgressRoot},
	},
	std::time::Duration,
};

mod consts;

#[component]
pub fn ProgressBarPage() -> Element {
	let mut progress = use_signal(|| 0.0_f32);
	let max_value = 80.0;

	use_future(move || async move {
		loop {
			let current_progress = *progress.peek();
			if current_progress < max_value {
				progress.set(current_progress + 2.0);
			}
			sleep(Duration::from_millis(200)).await;
		}
	});

	let features_list: Vec<&str> = Vec::from(["Possibility to display the progress of some task."]);

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Progress Bar",
			description: "An indicator showing the completion progress of a task.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex flex-col grow items-center justify-center gap-4 w-full",
					ProgressRoot {
						class: "w-56 h-2 rounded-md overflow-hidden relative bg-neutral-300",
						value: progress(),
						ProgressIndicator { class: "size-full bg-orange-600 rounded-md transition-transform duration-500" }
					}
					Button {
						class: "w-8 h-8 flex justify-center items-center bg-neutral-900 hover:bg-neutral-700 border border-neutral-300 text-neutral-300 hover:text-neutral-100 rounded-full focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors focus-visible:neutral-300 focus-visible:ring-offset-neutral-900",
						onclick: move |_| progress.set(0.0),
						Icon { icon: BsArrowCounterclockwise }
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
					h4 { class: "font-medium text-lg text-orange-300", "ProgressRoot" }
					p { "Wrapps all parts of the progress component." }
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "value".into(),
										prop_default: "0.0".into(),
										prop_type: "f32".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "max".into(),
										prop_default: "100.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some("The maximum progress value".into()),
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
										value: "init | loading | completed".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-valuemax".into(),
										value: "<f32>".into(),
										description: "Maximum value".into(),
								},
								AttrsStruct {
										attr: "aria-valuemin".into(),
										value: "0".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-valuenow".into(),
										value: "<f32>".into(),
										description: "Current value of the progress".into(),
								},
								AttrsStruct {
										attr: "aria-valuetext".into(),
										value: "<f32>%".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-value".into(),
										value: "<f32>".into(),
										description: "Current value of the progress".into(),
								},
								AttrsStruct {
										attr: "data-max".into(),
										value: "<f32>".into(),
										description: "Maximum value".into(),
								},
						]),
					}
				}
				div {
					h4 { class: "font-medium text-lg text-orange-300", "ProgressIndicator" }
					p {
						"Shows a the progress visually. Must be included in "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"ProgressRoot"
						}
						" component."
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
										value: "init | loading | completed".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-value".into(),
										value: "<f32>".into(),
										description: "Current value of the progress".into(),
								},
								AttrsStruct {
										attr: "data-max".into(),
										value: "<f32>".into(),
										description: "Maximum value".into(),
								},
						]),
					}
				}
			}
		}
	}
}
