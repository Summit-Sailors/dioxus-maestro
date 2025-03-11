use {
	crate::components::ui::{component_section::ComponentSection, features::Features},
	async_std::task::sleep,
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::{
			fa_solid_icons::{FaCopy, FaFaceSmile},
			ld_icons::LdX,
		},
		Icon,
	},
	maestro_headless::{
		button::Button,
		dialog::{Dialog, DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger},
		select::{Select, SelectDropdown, SelectOption, SelectTrigger, SelectValue},
	},
	maestro_toast::{ctx::use_toast, toast_info::ToastInfo, toast_position::EToastPosition},
	std::time::Duration,
};

#[component]
pub fn HeadlessDemo() -> Element {
	let mut toast = use_toast();
	let mut disabled = use_signal(|| false);
	let mut pending = use_signal(|| false);
	let mut popup_open = use_signal(|| false);
	let options = Vec::from([
		SelectOption { value: 1, label: "Apple".into(), disabled: false },
		SelectOption { value: 2, label: "Banana".into(), disabled: false },
		SelectOption { value: 3, label: "Ice-Cream".into(), disabled: false },
		SelectOption { value: 4, label: "Coffee".into(), disabled: false },
		SelectOption { value: 5, label: "Salt".into(), disabled: true },
		SelectOption { value: 6, label: "Chocolate".into(), disabled: false },
	]);
	let mut selected = use_signal::<Option<i32>>(|| None);

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
					DialogPortal {
						DialogOverlay { class: "w-full h-full fixed top-0 left-0 bottom-0 right-0 bg-slate-900/20 inset-0 backdrop-blur-sm z-[100]" }
						DialogContent { class: "w-full h-96 max-w-lg max-h-[95vh] fixed z-[110] left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rounded bg-slate-100 shadow border border-slate-600 flex flex-col gap-6 px-6 py-8",
							div { class: "flex justify-between gap-4",
								DialogTitle { class: "font-medium text-2xl text-slate-900",
									"Uncontrolled dialog"
								}
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
				}

				Dialog { open: popup_open,
					// on_open_change: move |_| popup_open.toggle(),
					DialogTrigger { class: "rounded-full w-fit px-3 py-2 h-12 focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors bg-orange-600 border border-slate-200 text-slate-100 hover:bg-orange-800  focus-visible:ring-orange-200 focus-visible:ring-offset-black focus-visible:bg-orange-800 aria-[disabled=true]:opacity-50 aria-[disabled=true]:pointer-events-none data-[pending=true]:bg-slate-500",
						"Open Dialog"
					}
					DialogPortal {
						DialogOverlay { class: "w-full h-full fixed top-0 left-0 bottom-0 right-0 z-[100] bg-slate-900/20 inset-0 backdrop-blur-sm" }
						DialogContent { class: "w-full h-96  max-w-lg fixed z-[110] max-h-[95vh] left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rounded bg-slate-100 shadow border border-slate-600 flex flex-col gap-6 px-6 py-8",
							div { class: "flex justify-between gap-4",
								DialogTitle { class: "font-medium text-2xl text-slate-900",
									"Controlled dialog"
								}
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
								pending: pending(),
								class: "rounded-full w-fit px-3 py-2 h-12 focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors bg-slate-200 border border-slate-200 text-slate-900 hover:bg-slate-900 hover:text-slate-200 focus-visible:ring-slate-200 focus-visible:ring-offset-black focus-visible:bg-slate-200 focus-visible:text-slate-900 aria-[disabled=true]:opacity-50 aria-[disabled=true]:pointer-events-none data-[pending=true]:bg-slate-500",
								onclick: dialog_close,
								"Close"
							}
						}
					}
				}
			}
		}
		div {
			Select {
				options,
				value: selected,
				is_searchable: true,
				class: "relative w-fit",
				SelectTrigger::<i32> { class: "rounded border border-slate-300 bg-slate-900 text-slate-100 min-w-36 flex justify-between items-center gap-4 px-3 py-2 min-h-12 ",
					SelectValue::<i32> {
						placeholder: "Chose something...",
						class: "data-[state=selected]:text-slate-100 data-[state=placeholder]:text-slate-500",
					}
				}
				SelectDropdown::<i32> {
					class: "absolute top-[100%] mt-2 rounded bg-slate-900 text-slate-200 border border-slate-700 z-10 px-2 py-4 [&_*]:transition-all",
					option_class: "data-[role=option]:flex data-[role=option]:items-center data-[role=option]:justify-between data-[role=option]:gap-4 data-[role=option]:px-2 data-[role=option]:py-3 data-[role=option]:hover:bg-slate-700 data-[role=option]:focus-visible::bg-slate-700 data-[role=search-container]:relative [&>[data-role=search]]:px-6 [&>[data-role=search]]:h-10 [&>[data-role=search]]:text-slate-800 [&>[data-role=search-icon]]:text-slate-500 [&>[data-role=search-icon]]:h-fit [&>[data-role=search-icon]]:m-auto [&>[data-role=search-icon]]:absolute [&>[data-role=search-icon]]:top-0 [&>[data-role=search-icon]]:bottom-0 [&>[data-role=search-icon]]:left-1 [&_[aria-hidden=true]]:opacity-0 [&_[data-role=clear]]:absolute [&_[data-role=clear]]:top-0 [&_[data-role=clear]]:bottom-0 [&_[data-role=clear]]:right-1 ",
				}
			}
		}
	}
}
