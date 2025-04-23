use {
	crate::components::{
		description_section::DescriptionSection,
		example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
		features_list::Features,
		tables::{PropsStruct, PropsTable},
	},
	consts::{EXAMPLE, EXAMPLE_ANATOMY},
	dioxus::prelude::*,
	maestro_headless::select::{OptionSelectedIndicator, SelectDropdown, SelectIcon, SelectOption, SelectRoot, SelectTrigger, SelectValue},
	maestro_ui::{
		avatar::{Avatar, AvatarFallback, AvatarImage, AvatarSize},
		button::Button,
		shared::{ESide, EVariant},
	},
	strum::IntoEnumIterator,
};
mod consts;
use crate::router::Route;

#[component]
pub fn AvatarStyledPage() -> Element {
	let mut size = use_signal(|| Vec::from([AvatarSize::Md.to_string()]));

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Avatar",
			description: "An image element with a fallback.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "flex flex-wrap gap-5 items-center mb-4",
				div { class: "flex flex-col gap-2 text-neutral-300 font-medium",
					"Size"
					SelectRoot {
						value: size(),
						on_value_change: move |value: Vec<String>| { size.set(value) },
						class: "relative w-fit",
						SelectTrigger { class: "rounded-sm border border-orange-400 bg-neutral-900 text-neutral-100 w-52 flex justify-between items-center gap-4 px-3 py-2 min-h-12 hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
							SelectValue {
								placeholder: "Chose something...",
								class: "data-[state=selected]:text-neutral-100 data-[state=placeholder]:text-neutral-500 overflow-ellipsis",
							}
							SelectIcon {}
						}
						SelectDropdown {
							side: ESide::Bottom,
							side_offset: 10.0,
							class: "rounded bg-neutral-900 text-neutral-200 border border-neutral-700 z-10 px-2 py-4 [&_*]:transition-all w-60 ",
							for item in AvatarSize::iter() {
								SelectOption {
									key: item.to_string(),
									value: item.to_string(),
									class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto",
									"{item.to_string()}"
									OptionSelectedIndicator { class: "w-4 h-4" }
								}
							}
						}
					}
				}
			}
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start",
					Avatar {
						size: AvatarSize::try_from(size().get(0).unwrap_or(&AvatarSize::default().to_string()))
								.ok()
								.unwrap_or(AvatarSize::default()),
						AvatarImage { src: "https://mis.dp.ua/wp-content/uploads/2023/07/pqma3f-c17x11x50px50p-c17x11x50px50p-15f939eddf2b09f7e6c097aad232da37.jpg" }
						AvatarFallback { "NA" }
					}
					Avatar {
						AvatarFallback { delay_ms: 300, "NA" }
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
						" variables for colors."
					}
				}
				li { class: "",
					span { class: "" }
					span {
						"Inherits all the props and behaviour of the"
						Button {
							variant: EVariant::Link,
							class: "text-orange-500 hover:text-orange-600",
							Link { to: Route::AvatarPage {}, tabindex: "-1", "headless avatar" }
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
							"Avatar"
						}
						" also takes "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"size"
						}
						" prop, which allows user set width and height of the avatar."
					}
				}
			}
		}
	}
}
