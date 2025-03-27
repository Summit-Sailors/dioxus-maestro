use {
	crate::components::ui::{component_section::ComponentSection, features::Features},
	async_std::task::sleep,
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::{
			fa_solid_icons::{FaCopy, FaFaceSmile},
			ld_icons::{LdAlignCenter, LdAlignLeft, LdAlignRight, LdSmile, LdX},
		},
		Icon,
	},
	dioxus_logger::tracing::info,
	maestro_headless::{
		accordion::{Accordion, AccordionContent, AccordionHeader, AccordionItem, AccordionTrigger, AccordionVariant},
		button::Button,
		checkbox::{Checkbox, CheckboxIndicator},
		collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
		dialog::{Dialog, DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogTitle, DialogTrigger},
		hover_card::{HoverCard, HoverCardArrow, HoverCardContent, HoverCardTrigger},
		popover::{Popover, PopoverArrow, PopoverContent, PopoverTrigger},
		range::{Range, RangeThumb, RangeTrack, RangeTrackWrapper},
		select::{OptionSelectedIndicator, Select, SelectDropdown, SelectIcon, SelectOption, SelectTrigger, SelectValue},
		switch::{Switch, SwitchIndicator},
		tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
		toggle::Toggle,
		toggle_group::{ToggleGroup, ToggleGroupItem},
		tooltip::{Tooltip, TooltipArrow, TooltipContent, TooltipProvider, TooltipTrigger},
		utils::{EAlign, EOrientation, ESide},
	},
	maestro_toast::{ctx::use_toast, toast_info::ToastInfo, toast_position::EToastPosition},
	std::time::Duration,
	tailwind_fuse::tw_merge,
};

