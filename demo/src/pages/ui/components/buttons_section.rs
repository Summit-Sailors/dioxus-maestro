use {
	crate::components::ui::component_section::ComponentSection,
	async_std::task::sleep,
	dioxus::prelude::*,
	dioxus_free_icons::{icons::fa_solid_icons::FaCopy, Icon},
	// maestro_ui::button::Button,
	maestro_headless::button::Button,
	maestro_toast::{ctx::use_toast, toast_info::ToastInfo, toast_position::EToastPosition},
	std::time::Duration,
};
#[component]
pub fn ButtonsSection() -> Element {
	let mut toast = use_toast();
	let mut disabled = use_signal(|| false);
	let mut pending = use_signal(|| false);

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

	let handle_button_click_disable = move |_| {
		spawn(async move {
			disabled.set(true);
			let info = ToastInfo {
				heading: Some("Button Click Handler".to_string()),
				context: "All buttons are disabled for a one minute".into(),
				icon: None,
				position: EToastPosition::TopRight,
				allow_toast_close: true,
				hide_after: 5,
			};
			toast.write().popup(info);
			sleep(Duration::from_secs(1)).await;
			disabled.set(false);
		});
	};

	let handle_button_click_pending = move |_| {
		spawn(async move {
			pending.set(true);
			let info = ToastInfo {
				heading: Some("Button Click Handler".to_string()),
				context: "Simulates toggle pending state".into(),
				icon: None,
				position: EToastPosition::TopRight,
				allow_toast_close: true,
				hide_after: 5,
			};
			toast.write().popup(info);
			sleep(Duration::from_secs(3)).await;
			pending.set(false);
		});
	};

	rsx! {
		ComponentSection {
			title: "Buttons",
			description: "Various button styles, sizes, and types with different variants, callbacks and states",
			div {
				id: "maestro-ui-buttons",
				class: "grid grid-cols-1 md:grid-cols-3 gap-6 md:w-4/5 mx-auto w-full",
				Button {
					r#type: "button",
					disabled: disabled(),
					onclick: move |_| handle_button_click("Filled button click!".to_string()),
					class: "flex items-center justify-center rounded-md px-3 py-2 h-10 transition-colors ease-linear focus-visible:outline-none text-slate-100 bg-indigo-500 hover:bg-indigo-700 focus-visible:bg-indigo-700 focus-visible:ring-1 focus-visible:ring-indigo-700 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900 disabled:opacity-50 disabled:pointer-events-none",
					"Filled"
				}
				Button {
					class: "flex items-center justify-center rounded-md px-3 py-2 h-10 transition-colors ease-linear focus-visible:outline-none text-slate-100 border border-slate-300 hover:border-slate-100 focus-visible:border-slate-100 focus-visible:ring-1 focus-visible:ring-slate-100 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900 disabled:opacity-50 disabled:pointer-events-none",
					r#type: "button",
					disabled: disabled(),
					onclick: move |_| handle_button_click("Outline Button clicked!".to_string()),
					"Outline"
				}
				Button {
					class: "flex items-center justify-center rounded-md px-3 py-2 h-10 transition-colors ease-linear focus-visible:outline-none text-slate-100 hover:bg-slate-800 focus-visible:bg-slate-800 focus-visible:ring-1 focus-visible:ring-slate-100 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900 disabled:opacity-50 disabled:pointer-events-none",
					r#type: "reset",
					disabled: disabled(),
					onclick: move |_| handle_button_click("Ghost Button clicked!".to_string()),
					"Ghost"
				}
				Button {
					class: "flex items-center justify-center rounded-md px-3 py-2 h-10 transition-colors ease-linear focus-visible:outline-none text-slate-100 bg-indigo-500 hover:bg-indigo-700 focus-visible:bg-indigo-700 focus-visible:ring-1 focus-visible:ring-indigo-700 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900 disabled:opacity-50 disabled:pointer-events-none",
					r#type: "button",
					disabled: true,
					onclick: move |_| handle_button_click("Never be".to_string()),
					"Filled Disabled"
				}
				Button {
					r#type: "button",
					disabled: disabled(),
					onclick: handle_button_click_disable,
					class: "flex items-center justify-center whitespace-nowrap rounded-md px-3 py-2 h-10 transition-colors ease-linear focus-visible:outline-none text-slate-900 bg-gray-100 border border-gray-100 hover:bg-transparent hover:text-slate-100 focus-visible:bg-transparent focus-visible:text-slate-100 focus-visible:ring-1 focus-visible:ring-gray-100 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900 disabled:opacity-50 disabled:pointer-events-none",
					"Disable All"
				}
				Button {
					r#type: "button",
					pending: pending(),
					onclick: handle_button_click_pending,
					class: "flex items-center justify-center whitespace-nowrap rounded-md px-3 py-2 h-10 transition-colors ease-linear focus-visible:outline-none text-slate-100 bg-teal-600 hover:bg-teal-700 focus-visible:bg-teal-700 focus-visible:ring-1 focus-visible:ring-teal-700 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900 data-[pending=true]:opacity-50 data-[pending=true]:pointer-events-none",
					"Toggle pending"
				}
				Button {
					class: "flex items-center justify-center rounded-full bg-slate-300 border border-slate-300 hover:bg-slate-600 hover:text-slate-50 h-10 w-10 text-slate-900 transition-colors ease-linear focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-slate-100 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900 disabled:opacity-50 disabled:pointer-events-none",
					r#type: "button",
					onclick: move |_| handle_button_click("Icon Button clicked!".to_string()),
					children: rsx! {
						Icon {
							title: "Icon Button",
							icon: FaCopy,
							width: 16,
							height: 16,
							class: "",
						}
					},
				}
			}
		}
	}
}
