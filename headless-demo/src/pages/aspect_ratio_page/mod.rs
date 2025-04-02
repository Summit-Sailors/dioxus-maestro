use {
	crate::components::{
		description_section::DescriptionSection,
		props_table::{PropsTable, TableBody},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{
		Icon,
		icons::bs_icons::{BsBrush, BsCheckLg, BsCode, BsCollection, BsEyedropper},
	},
	maestro_headless::{
		aspect_ratio::AspectRatio,
		collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
	},
};

#[component]
pub fn AspectRatioPage() -> Element {
	rsx! {
		DescriptionSection {
			title: "Aspect Ratio",
			description: "Displays content within a desired ratio.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start",
					div { class: "w-64 overflow-hidden rounded-md",
						AspectRatio { ratio: 16.0 / 9.0,
							img {
								class: "size-full object-cover",
								src: "https://ychef.files.bbci.co.uk/1280x720/p01x8qtv.jpg",
								alt: "The ocean",
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
use maestro_headless::aspect_ratio::AspectRatio;

rsx! {{
  div {{ 
    class: 'w-64 overflow-hidden rounded-md',
    AspectRatio {{
      ratio: 16.0 / 9.0,
      img {{
        class: 'size-full object-cover',
        src: 'https://ychef.files.bbci.co.uk/1280x720/p01x8qtv.jpg',
        alt: 'Ocean',
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
			ul { class: "fflex flex-col gap-2 *:flex *:items-center *:gap-2",
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Accepts any custom aspect ratio"
				}
			}
		}
		DescriptionSection {
			title: "Usage and Anatomy",
			description: "Import all parts and piece them together. Each part may be styled separately, accept own properties and additional attributes, e.g. \"data\" or \"aria\" (althought they are provided by default).",
			div { class: "grow flex flex-col rounded-md border border-neutral-800 bg-neutral-950 p-6",
				code { class: "font-mono whitespace-pre text-xs text-neutral-300",
					pre { "AspectRatio {{ }}" }
				}
			}
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Root Component" }
					p { "Wraps the content to constrain to a given ratio." }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "ratio".into(),
											prop_default: "1.0".into(),
											prop_type: "f32".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'li' attribules".into()),
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
