use {
	dioxus::prelude::*,
	maestro_headless::{
		shared::EOrientation,
		tabs::{TabsContent, TabsList, TabsRoot, TabsTrigger},
	},
	toggle_content::ToggleContent,
	toggle_group_content::ToggleGroupContent,
};

mod consts;
mod toggle_content;
mod toggle_group_content;

#[component]
pub fn TogglePage() -> Element {
	rsx! {
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			TabsRoot {
				default_value: "Toggle",
				orientation: EOrientation::Vertical,
				class: "flex flex-col gap-4",
				TabsList { class: "px-6 flex gap-3 border-b border-b-neutral-800 w-full",
					TabsTrigger {
						value: "Toggle",
						class: "py-2 lg:text-xl text-lg font-medium text-neutral-100 border-b border-b-transparent hover:border-b-orange-600/50 data-[state=active]:border-b-orange-600 data-[state=active]:text-orange-600 transition-all ease-linear focus-visible:outline-none focus-visible:border-b-orange-600 focus-visible:text-orange-600",
						"Toggle"
					}
					TabsTrigger {
						value: "ToggleGroupRoot",
						class: "py-2 lg:text-xl text-lg font-medium text-neutral-100 border-b border-b-transparent hover:border-b-orange-600/50 data-[state=active]:border-b-orange-600 data-[state=active]:text-orange-600 transition-all ease-linear focus-visible:outline-none focus-visible:border-b-orange-600 focus-visible:text-orange-600",
						"Toggle Group"
					}
				}
				TabsContent { value: "ToggleGroupRoot", ToggleGroupContent {} }
				TabsContent { value: "Toggle", ToggleContent {} }
			}
		}
	}
}
