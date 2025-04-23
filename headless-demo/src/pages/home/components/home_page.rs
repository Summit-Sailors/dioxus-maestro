use {
	crate::{components::description_section::DescriptionSection, router::Route},
	dioxus::prelude::*,
	dioxus_free_icons::{
		Icon,
		icons::bs_icons::{BsBrush, BsCode, BsCollection, BsEyedropper},
	},
};

#[component]
pub fn Home() -> Element {
	rsx! {
		section { class: "w-full bg-neutral-900 text-orange-600 flex items-center justify-center",
			div { class: "container py-8 flex flex-col gap-5",
				h1 { class: "2xl:text-6xl md:text-5xl text-4xl font-semibold md:w-2/3",
					"Maestro Components Library"
				}
				p { class: "text-xl font-medium text-neutral-200", "Chose what you need!" }
				div { class: "flex md:flex-row flex-col gap-6",
					Link {
						class: "w-full gap-3 p-6 flex flex-col rounded-md bg-neutral-800  shadow-neutral-600/20 border border-neutral-700 hover:shadow-md transition-all hover:bg-neutral-700/20 focus-visible:outline-none",
						to: Route::HeadlessHome {},
						h2 { class: "2xl:text-2xl md:text-xl text-lg font-medium text-orange-600",
							"Headless Lib"
						}
						p { class: "text-neutral-200", "Absolute Freedom of the UI developement" }
					}
					Link {
						class: "w-full gap-3 p-6 flex flex-col rounded-md bg-neutral-800  shadow-neutral-600/20 border border-neutral-700 hover:shadow-md transition-all hover:bg-neutral-700/20 focus-visible:outline-none",
						to: Route::StyledHome {},
						h2 { class: "2xl:text-2xl md:text-xl text-lg font-medium text-orange-600",
							"UI Lib"
						}
						p { class: "text-neutral-200",
							"Almost no headache about styling, just small tweaks"
						}
					}
				}
			}
		}
	}
}
