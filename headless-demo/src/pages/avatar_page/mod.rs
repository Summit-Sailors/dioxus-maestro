use {
	crate::components::{
		description_section::DescriptionSection,
		props_table::{PropsTable, TableBody},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsCheckLg},
	maestro_headless::{
		avatar::{Avatar, AvatarFallback, AvatarImage},
		collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
	},
};

#[component]
pub fn AvatarPage() -> Element {
	rsx! {
		DescriptionSection { title: "Avatar", description: "An image element with a fallback." }
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start",
					Avatar { class: "rounded-full flex justify-center items-center w-10 h-10 border border-neutral-50 overflow-hidden",
						AvatarImage {
							class: "object-cover size-full",
							src: "https://mis.dp.ua/wp-content/uploads/2023/07/pqma3f-c17x11x50px50p-c17x11x50px50p-15f939eddf2b09f7e6c097aad232da37.jpg",
						}
						AvatarFallback { class: "text-sm font-medium text-neutral-50", "NA" }
					}
					Avatar { class: "rounded-full flex justify-center items-center w-10 h-10 border border-neutral-50 overflow-hidden",
						AvatarFallback {
							delay_ms: 300,
							class: "text-sm font-medium text-neutral-50",
							"NA"
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
use maestro_headless::avatar::{{Avatar, AvatarFallback, AvatarImage}};

rsx! {{
  Avatar {{
    class: 'rounded-full flex justify-center items-center w-10 h-10 border border-neutral-50 overflow-hidden',
    AvatarImage {{
      src: 'https://mis.dp.ua/wp-content/uploads/2023/07/pqma3f-c17x11x50px50p-c17x11x50px50p-15f939eddf2b09f7e6c097aad232da37.jpg',
      class: 'object-cover size-full',
    }}
    AvatarFallback {{
      class: 'text-sm font-medium text-neutral-50',
      'NA'
    }}
  }}
    
  Avatar {{
    class: 'rounded-full flex justify-center items-center w-10 h-10 border border-neutral-50 overflow-hidden',
    AvatarFallback {{
      class: 'text-sm font-medium text-neutral-50',
      delay_ms: 300,
      'NA'
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
					"Fallback if image is absent or error while loading"
				}
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Fallback may accept any children"
				}
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Optional delay fallback rendering"
				}
			}
		}
		DescriptionSection {
			title: "Usage and Anatomy",
			description: "Import all parts and piece them together. Each part may be styled separately, accept own properties and additional attributes",
			div { class: "grow flex flex-col rounded-md border border-neutral-800 bg-neutral-950 p-6",
				code { class: "font-mono whitespace-pre text-xs text-neutral-300",
					pre { "Avatar {{

  AvatarImage {{  }}
  AvatarFallback {{  }}
}}" }
				}
			}
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Root Component" }
					p { "Wrapps all parts af the avatar." }
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
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Image Component" }
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
									TableBody {
											prop: "src".into(),
											prop_default: "".into(),
											prop_type: "String".into(),
											tooltip_text: None,
									},
									TableBody {
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
					h4 { class: "font-medium text-lg text-orange-300", "Fallback Component" }
					p {
						"Will be rendered when the image hasn't loaded (if an image is loading or has error) or there is no image at all."
					}
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "delay_ms".into(),
											prop_default: "None".into(),
											prop_type: "Option<u32>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'span' attribules".into()),
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
