use {
	crate::{
		components::{
			description_section::DescriptionSection,
			example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
			tables::PropsTable,
		},
		router::Route,
	},
	consts::{EXAMPLE, EXAMPLE_ANATOMY},
	dioxus::prelude::*,
	maestro_headless::switch::{SwitchIndicator, SwitchRoot},
	maestro_ui::{
		accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionVariant},
		button::Button,
		shared::{EOrientation, EVariant},
	},
};

mod consts;

#[component]
pub fn AccordionStyledPage() -> Element {
	let mut variant = use_signal(|| AccordionVariant::Single);
	let mut orientation = use_signal(|| EOrientation::Vertical);
	let mut disabled = use_signal(|| false);
	let mut collapsible = use_signal(|| true);
	let mut value = use_signal(|| Vec::from(["1".into()]));

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Accordion",
			description: "UI component that allows to toggle the visibility of content within sections, one or multiple sections at a time.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4",
			div { class: "flex flex-wrap gap-3 items-center mb-4",
				div { class: "flex items-center justify-center gap-2 text-neutral-100",
					SwitchRoot {
						checked: disabled(),
						on_toggle_change: move |v| disabled.set(v),
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Disable"
				}
				div { class: "flex items-center justify-center gap-2 text-neutral-100",
					SwitchRoot {
						checked: variant() == AccordionVariant::Multiple,
						on_toggle_change: move |v| {
								if v {
										variant.set(AccordionVariant::Multiple);
								} else {
										variant.set(AccordionVariant::Single);
								}
								value.set(Vec::from(["1".to_string()]));
						},
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Multiple"
				}
				div { class: "flex items-center justify-center gap-2 text-neutral-100",
					SwitchRoot {
						checked: orientation() == EOrientation::Horizontal,
						on_toggle_change: move |v| {
								if v {
										orientation.set(EOrientation::Horizontal);
								} else {
										orientation.set(EOrientation::Vertical);
								}
						},
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Horizontal"
				}
				div { class: "flex items-center justify-center gap-2 text-neutral-100",
					SwitchRoot {
						checked: collapsible(),
						on_toggle_change: move |v| collapsible.set(v),
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Collapsible"
				}
			}
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-6 items-center justify-center w-full overflow-auto",
					Accordion {
						class: "data-[orientation=horizontal]:h-24 h-full data-[orientation=vertical]:max-w-56 px-3",
						value: value(),
						on_value_change: move |v| value.set(v),
						variant: variant(),
						disabled: disabled(),
						collapsible: collapsible(),
						orientation: orientation(),
						for (key , title , text) in [
								("1", "Lorem ipsum", "...dolor sit amet"),
								("2", "consectetur adipiscing elit", "...sed do eiusmod tempor"),
								("3", "incididunt ut labore", "...et dolore magna aliqua"),
						]
						{
							AccordionItem { key, value: key,
								AccordionTrigger { "{title}" }
								AccordionContent { class: "[&>div]:flex [&>div]:justify-center [&>div]:items-center",
									"{text}"
								}
							}
						}
					}
				}
				ExampleCodeCollapsible { code: EXAMPLE }
			}
		}

		DescriptionSection { title: "Usage and Anatomy",
			ExampleCodeAnatomy { code: EXAMPLE_ANATOMY }
		}
		DescriptionSection { title: "Notes",
			ul { class: "flex flex-col space-y-2 *:flex *:gap-2 *:items-baseline [&>li>span:first-child]:w-1.5 [&>li>span:first-child]:h-1.5 [&>li>span:first-child]:rounded-full [&>li>span:first-child]:bg-orange-600",
				li { class: "",
					span { class: "" }
					span {
						"Provided default styling. All need to do - add to "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"input.css"
						}
						" variables for colors and animations or adjust provided example"
						Button {
							variant: EVariant::Link,
							class: "text-orange-500 hover:text-orange-600",
							Link { to: Route::AccordionPage {}, tabindex: "-1", "headless accordion" }
						}
					}
				}
				li { class: "",
					span { class: "" }
					span {
						"Inherits all the props and behaviour of the"
						Button {
							variant: EVariant::Link,
							class: "text-orange-500 hover:text-orange-600",
							Link { to: Route::AccordionPage {}, tabindex: "-1", "headless accordion" }
						}
					}
				}
				li {
					span { class: "" }
					span {
						"Each component takes additional prop "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"class"
						}
						" props for custom styling. "
					}
				}
				li {
					span { class: "" }
					span {
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"AccordionTrigger"
						}
						" also takes "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"hide_icon"
						}
						" prop, which allows user to hide default open/close indicator and use the custom one."
					}
				}
			}
		}
	}
}
