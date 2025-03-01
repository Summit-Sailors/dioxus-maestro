use {
	crate::{
		button::{Button, ButtonSize, ButtonVariant},
		input::{Input, InputVariant},
		select::SelectOption,
	},
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::{
			bs_icons::BsSearch,
			io_icons::{IoCheckmarkOutline, IoChevronDownOutline},
			ld_icons::LdX,
		},
		Icon,
	},
	std::cmp::Ordering,
	tailwind_fuse::*,
};

#[derive(Clone, PartialEq, Props)]
pub struct MultiSelectProps<T>
where
	T: Clone + PartialEq + std::fmt::Display + 'static,
{
	pub options: Vec<SelectOption<T>>,
	#[props(default = None)]
	pub current_value: Option<Vec<T>>,
	pub onchange: Option<EventHandler<Vec<T>>>,
	#[props(default = String::new())]
	pub label: String,
	#[props(default = String::new())]
	pub placeholder: String,
	#[props(default = String::new())]
	pub placeholder_class: String,
	#[props(default = String::new())]
	pub option_class: String,
	#[props(default = String::new())]
	pub dropdown_class: String,
	#[props(default = String::new())]
	pub container_class: String,
	#[props(default = String::new())]
	pub button_class: String,
	#[props(default = String::new())]
	pub button_wrapper_class: String,
	#[props(default = String::new())]
	pub label_class: String,
	#[props(default = String::new())]
	pub search_input_container_class: String,
	#[props(default = String::new())]
	pub search_clear_class: String,
	#[props(default = String::new())]
	pub search_input_class: String,
	pub icon_down: Option<Element>,
	pub icon_check: Option<Element>,
	pub option_renderer: Option<fn(&SelectOption<T>) -> Element>,
	pub onopen: Option<EventHandler<Option<Event<MouseData>>>>,
	pub onclose: Option<EventHandler<Option<Event<FocusData>>>>,
	#[props(default = false)]
	pub is_searchable: bool,
}

// classes may be extended also by using "maestro-multi-select-*_*" classname

