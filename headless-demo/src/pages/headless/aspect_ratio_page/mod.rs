use {
	crate::components::{
		description_section::DescriptionSection,
		example_code::ExampleCodeCollapsible,
		features_list::Features,
		tables::{PropsStruct, PropsTable},
	},
	consts::EXAMPLE,
	dioxus::prelude::*,
	maestro_headless::{
		aspect_ratio::AspectRatioRoot,
		range::{Range, RangeRoot, RangeThumb, RangeTrack},
	},
};

mod consts;

#[component]
pub fn AspectRatioPage() -> Element {
	let features_list: Vec<&str> = Vec::from(["Accepts any custom aspect ratio"]);
	let mut value = use_signal::<Vec<f32>>(|| Vec::from([16.0, 9.0]));

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Aspect Ratio",
			description: "Displays content within a desired ratio.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "mb-4",
				RangeRoot {
					class: "w-52 flex items-center",
					min: 1.0,
					max: 20.0,
					value: value(),
					on_value_change: move |v| { value.set(v) },
					RangeTrack { class: "flex-1 bg-neutral-600 rounded-full h-1",
						Range { class: "flex-1 bg-orange-600 rounded-full h-1" }
					}
					RangeThumb { class: "w-6 h-6 rounded-full bg-orange-600 flex items-center justify-center text-neutral-300 text-xs cursor-pointer transition-colors hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
						"{value().get(0).unwrap_or(&1.0):.0}"
					}
					RangeThumb { class: "w-6 h-6 rounded-full bg-orange-600 flex items-center justify-center text-neutral-300 text-xs cursor-pointer transition-colors hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
						"{value().get(1).unwrap_or(&1.0):.0}"
					}
				}
			}
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start",
					div { class: "w-64 overflow-hidden rounded-md relative",
						div { class: "absolute p-2 rounded-sm bg-neutral-900/60 top-1 right-1 text-neutral-100 text-center z-10",
							"Ratio: {value().get(0).unwrap_or(&1.0):.0} / {value().get(1).unwrap_or(&1.0):.0}"
						}
						AspectRatioRoot { ratio: value().get(0).unwrap_or(&1.0_f32) / value().get(1).unwrap_or(&1.0_f32),
							img {
								class: "size-full object-cover",
								src: "https://ychef.files.bbci.co.uk/1280x720/p01x8qtv.jpg",
								alt: "The ocean",
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
			div { class: "grow flex flex-col rounded-md border border-neutral-800 bg-neutral-950 p-6",
				code { class: "font-mono whitespace-pre text-xs text-neutral-300",
					pre { "AspectRatioRoot {{ }}" }
				}
			}
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "AspectRatioRoot" }
					p { "Wraps the content to constrain to a given ratio." }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									PropsStruct {
											prop: "ratio".into(),
											prop_default: "1.0".into(),
											prop_type: "f32".into(),
											tooltip_text: None,
									},
									PropsStruct {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'li' attribules".into()),
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
}
