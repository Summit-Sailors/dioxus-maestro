use {
	crate::{toast_code::EToastCode, toast_manager::ToastManager, toast_position::EToastPosition},
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::bs_icons::{BsCheckCircleFill, BsExclamationCircleFill, BsInfoCircleFill, BsXCircleFill},
		Icon,
	},
};

#[component]
pub fn ToastFrame(mut manager: Signal<ToastManager>) -> Element {
	let mut bottom_left_ele: Vec<VNode> = vec![];
	let mut bottom_right_ele: Vec<VNode> = vec![];
	let mut top_left_ele: Vec<VNode> = vec![];
	let mut top_right_ele: Vec<VNode> = vec![];
	for (id, item) in manager.read().list.iter() {
		let current_id = *id;
		let (icon_element, color_classes) = if let Some(icon) = &item.info.icon {
			match icon {
				EToastCode::Success => (
					rsx! {
						Icon { width: 20, height: 20, icon: BsCheckCircleFill }
					},
					"bg-[#3c763d] text-[#dff0d8] border-[#d6e9c6]",
				),
				EToastCode::Warning => (
					rsx! {
						Icon {
							width: 20,
							height: 20,
							icon: BsExclamationCircleFill,
						}
					},
					"bg-[#8a6d3b] text-[#fcf8e3] border-[#faebcc]",
				),
				EToastCode::Error => (
					rsx! {
						Icon { width: 20, height: 20, icon: BsXCircleFill }
					},
					"bg-[#a94442] text-[#f2dede] border-[#ebccd1]",
				),
				EToastCode::Info => (
					rsx! {
						Icon { width: 20, height: 20, icon: BsInfoCircleFill }
					},
					"bg-[#31708f] text-[#d9edf7] border-[#bce8f1]",
				),
			}
		} else {
			(rsx! {}, "bg-gray-700 text-white")
		};
		let element = rsx! {
			div {
				class: "block w-full p-2.5 mb-1 rounded font-sans text-xs leading-[17px] relative pointer-events-auto bg-gray-700 text-white {color_classes}",
				id: "{id}",
				div { class: "absolute top-1 left-2", {icon_element} }
				if item.info.allow_toast_close {
					div {
						class: "absolute top-1 right-2 text-sm cursor-pointer",
						onclick: move |_| {
								manager.write().list.remove(&current_id);
						},
						"×"
					}
				} else {
					div {}
				}
				if let Some(v) = &item.info.heading {
					h2 { class: "font-sans text-sm mb-2 bg-none inherit leading-inherit",
						"{v}"
					}
				} else {
					div {}
				}
				span { dangerous_inner_html: "{item.info.context}" }
			}
		};
		if item.info.position == EToastPosition::BottomLeft {
			bottom_left_ele.push(element?);
		} else if item.info.position == EToastPosition::BottomRight {
			bottom_right_ele.push(element?);
		} else if item.info.position == EToastPosition::TopLeft {
			top_left_ele.push(element?);
		} else if item.info.position == EToastPosition::TopRight {
			top_right_ele.push(element?);
		}
	}
	let _ = use_resource(move || async move {
		loop {
			let now = chrono::Local::now().timestamp();
			manager.write().list.retain(|_, item| now < item.hide_after);
			time_sleep(100).await;
		}
	});

	rsx! {
		div { class: "toast-scope",
			div {
				class: "block fixed w-64 pointer-events-none m-0 p-0 z-[9000] bottom-5 left-5",
				id: "wrap-bottom-left",
				{bottom_left_ele.into_iter()}
			}
			div {
				class: "block fixed w-64 pointer-events-none m-0 p-0 z-[9000] bottom-5 right-5",
				id: "wrap-bottom-right",
				{bottom_right_ele.into_iter()}
			}
			div {
				class: "block fixed w-64 pointer-events-none m-0 p-0 z-[9000] top-5 left-5",
				id: "wrap-top-left",
				{top_left_ele.into_iter()}
			}
			div {
				class: "block fixed w-64 pointer-events-none m-0 p-0 z-[9000] top-5 right-5",
				id: "wrap-top-right",
				{top_right_ele.into_iter()}
			}
		}
	}
}

async fn time_sleep(interval: usize) {
	#[cfg(feature = "web")]
	gloo_timers::future::TimeoutFuture::new(interval as u32).await;

	#[cfg(feature = "desktop")]
	tokio::time::sleep(tokio::time::Duration::from_millis(interval as u64)).await;
}
