use {
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::io_icons::{IoCheckmarkOutline, IoChevronDownOutline},
		Icon,
	},
	tailwind_fuse::tw_join,
};

#[component]
pub fn MultiSelect<T: std::clone::Clone + std::cmp::PartialEq + std::fmt::Display + 'static>(
	values: Vec<T>, current_value: Vec<T>, callback: EventHandler<T>, label: Option<String>, placeholder: Option<String>,
) -> Element {
	let mut is_opened = use_signal(|| false);

	rsx! {
		div { class: "flex flex-col gap-2 w-full",
			if let Some(label) = label {
				span { class: "text-gray-400", "{label}" }
			}
			div {
				class: tw_join!(
          "relative w-full bg-gray-800 text-gray-100 border border-gray-500 rounded-md hover:border-indigo-300 transition-colors ease-linear",
          is_opened().then_some("ring-1 ring-indigo-500")
				),
				tabindex: -1,
				button {
					class: "relative flex bg-gray-800 text-gray-100 py-2 px-3 w-full h-full rounded-md focus:outline-none focus:ring-1 focus:ring-indigo-500",
					onfocusout: move |_| is_opened.set(false),
					onmousedown: move |ev| {
            ev.prevent_default();
            ev.stop_propagation();
            is_opened.toggle();
					},
					if let Some(placeholder) = placeholder {
						if current_value.is_empty() {
							"{placeholder}"
						} else {
							{format!("{} items selected", current_value.len())}
						}
					}
					Icon {
						width: 16,
						height: 16,
						icon: IoChevronDownOutline,
						class: tw_join!(
              "absolute top-0 bottom-0 my-auto right-3 transition-all ease-linear fill-none",
              is_opened().then_some("rotate-180")
						),
					}
				}
				div {
					class: tw_join!(
            "absolute flex flex-col gap-1 bg-gray-800 text-gray-200 p-4 w-full left-0 right-0 top-[100%] mt-3 rounded-md border border-gray-700",
            if is_opened() { "flex z-40" } else { "hidden -z-40" }
					),
					{values.iter().map(|value| {
            let value_clone = value.clone();
            rsx! {
              div {
                key: "{value}",
                id: "{value}",
                class: "flex w-full items-center py-2 hover:bg-gray-700 rounded px-3 cursor-pointer",
                onclick: move |_| {
                  is_opened.set(false);
                  callback.call(value_clone.clone());
                },
                "{value}"
                Icon {
                  width: 16,
                  height: 16,
                  icon: IoCheckmarkOutline,
                  class: tw_join!(
                    "fill-none ml-auto",
                    if current_value.contains(&value_clone) { "opacity-100" } else { "opacity-0" }
                  ),
                }
              }
            }
        })}
        }
      }
    }
  }
}
