use {
	crate::{
		components::{
			description_section::DescriptionSection,
			example_code::ExampleCodeCollapsible,
			features_list::Features,
			tables::{PropsStruct, PropsTable},
		},
		router::Route,
	},
	async_std::task::sleep,
	conts::EXAMPLE,
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::ld_icons::LdLoader},
	maestro_headless::{
		select::{OptionSelectedIndicator, SelectDropdown, SelectIcon, SelectOption, SelectRoot, SelectTrigger, SelectValue},
		switch::{SwitchIndicator, SwitchRoot},
	},
	maestro_ui::{
		button::{Button, ButtonRound, ButtonSize, ButtonVariant},
		shared::ESide,
	},
	std::time::Duration,
	strum::IntoEnumIterator,
};
mod conts;

#[component]
pub fn ButtonStyledPage() -> Element {
	let mut is_pending = use_signal(|| false);
	let mut size = use_signal(|| Vec::from([ButtonSize::Md.to_string()]));
	let mut round = use_signal(|| Vec::from([ButtonRound::Md.to_string()]));
	let mut variant = use_signal(|| Vec::from([ButtonVariant::Primary.to_string()]));
	let mut disabled = use_signal(|| false);

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Button",
			description: "Button component with different states, such as pending and disabled.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "flex flex-wrap gap-5 items-center mb-4",
				div { class: "flex flex-col gap-2 text-neutral-300 font-medium",
					"Variant"
					SelectRoot {
						value: variant(),
						on_value_change: move |value: Vec<String>| { variant.set(value) },
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
							for item in ButtonVariant::iter() {
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
							for item in ButtonSize::iter() {
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
				div { class: "flex flex-col gap-2 text-neutral-300 font-medium",
					"Border radius"
					SelectRoot {
						value: round(),
						on_value_change: move |value: Vec<String>| { round.set(value) },
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
							for item in ButtonRound::iter() {
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
			div { class: "flex flex-wrap gap-5 items-center mb-4",
				div { class: "flex flex-wrap items-center gap-2 text-neutral-100 mb-4 mt-5",
					SwitchRoot {
						checked: disabled(),
						on_toggle_change: move |v| disabled.set(v),
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Disable"
				}
			}
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start",
					Button {
						pending: is_pending(),
						disabled: disabled(),
						size: ButtonSize::try_from(size().get(0).unwrap_or(&ButtonSize::default().to_string()))
								.ok()
								.unwrap_or(ButtonSize::default()),
						variant: ButtonVariant::try_from(
										variant().get(0).unwrap_or(&ButtonVariant::default().to_string()),
								)
								.ok()
								.unwrap_or(ButtonVariant::default()),
						round: ButtonRound::try_from(round().get(0).unwrap_or(&ButtonRound::default().to_string()))
								.ok()
								.unwrap_or(ButtonRound::default()),
						onclick: move |_| {
								is_pending.set(true);
								spawn(async move {
										sleep(Duration::from_millis(5000)).await;
										is_pending.set(false);
								});
						},
						if is_pending() {
							{
									rsx! {
										"Pending"
										Icon { icon: LdLoader, class: "animate-spin ease-linear" }
									}
							}
						} else {
							"Active"
						}
					}
				}
				ExampleCodeCollapsible { code: EXAMPLE }
			}
		}
		DescriptionSection { title: "Usage and Anatomy",
			div { class: "grow flex flex-col rounded-md border border-neutral-800 bg-neutral-950 p-6",
				code { class: "font-mono whitespace-pre text-xs text-neutral-300",
					pre { "Button {{ }}" }
				}
			}
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					p {
						"Inherits all the props and behaviour of the"
						Button {
							variant: ButtonVariant::Link,
							class: "text-orange-500 hover:text-orange-600",
							Link { to: Route::ButtonPage {}, tabindex: "-1", "headless button" }
						}
					}
					PropsTable {
						content: Vec::from([
								PropsStruct {
										prop: "class".into(),
										prop_default: "''".into(),
										prop_type: "String".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "variant".into(),
										prop_default: "ButtonVariant::Primary".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Determines the main look of the button".into()),
								},
								PropsStruct {
										prop: "size".into(),
										prop_default: "ButtonSize::Md".into(),
										prop_type: "ButtonSize".into(),
										tooltip_text: Some("Determines the height of the button".into()),
								},
								PropsStruct {
										prop: "round".into(),
										prop_default: "ButtonRound::Md".into(),
										prop_type: "ButtonRound".into(),
										tooltip_text: Some("Determines border radius of the button".into()),
								},
						]),
					}
				}
			}
		}
	}
}
