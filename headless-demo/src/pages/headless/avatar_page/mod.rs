use {
	crate::components::{
		description_section::DescriptionSection,
		example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
		features_list::Features,
		tables::{PropsStruct, PropsTable},
	},
	consts::{EXAMPLE, EXAMPLE_ANATOMY},
	dioxus::prelude::*,
	maestro_headless::avatar::{AvatarFallback, AvatarImage, AvatarRoot},
};

mod consts;

#[component]
pub fn AvatarPage() -> Element {
	let features_list: Vec<&str> =
		Vec::from(["Fallback if image is absent or error while loading", "Fallback may accept any children", "Optional delay fallback rendering"]);

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Avatar",
			description: "An image element with a fallback.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start",
					AvatarRoot { class: "rounded-full flex justify-center items-center w-10 h-10 border border-neutral-50 overflow-hidden",
						AvatarImage {
							class: "object-cover size-full",
							src: "https://mis.dp.ua/wp-content/uploads/2023/07/pqma3f-c17x11x50px50p-c17x11x50px50p-15f939eddf2b09f7e6c097aad232da37.jpg",
						}
						AvatarFallback { class: "text-sm font-medium text-neutral-50", "NA" }
					}
					AvatarRoot { class: "rounded-full flex justify-center items-center w-10 h-10 border border-neutral-50 overflow-hidden",
						AvatarFallback {
							delay_ms: 300,
							class: "text-sm font-medium text-neutral-50",
							"NA"
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
					h4 { class: "font-medium text-lg text-orange-300", "AvatarRoot" }
					p { "Wrapps all parts of the avatar." }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
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
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "AvatarImage" }
					p {
						"Represents avatar image. It will be rendered only if image has successfully loaded and has "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"src"
						}
						" prop."
					}
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									PropsStruct {
											prop: "src".into(),
											prop_default: "".into(),
											prop_type: "String".into(),
											tooltip_text: None,
									},
									PropsStruct {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'img' attribules".into()),
									},
							]),
						}
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "AvatarFallback" }
					p {
						"Will be rendered when the image hasn't loaded (if an image is loading or has error) or there is no image at all."
					}
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									PropsStruct {
											prop: "delay_ms".into(),
											prop_default: "None".into(),
											prop_type: "Option<u32>".into(),
											tooltip_text: None,
									},
									PropsStruct {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'span' attribules".into()),
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
}