#[component]
pub fn MultiSelect<T: Clone + PartialEq + std::fmt::Display + 'static>(props: MultiSelectProps<T>) -> Element {
	let mut is_opened = use_signal(|| false);
	let mut selected_options = use_signal(|| props.current_value.clone().unwrap_or_default());
	let mut search_input = use_signal(String::new);

	let MultiSelectProps { options, current_value, placeholder, ref placeholder_class,  .. } = props;

	let display_options = use_memo(use_reactive!(|(current_value)| {
		let mut options =
			options.clone().into_iter().filter(|option| option.label.to_lowercase().contains(&search_input().to_lowercase())).collect::<Vec<SelectOption<T>>>();
		options.sort_by(|a, b| {
			if let Some(current_value) = &current_value {
				let a_in_current = current_value.contains(&a.value);
				let b_in_current = current_value.contains(&b.value);

				if a_in_current && !b_in_current {
					Ordering::Less
				} else if !a_in_current && b_in_current {
					Ordering::Greater
				} else {
					Ordering::Equal
				}
			} else {
				Ordering::Equal
			}
		});
		options
	}));

	let display_value = if selected_options().is_empty() {
		placeholder.clone()
	} else {
		selected_options()
			.iter()
			.filter_map(|value| display_options.iter().find(|opt| &opt.value == value).map(|opt| opt.label.clone()))
			.collect::<Vec<String>>()
			.join(", ")
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
		div { class: tw_merge!("flex flex-col gap-2 w-full relative", & props.container_class),
			if !props.label.is_empty() {
				span { class: tw_merge!("text-gray-400", & props.label_class), {props.label} }
			}
			div {
				class: tw_merge!(
          "relative w-full cursor-pointer maestro-multi-select-button__wrapper", & props
          .button_wrapper_class
				),
				button {
					class: tw_merge!(
            "min-h-12 border items-center border-gray-500 rounded-md transition-colors ease-linear text-gray-800 relative flex py-2 px-3 w-full h-full focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-gray-700 focus-visible:ring-offset-white maestro-multi-select-button",
            & props.button_class
					),
					onfocusout: move |ev| {
            is_opened.set(false);
            search_input.set(String::new());
            props.onclose.unwrap_or_default().call(Some(ev));
					},
					onmousedown: move |ev| {
            ev.prevent_default();
            ev.stop_propagation();
            is_opened.toggle();
            props.onopen.unwrap_or_default().call(Some(ev))
					},
					r#type: "button",
					span {
						class: tw_merge!(
							"line-clamp-1 pr-2 text-left",
							if display_value == placeholder { placeholder_class } else { "" }
						),
						"{display_value}"
					}
					{icon_down}
				}
				div {
					class: tw_merge!(
            "absolute flex flex-col gap-1 p-4 w-full left-0 right-0 top-[100%] mt-3 rounded-md max-h-48 overflow-y-auto bg-gray-100 border maestro-multi-select-dropdown",
            if is_opened() { "flex z-40" } else { "hidden -z-40" }, & props.dropdown_class
					),
					onclick: move |ev| {
						ev.stop_propagation();
					},
					if props.is_searchable {
						div {
							class: tw_merge!(
                "relative px-3 text-gray-500 maestro-multi-select-search_container", & props
                .search_input_container_class
							),
							Icon {
								width: 16,
								icon: BsSearch,
								class: "absolute top-2.5 left-3",
							}
							Input {
								r#type: "text",
								variant: InputVariant::Underlined,
								value: search_input(),
								class: tw_merge!(
                  "px-7 focus-visible:ring-0 focus-visible:ring-transparent focus-visible:ring-none focus-visible:ring-offset-0 maestro-multi-select-search_input",
                  & props.search_input_class
								),
								onchange: move |event: Event<FormData>| search_input.set(event.value()),
							}
							Button {
								variant: ButtonVariant::Icon,
								r#type: "button",
								size: ButtonSize::IconSm,
								class: tw_merge!(
                  "h-fit w-fit absolute top-2.5 right-3 text-gray-500 hover:text-gray-700 focus-visible:text-gray-700 focus-visible:ring-0 focus-visible:ring-transparent focus-visible:ring-none focus-visible:ring-offset-0 maestro-multi-select-search_clear",
                  & props.search_clear_class
								),
								onclick: move |event: Event<MouseData>| {
                  event.stop_propagation();
                  search_input.set(String::new());
								},
								Icon { width: 16, icon: LdX }
							}
						}
					}
					{
            display_options()
              .iter()
              .map(|option| {
                let option_clone = option.clone();
                rsx! {
                  div {
                    key: "{option.value}",
                    id: "{option.value}",
                    class: tw_merge!(
                      "flex w-full items-center py-2 hover:bg-gray-300 rounded px-3 cursor-pointer", &
                      props.option_class
                    ),
                    onclick: move |ev| {
                      ev.stop_propagation();
                      let mut current = selected_options().clone();
                      if current.contains(&option_clone.value) {
                        current.retain(|x| x != &option_clone.value);
                      } else {
                        current.push(option_clone.value.clone());
                      }
                      selected_options.set(current.clone());
                      search_input.set(String::new());
                      props.onchange.unwrap_or_default().call(current.clone());
                    },
                    {
                      if let Some(renderer) = props.option_renderer {
                          renderer(option)
                      } else {
                        rsx! {
                          "{option.label}"
                        }
                      }
                    }
                    if props.icon_check.is_some() {
                      {props.icon_check.clone().unwrap()}
                    } else {
                      Icon {
                        icon: IoCheckmarkOutline,
                        class: tw_merge!(
                          "fill-none ml-auto", if selected_options().contains(& option.value) {
                          "opacity-100" } else { "opacity-0" }
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
