use {
	checkbox_content::CheckboxContent,
	checkbox_group_content::CheckboxGroupContent,
	dioxus::prelude::*,
	maestro_headless::{
		shared::EOrientation,
		tabs::{TabsContent, TabsList, TabsRoot, TabsTrigger},
	},
};

mod checkbox_content;
mod checkbox_group_content;
mod consts;

#[component]
pub fn CheckboxStyledPage() -> Element {
	rsx! {
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			TabsRoot {
				default_value: "Checkbox",
				orientation: EOrientation::Vertical,
				class: "flex flex-col gap-4",
				TabsList { class: "px-6 flex gap-3 border-b border-b-neutral-800 w-full",
					TabsTrigger {
						value: "Checkbox",
						class: "py-2 lg:text-xl text-lg font-medium text-neutral-100 border-b border-b-transparent hover:border-b-orange-600/50 data-[state=active]:border-b-orange-600 data-[state=active]:text-orange-600 transition-all ease-linear focus-visible:outline-none focus-visible:border-b-orange-600 focus-visible:text-orange-600",
						"Checkbox"
					}
					TabsTrigger {
						value: "CheckboxGroup",
						class: "py-2 lg:text-xl text-lg font-medium text-neutral-100 border-b border-b-transparent hover:border-b-orange-600/50 data-[state=active]:border-b-orange-600 data-[state=active]:text-orange-600 transition-all ease-linear focus-visible:outline-none focus-visible:border-b-orange-600 focus-visible:text-orange-600",
						"Checkbox Group"
					}
				}
				TabsContent { value: "CheckboxGroup", CheckboxGroupContent {} }
				TabsContent { value: "Checkbox", CheckboxContent {} }
			}
		}
	}
}
