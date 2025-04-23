use {
	crate::components::description_section::DescriptionSection,
	dioxus::prelude::*,
	dioxus_free_icons::{
		Icon,
		icons::bs_icons::{BsBrush, BsCode, BsCollection, BsEyedropper},
	},
	maestro_ui::{
		button::Button,
		dialog::{Dialog, DialogBody, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger},
		shared::EVariant,
	},
};

#[component]
pub fn StyledHome() -> Element {
	rsx! {
		section { class: "w-full bg-neutral-900 text-orange-600 flex items-center justify-center",
			div { class: "container py-8 flex flex-col gap-5",
				h1 { class: "2xl:text-6xl md:text-5xl text-4xl font-semibold md:w-2/3",
					"Maestro UI Components Library"
				}
				p { class: "text-xl font-medium text-neutral-200",
					"Build beautiful interfaces, faster. Maestro UI provides a comprehensive set of pre-styled, accessible components with first-class support for customization. Whether you're launching a startup MVP or refining a polished design system, Maestro helps you move quickly—without sacrificing control."
				}
			}
		}
		DescriptionSection {
			title: "What is Maestro UI?",
			description: "Maestro UI is a component library designed to balance aesthetics and flexibility. Each component comes with a clean, modern default style, but underneath is a fully customizable structure that integrates seamlessly with Tailwind, tokens, or your own CSS system.",
		}
		section { class: "container flex flex-col gap-5 lg:py-8 py-6",
			h3 { class: "lg:text-xl text-lg font-medium", "Why Use Maestro UI?" }
			div { class: "grid 2xl:grid-cols-4 xl:grid-cols-2 grid-cols-1 gap-6",
				div { class: "flex flex-col gap-4 text-neutral-300 p-4 sm:p-6 rounded-b-sm bg-neutral-800/40",
					div { class: "flex flex-col gap-3 text-neutral-100",
						span { class: "w-10 h-10 rounded-full flex items-center justify-center bg-neutral-900 text-orange-600 border border-orange-600/50",
							Icon { icon: BsBrush }
						}
						h5 { class: "text-lg font-medium", "Beautiful & Flexible" }
					}
					p { "Beautiful by Default, Flexible by Design" }
				}
				div { class: "flex flex-col gap-4 text-neutral-300 p-4 sm:p-6 rounded-b-sm bg-neutral-800/40",
					div { class: "flex flex-col gap-3 text-neutral-100",
						span { class: "w-10 h-10 rounded-full flex items-center justify-center bg-neutral-900 text-orange-600 border border-orange-600/50",
							Icon { icon: BsCode }
						}
						h5 { class: "text-lg font-medium", "Rapid Prototyping" }
					}
					p { "Ship fast without compromising design quality. Focus on product, not pixels." }
				}

				div { class: "flex flex-col gap-4 text-neutral-300 p-4 sm:p-6 rounded-b-sm bg-neutral-800/40",
					div { class: "flex flex-col gap-3 text-neutral-100",
						span { class: "w-10 h-10 rounded-full flex items-center justify-center bg-neutral-900 text-orange-600 border border-orange-600/50",
							Icon { icon: BsCollection }
						}
						h5 { class: "text-lg font-medium", "Accessible & Ready" }
					}
					p {
						"Every component is built with accessibility in mind—keyboard navigation, ARIA attributes, and semantic structure baked in."
					}
				}
				div { class: "flex flex-col gap-4 text-neutral-300 p-4 sm:p-6 rounded-b-sm bg-neutral-800/40",
					div { class: "flex flex-col gap-3 text-neutral-100",
						span { class: "w-10 h-10 rounded-full flex items-center justify-center bg-neutral-900 text-orange-600 border border-orange-600/50",
							Icon { icon: BsEyedropper }
						}
						h5 { class: "text-lg font-medium", "Consistent & Maintainable" }
					}
					p {
						"Unify your product experience across features and pages with a reliable, scalable component system."
					}
				}
			}
		}
		section { class: "w-full bg-neutral-900 text-orange-600",
			div { class: "container flex flex-col gap-5 lg:py-8 py-6",
				h3 { class: "lg:text-3xl md:text-2xl text-xl font-medium", "Start Building" }
				div { class: "space-y-3",
					p { class: "text-neutral-200 lg:text-xl md:text-lg text-base",
						"You're free to build your UI exactly how you want, with complete control over the styling, layout, and user experience. All you need is to add a little css in your project."
					}
					div { class: "flex gap-2 items-center",
						Button {
							variant: EVariant::Link,
							class: "text-orange-500 hover:text-orange-600",
							Link { to: "#", tabindex: "-1", "input.css" }
						}
						Button {
							variant: EVariant::Link,
							class: "text-orange-500 hover:text-orange-600",
							Link { to: "#", tabindex: "-1", "animations.css" }
						}
					}
					p { class: "text-neutral-200 lg:text-xl md:text-lg text-base",
						"Dive into the documentation, explore our wide range of components, and see how easy it is to integrate headless components into your project."
					}
				}
			}
		}
	}
}
