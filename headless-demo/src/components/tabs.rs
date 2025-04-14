use {
	crate::components::tables::{AttrsStruct, AttrsTable, PropsStruct, PropsTable},
	dioxus::prelude::*,
	maestro_headless::{
		shared::EOrientation,
		tabs::{TabsRoot, TabsContent, TabsList, TabsTrigger},
	},
};

#[component]
pub fn PageTabs(props_list: Vec<PropsStruct>, attrs_list: Vec<AttrsStruct>) -> Element {
	rsx! {
		TabsRoot {
			default_value: "Props",
			orientation: EOrientation::Horizontal,
			class: "flex flex-col gap-4",
			TabsList { class: "flex gap-2 border-b border-b-neutral-800 w-full",
				TabsTrigger {
					value: "Props",
					class: "py-2 font-medium text-neutral-100 border-b border-b-transparent hover:border-b-neutral-500 data-[state=active]:border-b-neutral-100 data-[state=active]:neutral-100 transition-all ease-linear focus-visible:outline-none focus-visible:border-b-neutral-100 focus-visible:text-neutral-100",
					"Props"
				}
				TabsTrigger {
					value: "Attrs",
					class: "py-2 font-medium text-neutral-100 border-b border-b-transparent hover:border-b-neutral-500 data-[state=active]:border-b-neutral-100 data-[state=active]:neutral-100 transition-all ease-linear focus-visible:outline-none focus-visible:border-b-neutral-100 focus-visible:text-neutral-100",
					"Data/aria attrs"
				}
			}
			TabsContent { value: "Props",
				div { class: "overflow-hidden rounded-sm border border-neutral-700",
					PropsTable { content: props_list.clone() }
				}
			}
			TabsContent { value: "Attrs",
				div { class: "overflow-hidden rounded-sm border border-neutral-700",
					AttrsTable { content: attrs_list.clone() }
				}
			}
		}
	}
}
