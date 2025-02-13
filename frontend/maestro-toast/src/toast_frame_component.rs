use {
	crate::{toast_code::EToastCode, toast_manager::ToastManager, toast_position::EToastPosition},
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::bs_icons::{BsCheckCircleFill, BsExclamationCircleFill, BsInfoCircleFill, BsXCircleFill},
		Icon,
	},
	tailwind_fuse::tw_join,
};

/// 
/// Added custom classNames "maestro-toast" and "maestro-toast__*" - in input.css they may be used by developers to customize view
///  

#[component]
pub fn ToastFrame(mut manager: Signal<ToastManager>) -> Element {
  let mut bottom_left_ele: Vec<VNode> = vec![];
  let mut bottom_right_ele: Vec<VNode> = vec![];
  let mut top_left_ele: Vec<VNode> = vec![];
  let mut top_right_ele: Vec<VNode> = vec![];

  for (id, item) in manager.read().list.iter() {
    let current_id = *id;
    let (icon_element, color_classes, progress_color) = if let Some(icon) = &item.info.icon {
      match icon {
        EToastCode::Success => (
          rsx! { Icon { width: 16, height: 16, icon: BsCheckCircleFill } },
          "bg-[#3c763d] text-[#dff0d8] border-[#d6e9c6] maestro-toast__success",
          "bg-[#d6e9c6]"
        ),
        EToastCode::Warning => (
          rsx! { Icon { width: 16, height: 16, icon: BsExclamationCircleFill } },
          "bg-[#8a6d3b] text-[#fcf8e3] border-[#faebcc] maestro-toast__warning",
          "bg-[#faebcc]"
        ),
        EToastCode::Error => (
          rsx! { Icon { width: 16, height: 16, icon: BsXCircleFill } },
          "bg-[#a94442] text-[#f2dede] border-[#ebccd1] maestro-toast__error",
          "bg-[#ebccd1]"
        ),
        EToastCode::Info => (
          rsx! { Icon { width: 16, height: 16, icon: BsInfoCircleFill } },
          "bg-[#31708f] text-[#d9edf7] border-[#bce8f1] maestro-toast__info",
          "bg-[#bce8f1]"
        ),
      }
    } else {
      (rsx! {}, "bg-gray-700 text-white maestro-toast__default", "bg-gray-500")
    };

    // progress width based on remaining time
    let now = chrono::Local::now().timestamp();
    let total_duration = item.hide_after - item.hide_after;
    let remaining_time = item.hide_after - now;
    let progress_width = (remaining_time as f64 / total_duration as f64 * 100.0).max(0.0);

    let element = rsx! {
      div {
        class: "inline-flex flex-col min-w-[200px] max-w-sm p-2 mb-1 rounded font-sans text-xs leading-[17px] relative pointer-events-auto shadow-lg maestro-toast {color_classes}",
        id: "{id}",
        div {
          class: "flex items-center gap-1.5 mb-1 w-full",
          div {
            class: tw_join!("flex-shrink-0", &item.info.icon.is_none().then_some("hidden")),
            {icon_element}
          }
          if let Some(v) = &item.info.heading {
            h2 {
              class: "font-sans text-sm font-medium bg-none inherit leading-inherit flex-grow",
              "{v}"
            }
          }
          if item.info.allow_toast_close {
            div {
              class: "flex-shrink-0 text-sm cursor-pointer ml-2 hover:opacity-80",
              onclick: move |_| {
                manager.write().list.remove(&current_id);
              },
              "×"
            }
          }
        }
        div {
          class: "text-xs leading-tight whitespace-normal break-words",
          dangerous_inner_html: "{item.info.context}"
        }
        // progress bar
        div {
          class: "absolute bottom-0 left-0 h-0.5 {progress_color} transition-all duration-100",
          style: "width: {progress_width}%"
        }
      }
    };

    match item.info.position {
      EToastPosition::BottomLeft => bottom_left_ele.push(element?),
      EToastPosition::BottomRight => bottom_right_ele.push(element?),
      EToastPosition::TopLeft => top_left_ele.push(element?),
      EToastPosition::TopRight => top_right_ele.push(element?),
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
    div {
      class: "toast-scope",
      div {
        class: "fixed pointer-events-none m-0 p-0 z-[9000] bottom-5 left-5 flex flex-col items-start",
        id: "wrap-bottom-left",
        {bottom_left_ele.into_iter()}
      }
      div {
        class: "fixed pointer-events-none m-0 p-0 z-[9000] bottom-5 right-5 flex flex-col items-end",
        id: "wrap-bottom-right",
        {bottom_right_ele.into_iter()}
      }
      div {
        class: "fixed pointer-events-none m-0 p-0 z-[9000] top-5 left-5 flex flex-col items-start",
        id: "wrap-top-left",
        {top_left_ele.into_iter()}
      }
      div {
        class: "fixed pointer-events-none m-0 p-0 z-[9000] top-5 right-5 flex flex-col items-end",
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
