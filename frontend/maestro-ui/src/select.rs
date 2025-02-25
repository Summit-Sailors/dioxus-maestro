use {
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::io_icons::{IoCheckmarkOutline, IoChevronDownOutline},
		Icon,
	},
	tailwind_fuse::*,
};

#[derive(Clone, PartialEq)]
pub struct SelectOption<T> {
	pub label: String,
	pub value: T,
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T>
where
	T: Clone + PartialEq + std::fmt::Display + 'static,
{
	pub options: Vec<SelectOption<T>>,
	pub current_value: Option<T>,
	pub multi: bool,
	pub callback: Option<EventHandler<T>>,
	pub multi_callback: Option<EventHandler<Vec<T>>>,
	pub label: Option<String>,
	pub placeholder: Option<String>,
	pub option_class: Option<String>,
	pub dropdown_class: Option<String>,
	pub container_class: Option<String>,
	pub button_class: Option<String>,
	pub button_wrapper_class: Option<String>,
	pub label_class: Option<String>,
	pub icon_down: Option<Element>,
	pub icon_check: Option<Element>,
	pub option_renderer: Option<fn(&SelectOption<T>) -> Element>,
	pub multi_current_values: Option<Vec<T>>,
	pub toggle_open_callback: Option<EventHandler<Event<MouseData>>>,
	pub toggle_close_callback: Option<EventHandler<Event<FocusData>>>,
}

// classes may be extended also by using "maestro-select-*_*" classname

#[component]
pub fn Select<T: Clone + PartialEq + std::fmt::Display + 'static>(props: SelectProps<T>) -> Element {
	let mut is_opened = use_signal(|| false);
	let mut selected_options = use_signal(|| props.multi_current_values.clone().unwrap_or_default());

	let display_value = if props.multi {
		if selected_options().is_empty() {
			props.placeholder.clone().unwrap_or_default()
		} else {
			let items: Vec<String> =
				selected_options().iter().filter_map(|value| props.options.iter().find(|opt| &opt.value == value).map(|opt| opt.label.clone())).collect();

			let joined = items.join(", ");

			if joined.chars().count() > 50 {
				format!("{} (+{} more)", joined.chars().take(47).collect::<String>(), selected_options().len())
			} else {
				joined
			}
		}
	} else {
		props
			.current_value
			.as_ref()
			.and_then(|value| props.options.iter().find(|opt| &opt.value == value).map(|opt| opt.label.clone()))
			.unwrap_or_else(|| props.placeholder.clone().unwrap_or_default())
	};

	let icon_down = props.icon_down.clone().unwrap_or_else(|| {
		rsx! {
			Icon {
				width: 16,
				height: 16,
				icon: IoChevronDownOutline,
				class: tw_join!(
						"absolute top-0 bottom-0 my-auto right-3 transition-all ease-linear fill-none",
						(is_opened()).then_some("rotate-180")
				),
			}
		}
	});

	rsx! {
		div {
			class: tw_merge!(
					"flex flex-col gap-2 w-full relative ", props.container_class.clone()
					.unwrap_or_default()
			),
			if let Some(label) = props.label.clone() {
				span { class: tw_merge!("text-gray-400", props.label_class.clone().unwrap_or_default()),
					{label}
				}
			}
			div {
				class: tw_merge!(
						"relative w-full border border-gray-500 rounded-md transition-colors ease-linear cursor-pointer maestro-select-button__wrapper",
						props.button_wrapper_class.clone().unwrap_or_default()
				),
				tabindex: -1,
				button {
					class: tw_merge!(
							"text-gray-500 relative flex py-2 px-3 w-full h-full rounded-md focus:outline-none maestro-select-button",
							props.button_class.clone().unwrap_or_default()
					),
					onfocusout: move |ev| {
							is_opened.set(false);
							props.toggle_close_callback.unwrap_or_default().call(ev);
					},
					onmousedown: move |ev| {
							ev.prevent_default();
							ev.stop_propagation();
							is_opened.toggle();
							props.toggle_open_callback.unwrap_or_default().call(ev)
					},
					r#type: "button",
					span { "{display_value}" }
					{icon_down}
				}
				div {
					class: tw_join!(
							"absolute flex flex-col gap-1 p-4 w-full left-0 right-0 top-[100%] mt-3 rounded-md max-h-48 overflow-y-auto bg-gray-100 border maestro-select-dropdown",
							if is_opened() { "flex z-40" } else { "hidden -z-40" }, props.dropdown_class
							.clone().unwrap_or_default()
					),
					onclick: move |ev| {
							ev.stop_propagation();
					},
					{
            props.options.iter().map(|option| {
              let option_clone = option.clone();
              rsx! {
                div {
                  key: "{option.value}",
                  id: "{option.value}",
                  class: tw_join!(
                    "flex w-full items-center py-2 hover:bg-gray-700 rounded px-3 cursor-pointer",
                    props.option_class.clone().unwrap_or_default()
                  ),
                  onclick: move |ev| {
                    ev.stop_propagation();
                    if props.multi {
                      let mut current = selected_options().clone();
                      if current.contains(&option_clone.value) {
                        current.retain(|x| x != &option_clone.value);
                      } else {
                        current.push(option_clone.value.clone());
                      }
                      selected_options.set(current.clone());
                      if let Some(multi_cb) = props.multi_callback.clone() {
                        multi_cb.call(current);
                      }
                    } else {
                      is_opened.set(false);
                      if let Some(callback) = props.callback.clone() {
                        callback.call(option_clone.value.clone());
                      }
                    }
                  },
                  {
                    if let Some(renderer) = props.option_renderer {
                      renderer(&option)
                    } else {
                      rsx! { "{option.label}" }
                    }
                  }
                  if props.icon_check.is_some() {
                    {props.icon_check.clone().unwrap()}
                  } else {
                    Icon {
                      icon: IoCheckmarkOutline,
                      class: tw_join!(
                        "fill-none ml-auto", if props.multi && selected_options().contains(& option
                        .value) || ! props.multi && props.current_value.as_ref() == Some(& option.value)
                        { "opacity-100" } else { "opacity-0" }
                      ),
                    }
                  }
                }
              }
            })
          }
        }
			}
		}
	}
}
