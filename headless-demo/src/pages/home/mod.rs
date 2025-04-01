use {
	crate::components::description_section::DescriptionSection,
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
					"Maestro Headless Components Library"
				}
				p { class: "text-xl font-medium text-neutral-200",
					"Create custom user interfaces. Whether you're developing a complex web application or a simple website, headless components allow you to focus on your unique design while we handle the functional logic."
				}
			}
		}
		DescriptionSection {
			title: "What is a Headless Component?",
			description: "A headless component is a UI element that handles functionality and logic, but leaves the styling and structure completely up to you. This means you can create fully customizable, accessible, and reusable components with the freedom to integrate them seamlessly into any design system.",
		}
		section { class: "container flex flex-col gap-5 lg:py-8 py-6",
			h3 { class: "lg:text-xl text-lg font-medium", "Why Choose Headless Components?" }
			div { class: "grid 2xl:grid-cols-4 xl:grid-cols-2 grid-cols-1 gap-6",
				div { class: "flex flex-col gap-4 text-neutral-300 p-4 sm:p-6 rounded-b-sm bg-neutral-800/40",
					div { class: "flex flex-col gap-3 text-neutral-100",
						span { class: "w-10 h-10 rounded-full flex items-center justify-center bg-neutral-900 text-orange-600 border border-orange-600/50",
							Icon { icon: BsBrush }
						}
						h5 { class: "text-lg font-medium", "Customizable Design" }
					}
					p { "Tailor every aspect of your UI to match your vision" }
				}
				div { class: "flex flex-col gap-4 text-neutral-300 p-4 sm:p-6 rounded-b-sm bg-neutral-800/40",
					div { class: "flex flex-col gap-3 text-neutral-100",
						span { class: "w-10 h-10 rounded-full flex items-center justify-center bg-neutral-900 text-orange-600 border border-orange-600/50",
							Icon { icon: BsCode }
						}
						h5 { class: "text-lg font-medium", "Maintainable and Reusable" }
					}
					p { "Keep your code clean, modular, and easy to maintain with reusable components." }
				}

				div { class: "flex flex-col gap-4 text-neutral-300 p-4 sm:p-6 rounded-b-sm bg-neutral-800/40",
					div { class: "flex flex-col gap-3 text-neutral-100",
						span { class: "w-10 h-10 rounded-full flex items-center justify-center bg-neutral-900 text-orange-600 border border-orange-600/50",
							Icon { icon: BsCollection }
						}
						h5 { class: "text-lg font-medium", "Modular Design" }
					}
					p {
						"Pick and choose only the components you need to build a system that suits your needs."
					}
				}
				div { class: "flex flex-col gap-4 text-neutral-300 p-4 sm:p-6 rounded-b-sm bg-neutral-800/40",
					div { class: "flex flex-col gap-3 text-neutral-100",
						span { class: "w-10 h-10 rounded-full flex items-center justify-center bg-neutral-900 text-orange-600 border border-orange-600/50",
							Icon { icon: BsEyedropper }
						}
						h5 { class: "text-lg font-medium", "Unopinionated Structure" }
					}
					p { "No predefined styles or layouts—build your UI from the ground up." }
				}
			}
		}
		section { class: "w-full bg-neutral-900 text-orange-600",
			div { class: "container flex flex-col gap-5 lg:py-8 py-6",
				h3 { class: "lg:text-3xl md:text-2xl text-xl font-medium", "Start Building" }
				div { class: "space-y-3",
					p { class: "text-neutral-200 lg:text-xl md:text-lg text-base",
						"You're free to build your UI exactly how you want, with complete control over the styling, layout, and user experience."
					}
					p { class: "text-neutral-200 lg:text-xl md:text-lg text-base",
						"Dive into the documentation, explore our wide range of components, and see how easy it is to integrate headless components into your project."
					}
				}
			}
		}
	}
}