#[component]
pub fn HeadlessDemo() -> Element {
	let mut toast = use_toast();
	let mut disabled = use_signal(|| false);
	let mut pending = use_signal(|| false);
	let mut popup_open = use_signal(|| false);
	let mut toggle = use_signal(|| false);
	let mut group_toggle_value = use_signal(|| String::from("1"));
	let mut checked = use_signal(|| true);
	let mut is_open = use_signal(|| false);
	let mut is_open_2 = use_signal(|| false);
	let mut selected = use_signal::<Vec<String>>(|| Vec::new());
	let mut multi_selected = use_signal::<Vec<String>>(|| Vec::new());
	let mut range_1: Signal<Vec<f32>> = use_signal(|| Vec::from([0.0]));
	let mut range_2: Signal<Vec<f32>> = use_signal(|| Vec::from([5.0]));
	let mut range_3: Signal<Vec<f32>> = use_signal(|| Vec::from([0.0, 15.0]));

	let mut handle_button_click = move |button_message: String| {
		let info = ToastInfo {
			heading: Some("Button Click Handler".to_string()),
			context: button_message,
			icon: None,
			position: EToastPosition::TopRight,
			allow_toast_close: true,
			hide_after: 5,
		};
		toast.write().popup(info);
	};

	let mut handle_pending_click = move |_| {
		spawn(async move {
			pending.set(true);
			let info = ToastInfo {
				heading: Some("Button Click Handler".to_string()),
				context: "Simulates async operation. In 5 second will be active".into(),
				icon: None,
				position: EToastPosition::TopRight,
				allow_toast_close: true,
				hide_after: 5,
			};
			toast.write().popup(info);
			sleep(Duration::from_secs(5)).await;
			pending.set(false);
		});
	};

	let mut dialog_close = move |_| {
		spawn(async move {
			pending.set(true);
			sleep(Duration::from_secs(5)).await;
			pending.set(false);
			popup_open.set(false);
		});
	};

	rsx! {
		div {
			id: "maestro-ui",
			class: "mx-auto p-4 bg-slate-900 rounded-lg shadow-lg",
			div { class: "mb-8",
				h1 { class: "text-slate-100 text-center text-3xl font-bold mb-2", "Maestro UI" }
				p { class: "text-slate-300 text-center",
					"Maestro UI is a comprehensive, type-safe, and highly customizable UI component library for Dioxus, designed to provide developers with powerful, flexible, and elegant UI building blocks."
				}
			}

			div { id: "maestro-ui-features", class: "flex space-x-2",
				Features {
					title: "Features".to_string(),
					features: vec![
							"Type Safety: Leverages Rust's type system".to_string(),
							"Reactive Design: Built for Dioxus's reactive paradigm".to_string(),
							"Flexible Styling: Tailwind CSS integration".to_string(),
							"Comprehensive Components: Wide range of UI elements".to_string(),
							"Performance: Efficient and lightweight".to_string(),
					],
				}
			}

			// buttons section
			ComponentSection {
				title: "Buttons",
				description: "Various button styles, sizes, and types with different variants",
				div {
					id: "maestro-ui-buttons",
					class: "flex flex-wrap gap-6 md:w-4/5 mx-auto w-full",
					Button {
						r#type: "button",
						onclick: move |_| handle_button_click("Default Button clicked!".to_string()),
						"Default: no classnames"
					}
					Button {
						disabled: disabled(),
						class: "rounded-lg text-slate-200 bg-indigo-600 w-fit px-3 py-2 h-12 focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors hover:bg-indigo-800 focus-visible:ring-indigo-800 focus-visible:ring-offset-black focus-visible:bg-indigo-800 aria-[disabled=true]:opacity-50 aria-[disabled=true]:pointer-events-none data-[pending=true]:pointer-events-none data-[pending=true]:bg-indigo-400",
						r#type: "button",
						id: "IndigoButton",
						"With classNames"
					}
					Button {
						class: "rounded-full text-slate-200 w-fit px-3 py-2 h-12 focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors hover:bg-slate-700 focus-visible:ring-slate-700 focus-visible:ring-offset-black focus-visible:bg-slate-700 aria-[disabled=true]:opacity-50 aria-[disabled=true]:pointer-events-none data-[pending=true]:pointer-events-none",
						r#type: "reset",
						aria_controls: "IndigoButton",
						onclick: move |_| disabled.toggle(),
						if disabled() {
							"Enable Prev"
						} else {
							"Disable Prev"
						}
					}
					Button {
						pending: pending(),
						class: "rounded-full w-fit px-3 py-2 h-12 focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors bg-slate-200 border border-slate-200 text-slate-900 hover:bg-slate-900 hover:text-slate-200 focus-visible:ring-slate-200 focus-visible:ring-offset-black focus-visible:bg-slate-200 focus-visible:text-slate-900 aria-[disabled=true]:opacity-50 aria-[disabled=true]:pointer-events-none data-[pending=true]:bg-slate-500",
						r#type: "reset",
						onclick: handle_pending_click,
						"Simulates Pending"
					}
					Button {
						class: "bg-slate-300 hover:bg-slate-600 text-slate-900 w-12 h-12 border-slate-100 border rounded-full flex items-center justify-center",
						r#type: "button",
						onclick: move |_| handle_button_click("Icon Button clicked!".to_string()),
						Icon {
							title: "Icon Button",
							icon: FaCopy,
							width: 24,
							height: 24,
						}
					}
				}
			}
		}

		// buttons section
		ComponentSection { title: "Dialog", description: "Dialog states",
			div {
				id: "maestro-ui-buttons",
				class: "flex flex-wrap gap-6 md:w-4/5 mx-auto w-full",

				Dialog {
					DialogTrigger { class: "rounded-full w-fit px-3 py-2 h-12 focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors bg-slate-200 border border-slate-200 text-slate-900 hover:bg-slate-900 hover:text-slate-200 focus-visible:ring-slate-200 focus-visible:ring-offset-black focus-visible:bg-slate-200 focus-visible:text-slate-900 aria-[disabled=true]:opacity-50 aria-[disabled=true]:pointer-events-none data-[pending=true]:bg-slate-500",
						"Open Dialog"
					}
					DialogOverlay { class: "w-full h-full fixed top-0 left-0 bottom-0 right-0 bg-slate-900/20 inset-0 backdrop-blur-sm z-[100]" }
					DialogContent { class: "w-full h-96 max-w-lg max-h-[95vh] fixed z-[110] left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rounded bg-slate-100 shadow border border-slate-600 flex flex-col gap-6 px-6 py-8",
						div { class: "flex justify-between gap-4",
							DialogTitle { class: "font-medium text-2xl text-slate-900", "Uncontrolled dialog" }
							DialogClose {
								title: "Close my popup",
								class: "text-slate-500 hover:text-slate-900 transition-colors w-fit h-fit",
								Icon { width: 16, height: 16, icon: LdX }
							}
						}
						DialogDescription { class: "text-slate-600",
							"This dialog is controlled by dialog component itself"
						}
					}
				}

				Dialog {
					open: popup_open(),
					on_open_change: move |value: Option<bool>| popup_open.set(value.unwrap_or_default()),
					DialogTrigger { class: "rounded-full w-fit px-3 py-2 h-12 focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors bg-orange-600 border border-slate-200 text-slate-100 hover:bg-orange-800  focus-visible:ring-orange-200 focus-visible:ring-offset-black focus-visible:bg-orange-800 aria-[disabled=true]:opacity-50 aria-[disabled=true]:pointer-events-none data-[pending=true]:bg-slate-500",
						"Open Dialog"
					}
					DialogOverlay { class: "w-full h-full fixed top-0 left-0 bottom-0 right-0 z-[100] bg-slate-900/20 inset-0 backdrop-blur-sm data-[state=closed]:animate-fade-out data-[state=closed]:duration-300 data-[state=open]:animate-fade-in data-[state=open]:duration-100" }
					DialogContent { class: "w-full h-96  max-w-lg fixed z-[110] max-h-[95vh] left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rounded bg-slate-100 shadow border border-slate-600 flex flex-col gap-6 px-6 py-8 data-[state=closed]:animate-fade-out data-[state=closed]:duration-300 data-[state=open]:animate-fade-in data-[state=open]:duration-100",
						div { class: "flex justify-between gap-4",
							DialogTitle { class: "font-medium text-2xl text-slate-900", "Controlled dialog" }
							DialogClose {
								title: "Close my popup",
								class: "text-slate-500 hover:text-slate-900 transition-colors",
								Icon { icon: FaFaceSmile }
							}
						}
						DialogDescription { class: "text-slate-600",
							"This dialog is controlled by user. Props 'open' and 'on_open_change' passed. Also used custom close Icon. The button below has onclick handler and closes dialog in 5 seconds"
						}
						Button {
							pending,
							class: "rounded-full w-fit px-3 py-2 h-12 focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors bg-slate-200 border border-slate-200 text-slate-900 hover:bg-slate-900 hover:text-slate-200 focus-visible:ring-slate-200 focus-visible:ring-offset-black focus-visible:bg-slate-200 focus-visible:text-slate-900 aria-[disabled=true]:opacity-50 aria-[disabled=true]:pointer-events-none data-[pending=true]:bg-slate-500",
							onclick: dialog_close,
							"Close"
						}
					}
				}
			}
		}

		div { class: "flex gap-6",
			Select {
				value: selected(),
				on_value_change: move |value: Option<Vec<String>>| { selected.set(value.unwrap_or_default()) },
				class: "relative w-fit",
				SelectTrigger { class: "rounded border border-slate-300 bg-slate-900 text-slate-100 w-36 flex justify-between items-center gap-4 px-3 py-2 min-h-12 ",
					SelectValue {
						placeholder: "Chose something...",
						class: "data-[state=selected]:text-slate-100 data-[state=placeholder]:text-slate-500 overflow-ellipsis",
					}
					SelectIcon {}
				}
				SelectDropdown {
					side: ESide::Bottom,
					side_offset: 10.0,
					class: "rounded bg-slate-900 text-slate-200 border border-slate-700 z-10 px-2 py-4 [&_*]:transition-all  w-60",
					SelectOption {
						key: 1,
						value: "apple",
						selected: selected().contains(&"apple".to_string()),
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Apple"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
					SelectOption {
						key: 2,
						value: "banana",
						selected: selected().contains(&"banana".to_string()),
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Banana"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
					SelectOption {
						key: 3,
						value: "ice-cream",
						selected: selected().contains(&"ice-cream".to_string()),
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Ice-Cream"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
					SelectOption {
						key: 4,
						selected: selected().contains(&"coffee".to_string()),
						value: "coffee",
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Coffee"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
					SelectOption {
						key: 5,
						selected: selected().contains(&"salt".to_string()),
						value: "salt",
						disabled: true,
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Salt"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
					SelectOption {
						key: 6,
						selected: selected().contains(&"chocolatte".to_string()),
						value: "chocolatte",
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Chocolatte"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
				}
			}
			Select {
				multi: true,
				value: multi_selected(),
				on_value_change: move |value: Option<Vec<String>>| { multi_selected.set(value.unwrap_or_default()) },
				class: "relative w-fit",
				SelectTrigger { class: "rounded border border-slate-300 bg-slate-900 text-slate-100 w-60 flex justify-between items-center gap-4 px-3 py-2 min-h-12 ",
					SelectValue {
						placeholder: "Chose something...",
						class: "data-[state=selected]:text-slate-100 data-[state=placeholder]:text-slate-500 overflow-ellipsis line-clamp-1",
					}
					SelectIcon {}
				}
				SelectDropdown {
					side: ESide::Bottom,
					side_offset: 10.0,
					class: "rounded bg-slate-900 text-slate-200 border border-slate-700 z-10 px-2 py-4 [&_*]:transition-all w-60",
					SelectOption {
						value: "apple",
						selected: multi_selected().contains(&"apple".to_string()),
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Apple"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
					SelectOption {
						key: 1,
						value: "banana",
						selected: multi_selected().contains(&"banana".to_string()),
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Banana"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
					SelectOption {
						key: 2,
						value: "ice-cream",
						selected: multi_selected().contains(&"ice-cream".to_string()),
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Ice-Cream"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
					SelectOption {
						key: 3,
						selected: multi_selected().contains(&"coffee".to_string()),
						value: "coffee",
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Coffee"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
					SelectOption {
						key: 4,
						selected: multi_selected().contains(&"salt".to_string()),
						value: "salt",
						disabled: true,
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Salt"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
					SelectOption {
						key: 5,
						selected: multi_selected().contains(&"chocolatte".to_string()),
						value: "chocolatte",
						class: "flex items-center justify-between gap-4 px-2 py-3 hover:bg-slate-700 focus-visible::bg-slate-700 ",
						"Chocolatte"
						OptionSelectedIndicator { class: "w-4 h-4" }
					}
				}
			}
		}
		div { class: "flex gap-6",
			Accordion {
				collapsible: false,
				default_value: Vec::from(["1".into()]),
				class: "relative w-48 flex flex-col px-4 py-2",
				variant: AccordionVariant::Single,
				AccordionItem { value: "1", class: "p-2 flex flex-col gap-3",
					AccordionTrigger { class: "rounded hover:bg-slate-200 focus-visible:bg-slate-200",
						AccordionHeader { "Value1" }
					}
					AccordionContent { class: "data-[state=open]:flex hidden",
						"Yes. It adheres to the WAI-ARIA design pattern."
					}
				}
				AccordionItem { value: "2", class: "p-2 flex flex-col gap-3",
					AccordionTrigger { class: "rounded hover:bg-slate-200 focus-visible:bg-slate-200",
						AccordionHeader { "Value2" }
					}
					AccordionContent { class: "data-[state=open]:flex hidden",
						"Yes. It adheres to the WAI-ARIA design pattern."
					}
				}
				AccordionItem {
					value: "3",
					disabled: true,
					class: "p-2 flex flex-col gap-3 data-[disabled=true]:opacity-50",
					AccordionTrigger { class: "rounded hover:bg-slate-200 focus-visible:bg-slate-200",
						AccordionHeader { "Value3 disabled" }
					}
					AccordionContent { class: "data-[state=open]:flex hidden",
						"Yes. It adheres to the WAI-ARIA design pattern."
					}
				}
				AccordionItem {
					value: "4",
					class: "p-2 flex flex-col gap-3 data-[disabled=true]:opacity-50",
					AccordionTrigger { class: "rounded hover:bg-slate-200 focus-visible:bg-slate-200",
						AccordionHeader { "Value4" }
					}
					AccordionContent { class: "data-[state=open]:flex hidden",
						"Yes. It adheres to the WAI-ARIA design pattern."
					}
				}
			}

			Accordion {
				default_value: Vec::from(["1".into()]),
				variant: AccordionVariant::Multiple,
				class: "relative w-48 flex flex-col",
				AccordionItem { value: "1", class: "p-2 flex flex-col gap-3",
					AccordionTrigger { class: "rounded hover:bg-slate-200 focus-visible:bg-slate-200",
						AccordionHeader { "Value1" }
					}
					AccordionContent { class: "data-[state=open]:flex hidden",
						"Yes. It adheres to the WAI-ARIA design pattern."
					}
				}
				AccordionItem { value: "2", class: "p-2 flex flex-col gap-3",
					AccordionTrigger { class: "rounded hover:bg-slate-200 focus-visible:bg-slate-200",
						AccordionHeader { "Value2" }
					}
					AccordionContent { class: "data-[state=open]:flex hidden",
						"Yes. It adheres to the WAI-ARIA design pattern."
					}
				}
				AccordionItem {
					value: "3",
					disabled: true,
					class: "p-2 flex flex-col gap-3 disabled:opacity-50",
					AccordionTrigger { class: "rounded hover:bg-slate-200 focus-visible:bg-slate-200",
						AccordionHeader { "Value3 disabled" }
					}
					AccordionContent { class: "data-[state=open]:flex hidden",
						"Yes. It adheres to the WAI-ARIA design pattern."
					}
				}
				AccordionItem { value: "4", class: "p-2 flex flex-col gap-3",
					AccordionTrigger { class: "rounded hover:bg-slate-200 focus-visible:bg-slate-200",
						AccordionHeader { "Value4" }
					}
					AccordionContent { class: "data-[state=open]:flex hidden",
						"Yes. It adheres to the WAI-ARIA design pattern."
					}
				}
			}
		}
		div { class: "flex gap-6",
			Toggle {
				class: "aria-[pressed=true]:bg-orange-700 bg-orange-500 text-slate-50 flex justify-center items-center p-3 w-12 h-12 rounded",
				pressed: toggle(),
				value: "on",
				on_toggle_change: move |value: Option<bool>| toggle.set(value.unwrap_or_default()),
			}
			Toggle {
				value: "on",
				class: "aria-[pressed=true]:bg-orange-700 text-slate-50  bg-orange-500 flex justify-center items-center p-3 w-12 h-12 rounded",
				default_pressed: toggle(),
				Icon { icon: LdSmile }
			}
		}
		div { class: "flex gap-6",
			ToggleGroup {
				class: "flex justify-center items-center rounded overflow-hidden border border-slate-700",
				value: group_toggle_value(),
				on_value_chenge: move |value: Option<String>| group_toggle_value.set(value.unwrap_or_default()),
				ToggleGroupItem {
					class: "data-[state=on]:bg-slate-200  data-[state=on]:text-slate-900 border-r border-r-slate-700 bg-slate-600 text-slate-50 flex justify-center items-center p-3 w-12 h-12",
					value: "1",
					Icon { icon: LdAlignRight }
				}
				ToggleGroupItem {
					class: "data-[state=on]:bg-slate-200 data-[state=on]:text-slate-900 bg-slate-600 text-slate-50 flex justify-center items-center p-3 w-12 h-12",
					value: "2",
					Icon { icon: LdAlignCenter }
				}
				ToggleGroupItem {
					class: "data-[state=on]:bg-slate-200  data-[state=on]:text-slate-900 border-l border-l-slate-700 bg-slate-600 text-slate-50 flex justify-center items-center p-3 w-12 h-12",
					value: "3",
					Icon { icon: LdAlignLeft }
				}
			}
		}
		div { class: "flex gap-6",
			div { class: "flex justify-center items-center gap-3",
				Checkbox {
					class: "w-10 h-10 rounded flex items-center justify-center border border-slate-100",
					value: group_toggle_value(),
					name: "box",
					CheckboxIndicator { class: "text-slate-100 " }
				}
				span { class: "text-slate-100", "Check" }
			}
			div { class: "flex justify-center items-center gap-3",
				Checkbox {
					class: "w-10 h-10 rounded flex items-center justify-center border border-slate-100",
					value: 1_i32.to_string(),
					name: "box",
					checked: checked(),
					on_change: move |v: Option<bool>| {
							checked.set(v.unwrap_or_default());
					},
					CheckboxIndicator { class: "text-slate-100 " }
				}
				span { class: "text-slate-100", "Check" }
			}
		}
		div { class: "flex gap-6",
			Collapsible { class: "flex flex-col data-[state=open]:gap-4 gap-0",
				div { class: "flex justify-between items-center gap-3 px-6 py-3",
					span { class: "text-slate-100", "Collapsible" }
					CollapsibleTrigger { class: "w-10 h-10 rounded-full flex items-center justify-center border border-slate-300 text-slate-300",
						Icon { icon: LdSmile }
					}
				}
				CollapsibleContent { class: "overflow-hidden transition-all ease-linear data-[state=closed]:h-0 data-[state=]:h-fit",
					span { class: "text-slate-100", "Content of collapsible" }
				}
			}
		}
		div { class: "flex gap-6",
			Switch { class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-teal-200 data-[state=checked]:bg-teal-400 border border-teal-600",
				SwitchIndicator { class: "relative translate-x-0.5 data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-teal-600" }
			}
		}
		div { class: "pb-44 flex gap-6",
			Popover {
				class: "mx-auto w-64",
				open: is_open(),
				on_open_change: move |value: Option<bool>| {
						is_open.set(value.unwrap_or_default());
				},
				// is_arrow_hidden: true,
				PopoverTrigger {
					class: "w-full",
					style: "background: #007bff; color: white; padding: 8px 16px; border-radius: 4px; cursor: pointer;",
					"Click to toggle popper"
				}
				PopoverContent {
					side: ESide::Top,
					side_offset: 8.0,
					align: EAlign::Center,
					class: "content bg-white text-slate-900 rounded-sm w-56 p-4 data-[state=open]:animate-fade-in data-[state=closed]:duration-500 data-[state=closed]:animate-fade-out",

					"This is popper content"

					PopoverArrow { width: 16.0, height: 8.0, style: "color: white;" }
				}
			}
		}
		div { class: "py-6",
			Tabs { default_value: "1", class: "flex flex-col gap-4",

				TabsList { class: "w-full flex items-center gap-6",
					TabsTrigger {
						value: "1",
						class: "text-orange-500 data-[state=active]:text-orange-700 data-[state=active]:underline",
						"One"
					}
					TabsTrigger {
						value: "2",
						class: "text-orange-500 data-[state=active]:text-orange-700 data-[state=active]:underline",
						"Two"
					}
					TabsTrigger {
						value: "3",
						class: "text-orange-500 data-[state=active]:text-orange-700 data-[state=active]:underline disabled:opacity-50",
						disabled: true,
						"Three"
					}
					TabsTrigger {
						value: "4",
						class: "text-orange-500 data-[state=active]:text-orange-700 data-[state=active]:underline disabled:opacity-50",
						"Four"
					}
				}
				TabsContent { value: "1", "Content-1" }
				TabsContent { value: "2", "Content-2" }
				TabsContent { value: "3", "Content-3" }
				TabsContent { value: "4", "Content-4" }
			}
		}

		div { class: "pb-20",
			TooltipProvider { class: "w-fit mx-auto",
				Tooltip { class: "w-fit group",
					TooltipTrigger { class: "mx-auto w-12 h-12 bg-slate-200 text-slate-800 rounded-full",
						"+"
					}
					TooltipContent {
						side: ESide::Top,
						side_offset: 8.0,
						align: EAlign::Center,
						class: "group-data-[state=open]:opacity-100 group-data-[state=closed]:opacity-0 bg-white text-slate-900 rounded-sm w-56 p-4  transition-opacity ease-linear",

						"This is popper content"

						TooltipArrow {
							width: 16.0,
							height: 8.0,
							style: "color: white;",
						}
					}
				}
			}
		}

		div { class: "",
			HoverCard { class: "w-fit",
				HoverCardTrigger { class: "mx-auto w-12 h-12 bg-slate-200 text-slate-800 rounded-full",
					"*"
				}
				HoverCardContent {
					side: ESide::Bottom,
					side_offset: 8.0,
					align: EAlign::Center,
					class: "content bg-white text-slate-900 rounded-sm w-56 p-4",

					"This is popper content"

					HoverCardArrow { width: 16.0, height: 8.0, style: "color: white;" }
				}
			}
		}

		div { class: "py-10 flex flex-col gap-8",
			Range {
				class: "w-52 flex items-center",
				value: range_1(),
				on_value_change: move |v| {
						if let Some(v) = v {
								range_1.set(v)
						}
				},
				RangeTrackWrapper { class: "flex-1 bg-slate-600 rounded-full h-1",
					RangeTrack { class: "flex-1 bg-slate-300 rounded-full h-1" }
					RangeThumb { class: "w-5 h-5 rounded-full bg-slate-100" }
				}
			}
			Range {
				class: "w-1 flex items-center",
				orientation: EOrientation::Vertical,
				max: 200.0,
				step: 10.0,
				default_value: Vec::from([40.0]),
				RangeTrackWrapper { class: "flex-1 bg-slate-600 rounded-full h-52",
					RangeTrack { class: "flex-1 bg-slate-300 rounded-full" }
					RangeThumb { class: "w-5 h-5 rounded-full bg-slate-100" }
				}
			}

			Range {
				class: "w-52 flex items-center",
				value: range_3(),
				on_value_change: move |v| {
						if let Some(v) = v {
								range_3.set(v)
						}
				},
				min_steps_between_thumbs: 10.0,
				RangeTrackWrapper { class: "flex-1 bg-slate-600 rounded-full h-[3px]",
					RangeTrack { class: "flex-1 bg-slate-300 rounded-full h-1" }
					RangeThumb { class: "w-3 h-3 rounded-full bg-slate-100" }
					RangeThumb { class: "w-3 h-3 rounded-full bg-blue-100" }
				}
			}

			p { "{range_1().get(0).unwrap_or(&0.0_f32)}" }

			p { "{range_3().get(0).unwrap_or(&0.0_f32)} - {range_3().get(1).unwrap_or(&0.0_f32)}" }
		}
	}
}
